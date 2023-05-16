use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::{CommandOptionType, CommandType},
        interaction::application_command::{CommandDataOption, CommandDataOptionValue},
    },
};

use crate::api::bard::Bard;
use crate::api::chattable::Chattable;

pub fn run(options: &[CommandDataOption]) -> String {
    let option = options
        .get(0)
        .expect("No instruction provided")
        .resolved
        .as_ref()
        .expect("Expected valid object");

    if let CommandDataOptionValue::String(instruction) = option {
        let bard = Bard::new(None);
        bard.chat(instruction.to_string())
    } else {
        "Invalid instruction".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("chat")
        .description("Chat with bard")
        .kind(CommandType::ChatInput)
        .create_option(|option| {
            option
                .name("instruction")
                .description("Instruction for bard")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
