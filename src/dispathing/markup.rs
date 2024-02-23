use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

const BUTTON_BACK: &str = "Back";
const BUTTON_CONTINUE: &str = "Continue";

pub fn markup(page: usize, len: usize) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new([
        if len == 0 {
            vec![]
        } else if page == 0 {
            vec![
                InlineKeyboardButton::callback(BUTTON_CONTINUE, "1")
            ]
        } else if page == (len-1) {
            vec![
                InlineKeyboardButton::callback(BUTTON_BACK, (page-1).to_string())
            ]  
        } else {
            vec![
                InlineKeyboardButton::callback(BUTTON_BACK, (page-1).to_string()),
                InlineKeyboardButton::callback(BUTTON_CONTINUE, (page+1).to_string())
            ]
        }
    ])
}