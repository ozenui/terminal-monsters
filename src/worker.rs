#[allow(unused_imports)]
use app::models::dex::{get_dex_mon_by_id, load_dex, DexMon, Family};
use app::models::party::{initialize_party, save_party, PartyMon};
use notify_rust::Notification;
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

#[allow(dead_code)]
fn main() -> io::Result<()> {
    let dex = load_dex();
    let mut party = initialize_party().unwrap_or_else(|_| vec![]);

    // Map dex commands
    let mut collect_command_map: HashMap<&str, &DexMon> = HashMap::new();
    let mut exp_command_map: HashMap<&str, &DexMon> = HashMap::new();
    for dex_mon in &dex {
        for command in &dex_mon.collect_cmds {
            collect_command_map.insert(command, dex_mon);
        }
        for command in &dex_mon.exp_commands {
            exp_command_map.insert(command, dex_mon);
        }
    }

    // Read input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Distribute 1 Exp. to the team by default
    for mon in &mut party {
        mon.gain_experience(1);
    }

    // Process commands
    for line in reader.lines() {
        let command = line?;
        process_command(&command, &mut party, &collect_command_map, &exp_command_map)?;
    }

    Ok(())
}

fn process_command(
    command: &str,
    party: &mut Vec<PartyMon>,
    collect_command_map: &HashMap<&str, &DexMon>,
    exp_command_map: &HashMap<&str, &DexMon>,
) -> io::Result<()> {
    for (value, dex_mon) in collect_command_map {
        if command.contains(value) {
            handle_collect_command(dex_mon, party)?;
        }
    }

    for (value, dex_mon) in exp_command_map {
        if command.contains(value) {
            handle_exp_command(dex_mon, party)?;
        }
    }

    Ok(())
}

fn handle_collect_command(dex_mon: &DexMon, party: &mut Vec<PartyMon>) -> io::Result<()> {
    if let Some(mon) = party.iter_mut().find(|mon| mon.dex_id == dex_mon.id) {
        mon.gain_experience(33);
        let _ = Notification::new()
            .summary(&format!("{} gained 33 Exp!", dex_mon.name))
            .body("Run `tm` from your terminal to check your party.")
            .icon("firefox")
            .show();
    } else {
        party.push(PartyMon::new(dex_mon.id, 1, (0, 100)));
        let _ = Notification::new()
            .summary(&format!("{} joined your party!", dex_mon.name))
            .body("Run `tm` from your terminal to check your party.")
            .icon("firefox")
            .show();
    }
    save_party(&party)
}

fn handle_exp_command(dex_mon: &DexMon, party: &mut Vec<PartyMon>) -> io::Result<()> {
    if let Some(mon) = party.iter_mut().find(|mon| mon.dex_id == dex_mon.id) {
        mon.gain_experience(1);
    }
    save_party(&party)
}
