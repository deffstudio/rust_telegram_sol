use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::keypair::read_keypair_file;
use solana_sdk::signer::Signer;
use std::sync::Arc;
use teloxide::Bot;
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting telegram bot...");

    let rpc_client = Arc::new(RpcClient::new_with_commitment(
        "http://localhost:8899",
        CommitmentConfig::confirmed(),
    ));

    let keypair = Arc::new(
        read_keypair_file("/root/.config/solana/id.json").expect("Failed to read keypair"),
    );

    dotenv().ok();
    let bot = Bot::from_env();

    Command::repl(bot.clone(), move |message: Message, command| {
        let rpc_client = rpc_client.clone();
        let keypair = keypair.clone();
        let bot = bot.clone();
        async move { answer(bot, message, command, &rpc_client, &keypair).await }
    })
    .await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this help message.")]
    Help,
    #[command(description = "get your SOL balance.")]
    Balance,
}

async fn answer(
    bot: Bot,
    msg: Message,
    cmd: Command,
    rpc_client: &RpcClient,
    keypair: &Keypair,
) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Balance => {
            let balance = rpc_client.get_balance(&keypair.pubkey());
            match balance {
                Ok(balance) => {
                    bot.send_message(msg.chat.id, format!("Your SOL Balance: {}", balance))
                        .await?
                }
                Err(err) => {
                    bot.send_message(msg.chat.id, format!("Error fetching balance: {:?}", err))
                        .await?
                }
            }
        }
    };

    Ok(())
}
