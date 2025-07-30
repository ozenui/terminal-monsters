use terminal_monsters_lib::shared::{collect_exp, collect_monster, load_dex, load_party, Matcher};

use std::io::{BufRead, Result};

fn main() -> Result<()> {
    run_worker()
}

fn run_worker() -> Result<()> {
    let dex = load_dex();
    let mut party = load_party()?;

    let stdin = std::io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let command = line?;

        for monster in &dex {
            for collect_command in &monster.collect_cmds {
                if matches(&command, &collect_command.matcher) {
                    collect_monster(monster, &mut party, collect_command)?;
                }
            }

            for exp_command in &monster.exp_cmds {
                if matches(&command, &exp_command.matcher) {
                    collect_exp(monster, &mut party, exp_command)?;
                }
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
