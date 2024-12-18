use clap::{Args, Parser, Subcommand, ValueEnum};
use std::str::FromStr;

/// Send messages to Slack using this CLI tool. It allows you to send various types of messages, including text, blocks, and attachments, to specific Slack channels.
#[derive(Debug, Parser)]
#[command(name = "slack")]
#[command(
    about = "Send messages to Slack\ne.g. slack send --kind text --chanel general \"hello world\"",
    long_about = "This CLI tool enables users to send messages to Slack channels. You can specify the type of message (text, block, or attachment) and provide the necessary details like the channel and message content. It's a convenient way to interact with the Slack API directly from the command line."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Send(SendArgs),
}

#[derive(Debug, Args)]
struct SendArgs {
    /// Slack channel to send the message to. Accepts either the channel name (e.g., 'general') or the channel ID.
    #[arg(short, long)]
    channel: String,

    /// Type of message to send. Valid kinds include:
    /// - `Text`: Plain text message.
    /// - `Block`: A block-based message structure (JSON object).
    /// - `Attachment`: An attachment message (JSON object).
    #[arg(short, long)]
    kind: MessageKind,

    /// The content of the message to send. The expected JSON structure depends on the message kind:
    /// - `Text`: A plain string (e.g., "Hello, world!").
    /// - `Block`: A JSON object representing a Slack block kit layout (e.g., {"type": "section", "text": {"type": "mrkdwn", "text": "Hello, world!"}}).
    /// - `Attachment`: A JSON object representing a Slack message attachment (e.g., {"fallback": "Required plain-text summary", "text": "Optional text to appear in the attachment"}).
    message: String,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum MessageKind {
    Text,
    Block,
    Attachment,
}

#[tokio::main]
pub async fn main() {
    env_logger::init();

    let token = match dotenvy::var("SLACK_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            eprintln!(
                "Error: SLACK_TOKEN is not set. Please set it in your environment variables."
            );
            std::process::exit(1);
        }
    };

    let cli = Cli::parse();

    let client = slack_api_client::SlackClient::new(token);

    match cli.command {
        Commands::Send(args) => {
            let message = match args.kind {
                MessageKind::Text => slack_api_client::CreateMessage::Text(args.message),
                MessageKind::Block => {
                    let message = match serde_json::Value::from_str(&args.message) {
                        Ok(m) => m,
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
                    };
                    if !message.is_array() {
                        eprintln!("a message of kind `Block` must be a json array");
                        std::process::exit(1);
                    }
                    log::debug!("{:#?}", message);
                    slack_api_client::CreateMessage::Blocks(message)
                }
                MessageKind::Attachment => {
                    let message = match serde_json::Value::from_str(&args.message) {
                        Ok(m) => m,
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
                    };
                    if !message.is_array() {
                        eprintln!("a message of kind `Attachment` must be a json array");
                        std::process::exit(1);
                    }
                    slack_api_client::CreateMessage::Attachments(message)
                }
            };

            let response = message
                .send_to_channel(&client, args.channel.trim().trim_matches('#').to_string())
                .await;

            match response {
                Ok(ok) => {
                    log::debug!("{:#?}", ok);
                    match ok.text().await {
                        Ok(ok) => log::debug!("{:#}", ok),
                        Err(error) => eprintln!("{}", error),
                    };
                }
                Err(error) => eprintln!("{:#?}", error),
            }
        }
    }
}
