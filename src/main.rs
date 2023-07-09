use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs::File, io::BufReader};

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
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Input file path.");
        return;
    }

    let file_name = &args[1];

    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let deserialized: Project = serde_json::from_reader(reader).unwrap();

    // {id: Page}
    let mut id_map = HashMap::new();
    for page in deserialized.pages {
        id_map.insert(page.id.clone(), page.clone());
    }

    // {tag: [id]}
    let mut tag_map: HashMap<String, Vec<String>> = HashMap::new();
    for m in id_map {
        for line in m.1.lines {
            if line.starts_with("#") {
                tag_map
                    .entry(line)
                    .or_insert(Vec::new())
                    .push(m.1.id.to_string());
            }
        }
    }

    let mut tags: Vec<(usize, String)> = Vec::new();
    for m in tag_map {
        tags.push((m.1.len(), m.0));
    }

    tags.sort_by(|a, b| b.0.cmp(&a.0));

    for tag in tags {
        println!("{}, {}", tag.1, tag.0);
    }
}
