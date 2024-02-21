use telepages::prelude::*;

extern crate telepages;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");
    let bot = TelegramBot::from_env();
    
    bot.repl(vec![
        Page::builder()
            .text(Some("Start page".into()))
            .build(),
        Page::builder()
            .text(Some("Info page".into()))
            .build(),
        Page::builder()
            .text(Some("End page".into()))
            .build()
    ]).await;
}
