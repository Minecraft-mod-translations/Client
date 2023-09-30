use serde::{Deserialize, Serialize};
use crate::req::request_get;

pub struct ModDataBase {

}

const LIST: &str = "https://github.com/Minecraft-mod-translations/Cloud/raw/main/list.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct ModsList {
    pub(crate) lists: Vec<String>,
}

impl ModDataBase {
    pub(crate) fn get_list() -> Result<ModsList, String> {
        match request_get(LIST) {
            None => { Err(String::from("Error downloading list")) }
            Some(list_string) => {
                match  serde_json::from_str::<ModsList>(list_string.as_str()) {
                    Ok(list) => { Ok(list) }
                    Err(_) => { Err(String::from("Error parsing list")) }
                }

            }
        }
    }
}