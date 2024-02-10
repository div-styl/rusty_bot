use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn answer(_options: &[ResolvedOption]) -> String {
    let help_rep: String = String::from(
    "\
    # 🤖 The Available Commands are 🤖:
    - `/help` **get the available commands**
    - `/report` **call admin**
    - `/roles` **add a role to a user**
    ",
    );

    help_rep
}

pub fn ask() -> CreateCommand {
    CreateCommand::new("help").description("get the available commands")
}
