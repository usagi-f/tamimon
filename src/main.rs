mod app;
mod game;
mod save;
mod ui;

use clap::Parser;

#[derive(Parser)]
#[command(name = "tamimon", version, about = "Terminal Monster - CLI育成放置ゲーム")]
struct Cli {}

#[tokio::main]
async fn main() {
    let _cli = Cli::parse();
    if let Err(e) = app::run().await {
        eprintln!("エラーが発生しました: {:#}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use crate::game::actions::Action;
    use crate::game::evolution::all_species_names;
    use crate::game::pet::MoodLevel;
    use crate::ui::ascii_art;

    #[test]
    fn all_species_have_unique_idle_art() {
        let names = all_species_names();
        let mut seen = std::collections::HashMap::new();
        let mut dupes = Vec::new();

        for name in &names {
            let art = ascii_art::get_art(name, MoodLevel::Normal, 0);
            let key = art.join("|");
            if let Some(prev) = seen.get(&key) {
                dupes.push(format!("  {} == {}", name, prev));
            } else {
                seen.insert(key, name.to_string());
            }
        }

        if !dupes.is_empty() {
            panic!("Duplicate idle art found ({} pairs):\n{}", dupes.len(), dupes.join("\n"));
        }
    }

    #[test]
    fn all_species_have_action_art() {
        let names = all_species_names();
        let actions = [Action::Talk, Action::Play, Action::Train, Action::Relax];
        let mut missing = Vec::new();

        for name in &names {
            for action in &actions {
                let art = ascii_art::get_action_art(name, *action, 0);
                let joined = art.join("");
                if joined.contains("？_？") {
                    missing.push(format!("  {} / {:?}", name, action));
                }
            }
        }

        if !missing.is_empty() {
            panic!(
                "Species missing action art ({} entries):\n{}",
                missing.len(),
                missing.join("\n")
            );
        }
    }
}
