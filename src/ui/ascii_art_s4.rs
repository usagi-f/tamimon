//! Hand-crafted ASCII art for all Stage 4 (mutation) species.
//!
//! Each species has a unique 5-line silhouette.

use crate::game::actions::Action;
use crate::game::pet::MoodLevel;

/// Returns hand-crafted idle art for a Stage 4 species, or None if not found.
pub fn get_s4_art(species: &str, mood: MoodLevel, frame: usize) -> Option<Vec<String>> {
    let art: &[&str] = match species {
        "ゲンソウ" => gensou_art(mood, frame),
        "エーテル" => eether_art(mood, frame),
        "カイザー" => kaizer_art(mood, frame),
        "ハクチュウ" => hakuchuu_art(mood, frame),
        "コンゲン" => kongen_art(mood, frame),
        "キセキ" => kiseki_art(mood, frame),
        "ムゲンダイ" => mugendai_art(mood, frame),
        "ナナシ" => nanashi_art(mood, frame),
        _ => return None,
    };
    Some(art.iter().map(|s: &&str| s.to_string()).collect())
}

/// Returns hand-crafted action art for a Stage 4 species, or None if not found.
pub fn get_s4_action_art(species: &str, action: Action, frame: usize) -> Option<Vec<String>> {
    let art: &[&str] = match species {
        "ゲンソウ" => gensou_action(action, frame),
        "エーテル" => eether_action(action, frame),
        "カイザー" => kaizer_action(action, frame),
        "ハクチュウ" => hakuchuu_action(action, frame),
        "コンゲン" => kongen_action(action, frame),
        "キセキ" => kiseki_action(action, frame),
        "ムゲンダイ" => mugendai_action(action, frame),
        "ナナシ" => nanashi_action(action, frame),
        _ => return None,
    };
    Some(art.iter().map(|s: &&str| s.to_string()).collect())
}

// ===== ゲンソウ (gensou) - Multi-armed war deity with halo =====
fn gensou_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[
            "    ◇✧◇    ",
            "  ╲(◎∀◎)╱  ",
            " ═╡ ████ ╞═ ",
            "   ║ ▓▓ ║   ",
            "   ╱╲  ╱╲   ",
        ],
        (MoodLevel::High, _) => &[
            "   ◇✧✧◇  ! ",
            "  ╲(◎▽◎)╱  ",
            " ═╡ ████ ╞═ ",
            "   ║ ▓▓ ║   ",
            "   ╱╲  ╱╲   ",
        ],
        (MoodLevel::Normal, 0) => &[
            "    ◇✧◇    ",
            "  ╲(◎ω◎)╱  ",
            " ═╡ ████ ╞═ ",
            "   ║ ▓▓ ║   ",
            "   ╱╲  ╱╲   ",
        ],
        (MoodLevel::Normal, _) => &[
            "    ◇✧◇    ",
            "  ╲(◎─◎)╱  ",
            " ═╡ ████ ╞═ ",
            "   ║ ▓▓ ║   ",
            "    ╱╲╱╲    ",
        ],
        (MoodLevel::Low, 0) => &[
            "    ◇·◇    ",
            "   (◎_◎)   ",
            "  ═╡ ██ ╞═  ",
            "   ║ ▓▓ ║   ",
            "   ╱╲  ╱╲   ",
        ],
        (MoodLevel::Low, _) => &[
            "    ◇·◇    ",
            "   (◎_◎)   ",
            "  ═╡ ██ ╞═  ",
            "   ║ ▓▓ ║   ",
            "    ╱╲╱╲    ",
        ],
    }
}
fn gensou_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[
            "    ◇✧◇    ",
            "  ╲(◎ω◎)ﾉ  ",
            " ═╡ ████ ╞═ ",
            "   ║ ▓▓ ║   ",
            "   ╱╲  ╱╲   ",
        ],
        (Action::Talk, _) => &[
            "    ◇✧◇    ",
            " ﾉ(◎ω◎)╱   ",
            " ═╡ ████ ╞═ ",
            "   ║ ▓▓ ║   ",
            "   ╱╲  ╱╲   ",
        ],
        (Action::Play, 0) => &[
            "   ◇✧✧◇ ♪  ",
            "  ╲(◎▽◎)╱  ",
            " ═╡ ████ ╞═ ",
            "   ║ ▓▓ ║   ",
            "   ╱╲  ╱╲   ",
        ],
        (Action::Play, _) => &[
            " ♪ ◇✧✧◇    ",
            "  ╲(◎▽◎)╱  ",
            " ═╡ ████ ╞═ ",
            "   ║ ▓▓ ║   ",
            "    ╱╲╱╲    ",
        ],
        (Action::Train, 0) => &[
            "   ◇✧✧✧◇ !!",
            "  ╲(◎益◎)╱  ",
            "══╡ ████ ╞══",
            "   ║ ▓▓ ║   ",
            "  ╱╲    ╱╲  ",
        ],
        (Action::Train, _) => &[
            "!! ◇✧✧✧◇   ",
            "  ╲(◎益◎)╱  ",
            "══╡ ████ ╞══",
            "   ║ ▓▓ ║   ",
            "  ╱╲    ╱╲  ",
        ],
        (Action::Relax, 0) => &[
            "    ◇·◇    ",
            "   (◎_◎)   ",
            "  ═╡ ██ ╞═  ",
            "   ║ ▓▓ ║ z ",
            "   ╱╲  ╱╲   ",
        ],
        (Action::Relax, _) => &[
            "    ◇·◇    ",
            "   (◎_◎)   ",
            "  ═╡ ██ ╞═  ",
            "   ║ ▓▓ ║zZ ",
            "   ╱╲  ╱╲   ",
        ],
    }
}

// ===== エーテル (eether) - Regressed to egg-like single spark =====
fn eether_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[
            "            ",
            "    . · .   ",
            "   ( ✧ )    ",
            "    ' '     ",
            "            ",
        ],
        (MoodLevel::High, _) => &[
            "            ",
            "   · . ·    ",
            "   ( ✧ ) !  ",
            "    ' '     ",
            "            ",
        ],
        (MoodLevel::Normal, 0) => &[
            "            ",
            "            ",
            "   ( ✧ )    ",
            "    '       ",
            "            ",
        ],
        (MoodLevel::Normal, _) => &[
            "            ",
            "            ",
            "    (✧ )    ",
            "     '      ",
            "            ",
        ],
        (MoodLevel::Low, 0) => &[
            "            ",
            "            ",
            "    (· )    ",
            "     '      ",
            "            ",
        ],
        (MoodLevel::Low, _) => &[
            "            ",
            "            ",
            "   ( ·)     ",
            "    '       ",
            "            ",
        ],
    }
}
fn eether_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[
            "            ",
            "            ",
            "   ( ✧ )ﾉ   ",
            "    '       ",
            "            ",
        ],
        (Action::Talk, _) => &[
            "            ",
            "            ",
            "  ﾉ( ✧ )    ",
            "     '      ",
            "            ",
        ],
        (Action::Play, 0) => &[
            "            ",
            "    · ✧ ·   ",
            "   ( ✧ ) ♪  ",
            "    ' '     ",
            "            ",
        ],
        (Action::Play, _) => &[
            "            ",
            "   ✧ · ✧    ",
            " ♪ ( ✧ )    ",
            "    ' '     ",
            "            ",
        ],
        (Action::Train, 0) => &[
            "            ",
            "   · · · !  ",
            "   ( ✦ )    ",
            "    ' '     ",
            "            ",
        ],
        (Action::Train, _) => &[
            "            ",
            " ! · · ·    ",
            "   ( ✦ )    ",
            "    ' '     ",
            "            ",
        ],
        (Action::Relax, 0) => &[
            "            ",
            "            ",
            "   ( · )  z ",
            "    '       ",
            "            ",
        ],
        (Action::Relax, _) => &[
            "            ",
            "            ",
            "   ( · ) zZ ",
            "    '       ",
            "            ",
        ],
    }
}

// ===== カイザー (kaizer) - Crowned emperor with cape =====
fn kaizer_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[
            "    ♔♔♔     ",
            " ╔═(★▽★)═╗ ",
            " ║  ████  ║ ",
            " ╚╗  ██  ╔╝ ",
            "   ╚╝  ╚╝   ",
        ],
        (MoodLevel::High, _) => &[
            "    ♔♔♔   ! ",
            " ╔═(★∀★)═╗ ",
            " ║  ████  ║ ",
            " ╚╗  ██  ╔╝ ",
            "   ╚╝  ╚╝   ",
        ],
        (MoodLevel::Normal, 0) => &[
            "    ♔♔♔     ",
            " ╔═(★ω★)═╗ ",
            " ║  ████  ║ ",
            " ╚╗  ██  ╔╝ ",
            "   ╚╝  ╚╝   ",
        ],
        (MoodLevel::Normal, _) => &[
            "    ♔♔♔     ",
            " ╔═(★─★)═╗ ",
            " ║  ████  ║ ",
            " ╚╗  ██  ╔╝ ",
            "   ╚╝  ╚╝   ",
        ],
        (MoodLevel::Low, 0) => &[
            "    ♔♔♔     ",
            "  ╔(★_★)╗  ",
            "  ║ ████ ║  ",
            "  ╚╗ ██ ╔╝  ",
            "   ╚╝╚╝     ",
        ],
        (MoodLevel::Low, _) => &[
            "    ♔♔♔     ",
            "  ╔(★_★)╗  ",
            "  ║ ████ ║  ",
            "  ╚╗ ██ ╔╝  ",
            "    ╚╝╚╝    ",
        ],
    }
}
fn kaizer_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[
            "    ♔♔♔     ",
            " ╔═(★ω★)ﾉ╗ ",
            " ║  ████  ║ ",
            " ╚╗  ██  ╔╝ ",
            "   ╚╝  ╚╝   ",
        ],
        (Action::Talk, _) => &[
            "    ♔♔♔     ",
            " ╔ﾉ(★ω★)═╗ ",
            " ║  ████  ║ ",
            " ╚╗  ██  ╔╝ ",
            "   ╚╝  ╚╝   ",
        ],
        (Action::Play, 0) => &[
            "    ♔♔♔  ♪  ",
            " ╔═(★▽★)═╗ ",
            " ║  ████  ║ ",
            " ╚╗  ██  ╔╝ ",
            "   ╚╝  ╚╝   ",
        ],
        (Action::Play, _) => &[
            " ♪  ♔♔♔     ",
            " ╔═(★▽★)═╗ ",
            " ║  ████  ║ ",
            " ╚╗  ██  ╔╝ ",
            "   ╚╝  ╚╝   ",
        ],
        (Action::Train, 0) => &[
            "    ♔♔♔  !! ",
            " ╔═(★益★)═╗",
            " ║ ██████ ║ ",
            " ╚╗  ██  ╔╝ ",
            "  ╚╝    ╚╝  ",
        ],
        (Action::Train, _) => &[
            " !! ♔♔♔     ",
            "╔═(★益★)═╗ ",
            " ║ ██████ ║ ",
            " ╚╗  ██  ╔╝ ",
            "  ╚╝    ╚╝  ",
        ],
        (Action::Relax, 0) => &[
            "    ♔♔♔     ",
            "  ╔(★_★)╗  ",
            "  ║ ████ ║  ",
            "  ╚╗ ██ ╔╝z ",
            "   ╚╝╚╝     ",
        ],
        (Action::Relax, _) => &[
            "    ♔♔♔     ",
            "  ╔(★_★)╗  ",
            "  ║ ████ ║  ",
            "  ╚╗ ██ ╔╝zZ",
            "    ╚╝╚╝    ",
        ],
    }
}

// ===== ハクチュウ (hakuchuu) - Daydream: half-dissolving into light =====
fn hakuchuu_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[
            "  ～☁～☁～  ",
            "  ░(○▽○)░  ",
            "   ▒▓█▓▒   ",
            "    ░▓░     ",
            "     ~      ",
        ],
        (MoodLevel::High, _) => &[
            "  ☁～☁～☁ ! ",
            "  ░(○∀○)░  ",
            "   ▒▓█▓▒   ",
            "    ░▓░     ",
            "     ~      ",
        ],
        (MoodLevel::Normal, 0) => &[
            "  ～☁～☁～  ",
            "  ░(○ω○)░  ",
            "   ▒▓█▓▒   ",
            "    ░▓░     ",
            "     ~      ",
        ],
        (MoodLevel::Normal, _) => &[
            "  ☁～☁～☁  ",
            "  ░(○─○)░  ",
            "   ▒▓█▓▒   ",
            "    ░▓░     ",
            "      ~     ",
        ],
        (MoodLevel::Low, 0) => &[
            "   ～ ～ ～  ",
            "  ░(○_○)░  ",
            "    ▒▓▒    ",
            "    ░░     ",
            "     ~      ",
        ],
        (MoodLevel::Low, _) => &[
            "    ～ ～    ",
            "  ░(○_○)░  ",
            "    ▒▓▒    ",
            "     ░     ",
            "      ~     ",
        ],
    }
}
fn hakuchuu_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[
            "  ～☁～☁～  ",
            "  ░(○ω○)ﾉ  ",
            "   ▒▓█▓▒   ",
            "    ░▓░     ",
            "     ~      ",
        ],
        (Action::Talk, _) => &[
            "  ☁～☁～☁  ",
            " ﾉ(○ω○)░   ",
            "   ▒▓█▓▒   ",
            "    ░▓░     ",
            "     ~      ",
        ],
        (Action::Play, 0) => &[
            "  ～☁～☁～♪ ",
            "  ░(○▽○)░  ",
            "   ▒▓█▓▒   ",
            "    ░▓░     ",
            "     ~      ",
        ],
        (Action::Play, _) => &[
            "♪ ☁～☁～☁  ",
            "  ░(○▽○)░  ",
            "   ▒▓█▓▒   ",
            "    ░▓░     ",
            "      ~     ",
        ],
        (Action::Train, 0) => &[
            "  ～☁～☁～!!",
            "  ░(○益○)░  ",
            "  ▒▓███▓▒  ",
            "    ░▓░     ",
            "     ~      ",
        ],
        (Action::Train, _) => &[
            "!!～☁～☁～  ",
            "  ░(○益○)░  ",
            "  ▒▓███▓▒  ",
            "    ░▓░     ",
            "     ~      ",
        ],
        (Action::Relax, 0) => &[
            "   ～ ～ ～  ",
            "  ░(○_○)░  ",
            "    ▒▓▒  z ",
            "    ░░     ",
            "     ~      ",
        ],
        (Action::Relax, _) => &[
            "    ～ ～    ",
            "  ░(○_○)░  ",
            "    ▒▓▒  zZ",
            "     ░     ",
            "      ~     ",
        ],
    }
}

// ===== コンゲン (kongen) - Primordial monolith =====
fn kongen_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[
            " ╔══████══╗ ",
            " ║ (●▽●)  ║ ",
            " ╠══████══╣ ",
            " ╠╦╝    ╚╦╣ ",
            " ╩╩      ╩╩ ",
        ],
        (MoodLevel::High, _) => &[
            " ╔══████══╗!",
            " ║ (●∀●)  ║ ",
            " ╠══████══╣ ",
            " ╠╦╝    ╚╦╣ ",
            " ╩╩      ╩╩ ",
        ],
        (MoodLevel::Normal, 0) => &[
            " ╔══████══╗ ",
            " ║ (●ω●)  ║ ",
            " ╠══████══╣ ",
            " ╠╦╝    ╚╦╣ ",
            " ╩╩      ╩╩ ",
        ],
        (MoodLevel::Normal, _) => &[
            " ╔══████══╗ ",
            " ║ (●─●)  ║ ",
            " ╠══████══╣ ",
            " ╠╦╝    ╚╦╣ ",
            " ╩╩      ╩╩ ",
        ],
        (MoodLevel::Low, 0) => &[
            " ╔══████══╗ ",
            " ║ (●_●)  ║ ",
            " ╠══████══╣ ",
            " ╠╦╝    ╚╦╣ ",
            " ╩╩      ╩╩ ",
        ],
        (MoodLevel::Low, _) => &[
            " ╔══████══╗ ",
            " ║  (●_●) ║ ",
            " ╠══████══╣ ",
            " ╠╦╝    ╚╦╣ ",
            " ╩╩      ╩╩ ",
        ],
    }
}
fn kongen_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[
            " ╔══████══╗ ",
            " ║ (●ω●)ﾉ║ ",
            " ╠══████══╣ ",
            " ╠╦╝    ╚╦╣ ",
            " ╩╩      ╩╩ ",
        ],
        (Action::Talk, _) => &[
            " ╔══████══╗ ",
            " ║ﾉ(●ω●) ║ ",
            " ╠══████══╣ ",
            " ╠╦╝    ╚╦╣ ",
            " ╩╩      ╩╩ ",
        ],
        (Action::Play, 0) => &[
            " ╔══████══╗♪",
            " ║ (●▽●)  ║ ",
            " ╠══████══╣ ",
            " ╠╦╝    ╚╦╣ ",
            " ╩╩      ╩╩ ",
        ],
        (Action::Play, _) => &[
            "♪╔══████══╗ ",
            " ║ (●▽●)  ║ ",
            " ╠══████══╣ ",
            " ╠╦╝    ╚╦╣ ",
            " ╩╩      ╩╩ ",
        ],
        (Action::Train, 0) => &[
            " ╔══████══╗!!",
            " ║ (●益●)  ║ ",
            " ╠══████══╣ ",
            " ╠╦╝    ╚╦╣ ",
            "╩╩        ╩╩",
        ],
        (Action::Train, _) => &[
            "!!╔══████══╗ ",
            " ║  (●益●) ║ ",
            " ╠══████══╣ ",
            " ╠╦╝    ╚╦╣ ",
            "╩╩        ╩╩",
        ],
        (Action::Relax, 0) => &[
            " ╔══████══╗ ",
            " ║ (●_●)  ║ ",
            " ╠══████══╣z",
            " ╠╦╝    ╚╦╣ ",
            " ╩╩      ╩╩ ",
        ],
        (Action::Relax, _) => &[
            " ╔══████══╗ ",
            " ║ (●_●)  ║ ",
            " ╠══████══╣zZ",
            " ╠╦╝    ╚╦╣ ",
            " ╩╩      ╩╩ ",
        ],
    }
}

// ===== キセキ (kiseki) - Miracle: winged jeweled being =====
fn kiseki_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[
            "   ✧ ☆ ✧   ",
            " ╱╲(♥▽♥)╱╲ ",
            " ╲╱ ◇◆◇ ╲╱ ",
            "   ╰─┤├─╯  ",
            "     ╱╲     ",
        ],
        (MoodLevel::High, _) => &[
            "   ✧ ☆ ✧ ! ",
            " ╱╲(♥∀♥)╱╲ ",
            " ╲╱ ◇◆◇ ╲╱ ",
            "   ╰─┤├─╯  ",
            "     ╱╲     ",
        ],
        (MoodLevel::Normal, 0) => &[
            "   ✧ ☆ ✧   ",
            " ╱╲(♥ω♥)╱╲ ",
            " ╲╱ ◇◆◇ ╲╱ ",
            "   ╰─┤├─╯  ",
            "     ╱╲     ",
        ],
        (MoodLevel::Normal, _) => &[
            "   ✧ ☆ ✧   ",
            " ╱╲(♥─♥)╱╲ ",
            " ╲╱ ◇◆◇ ╲╱ ",
            "   ╰─┤├─╯  ",
            "     ╱╲     ",
        ],
        (MoodLevel::Low, 0) => &[
            "     ☆     ",
            "  ╱(♥_♥)╲  ",
            "  ╲ ◇◆◇ ╱  ",
            "   ╰─┤├─╯  ",
            "     ╱╲     ",
        ],
        (MoodLevel::Low, _) => &[
            "     ☆     ",
            "  ╱(♥_♥)╲  ",
            "  ╲ ◇◆◇ ╱  ",
            "   ╰─┤├─╯  ",
            "    ╱╲      ",
        ],
    }
}
fn kiseki_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[
            "   ✧ ☆ ✧   ",
            " ╱╲(♥ω♥)ﾉ╲ ",
            " ╲╱ ◇◆◇ ╲╱ ",
            "   ╰─┤├─╯  ",
            "     ╱╲     ",
        ],
        (Action::Talk, _) => &[
            "   ✧ ☆ ✧   ",
            " ╱ﾉ(♥ω♥)╱╲ ",
            " ╲╱ ◇◆◇ ╲╱ ",
            "   ╰─┤├─╯  ",
            "     ╱╲     ",
        ],
        (Action::Play, 0) => &[
            "   ✧ ☆ ✧ ♪ ",
            " ╱╲(♥▽♥)╱╲ ",
            " ╲╱ ◇◆◇ ╲╱ ",
            "   ╰─┤├─╯  ",
            "     ╱╲     ",
        ],
        (Action::Play, _) => &[
            " ♪ ✧ ☆ ✧   ",
            " ╱╲(♥▽♥)╱╲ ",
            " ╲╱ ◇◆◇ ╲╱ ",
            "   ╰─┤├─╯  ",
            "     ╱╲     ",
        ],
        (Action::Train, 0) => &[
            "  ✧☆✧☆✧ !! ",
            " ╱╲(♥益♥)╱╲",
            " ╲╱ ◇◆◇ ╲╱ ",
            "   ╰─┤├─╯  ",
            "    ╱╲╱╲    ",
        ],
        (Action::Train, _) => &[
            " !! ✧☆✧☆✧  ",
            "╱╲(♥益♥)╱╲ ",
            " ╲╱ ◇◆◇ ╲╱ ",
            "   ╰─┤├─╯  ",
            "    ╱╲╱╲    ",
        ],
        (Action::Relax, 0) => &[
            "     ☆     ",
            "  ╱(♥_♥)╲  ",
            "  ╲ ◇◆◇ ╱ z",
            "   ╰─┤├─╯  ",
            "     ╱╲     ",
        ],
        (Action::Relax, _) => &[
            "     ☆     ",
            "  ╱(♥_♥)╲  ",
            "  ╲ ◇◆◇ ╱zZ",
            "   ╰─┤├─╯  ",
            "     ╱╲     ",
        ],
    }
}

// ===== ムゲンダイ (mugendai) - Infinity ouroboros =====
fn mugendai_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[
            "  ╭━━∞━━╮  ",
            " ╭┨◉ ▽ ◉┠╮ ",
            " ┃ ██████ ┃ ",
            " ╰━██████━╯ ",
            "    ∞∞∞     ",
        ],
        (MoodLevel::High, _) => &[
            "  ╭━━∞━━╮ ! ",
            " ╭┨◉ ∀ ◉┠╮ ",
            " ┃ ██████ ┃ ",
            " ╰━██████━╯ ",
            "    ∞∞∞     ",
        ],
        (MoodLevel::Normal, 0) => &[
            "  ╭━━∞━━╮  ",
            " ╭┨◉ ω ◉┠╮ ",
            " ┃ ██████ ┃ ",
            " ╰━██████━╯ ",
            "    ∞∞∞     ",
        ],
        (MoodLevel::Normal, _) => &[
            "  ╭━━∞━━╮  ",
            " ╭┨◉ ─ ◉┠╮ ",
            " ┃ ██████ ┃ ",
            " ╰━██████━╯ ",
            "     ∞∞∞    ",
        ],
        (MoodLevel::Low, 0) => &[
            "  ╭━━∞━━╮  ",
            " ╭┨◉ _ ◉┠╮ ",
            " ┃ ██████ ┃ ",
            " ╰━██████━╯ ",
            "    ∞∞∞     ",
        ],
        (MoodLevel::Low, _) => &[
            "  ╭━━∞━━╮  ",
            " ╭┨◉ _ ◉┠╮ ",
            " ┃ ██████ ┃ ",
            " ╰━██████━╯ ",
            "     ∞∞∞    ",
        ],
    }
}
fn mugendai_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[
            "  ╭━━∞━━╮  ",
            " ╭┨◉ ω ◉┠ﾉ ",
            " ┃ ██████ ┃ ",
            " ╰━██████━╯ ",
            "    ∞∞∞     ",
        ],
        (Action::Talk, _) => &[
            "  ╭━━∞━━╮  ",
            " ﾉ┨◉ ω ◉┠╮ ",
            " ┃ ██████ ┃ ",
            " ╰━██████━╯ ",
            "    ∞∞∞     ",
        ],
        (Action::Play, 0) => &[
            "  ╭━━∞━━╮♪ ",
            " ╭┨◉ ▽ ◉┠╮ ",
            " ┃ ██████ ┃ ",
            " ╰━██████━╯ ",
            "    ∞∞∞     ",
        ],
        (Action::Play, _) => &[
            "♪ ╭━━∞━━╮  ",
            " ╭┨◉ ▽ ◉┠╮ ",
            " ┃ ██████ ┃ ",
            " ╰━██████━╯ ",
            "     ∞∞∞    ",
        ],
        (Action::Train, 0) => &[
            "  ╭━━∞━━╮!!",
            " ╭┨◉益 ◉┠╮ ",
            " ┃████████┃ ",
            " ╰━██████━╯ ",
            "   ∞∞∞∞∞    ",
        ],
        (Action::Train, _) => &[
            "!!╭━━∞━━╮  ",
            " ╭┨◉ 益◉┠╮ ",
            " ┃████████┃ ",
            " ╰━██████━╯ ",
            "   ∞∞∞∞∞    ",
        ],
        (Action::Relax, 0) => &[
            "  ╭━━∞━━╮  ",
            " ╭┨◉ _ ◉┠╮ ",
            " ┃ ██████ ┃z",
            " ╰━██████━╯ ",
            "    ∞∞∞     ",
        ],
        (Action::Relax, _) => &[
            "  ╭━━∞━━╮  ",
            " ╭┨◉ _ ◉┠╮ ",
            " ┃ ██████┃zZ",
            " ╰━██████━╯ ",
            "    ∞∞∞     ",
        ],
    }
}

// ===== ナナシ (nanashi) - Nameless: drifting particles (former Ether design) =====
fn nanashi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[
            "    · ✧ ·   ",
            "  ·       · ",
            " ✧  · · ·  ✧",
            "  ·  · · ·  ",
            "    ✧   ✧   ",
        ],
        (MoodLevel::High, _) => &[
            "   ✧ · ✧  ! ",
            "  ·       · ",
            " ✧ · · · · ✧",
            "  · ·   · · ",
            "    ✧   ✧   ",
        ],
        (MoodLevel::Normal, 0) => &[
            "    · ✧ ·   ",
            "  ·       · ",
            " ✧  · · ·  ✧",
            "  ·   · ·  · ",
            "    ✧   ✧   ",
        ],
        (MoodLevel::Normal, _) => &[
            "   · ✧  ·   ",
            "  ·       · ",
            "  ✧ · · ·  ✧",
            "  ·  · ·  · ",
            "   ✧    ✧   ",
        ],
        (MoodLevel::Low, 0) => &[
            "     ·      ",
            "   ·     ·  ",
            "  ✧  · ·  ✧ ",
            "   ·   ·    ",
            "    ✧   ✧   ",
        ],
        (MoodLevel::Low, _) => &[
            "      ·     ",
            "  ·      ·  ",
            "  ✧ ·  ·  ✧ ",
            "   ·    ·   ",
            "   ✧    ✧   ",
        ],
    }
}
fn nanashi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[
            "    · ✧ ·   ",
            "  ·       · ",
            " ✧  · · ·  ✧ﾉ",
            "  ·   · ·  · ",
            "    ✧   ✧   ",
        ],
        (Action::Talk, _) => &[
            "   · ✧  ·   ",
            "  ·       · ",
            "ﾉ✧  · · ·  ✧",
            "  ·  · ·  · ",
            "   ✧    ✧   ",
        ],
        (Action::Play, 0) => &[
            "    · ✧ · ♪ ",
            "  ·       · ",
            " ✧  · · ·  ✧",
            "  ·  · · ·  ",
            "    ✧   ✧   ",
        ],
        (Action::Play, _) => &[
            " ♪  · ✧ ·   ",
            "  ·       · ",
            " ✧ · · · · ✧",
            "  · ·   · · ",
            "   ✧    ✧   ",
        ],
        (Action::Train, 0) => &[
            "   ✧·✧·✧ !! ",
            "  ·       · ",
            " ✧ · · · · ✧",
            "  · · · · · ",
            "   ✧ ✧ ✧    ",
        ],
        (Action::Train, _) => &[
            " !! ✧·✧·✧   ",
            "  ·       · ",
            " ✧· · · · ·✧",
            "  · · · · · ",
            "    ✧ ✧ ✧   ",
        ],
        (Action::Relax, 0) => &[
            "     ·      ",
            "   ·     ·  ",
            "  ✧  · ·  ✧z",
            "   ·   ·    ",
            "    ✧   ✧   ",
        ],
        (Action::Relax, _) => &[
            "      ·     ",
            "  ·      ·  ",
            "  ✧ ·  ·  ✧zZ",
            "   ·    ·   ",
            "   ✧    ✧   ",
        ],
    }
}
