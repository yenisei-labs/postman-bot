use teloxide::{prelude::*, utils::command::BotCommands, update_listeners::webhooks};
use std::env;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot = Bot::from_env();

    match env::var("WEBHOOK_ADDR") {
        // Webhook.
        Ok(value) => {
            // Generate an address for the server.
            let port: u16 = env::var("WEBHOOK_PORT")
                .unwrap_or(String::from("8080"))
                .parse()
                .expect("Port is not an integer");
            let addr = ([0, 0, 0, 0], port).into();

            // Url that will be sent to Telegram.
            let url = value.parse().expect("Couldn't parse webhook url");

            let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
                .await
                .expect("Couldn't setup webhook");

            Command::repl_with_listener(bot, answer, listener).await;
        },
        // Long pooling.
        Err(_) => {
            Command::repl(bot, answer).await;
        }
    }
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "I know the following commands:")]
enum Command {
    #[command(description = "show chat ID.")]
    ID,
    #[command(description = "show this message.")]
    Help,
    Start,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help | Command::Start => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?
        },
        Command::ID => {
            bot.send_message(msg.chat.id, msg.chat.id.to_string()).await?
        },
    };

    Ok(())
}
