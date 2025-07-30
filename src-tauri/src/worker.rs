use once_cell::sync::Lazy;
use std::io::{BufRead, Result};
use terminal_monsters_lib::shared::{
    collect_exp, collect_monster, load_dex, load_party, Command, DexMon, Matcher, PartyMon,
};

enum Action<'a> {
    Collect(&'a DexMon, &'a Command),
    Exp(&'a DexMon, &'a Command),
}

static DEX: Lazy<Vec<DexMon>> = Lazy::new(load_dex);
static mut PARTY: Lazy<Vec<PartyMon>> = Lazy::new(|| load_party().expect("Failed to load party"));

fn main() -> Result<()> {
    run_worker()
}

fn run_worker() -> Result<()> {
    let stdin = std::io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let command = line?;
        let mut actions = Vec::new();

        for monster in &*DEX {
            for collect_command in &monster.collect_cmds {
                if matches(&command, &collect_command.matcher) {
                    actions.push(Action::Collect(monster, collect_command));
                }
            }

            for exp_command in &monster.exp_cmds {
                if matches(&command, &exp_command.matcher) {
                    actions.push(Action::Exp(monster, exp_command));
                }
            }
        }

        for action in actions {
            match action {
                Action::Collect(monster, collect_command) => unsafe {
                    collect_monster(monster, &mut *PARTY, collect_command)?;
                },
                Action::Exp(monster, exp_command) => unsafe {
                    collect_exp(monster, &mut *PARTY, exp_command)?;
                },
            }
        }
    }

    Ok(())
}

fn matches(command: &str, matcher: &Matcher) -> bool {
    match matcher {
        Matcher::Exact(s) => command == s,
        Matcher::StartsWith(s) => command.starts_with(s),
        Matcher::Contains(s) => command.contains(s),
    }
}
