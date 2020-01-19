use std::env;

use futures::StreamExt;
use telegram_bot::*;

#[derive(Debug, PartialEq, Eq)]
enum State {
    Idle,
    Where,
    When,
    WithWhom,
    WhatAgenda,
}

impl State {
    fn next(self) -> Self {
        match self {
            Self::Idle => Self::Where,
            Self::Where => Self::When,
            Self::When => Self::WithWhom,
            Self::WithWhom => Self::WhatAgenda,
            Self::WhatAgenda => Self::Idle,
        }
    }

    fn message(&self) -> &'static str {
       match self {
           Self::Idle => "пшли пшли и они пшли",
           Self::Where => "куда?",
           Self::When => "когда?",
           Self::WithWhom => "с кем?",
           Self::WhatAgenda => "какова повестка?",
       }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);
    let mut stream = api.stream();
    let mut state = State::Idle;

    while let Some(update) = stream.next().await {
        if let UpdateKind::Message(message) = update?.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                if state != State::Idle && (data == "/отмена" || data == "отмена") {
                    state = State::Idle;
                    api.send(message.chat.text("алё отмена! отмена!")).await?;
                } else if state == State::Idle && data == "/пшли" || state != State::Idle {
                    state = state.next();
                    api.send(message.chat.text(state.message())).await?;
                }
            }
        }
    }

    Ok(())
}
