use terminal_monsters_lib::data::{Dex, DexMon, Party};
use terminal_monsters_lib::utils::send_system_notification;

use std::collections::HashMap;
use std::io::{BufRead, Result};

fn main() -> Result<()> {
    run_worker()
}

fn run_worker() -> Result<()> {
    let dex = Dex::load().expect("Unable to load dex.");
    let mut party = Party::load().expect("Unable to load party.");

    let mut keys: HashMap<&str, &DexMon> = HashMap::new();
    for mon in &dex.mons {
        for key in &mon.keys {
            keys.insert(key.as_str(), mon);
        }
    }

    for mon in &mut party.mons {
        if mon.gain_experience(7) {
            send_system_notification(&format!(
                "{} grew to level {}!",
                Dex::find(&dex, mon.id)
                    .expect("Monster ID should exist in Dex.")
                    .name,
                mon.level
            ));
        }
    }

    let stdin = std::io::stdin();
    let reader = stdin.lock();
    for line in reader.lines() {
        let line = line?;
        let matches: Vec<_> = keys.iter().filter(|(&key, _)| line.contains(key)).collect();

        for (_, &mon) in matches {
            mon.collect(&mut party);
            send_system_notification(&format!("{} joined your party!", &mon.name));
        }
    }

    party.save().expect("Unable to save party.");

    Ok(())
}
