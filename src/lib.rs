use std::error::Error;
use std::fmt::Debug;
use std::fs;
use json::JsonValue;

pub fn run(path: &str) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(&path)?;
    let parsed_json = json::parse(&file_content)?;

    // Serialise JSON into CommandData:
    let parsed_type = &parsed_json["type"];
    let parsed_data = &parsed_json["data"];
    if parsed_type.is_null() || parsed_data.is_null() {
        return Err("Couldn't find a type or data value in file. Please check it and run again.".into());
    }

    let serialised_json = match serialise_json(parsed_type, parsed_data) {
        Ok(result) => result,
        Err(err) => return Err(format!("Error parsing JSON: {}", err).into())
    };

    println!("Successfully read file and parsed JSON: {:?}", serialised_json);

    Ok(())
}

pub fn serialise_json(parsed_type: &JsonValue, parsed_data: &JsonValue) -> Result<CommandData, Box<dyn Error>> {
    if parsed_type.eq("DM") {
        let parsed_recipient = parsed_data["recipient"].as_str().ok_or("Something went wrong parsing message recipient.")?;

        let parsed_message_amount = parsed_data["amount"].as_u32().ok_or("Something went wrong parsing message amount.")?;
        let parsed_message_content = parsed_data["content"].as_str().ok_or("Something went wrong parsing message content.")?;
        let parsed_message_vary_capitalization = parsed_data["vary"].as_bool().ok_or("Something went wrong parsing message capitalization varying.")?;

        Ok(CommandData::DM {
            recipient: parsed_recipient.to_string(),
            message_amount: parsed_message_amount,
            message_content: parsed_message_content.to_string(),
            message_vary_capitalization: parsed_message_vary_capitalization
        })
    }
    else {
        Err("Type doesn't match any known value.".into())
    }
}

pub fn parse_arguments(args: Vec<String>) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("Not enough arguments. Please ensure the format used is: ./pro_parser <file_path>!");
    }
    Ok(args[1].clone())
}

#[derive(Debug)]
pub enum CommandData {
    DM {
        recipient: String,
        message_amount: u32,
        message_content: String,
        message_vary_capitalization: bool,
    }
}