use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use sysinfo::{System, SystemExt};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum Family {
    Scripting,
    Web,
    Mobile,
    Gaming,
    Database,
    Systems,
    Neural,
    Mythical,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Matcher {
    Exact(String),
    StartsWith(String),
    Contains(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Command {
    pub matcher: Matcher,
    pub exp: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JsonDexMon {
    pub id: u32,
    pub name: String,
    pub title: String,
    pub family: Family,
    pub appearance: String,
    pub description: String,
    pub rarity: u32,
    pub collect_cmds: Vec<Command>,
    pub exp_cmds: Vec<Command>,
}

#[derive(Serialize, Debug, Clone)]
pub struct DexMon {
    pub id: u32,
    pub name: String,
    pub title: String,
    pub family: Family,
    pub appearance: String,
    pub description: String,
    pub rarity: u32,
    pub key: Uuid,
    pub collect_cmds: Vec<Command>,
    pub exp_cmds: Vec<Command>,
}

fn get_dex_file_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/shared/dex.json");
    path
}

fn get_machine_id() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();
    if let Some(machine_id) = sys.host_name() {
        machine_id
    } else {
        "default-machine-id".to_string()
    }
}

pub fn load_dex() -> Vec<DexMon> {
    let path = get_dex_file_path();
    let file = File::open(path).expect("Could not find dex file");
    let reader = BufReader::new(file);
    let json_dex: Vec<JsonDexMon> =
        serde_json::from_reader(reader).expect("Could not parse dex file");

    let machine_id = get_machine_id();
    let namespace = Uuid::new_v5(&Uuid::NAMESPACE_DNS, machine_id.as_bytes());

    json_dex
        .into_iter()
        .map(|json_mon| DexMon {
            id: json_mon.id,
            key: Uuid::new_v5(&namespace, json_mon.name.as_bytes()),
            name: json_mon.name,
            title: json_mon.title,
            family: json_mon.family,
            appearance: json_mon.appearance,
            description: json_mon.description,
            rarity: json_mon.rarity,
            collect_cmds: json_mon.collect_cmds,
            exp_cmds: json_mon.exp_cmds,
        })
        .collect()
}

#[allow(dead_code)]
pub fn get_dex_mon_by_id(id: u32) -> Option<DexMon> {
    load_dex().into_iter().find(|mon| mon.id == id)
}
