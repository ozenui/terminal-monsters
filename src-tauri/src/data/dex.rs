use crate::data::{Party, PartyMon};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io::{self};

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
    Ancient,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DexMon {
    pub id: u32,
    pub name: String,
    pub family: Family,
    pub rarity: u32,
    pub keys: HashSet<String>,
}

impl DexMon {
    /// Add a DexMon to the user's collection
    pub fn collect(&self, collection: &mut Party) {
        let new_mon = PartyMon::new(self.id);
        collection.mons.push(new_mon);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dex {
    pub mons: Vec<DexMon>,
}

impl Dex {
    pub fn find(&self, id: u32) -> io::Result<&DexMon> {
        for mon in &self.mons {
            if mon.id == id {
                return Ok(mon);
            }
        }
        Err(io::Error::new(io::ErrorKind::NotFound, "Dexmon not found"))
    }

    pub fn load() -> io::Result<Self> {
        Ok(Dex {
            mons: vec![
                DexMon {
                    id: 0,
                    name: "Shellora".to_string(),
                    family: Family::Scripting,
                    rarity: 1,
                    keys: HashSet::from(["tm".to_string(), "sh".to_string()]),
                },
                DexMon {
                    id: 1,
                    name: "Basharoo".to_string(),
                    family: Family::Scripting,
                    rarity: 1,
                    keys: HashSet::from(["bash".to_string()]),
                },
                DexMon {
                    id: 2,
                    name: "Zishan".to_string(),
                    family: Family::Scripting,
                    rarity: 1,
                    keys: HashSet::from(["zsh".to_string()]),
                },
                DexMon {
                    id: 3,
                    name: "Javascriptik".to_string(),
                    family: Family::Web,
                    rarity: 1,
                    keys: HashSet::from([
                        "node".to_string(),
                        "npm".to_string(),
                        "pnpm".to_string(),
                        "yarn".to_string(),
                        "bun".to_string(),
                    ]),
                },
                DexMon {
                    id: 4,
                    name: "Typescriptik".to_string(),
                    family: Family::Web,
                    rarity: 1,
                    keys: HashSet::from(["tsc".to_string(), "tsc --init".to_string()]),
                },
                DexMon {
                    id: 5,
                    name: "Reacto".to_string(),
                    family: Family::Web,
                    rarity: 1,
                    keys: HashSet::from([
                        "npm create react-app".to_string(),
                        "yarn create react-app".to_string(),
                        "pnpm create react-app".to_string(),
                        "bun create react-app".to_string(),
                        "npx create-next-app@latest".to_string(),
                        "npx create-remix".to_string(),
                        "npx create-gatsby".to_string(),
                        "npx create-expo-app".to_string(),
                    ]),
                },
                DexMon {
                    id: 6,
                    name: "Vuesaur".to_string(),
                    family: Family::Web,
                    rarity: 1,
                    keys: HashSet::from([
                        "npm create vue@latest".to_string(),
                        "pnpm create vue@latest".to_string(),
                        "yarn create vue@latest".to_string(),
                        "bun create vue@latest".to_string(),
                    ]),
                },
                DexMon {
                    id: 7,
                    name: "Sveltra".to_string(),
                    family: Family::Web,
                    rarity: 1,
                    keys: HashSet::from([
                        "npm create svelte@latest".to_string(),
                        "pnpm create svelte@latest".to_string(),
                        "yarn create svelte@latest".to_string(),
                        "bun create svelte@latest".to_string(),
                        "npx sv create".to_string(),
                        "bunx sv create".to_string(),
                    ]),
                },
                DexMon {
                    id: 8,
                    name: "Solidon".to_string(),
                    family: Family::Web,
                    rarity: 1,
                    keys: HashSet::from([
                        "npx degit solidjs/templates".to_string(),
                        "npm install solid-js".to_string(),
                        "yarn add solid-js".to_string(),
                        "pnpm add solid-js".to_string(),
                        "bun add solid-js".to_string(),
                    ]),
                },
                DexMon {
                    id: 9,
                    name: "Swifty".to_string(),
                    family: Family::Mobile,
                    rarity: 2,
                    keys: HashSet::from(["swift".to_string()]),
                },
                DexMon {
                    id: 10,
                    name: "Kotlinite".to_string(),
                    family: Family::Mobile,
                    rarity: 1,
                    keys: HashSet::from([
                        "kotlinc -script".to_string(),
                        "gradle init --type kotlin-application".to_string(),
                    ]),
                },
                DexMon {
                    id: 11,
                    name: "Flutterix".to_string(),
                    family: Family::Mobile,
                    rarity: 1,
                    keys: HashSet::from(["flutter create".to_string()]),
                },
                DexMon {
                    id: 12,
                    name: "Unito".to_string(),
                    family: Family::Gaming,
                    rarity: 2,
                    keys: HashSet::from(["unity".to_string()]),
                },
                DexMon {
                    id: 13,
                    name: "Luamon".to_string(),
                    family: Family::Gaming,
                    rarity: 1,
                    keys: HashSet::from(["lua".to_string()]),
                },
                DexMon {
                    id: 14,
                    name: "Godotaur".to_string(),
                    family: Family::Gaming,
                    rarity: 1,
                    keys: HashSet::from(["godot".to_string()]),
                },
                DexMon {
                    id: 15,
                    name: "Squeel".to_string(),
                    family: Family::Database,
                    rarity: 1,
                    keys: HashSet::from(["mysql".to_string()]),
                },
                DexMon {
                    id: 16,
                    name: "Squeelite".to_string(),
                    family: Family::Database,
                    rarity: 1,
                    keys: HashSet::from(["psql".to_string(), "supabase".to_string()]),
                },
                DexMon {
                    id: 17,
                    name: "Supamon".to_string(),
                    family: Family::Database,
                    rarity: 2,
                    keys: HashSet::from(["mongo".to_string(), "mongod".to_string()]),
                },
                DexMon {
                    id: 18,
                    name: "Rustaking".to_string(),
                    family: Family::Systems,
                    rarity: 3,
                    keys: HashSet::from([
                        "rustc".to_string(),
                        "rustup".to_string(),
                        "cargo".to_string(),
                        "tauri".to_string(),
                    ]),
                },
                DexMon {
                    id: 19,
                    name: "Gogomon".to_string(),
                    family: Family::Systems,
                    rarity: 2,
                    keys: HashSet::from(["go mod init".to_string()]),
                },
                DexMon {
                    id: 20,
                    name: "Ceezor".to_string(),
                    family: Family::Systems,
                    rarity: 3,
                    keys: HashSet::from(["gcc -o".to_string()]),
                },
                DexMon {
                    id: 21,
                    name: "Pluceezor".to_string(),
                    family: Family::Systems,
                    rarity: 3,
                    keys: HashSet::from(["g++ -o".to_string()]),
                },
                DexMon {
                    id: 22,
                    name: "Pythor".to_string(),
                    family: Family::Neural,
                    rarity: 1,
                    keys: HashSet::from(["python".to_string()]),
                },
                DexMon {
                    id: 23,
                    name: "Tensora".to_string(),
                    family: Family::Neural,
                    rarity: 3,
                    keys: HashSet::from(["pip install tensorflow".to_string()]),
                },
                DexMon {
                    id: 24,
                    name: "Pandasora".to_string(),
                    family: Family::Neural,
                    rarity: 3,
                    keys: HashSet::from(["pip install pandas".to_string()]),
                },
                DexMon {
                    id: 25,
                    name: "Fortran".to_string(),
                    family: Family::Ancient,
                    rarity: 5,
                    keys: HashSet::from([
                        "sudo apt-get install gfortran".to_string(),
                        "brew install gcc".to_string(),
                        "gfortran -o".to_string(),
                    ]),
                },
            ],
        })
    }
}
