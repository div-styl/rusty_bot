use serenity::{
    async_trait,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    model::{
        application::{Command, Interaction},
        gateway::Ready,
        id::GuildId,
    },
    prelude::*,
};

use crate::more_cmd::help;
use color_print::cprintln;

use tracing::info;

pub struct Handler {
    pub guild_key: String,
}

#[async_trait]
impl EventHandler for Handler {
    /// on interaction
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            cprintln!("<y> The Received Command is: {command:#?} </>");

            // match the content of the command
            let content_cmd = match command.data.name.as_str() {
                "help" => Some(help::answer(&command.data.options())),
                _ => Some("I don't know that command".to_string()),
            };
            // check the content of the command
            if let Some(content) = content_cmd {
                let response = CreateInteractionResponseMessage::new().content(content);
                let build = CreateInteractionResponse::Message(response);
                if let Err(why) = command.create_response(&ctx.http, build).await {
                    cprintln!("<r>Couldn't send slash command response: {why:#?} </>");
                }
            }
        }
    }
    /// on ready
    async fn ready(&self, ctx: Context, the_bot: Ready) {
        // check if the name of the bot is set well
        info!("{} is connected!", the_bot.user.name);

        // get secret key of the guild_id from `Secrets.toml`
        let guild_id = GuildId::new(self.guild_key.parse::<u64>().unwrap());
        // create the commands
        let commands = guild_id.set_commands(&ctx.http, vec![help::ask()]).await;

        cprintln!("<m>I created the following global slash command: {commands:#?} </> ");

        let guild_command = Command::create_global_command(&ctx.http, help::ask()).await;

        println!("I created the following global slash command: {guild_command:#?}");
    }
}
