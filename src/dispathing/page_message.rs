use teloxide::{payloads::{EditMessageText, EditMessageTextSetters, SendMessage, SendMessageSetters}, requests::{self, JsonRequest, Requester}, types::{CallbackQuery, Message}, Bot};

use crate::dispathing::markup;

#[macro_export]
macro_rules! handle_request {
    ($request:expr, $pages:expr, $page_index:expr) => {
        {
            let page = $pages.get($page_index).unwrap();

            $request
                .text(page.text.clone().unwrap())
                .reply_markup(markup::markup($page_index, $pages.len()))
        }
    };
}