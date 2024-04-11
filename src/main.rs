use std::io::Cursor;

use adaptemoji::AdaptiveEmojiConvert;
use image::imageops::FilterType;
use image::{EncodableLayout, ImageFormat, ImageResult};
use teloxide::prelude::*;
use teloxide::types::{Document, InputFile, InputMedia, InputMediaDocument};
use tokio::io;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let document = match msg.document() {
            Some(document) => document,
            None => return Ok(()),
        };

        if let Err(err) = convert(&bot, &msg, document).await {
            bot.send_message(msg.chat.id, err.to_string()).await?;
        };

        Ok(())
    })
    .await;
}

async fn convert(bot: &Bot, msg: &Message, document: &Document) -> anyhow::Result<Vec<Message>> {
    let file = bot.get_file(document.file.id.to_owned()).await?;
    let bytes = reqwest::get(format!(
        "https://api.telegram.org/file/bot{}/{}",
        bot.token(),
        file.path
    ))
    .await?
    .bytes()
    .await?;

    let prepared_img = image::load_from_memory(&bytes)?
        .resize(100, 100, FilterType::Triangle)
        .to_luma_alpha8();

    let file_name = document
        .file_name
        .to_owned()
        .ok_or(io::Error::from(io::ErrorKind::NotFound))?;

    let res_images = [false, true]
        .iter()
        .map(|inverted| -> ImageResult<InputMedia> {
            let img = prepared_img.convert_adaptive(*inverted);
            let mut cursor = Cursor::new(Vec::new());

            img.write_to(&mut cursor, ImageFormat::Png)?;
            
            let input_file = InputFile::memory(img.as_bytes()).file_name(file_name.to_owned());

            let input_media_document = InputMediaDocument::new(input_file);
            Ok(InputMedia::Document(input_media_document))
        })
        .collect::<ImageResult<Vec<InputMedia>>>()?;

    let res_msg = bot.send_media_group(msg.chat.id, res_images).await?;

    Ok(res_msg)
}
