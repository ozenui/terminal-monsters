use crate::shared::dex::load_dex;
use crate::shared::dex::DexMon;
use crate::utils::notifications::send_system_notification;

use dirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct PartyMon {
    pub dex_id: u32,
    pub level: u32,
    pub experience_range: (u32, u32),
}

impl PartyMon {
    pub fn new(dex_id: u32, level: u32, experience_range: (u32, u32)) -> Self {
        Self {
            dex_id,
            level,
            experience_range,
        }
    }

    pub fn gain_experience(&mut self, points: u32) -> bool {
        let mut leveled_up = false;
        self.experience_range.0 += points;
        while self.experience_range.0 >= self.experience_range.1 {
            self.level += 1;
            leveled_up = true;
            self.experience_range.0 -= self.experience_range.1;
            self.experience_range.1 = calculate_next_level_exp(self.level);
        }
        leveled_up
    }
}

/// Calculate experience needed for next level.
fn calculate_next_level_exp(level: u32) -> u32 {
    if level == 1 {
        100
    } else {
        100 + (level.pow(3))
    }
}

/// Returns the path to the party.json file containing the party data. Creates the directory if it doesn't exist.
fn get_party_file_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(".config/terminal-monsters/.data");

    if !path.exists() {
        fs::create_dir_all(&path).unwrap();
    }

    path.push(".party.json");
    path
}

/// Loads the user's party from file, creating a new one with a starter monster if it doesn't exist
pub fn load_party() -> io::Result<Vec<PartyMon>> {
    let party_file_path = get_party_file_path();

    if !party_file_path.exists() {
        let starter_mon = load_dex().first().unwrap().clone();
        let party = vec![PartyMon::new(
            starter_mon.id,
            1,
            (0, calculate_next_level_exp(1)),
        )];

        save_party(&party)?;
        send_system_notification(&format!("{} joined your party!", starter_mon.name));

        return Ok(party);
    }

    let file = File::open(&party_file_path)?;
    let reader = BufReader::new(file);
    let mut party: Vec<PartyMon> = serde_json::from_reader(reader)?;
    party.sort_by_key(|mon| mon.dex_id);
    Ok(party)
}

/// Saves the party to a JSON file.
pub fn save_party(party: &[PartyMon]) -> io::Result<()> {
    let party_file_path = get_party_file_path();
    let file = File::create(&party_file_path)?;
    let writer = BufWriter::new(file);

    serde_json::to_writer(writer, party)?;

    Ok(())
}

/// Adds a new monster to the party. Collects experience if the monster is already added to the party.
pub fn collect_monster(dex_mon: &DexMon, party: &mut Vec<PartyMon>) -> std::io::Result<()> {
    if let Some(mon) = party.iter_mut().find(|mon| mon.dex_id == dex_mon.id) {
        if mon.gain_experience(7) {
            send_system_notification(&format!("{} grew to level {}!", dex_mon.name, mon.level));
        }
    } else {
        party.push(PartyMon::new(
            dex_mon.id,
            1,
            (0, calculate_next_level_exp(1)),
        ));
        send_system_notification(&format!("{} joined your party!", dex_mon.name));
    }
    save_party(&party)
}

/// Collects experience points.
pub fn collect_exp(dex_mon: &DexMon, party: &mut Vec<PartyMon>) -> std::io::Result<()> {
    if let Some(mon) = party.iter_mut().find(|mon| mon.dex_id == dex_mon.id) {
        if mon.gain_experience(7) {
            send_system_notification(&format!("{} grew to level {}!", dex_mon.name, mon.level));
        }
    }
    save_party(&party)
}
