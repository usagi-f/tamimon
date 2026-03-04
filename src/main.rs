mod app;
mod game;
mod save;
mod ui;

use clap::Parser;

#[derive(Parser)]
#[command(name = "tamimon", version, about = "Terminal Monster - CLI育成放置ゲーム")]
struct Cli {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _cli = Cli::parse();
    app::run().await
}

#[cfg(test)]
mod tests {
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
}
