use color_print::cprintln;
use serenity::{
    all::{Http, RoleId},
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use tracing::info;



/// listing all the commands that rusty-bot can do
const HELP_MSG: &str = "\
# 🤖 The Available Commands are 🤖:
- `!help_doc` **provides a short sheet for notes of lectures that are available**
- `!exm` **tell the date of the up coming exam, catch up and results**
- `!report` **call admin**
- `!ROLES` **add a role to a user**
";
/// list the command that but can do with help_doc
const HELP_DOC_MSG: &str = "\
# 📕 Hello dear student please pick which semester you belong to 📕 :
## the usage of command `e.g: !s5`:
** A link of a google drive will be sent to you in your dm **
- `!s1` : **semester 1 docs**
- `!s2` : **semester 2 docs**
- `!s3` : **semester 3 docs**
- `!s4` : **semester 4 docs**
- `!s5` : **semester 5 docs**
- `!s6` : **semester 6 docs**
### **Note:** for more commands please use the command `!help`
";

/// exam details for the upcoming exams
const EXAM_MSG: &str = "\
# 📕 The Exams Details 📕 :

## **Exam Date**
- **Exam Date** : **From 25th to 30th Dec 2023**
- **Results**: **From 15th to 16th Jan 2024**
- **Exam catch up** : **From 29th to 2nd Feb 2024**
- **Results**: **8th Feb 2024**
";

/// available ROLES in the server
const ROLE_MSG: &str = "\
# 🤖 Available Roles 🤖 :
- `!std`: Student at benmsik
- `!tch`: working as a teacher and studying at ben msik
- `!nerd`: nerd of the class
";
/// struct of ROLES available
struct RlId {
    std: u64,
    tch: u64,
    nerd: u64,
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

        const ROLE_CMD:[&str;3] = ["!std", "!tch", "!nerd"]; // TODO: change to Vec?
        const ROLE_ID: RlId = RlId { // TODO: use enums that
                std: 1196562025690714173,
                tch: 1196562094519234741 ,
                nerd: 1196516531241242635,
            };

        match new_message.content.as_str() {
            "!help_doc" => {
                let http = self.get_http(&ctx).await;
                if let Err(e) = new_message.channel_id.say(http, HELP_DOC_MSG).await {
                    cprintln!("<s><R> Error sending message: {:?} </><s>", e);
                }
            }
            "!help" => {
                let http = self.get_http(&ctx).await;
                if let Err(wth) = new_message.channel_id.say(http, HELP_MSG).await {
                    cprintln!("<s><R> Error sending message: {:?} </><s>", wth);
                }
            }
            "!exm" => {
                let http = self.get_http(&ctx).await;
                if let Err(wth) = new_message.channel_id.say(http, EXAM_MSG).await {
                    cprintln!("<s><R> Error sending message: {:?} </><s>", wth);
                }
            }
            "!ROLES" => {
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
                                    let mut role_id: RoleId = RoleId::new(ROLE_ID.std); // Default to std role

                                    // Assign the correct role ID based on the index
                                    match index {
                                        1 => role_id = RoleId::new(ROLE_ID.tch),
                                        2 => role_id = RoleId::new(ROLE_ID.nerd),
                                        _ => {} // Default to std role
                                    }

                                    // Add the role to the user
                                    if let Err(err) = member.add_role(&ctx.http, role_id).await {
                                        cprintln!("<s><R> Error adding role: {:?} </><s>", err);
                                    } else {
                                        let http = self.get_http(&ctx).await;
                                        if let Err(wrong) = new_message
                                            .channel_id
                                            .say(&http, &format!("The role **{}** is added successfully! ", cmd))
                                            .await
                                        {
                                            cprintln!("<s><R> Error sending message: {:?} </><s>", wrong);
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