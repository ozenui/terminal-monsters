use terminal_monsters_lib::shared::{collect_exp, collect_monster, load_dex, load_party, DexMon};

use std::collections::HashMap;
use std::io::{BufRead, Result};

fn main() -> Result<()> {
    run_worker()
}

fn run_worker() -> Result<()> {
    let dex = load_dex();
    let mut party = load_party()?;

    let mut collect_commands: HashMap<&str, &DexMon> = HashMap::new();
    let mut exp_commands: HashMap<&str, &DexMon> = HashMap::new();

    for monster in &dex {
        for cmd in &monster.collect_cmds {
            collect_commands.insert(cmd, monster);
        }
        for cmd in &monster.exp_commands {
            exp_commands.insert(cmd, monster);
        }
    }

    let stdin = std::io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let command = line?;

        let collect_matches: Vec<_> = collect_commands
            .iter()
            .filter(|(&key, _)| command.contains(key))
            .collect();

        let exp_matches: Vec<_> = exp_commands
            .iter()
            .filter(|(&key, _)| command.contains(key))
            .collect();

        for (_, &monster) in collect_matches {
            collect_monster(monster, &mut party)?;
        }

        for (_, &monster) in exp_matches {
            collect_exp(monster, &mut party)?;
        }
    }

    Ok(())
}
