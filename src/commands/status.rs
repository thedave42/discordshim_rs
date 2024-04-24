use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

pub fn register() -> CreateCommand {
    CreateCommand::new("status").description("Get the status of the printer");
}