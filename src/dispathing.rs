pub mod markup;
pub mod page_message;

use std::sync::Arc;

use teloxide::{dispatching::{Dispatcher, UpdateFilterExt}, dptree, payloads::{EditMessageTextSetters, SendMessageSetters}, requests::Requester, types::{CallbackQuery, Message, ParseMode, Update}, Bot};

use crate::{handle_request, prelude::Page};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn message_handler(bot: Bot, msg: Message, pages: Arc<Vec<Page>>) -> HandlerResult {
    handle_request!(
        bot.send_message(msg.chat.id, "1"),
        pages,
        0
    ).await?;
        
    Ok(())
}

pub async fn callback_query_handler(bot: Bot, callback: CallbackQuery, pages: Arc<Vec<Page>>) -> HandlerResult {
    let data = callback.data.unwrap();
    let msg = callback.message.unwrap();

    match data.parse::<usize>() {
        Ok(page_index) => {
            handle_request!(
                bot.edit_message_text(msg.chat.id, msg.id, "1"),
                pages,
                page_index
            ).await?;
        },
        Err(e) => {}
    }

    bot.answer_callback_query(callback.id).await?;
    Ok(())
}

pub struct TelegramBot(Bot);
impl TelegramBot {
    pub fn new(token: String) -> TelegramBot {
        TelegramBot(Bot::new(token))
    }

    pub fn from_env() -> TelegramBot {
        TelegramBot(Bot::from_env())
    }

    pub async fn repl(self, pages: Vec<Page>) {
        Dispatcher::builder(
            self.0,
            dptree::entry()
                .branch(Update::filter_message().endpoint(message_handler))
                .branch(Update::filter_callback_query().endpoint(callback_query_handler))
        )
            .dependencies(dptree::deps![
                Arc::new(pages)
            ])
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }
}
