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

// ===== ゲンソウ (gensou) - Multi-armed war deity: radial arms expand/contract with mood =====
fn gensou_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        // High: All arms fully extended, core blazing
        (MoodLevel::High, 0) => &[
            "✦╲  ◈  ╱✦  ",
            "╲╔══◆══╗╱   ",
            "═╡ ████ ╞═  ",
            "╱╚══◆══╝╲   ",
            "✦╱  ◈  ╲✦  ",
        ],
        (MoodLevel::High, _) => &[
            "✦╲◈    ╱✦  ",
            "╲╔══◆══╗╱   ",
            "═╡ ████ ╞═  ",
            "╱╚══◆══╝╲   ",
            "✦╱    ◈╲✦  ",
        ],
        // Normal: Arms at medium extension
        (MoodLevel::Normal, 0) => &[
            "  ╲  ◈  ╱   ",
            " ╲╔══◆══╗╱  ",
            " ═╡ ████╞═  ",
            " ╱╚══◆══╝╲  ",
            "  ╱  ◈  ╲   ",
        ],
        (MoodLevel::Normal, _) => &[
            "  ╲  ◇  ╱   ",
            " ╲╔══◆══╗╱  ",
            " ═╡ ████╞═  ",
            " ╱╚══◆══╝╲  ",
            "  ╱  ◇  ╲   ",
        ],
        // Low: Arms retracted, core dim
        (MoodLevel::Low, 0) => &[
            "     ·       ",
            "   ╔══◇══╗  ",
            "   ║ ████ ║  ",
            "   ╚══◇══╝  ",
            "     ·       ",
        ],
        (MoodLevel::Low, _) => &[
            "             ",
            "   ╔══·══╗  ",
            "   ║ ████ ║  ",
            "   ╚══·══╝  ",
            "             ",
        ],
    }
}
fn gensou_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[
            "  ╲  ◈  ╱   ",
            " ╲╔══◆══╗╱═ ",
            " ═╡ ████╞═  ",
            " ╱╚══◆══╝╲  ",
            "  ╱  ◈  ╲   ",
        ],
        (Action::Talk, _) => &[
            "  ╲  ◈  ╱   ",
            "═╲╔══◆══╗╱  ",
            " ═╡ ████╞═  ",
            " ╱╚══◆══╝╲  ",
            "  ╱  ◈  ╲   ",
        ],
        (Action::Play, 0) => &[
            "✦╲◈  ◈╱✦ ♪ ",
            "╲╔══◆══╗╱   ",
            "═╡ ████ ╞═  ",
            "╱╚══◆══╝╲   ",
            "✦╱◈  ◈╲✦   ",
        ],
        (Action::Play, _) => &[
            "♪✦╲◈ ◈╱✦   ",
            "╲╔══◆══╗╱   ",
            "═╡ ████ ╞═  ",
            "╱╚══◆══╝╲   ",
            "✦╱ ◈ ◈╲✦   ",
        ],
        (Action::Train, 0) => &[
            "✦✦╲◈!!╱✦✦  ",
            "╲╔══✦══╗╱   ",
            "╬╡████████╞╬",
            "╱╚══✦══╝╲   ",
            "✦✦╱◈!!╲✦✦  ",
        ],
        (Action::Train, _) => &[
            "!!✦╲◈◈╱✦!! ",
            "╲╔══✦══╗╱   ",
            "╬╡████████╞╬",
            "╱╚══✦══╝╲   ",
            "!!✦╱◈◈╲✦!! ",
        ],
        (Action::Relax, 0) => &[
            "     ·       ",
            "   ╔══◇══╗  ",
            "   ║ ████ ║z ",
            "   ╚══◇══╝  ",
            "     ·       ",
        ],
        (Action::Relax, _) => &[
            "             ",
            "   ╔══·══╗  ",
            "   ║ ████ ║zZ",
            "   ╚══·══╝  ",
            "             ",
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

// ===== カイザー (kaizer) - Face integrated with crown at row 1; grand robe dominates below =====
fn kaizer_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        // High: Joyful face crowned at top, robe spread wide
        (MoodLevel::High, 0) => &[
            "  ♔(▲)♔    ",
            "╱╔════════╗╲",
            "║║ ██████ ║║",
            "╲╚════════╝╱",
            "  ╙──┤├──╜  ",
        ],
        (MoodLevel::High, _) => &[
            "  ♔(▲)♔  ! ",
            "╱╔════════╗╲",
            "║║ ██████ ║║",
            "╲╚════════╝╱",
            "  ╙──┤├──╜  ",
        ],
        // Normal: Composed face, standard robe
        (MoodLevel::Normal, 0) => &[
            "   ♔(ω)♔   ",
            " ╔════════╗ ",
            " ║ ██████ ║ ",
            " ╚════════╝ ",
            "  ╙──┤├──╜  ",
        ],
        (MoodLevel::Normal, _) => &[
            "   ♔(─)♔   ",
            " ╔════════╗ ",
            " ║ ██████ ║ ",
            " ╚════════╝ ",
            "   ╙─┤├─╜  ",
        ],
        // Low: Sad face, robe contracted
        (MoodLevel::Low, 0) => &[
            "   ♔(_)♔   ",
            "  ╔══════╗  ",
            "  ║ ████ ║  ",
            "  ╚══════╝  ",
            "   ╙──╜     ",
        ],
        (MoodLevel::Low, _) => &[
            "   ♔(_)♔   ",
            "  ╔══════╗  ",
            "  ║ ████ ║  ",
            "  ╚══╦══╝  ",
            "     ╚╝     ",
        ],
    }
}
fn kaizer_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        // Talk: Sleeve extends from robe
        (Action::Talk, 0) => &[
            "   ♔(ω)♔   ",
            " ╔════════╗ﾉ",
            " ║ ██████ ║ ",
            " ╚════════╝ ",
            "  ╙──┤├──╜  ",
        ],
        (Action::Talk, _) => &[
            "   ♔(ω)♔   ",
            "ﾉ╔════════╗ ",
            " ║ ██████ ║ ",
            " ╚════════╝ ",
            "  ╙──┤├──╜  ",
        ],
        // Play: Robe sweeps wide, face delighted
        (Action::Play, 0) => &[
            "  ♔(▲)♔  ♪ ",
            "╱╔════════╗╲",
            "║║ ██████ ║║",
            "╲╚════════╝╱",
            "  ╙──┤├──╜  ",
        ],
        (Action::Play, _) => &[
            "♪ ♔(▲)♔    ",
            "╱╔════════╗╲",
            "║║ ██████ ║║",
            "╲╚════════╝╱",
            "  ╙──┤├──╜  ",
        ],
        // Train: Intense face, robe at absolute maximum
        (Action::Train, 0) => &[
            " ♔(益)♔ !!  ",
            "╔══════════╗",
            "║██████████║",
            "╚══════════╝",
            "  ╙──┤├──╜  ",
        ],
        (Action::Train, _) => &[
            "!! ♔(益)♔   ",
            "╔══════════╗",
            "║██████████║",
            "╚══════════╝",
            "  ╙──┤├──╜  ",
        ],
        // Relax: Sleepy face, robe contracted
        (Action::Relax, 0) => &[
            "   ♔(_)♔   ",
            "  ╔══════╗  ",
            "  ║ ████ ║z ",
            "  ╚══════╝  ",
            "   ╙──╜     ",
        ],
        (Action::Relax, _) => &[
            "   ♔(_)♔   ",
            "  ╔══════╗  ",
            "  ║ ████ ║zZ",
            "  ╚══╦══╝  ",
            "     ╚╝     ",
        ],
    }
}

// ===== ハクチュウ (hakuchuu) - Daydream: face floats at center row, clouds/light surround it =====
fn hakuchuu_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        // High: Dreaming face glowing at center, light radiates all around
        (MoodLevel::High, 0) => &[
            "  ✨☁✨☁✨  ",
            "  ～☁～☁～  ",
            " ☁ (○▽○) ☁ ",
            "  ░▒░▒░▒░  ",
            "    ～～     ",
        ],
        (MoodLevel::High, _) => &[
            " ✨☁✨☁✨ ! ",
            "  ☁～☁～☁  ",
            " ☁ (○∀○) ☁ ",
            "  ░▒░▒░▒░  ",
            "    ～～     ",
        ],
        // Normal: Calm face at center, mild clouds
        (MoodLevel::Normal, 0) => &[
            "  ～☁～☁～  ",
            "   ☁～☁   ",
            "  ☁(○ω○)☁  ",
            "   ▒░▒░▒   ",
            "    ～～    ",
        ],
        (MoodLevel::Normal, _) => &[
            "  ☁～☁～☁  ",
            "   ～☁～   ",
            "  ☁(○─○)☁  ",
            "   ▒░▒░▒   ",
            "     ～～   ",
        ],
        // Low: Drooping face, thin mist
        (MoodLevel::Low, 0) => &[
            "    ～ ～    ",
            "   ～ ～ ～  ",
            "  ☁(·_·)☁  ",
            "   ░░░░░   ",
            "     ～     ",
        ],
        (MoodLevel::Low, _) => &[
            "     ～     ",
            "    ～ ～   ",
            "  ☁(·_·)☁  ",
            "    ░░░    ",
            "      ～    ",
        ],
    }
}
fn hakuchuu_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        // Talk: Light flares from center face
        (Action::Talk, 0) => &[
            "  ～☁～☁～  ",
            "   ☁～☁   ",
            "  ☁(○ω○)☁ﾉ ",
            "   ▒░▒░▒   ",
            "    ～～    ",
        ],
        (Action::Talk, _) => &[
            "  ☁～☁～☁  ",
            "   ～☁～   ",
            "ﾉ ☁(○ω○)☁  ",
            "   ▒░▒░▒   ",
            "     ～～   ",
        ],
        // Play: Face delighted, light expands outward
        (Action::Play, 0) => &[
            "  ✨☁✨☁✨ ♪",
            "  ～☁～☁～  ",
            " ☁ (○▽○) ☁ ",
            "  ░▒░▒░▒░  ",
            "    ～～     ",
        ],
        (Action::Play, _) => &[
            "♪ ✨☁✨☁✨  ",
            "  ☁～☁～☁  ",
            " ☁ (○▽○) ☁ ",
            "  ░▒░▒░▒░  ",
            "    ～～     ",
        ],
        // Train: Face intense, light blazing
        (Action::Train, 0) => &[
            "  ✨☁✨☁✨!!",
            "  ☁～☁～☁  ",
            " ☁ (○益○) ☁ ",
            "  ░▒▓███▓▒░ ",
            "   ～～～    ",
        ],
        (Action::Train, _) => &[
            "!!✨☁✨☁✨  ",
            "  ☁～☁～☁  ",
            " ☁ (○益○) ☁ ",
            "  ░▒▓███▓▒░ ",
            "   ～～～    ",
        ],
        // Relax: Face sleepy, mist dims
        (Action::Relax, 0) => &[
            "    ～ ～    ",
            "   ～ ～ ～  ",
            "  ☁(·_·)☁  ",
            "   ░░░░░  z ",
            "     ～     ",
        ],
        (Action::Relax, _) => &[
            "     ～     ",
            "    ～ ～   ",
            "  ☁(·_·)☁  ",
            "    ░░░   zZ",
            "      ～    ",
        ],
    }
}

// ===== コンゲン (kongen) - Primordial monolith: rune patterns activate/dormant with mood =====
fn kongen_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        // High: Runes blazing, full power radiating
        (MoodLevel::High, 0) => &[
            " ╔══◆═◆══╗  ",
            " ║▓◆████◆▓║ ",
            " ╠══◆═◆══╣  ",
            " ╠╦╝    ╚╦╣ ",
            " ╩╩      ╩╩ ",
        ],
        (MoodLevel::High, _) => &[
            "╔══◆═◆══╗ ! ",
            " ║◆▓████▓◆║ ",
            " ╠══◆═◆══╣  ",
            " ╠╦╝    ╚╦╣ ",
            " ╩╩      ╩╩ ",
        ],
        // Normal: Runes partially active
        (MoodLevel::Normal, 0) => &[
            " ╔══════════╗",
            " ║▓ ██████▓ ║",
            " ╠══════════╣",
            " ╠╦╝      ╚╦╣",
            " ╩╩        ╩╩",
        ],
        (MoodLevel::Normal, _) => &[
            " ╔══════════╗",
            " ║ ▓██████▓ ║",
            " ╠══════════╣",
            " ╠╦╝      ╚╦╣",
            " ╩╩        ╩╩",
        ],
        // Low: Runes dormant, monolith inert
        (MoodLevel::Low, 0) => &[
            " ╔══════════╗",
            " ║  ██████  ║",
            " ╠══════════╣",
            " ╠╦╝      ╚╦╣",
            " ╩╩        ╩╩",
        ],
        (MoodLevel::Low, _) => &[
            " ╔══════════╗",
            " ║  ██████  ║",
            " ╠══════════╣",
            " ╠╦╝      ╚╦╣",
            "  ╩╩      ╩╩ ",
        ],
    }
}
fn kongen_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        // Talk: Rune pulse emitted from one side
        (Action::Talk, 0) => &[
            " ╔══════════╗",
            " ║▓ ██████▓ ║ﾉ",
            " ╠══════════╣",
            " ╠╦╝      ╚╦╣",
            " ╩╩        ╩╩",
        ],
        (Action::Talk, _) => &[
            " ╔══════════╗",
            "ﾉ║ ▓██████▓ ║",
            " ╠══════════╣",
            " ╠╦╝      ╚╦╣",
            " ╩╩        ╩╩",
        ],
        // Play: All runes glow simultaneously
        (Action::Play, 0) => &[
            " ╔══◆═◆══╗ ♪",
            " ║▓◆████◆▓║ ",
            " ╠══◆═◆══╣  ",
            " ╠╦╝    ╚╦╣ ",
            " ╩╩      ╩╩ ",
        ],
        (Action::Play, _) => &[
            "♪╔══◆═◆══╗  ",
            " ║◆▓████▓◆║ ",
            " ╠══◆═◆══╣  ",
            " ╠╦╝    ╚╦╣ ",
            " ╩╩      ╩╩ ",
        ],
        // Train: Maximum rune activation, base widens with power
        (Action::Train, 0) => &[
            "╔══◆═◆══╗!! ",
            "║◆▓██████▓◆║",
            "╠══◆═◆══╣   ",
            "╠╦╝    ╚╦╣  ",
            "╩╩      ╩╩  ",
        ],
        (Action::Train, _) => &[
            "!!╔══◆═◆══╗ ",
            " ║◆▓████▓◆║ ",
            " ╠══◆═◆══╣  ",
            "╠╦╝    ╚╦╣  ",
            "╩╩      ╩╩  ",
        ],
        // Relax: Runes fade
        (Action::Relax, 0) => &[
            " ╔══════════╗",
            " ║  ██████  ║",
            " ╠══════════╣z",
            " ╠╦╝      ╚╦╣",
            " ╩╩        ╩╩",
        ],
        (Action::Relax, _) => &[
            " ╔══════════╗",
            " ║  ██████  ║",
            " ╠══════════╣zZ",
            " ╠╦╝      ╚╦╣",
            " ╩╩        ╩╩",
        ],
    }
}

// ===== キセキ (kiseki) - Miracle: wings frame the body; face at center row, crystals surround =====
fn kiseki_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        // High: Wings fully spread, face joyful at crystal core
        (MoodLevel::High, 0) => &[
            "✧╲  ☆◆☆  ╱✧",
            "╱╲ ◆◈◆◈◆ ╲╱",
            "╲╱◈ (▽) ◈╱╲",
            "  ╰──◆◆──╯  ",
            "    ╱╲╱╲    ",
        ],
        (MoodLevel::High, _) => &[
            "✧╲ ☆◆☆! ╱✧ ",
            "╱╲ ◆◈◆◈◆ ╲╱",
            "╲╱◈ (∀) ◈╱╲",
            "  ╰──◆◆──╯  ",
            "    ╱╲╱╲    ",
        ],
        // Normal: Wings at mid-spread, calm face
        (MoodLevel::Normal, 0) => &[
            "   ✧ ☆ ✧   ",
            " ╱╲ ◆◈◆ ╲╱ ",
            " ╲╱ (ω) ╲╱ ",
            "   ╰──◆──╯  ",
            "     ╱╲     ",
        ],
        (MoodLevel::Normal, _) => &[
            "   ✧ ☆ ✧   ",
            " ╱╲ ◆◈◆ ╲╱ ",
            " ╲╱ (─) ╲╱ ",
            "   ╰──◆──╯  ",
            "     ╱╲     ",
        ],
        // Low: Wings folded, sad face, crystals dim
        (MoodLevel::Low, 0) => &[
            "     ☆      ",
            "  ╱ ◇◈◇ ╲  ",
            "  ╲ (_) ╱  ",
            "   ╰─◇─╯   ",
            "     ╱╲     ",
        ],
        (MoodLevel::Low, _) => &[
            "      ☆     ",
            "  ╱ ◇·◇ ╲  ",
            "  ╲ (_) ╱  ",
            "   ╰─·─╯   ",
            "    ╱╲      ",
        ],
    }
}
fn kiseki_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        // Talk: One wing tip extends outward
        (Action::Talk, 0) => &[
            "   ✧ ☆ ✧   ",
            " ╱╲ ◆◈◆ ╲╱ﾉ",
            " ╲╱ (ω) ╲╱ ",
            "   ╰──◆──╯  ",
            "     ╱╲     ",
        ],
        (Action::Talk, _) => &[
            "   ✧ ☆ ✧   ",
            "ﾉ╱╲ ◆◈◆ ╲╱ ",
            " ╲╱ (ω) ╲╱ ",
            "   ╰──◆──╯  ",
            "     ╱╲     ",
        ],
        // Play: Wings spread, face delighted
        (Action::Play, 0) => &[
            "✧╲  ☆◆☆  ╱✧♪",
            "╱╲ ◆◈◆◈◆ ╲╱",
            "╲╱◈ (▽) ◈╱╲",
            "  ╰──◆◆──╯  ",
            "    ╱╲╱╲    ",
        ],
        (Action::Play, _) => &[
            "♪✧╲ ☆◆☆  ╱✧",
            "╱╲ ◆◈◆◈◆ ╲╱",
            "╲╱◈ (▽) ◈╱╲",
            "  ╰──◆◆──╯  ",
            "    ╱╲╱╲    ",
        ],
        // Train: Wings at full power, intense face
        (Action::Train, 0) => &[
            "✧✧╲☆◆☆╱✧✧!!",
            "╱╲◆◈◆◈◆◈◆╲╱",
            "╲╱◈ (益) ◈╱╲",
            "  ╰──◆◆──╯  ",
            "   ╱╲╱╲╱╲   ",
        ],
        (Action::Train, _) => &[
            "!!✧✧╲☆◆╱✧✧ ",
            "╱╲◆◈◆◈◆◈◆╲╱",
            "╲╱◈ (益) ◈╱╲",
            "  ╰──◆◆──╯  ",
            "   ╱╲╱╲╱╲   ",
        ],
        // Relax: Wings fold, face sleepy
        (Action::Relax, 0) => &[
            "     ☆      ",
            "  ╱ ◇◈◇ ╲  ",
            "  ╲ (_) ╱ z ",
            "   ╰─◇─╯   ",
            "     ╱╲     ",
        ],
        (Action::Relax, _) => &[
            "      ☆     ",
            "  ╱ ◇·◇ ╲  ",
            "  ╲ (_) ╱ zZ",
            "   ╰─·─╯   ",
            "    ╱╲      ",
        ],
    }
}

// ===== ムゲンダイ (mugendai) - Infinity ouroboros: loops grow/shrink with mood =====
fn mugendai_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        // High: Loops at maximum expansion, core pulsing
        (MoodLevel::High, 0) => &[
            " ╭∞━━∞━━∞╮  ",
            "╭┨∞ ██████∞┠╮",
            "┃∞∞ ██████∞∞┃",
            "╰━∞━████━∞━╯ ",
            "   ∞∞∞∞∞∞∞  ",
        ],
        (MoodLevel::High, _) => &[
            " ╭∞━━∞━━∞╮! ",
            "╭┨∞ ██████∞┠╮",
            "┃∞∞ ██████∞∞┃",
            "╰━∞━████━∞━╯ ",
            "   ∞∞∞∞∞∞∞  ",
        ],
        // Normal: Standard loop size
        (MoodLevel::Normal, 0) => &[
            "  ╭━━∞━━╮   ",
            " ╭┨∞████∞┠╮ ",
            " ┃∞ ████ ∞┃ ",
            " ╰━∞████∞━╯ ",
            "    ∞∞∞∞    ",
        ],
        (MoodLevel::Normal, _) => &[
            "  ╭━━∞━━╮   ",
            " ╭┨∞████∞┠╮ ",
            " ┃∞ ████ ∞┃ ",
            " ╰━∞████∞━╯ ",
            "     ∞∞∞∞   ",
        ],
        // Low: Loops contracted, core dimmed
        (MoodLevel::Low, 0) => &[
            "   ╭━∞━╮    ",
            "  ╭┨∞██∞┠╮  ",
            "  ┃∞████∞┃  ",
            "  ╰━∞∞∞━╯  ",
            "    ∞∞∞     ",
        ],
        (MoodLevel::Low, _) => &[
            "   ╭━∞━╮    ",
            "  ╭┨∞██∞┠╮  ",
            "  ┃∞████∞┃  ",
            "  ╰━∞∞∞━╯  ",
            "     ∞∞∞    ",
        ],
    }
}
fn mugendai_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        // Talk: Loop extends in one direction
        (Action::Talk, 0) => &[
            "  ╭━━∞━━╮   ",
            " ╭┨∞████∞┠╮ﾉ",
            " ┃∞ ████ ∞┃ ",
            " ╰━∞████∞━╯ ",
            "    ∞∞∞∞    ",
        ],
        (Action::Talk, _) => &[
            "  ╭━━∞━━╮   ",
            "ﾉ╭┨∞████∞┠╮ ",
            " ┃∞ ████ ∞┃ ",
            " ╰━∞████∞━╯ ",
            "    ∞∞∞∞    ",
        ],
        // Play: Loops spin wide with joy
        (Action::Play, 0) => &[
            " ╭∞━━∞━━∞╮ ♪",
            "╭┨∞ ██████∞┠╮",
            "┃∞∞ ██████∞∞┃",
            "╰━∞━████━∞━╯ ",
            "   ∞∞∞∞∞∞∞  ",
        ],
        (Action::Play, _) => &[
            "♪╭∞━━∞━━∞╮  ",
            "╭┨∞ ██████∞┠╮",
            "┃∞∞ ██████∞∞┃",
            "╰━∞━████━∞━╯ ",
            "   ∞∞∞∞∞∞∞  ",
        ],
        // Train: Loops at absolute maximum
        (Action::Train, 0) => &[
            "╭∞━∞━∞━∞━∞╮!!",
            "┨∞∞████████∞∞┠",
            "┃∞∞████████∞∞┃",
            "╰━∞∞████∞∞━╯ ",
            "  ∞∞∞∞∞∞∞∞∞  ",
        ],
        (Action::Train, _) => &[
            "!!╭∞━∞━∞━∞╮  ",
            "┨∞∞████████∞∞┠",
            "┃∞∞████████∞∞┃",
            "╰━∞∞████∞∞━╯ ",
            "  ∞∞∞∞∞∞∞∞∞  ",
        ],
        // Relax: Loops contract
        (Action::Relax, 0) => &[
            "   ╭━∞━╮    ",
            "  ╭┨∞██∞┠╮  ",
            "  ┃∞████∞┃z  ",
            "  ╰━∞∞∞━╯  ",
            "    ∞∞∞     ",
        ],
        (Action::Relax, _) => &[
            "   ╭━∞━╮    ",
            "  ╭┨∞██∞┠╮  ",
            "  ┃∞████∞┃zZ  ",
            "  ╰━∞∞∞━╯  ",
            "     ∞∞∞    ",
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
