use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub name: String,
    pub display_name: String,
    pub exported: i64,
    pub users: Vec<User>,
    pub pages: Vec<Page>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub title: String,
    pub created: i64,
    pub updated: i64,
    pub id: String,
    pub lines: Vec<String>,
}

fn main() {
    let file_name = "tokizuoh-public.json";

    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let deserialized: Project = serde_json::from_reader(reader).unwrap();

    for page in deserialized.pages {
        println!("{}, {}", page.title, page.created);
    }
}
