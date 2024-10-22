mod deserializer;
mod schema;
mod validation;

pub use schema::*;
use std::io::{BufRead, BufReader};

use regex::Regex;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DdexMessage {
    NewRelease(NewReleaseMessage),
    PurgeRelease(PurgeReleaseMessage),
}

pub fn ddex_parse_str(str: String) -> Result<DdexMessage, String> {
    let re = Regex::new(r"NewReleaseMessage|PurgeReleaseMessage").expect("Error in regex");
    let message_type = re.find(&str).expect("Message type not found");

    match message_type.as_str() {
        "NewReleaseMessage" => {
            let parsed: Result<NewReleaseMessage, String> = yaserde::de::from_str(&str);
            return Ok(DdexMessage::NewRelease(
                parsed.or_else(|err| Err(format!("Parse error: {}", err)))?,
            ));
        }
        "PurseReleaseMessage" => {
            let parsed: Result<PurgeReleaseMessage, String> = yaserde::de::from_str(&str);
            return Ok(DdexMessage::PurgeRelease(
                parsed.or_else(|err| Err(format!("Parse error: {}", err)))?,
            ));
        }
        _ => Err("Unsupported ddex message type".to_string()),
    }
}

pub fn ddex_parse_file(path: &str) -> Result<DdexMessage, String> {
    let mut file = std::fs::File::open(path).expect("Failed to open the file");
    let mut reader = BufReader::new(file);

    let message_type_line = reader.lines().nth(1);

    if let Some(Ok(message_type_tag)) = message_type_line {
        let re = Regex::new(r"NewReleaseMessage|PurgeReleaseMessage").expect("Error in regex");
        let message_type = re.find(&message_type_tag).expect("Message type not found");

        file = std::fs::File::open(path).expect("Failed to open the file");
        reader = BufReader::new(file);

        match message_type.as_str() {
            "NewReleaseMessage" => {
                let parsed: Result<NewReleaseMessage, String> = yaserde::de::from_reader(reader);
                return Ok(DdexMessage::NewRelease(
                    parsed.or_else(|err| Err(format!("Parse error: {}", err)))?,
                ));
            }
            "PurseReleaseMessage" => {
                let parsed: Result<PurgeReleaseMessage, String> = yaserde::de::from_reader(reader);
                return Ok(DdexMessage::PurgeRelease(
                    parsed.or_else(|err| Err(format!("Parse error: {}", err)))?,
                ));
            }
            _ => Err("Unsupported ddex message type".to_string()),
        }
    } else {
        Err("Unable to detect message type".to_string())
    }
}
