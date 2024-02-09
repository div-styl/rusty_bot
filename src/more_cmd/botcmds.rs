use color_print::cprintln;
use serenity::{
    all::{Http, RoleId},
    async_trait,
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
};

use tracing::info;

/// listing all the commands that rusty-bot can do
const HELP_MSG: &str = "\
# 🤖 The Available Commands are 🤖:
- `!report` **call admin**
- `!roles` **add a role to a user**
";

/// available ROLES in the server
const ROLE_MSG: &str = "\
# 🤖 Available Roles 🤖 :
- `!ecom`: E-commerce guy
- `!iptv`: Iptv enthusiast
";
/// struct of ROLES available
struct RlId {
    ecom: u64,
    iptv: u64,
}

pub struct Handler;

// get the http so can be used multiple times
impl Handler {
    async fn get_http<'a>(&'a self, ctx: &'a Context) -> &Http {
        &ctx.http
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, new_message: Message) {
        const ROLE_CMD: [&str; 2] = ["!ecom", "!iptv"];
        const ROLE_ID: RlId = RlId {
            ecom: 1205082773774401586,
            iptv: 1205083027735191582,
        };

        match new_message.content.as_str() {
            "!help" => {
                let http = self.get_http(&ctx).await;
                if let Err(wth) = new_message.channel_id.say(http, HELP_MSG).await {
                    cprintln!("<s><R> Error sending message: {:?} </><s>", wth);
                }
            }
            "!report" => {
                // Admin user ID
                let admin_user_id = UserId::new(461940570106101773);
                let http = self.get_http(&ctx).await;

                let cnt = format!("{} called: {}", new_message.author.tag(), new_message.content);
                // Retrieve the User associated with the admin_user_id
                if let Ok(admin_user) = admin_user_id.to_user(&http).await {
                    // Create a direct message channel with the admin user
                    if let Ok(dm_channel) = admin_user.create_dm_channel(&http).await {
                        // Send the report message
                        if let Err(err) = dm_channel.say(&http, &cnt).await {
                            cprintln!("<s><R> Error sending message: {:?} </><s>", err);
                        } else {
                            cprintln!("<s><m>Report sent successfully!</m></s>");
                        }
                    } else {
                        cprintln!("<s><R>Failed to create DM channel with admin user</R></s>");
                    }
                } else {
                    cprintln!("<s><R>Failed to retrieve admin user</R></s>");
                }
            }
            "!roles" => {
                let http = self.get_http(&ctx).await;
                if let Err(wth) = new_message.channel_id.say(http, ROLE_MSG).await {
                    cprintln!("<s><R> Error sending message: {:?} </><s>", wth);
                }
            }
            _ => {
                for (index, &cmd) in ROLE_CMD.iter().enumerate() {
                    if new_message.content == cmd {
                        // Get the guild ID from the message
                        if let Some(guild_id) = new_message.guild_id {
                            // Get the user ID from the message author
                            let user_id = new_message.author.id;
                            // Get the member (user) from the message
                            match guild_id.member(&ctx.http, user_id).await {
                                Ok(member) => {
                                    let mut role_id: RoleId = RoleId::new(ROLE_ID.ecom); // Default to ecom role

                                    // Assign the correct role ID based on the index
                                    match index {
                                        1 => role_id = RoleId::new(ROLE_ID.iptv),
                                        _ => {} // Default to ecom role
                                    }

                                    // Add the role to the user
                                    if let Err(err) = member.add_role(&ctx.http, role_id).await {
                                        cprintln!("<s><R> Error adding role: {:?} </><s>", err);
                                    } else {
                                        let http = self.get_http(&ctx).await;
                                        if let Err(wrong) = new_message
                                            .channel_id
                                            .say(
                                                &http,
                                                &format!(
                                                    "The role **{}** is added successfully! ",
                                                    cmd
                                                ),
                                            )
                                            .await
                                        {
                                            cprintln!(
                                                "<s><R> Error sending message: {:?} </><s>",
                                                wrong
                                            );
                                        }
                                        cprintln!("<s><m> Role added successfully! </> </s>");
                                    }
                                }
                                Err(err) => {
                                    cprintln!("<s><R> Error getting member: {:?} </><s>", err);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    async fn ready(&self, _: Context, bot_info: Ready) {
        cprintln!("<s> {} is connected!", bot_info.user.name);
        info!("{} is connected!", bot_info.user.name);
    }
}
