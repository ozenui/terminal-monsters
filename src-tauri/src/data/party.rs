use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, BufReader};
use std::path::PathBuf;

use crate::utils::send_system_notification;

use super::Dex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartyMon {
    pub id: u32,
    pub level: u32,
    pub total_exp: u32,
    pub exp_range: (u32, u32),
}

impl PartyMon {
    /// Create a new PartyMon from a DexMon.
    pub fn new(dex_mon_id: u32) -> Self {
        PartyMon {
            id: dex_mon_id,
            level: 0,
            total_exp: 0,
            exp_range: (0, 100),
        }
    }

    // Add experience points to a giver Mon. Returns true if the mon has leveled up.
    pub fn gain_experience(&mut self, exp_points: u32) -> bool {
        let mut has_leveled_up = false;
        self.exp_range.0 += exp_points;

        while self.exp_range.0 >= self.exp_range.1 {
            self.level += 1;
            self.exp_range.0 -= self.exp_range.1; // Subtracts the level-up threshold from current exp
            self.exp_range.1 = 100 + (self.level.pow(3)); // Calculates new exp threshold for next level:

            has_leveled_up = true;
        }

        has_leveled_up
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Party {
    pub mons: Vec<PartyMon>,
}

impl Party {
    /// Get the path to the party save file
    fn get_save_path() -> io::Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found."))?;

        Ok(if cfg!(windows) {
            home_dir
                .join("AppData")
                .join("Local")
                .join("terminal-monsters")
                .join(".data")
                .join("party.json")
        } else {
            home_dir
                .join(".config")
                .join("terminal-monsters")
                .join(".data")
                .join("party.json")
        })
    }

    /// Create a new party with a starter monster
    pub fn new() -> io::Result<Self> {
        let dex = Dex::load()?;
        let starter_mon = dex
            .mons
            .first()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No monsters in Dex"))?;

        let party = Party {
            mons: vec![PartyMon::new(starter_mon.id)],
        };

        party.save()?;
        send_system_notification(&format!("{} joined your party!", starter_mon.name));

        Ok(party)
    }

    /// Load party from save file, create new if doesn't exist
    pub fn load() -> io::Result<Self> {
        let path = Self::get_save_path()?;

        if !path.exists() {
            return Self::new();
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let party = serde_json::from_reader(reader)?;

        Ok(party)
    }

    /// Save party to file
    pub fn save(&self) -> io::Result<()> {
        let save_path = Self::get_save_path()?;

        if let Some(parent) = save_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        fs::write(save_path, json)?;

        Ok(())
    }
}
