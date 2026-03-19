//! Hand-crafted ASCII art for all Stage 2 species.

use crate::game::actions::Action;
use crate::game::pet::MoodLevel;

/// Returns hand-crafted idle art for a Stage 2 species, or None if not found.
pub fn get_s2_art(species: &str, mood: MoodLevel, frame: usize) -> Option<Vec<String>> {
    let art: &[&str] = match species {
        // Chikara type
        "ドタン" => dotan_art(mood, frame),
        "ガシャ" => gasha_art(mood, frame),
        "ズンズン" => zunzun_art(mood, frame),
        "デカオ" => dekao_art(mood, frame),
        "ゴツモリ" => gotsumori_art(mood, frame),
        "ドンガメ" => dongame_art(mood, frame),
        // Odayaka type
        "ヒョロン" => hyoron_art(mood, frame),
        "フワモン" => fuwamon_art(mood, frame),
        "ユラリ" => yurari_art(mood, frame),
        "ネムタ" => nemuta_art(mood, frame),
        "ポワン" => powan_art(mood, frame),
        "ホワモコ" => howamoko_art(mood, frame),
        // Bouken type
        "クルル" => kururu_art(mood, frame),
        "トゲたろう" => togetarou_art(mood, frame),
        "ハネオ" => haneo_art(mood, frame),
        "ビョーン" => byoon_art(mood, frame),
        "ダッシュ" => dashu_art(mood, frame),
        "グルグル" => guruguru_art(mood, frame),
        // Normal type
        "ペタ" => peta_art(mood, frame),
        "ノホホ" => nohoho_art(mood, frame),
        "マジメ" => majime_art(mood, frame),
        "フツウ" => futsuu_art(mood, frame),
        "ナミナミ" => naminami_art(mood, frame),
        "テキトー" => tekitoo_art(mood, frame),
        // Wild type
        "メダマ" => medama_art(mood, frame),
        "ケモノ" => kemono_art(mood, frame),
        "ヌシ" => nushi_art(mood, frame),
        "カゲ" => kage_art(mood, frame),
        "ザワザワ" => zawazawa_art(mood, frame),
        "ヒトダマ" => hitodama_art(mood, frame),
        _ => return None,
    };
    Some(art.iter().map(|s: &&str| s.to_string()).collect())
}

/// Returns hand-crafted action art for a Stage 2 species, or None if not found.
pub fn get_s2_action_art(species: &str, action: Action, frame: usize) -> Option<Vec<String>> {
    let art: &[&str] = match species {
        // Chikara type
        "ドタン" => dotan_action(action, frame),
        "ガシャ" => gasha_action(action, frame),
        "ズンズン" => zunzun_action(action, frame),
        "デカオ" => dekao_action(action, frame),
        "ゴツモリ" => gotsumori_action(action, frame),
        "ドンガメ" => dongame_action(action, frame),
        // Odayaka type
        "ヒョロン" => hyoron_action(action, frame),
        "フワモン" => fuwamon_action(action, frame),
        "ユラリ" => yurari_action(action, frame),
        "ネムタ" => nemuta_action(action, frame),
        "ポワン" => powan_action(action, frame),
        "ホワモコ" => howamoko_action(action, frame),
        // Bouken type
        "クルル" => kururu_action(action, frame),
        "トゲたろう" => togetarou_action(action, frame),
        "ハネオ" => haneo_action(action, frame),
        "ビョーン" => byoon_action(action, frame),
        "ダッシュ" => dashu_action(action, frame),
        "グルグル" => guruguru_action(action, frame),
        // Normal type
        "ペタ" => peta_action(action, frame),
        "ノホホ" => nohoho_action(action, frame),
        "マジメ" => majime_action(action, frame),
        "フツウ" => futsuu_action(action, frame),
        "ナミナミ" => naminami_action(action, frame),
        "テキトー" => tekitoo_action(action, frame),
        // Wild type
        "メダマ" => medama_action(action, frame),
        "ケモノ" => kemono_action(action, frame),
        "ヌシ" => nushi_action(action, frame),
        "カゲ" => kage_action(action, frame),
        "ザワザワ" => zawazawa_action(action, frame),
        "ヒトダマ" => hitodama_action(action, frame),
        _ => return None,
    };
    Some(art.iter().map(|s: &&str| s.to_string()).collect())
}
// ===== Stage 2 Species Art (hand-crafted, 2-line each) =====

// --- Chikara type ---

// 1. ドタン - Muscular stomper with thick arms
fn dotan_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  ᕙ(≧▽≧)ᕗ!", "    ╚╝╚╝", "", ""],
        (MoodLevel::High, _) => &["", " !ᕙ(≧▽≧)ᕗ", "    ╚╝╚╝", "", ""],
        (MoodLevel::Normal, 0) => &["", "  ᕙ(・益・)ᕗ", "    ╚╝╚╝", "", ""],
        (MoodLevel::Normal, _) => &["", "   ᕙ(・益・)ᕗ", "     ╚╝╚╝", "", ""],
        (MoodLevel::Low, 0) => &["", "  ᕙ(￣_￣)ᕗ", "    ╚╝╚╝", "", ""],
        (MoodLevel::Low, _) => &["", "  ᕙ(￣ ￣)ᕗ", "    ╚╝╚╝", "", ""],
    }
}

fn dotan_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  ᕙ(・益・)ᕗﾉ", "    ╚╝╚╝", "", ""],
        (Action::Talk, _) => &["", " ﾉᕙ(・益・)ᕗ", "    ╚╝╚╝", "", ""],
        (Action::Play, 0) => &["", "  ᕙ(≧▽≧)ᕗ♪", "    ╚╝╚╝", "", ""],
        (Action::Play, _) => &["", " ♪ᕙ(≧▽≧)ᕗ", "    ╚╝╚╝", "", ""],
        (Action::Train, 0) => &["", "  ᕙ(≧益≧)ᕗ!!", "    ╚╝  ╚╝", "", ""],
        (Action::Train, _) => &["", "  ᕙ(≧益≧)ᕗ", "   ╚╝╚╝", "", ""],
        (Action::Relax, 0) => &["", "  ᕙ(˘_˘)ᕗ～", "    ╚╝╚╝", "", ""],
        (Action::Relax, _) => &["", "  ᕙ(˘ ˘)ᕗ～z", "    ╚╝╚╝", "", ""],
    }
}

// 2. ガシャ - Armored shell creature
fn gasha_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  ┏(≧▽≧)┓!", "   ┗━━━┛", "", ""],
        (MoodLevel::High, _) => &["", " !┏(≧▽≧)┓", "   ┗━━━┛", "", ""],
        (MoodLevel::Normal, 0) => &["", "  ┏(・ω・)┓", "   ┗━━━┛", "", ""],
        (MoodLevel::Normal, _) => &["", "   ┏(・ω・)┓", "    ┗━━━┛", "", ""],
        (MoodLevel::Low, 0) => &["", "  ┏(￣_￣)┓", "   ┗━━━┛", "", ""],
        (MoodLevel::Low, _) => &["", "  ┏(￣ ￣)┓", "   ┗━━━┛", "", ""],
    }
}

fn gasha_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  ┏(・ω・)┓ﾉ", "   ┗━━━┛", "", ""],
        (Action::Talk, _) => &["", " ﾉ┏(・ω・)┓", "   ┗━━━┛", "", ""],
        (Action::Play, 0) => &["", "  ┏(≧▽≧)┓♪", "   ┗━━━┛", "", ""],
        (Action::Play, _) => &["", " ♪┏(≧▽≧)┓", "   ┗━━━┛", "", ""],
        (Action::Train, 0) => &["", "  ┏(≧益≧)┓!!", "   ┗━━━━┛", "", ""],
        (Action::Train, _) => &["", "  ┏(≧益≧)┓", "  ┗━━━━┛", "", ""],
        (Action::Relax, 0) => &["", "  ┏(˘ω˘)┓～", "   ┗━━━┛", "", ""],
        (Action::Relax, _) => &["", "  ┏(˘ ˘)┓～z", "   ┗━━━┛", "", ""],
    }
}

// 3. ズンズン - Forward-charging marcher
fn zunzun_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  >(°▽°)>!", "    /▓▓\\", "", ""],
        (MoodLevel::High, _) => &["", "   >(°▽°)>!", "     /▓▓\\", "", ""],
        (MoodLevel::Normal, 0) => &["", "  >(°益°)>", "    /▓▓\\", "", ""],
        (MoodLevel::Normal, _) => &["", "   >(°益°)>", "     /▓▓\\", "", ""],
        (MoodLevel::Low, 0) => &["", "  >(°_°)>", "    /▓▓\\", "", ""],
        (MoodLevel::Low, _) => &["", "  >(° °)>", "    /▓▓\\", "", ""],
    }
}

fn zunzun_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  >(°益°)>ﾉ", "    /▓▓\\", "", ""],
        (Action::Talk, _) => &["", " ﾉ>(°益°)>", "    /▓▓\\", "", ""],
        (Action::Play, 0) => &["", "  >(°▽°)>♪", "    /▓▓\\", "", ""],
        (Action::Play, _) => &["", " ♪>(°▽°)>", "     /▓▓\\", "", ""],
        (Action::Train, 0) => &["", "  >>(°益°)>>!!", "     /▓▓\\", "", ""],
        (Action::Train, _) => &["", "  >(°益°)>", "   /▓▓\\", "", ""],
        (Action::Relax, 0) => &["", "  >(˘_˘)>～", "    /▓▓\\", "", ""],
        (Action::Relax, _) => &["", "  >(˘ ˘)>～z", "    /▓▓\\", "", ""],
    }
}

// 4. デカオ - Comically huge round face
fn dekao_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " ((◎▽◎))!", "     ωω", "", ""],
        (MoodLevel::High, _) => &["", "!((◎▽◎))", "     ωω", "", ""],
        (MoodLevel::Normal, 0) => &["", " ((◎ω◎))", "     ωω", "", ""],
        (MoodLevel::Normal, _) => &["", "  ((◎ω◎))", "      ωω", "", ""],
        (MoodLevel::Low, 0) => &["", " ((◎_◎))", "     ωω", "", ""],
        (MoodLevel::Low, _) => &["", " ((◎ ◎))", "     ωω", "", ""],
    }
}

fn dekao_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " ((◎ω◎))ﾉ", "     ωω", "", ""],
        (Action::Talk, _) => &["", "ﾉ((◎ω◎))", "     ωω", "", ""],
        (Action::Play, 0) => &["", " ((◎▽◎))♪", "     ωω", "", ""],
        (Action::Play, _) => &["", "♪((◎▽◎))", "     ωω", "", ""],
        (Action::Train, 0) => &["", " ((◎益◎))9", "     ωω", "", ""],
        (Action::Train, _) => &["", "9((◎益◎))", "     ωω", "", ""],
        (Action::Relax, 0) => &["", " ((◎_◎))～", "     ωω", "", ""],
        (Action::Relax, _) => &["", " ((◎ ◎))～z", "     ωω", "", ""],
    }
}

// 5. ゴツモリ - Rocky armored creature
fn gotsumori_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " ◆(▲▽▲)◆!", "  ╚████╝", "", ""],
        (MoodLevel::High, _) => &["", "!◆(▲▽▲)◆", "  ╚████╝", "", ""],
        (MoodLevel::Normal, 0) => &["", " ◆(▲益▲)◆", "  ╚████╝", "", ""],
        (MoodLevel::Normal, _) => &["", "  ◆(▲益▲)◆", "   ╚████╝", "", ""],
        (MoodLevel::Low, 0) => &["", " ◆(▲_▲)◆", "  ╚████╝", "", ""],
        (MoodLevel::Low, _) => &["", " ◆(▲ ▲)◆", "  ╚████╝", "", ""],
    }
}

fn gotsumori_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " ◆(▲益▲)◆ﾉ", "  ╚████╝", "", ""],
        (Action::Talk, _) => &["", "ﾉ◆(▲益▲)◆", "  ╚████╝", "", ""],
        (Action::Play, 0) => &["", " ◆(▲▽▲)◆♪", "  ╚████╝", "", ""],
        (Action::Play, _) => &["", "♪◆(▲▽▲)◆", "  ╚████╝", "", ""],
        (Action::Train, 0) => &["", " ◆(▲益▲)◆!!", "  ╚████╝", "", ""],
        (Action::Train, _) => &["", " ◆(▲益▲)◆", " ╚████╝", "", ""],
        (Action::Relax, 0) => &["", " ◆(▲_▲)◆～", "  ╚████╝", "", ""],
        (Action::Relax, _) => &["", " ◆(▲ ▲)◆～z", "  ╚████╝", "", ""],
    }
}

// 6. ドンガメ - Slow turtle with shell
fn dongame_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  (◎▽◎)⊃!", "  甲甲甲甲", "", ""],
        (MoodLevel::High, _) => &["", " !(◎▽◎)⊃", "  甲甲甲甲", "", ""],
        (MoodLevel::Normal, 0) => &["", "  (◎ω◎)⊃", "  甲甲甲甲", "", ""],
        (MoodLevel::Normal, _) => &["", "   (◎ω◎)⊃", "   甲甲甲甲", "", ""],
        (MoodLevel::Low, 0) => &["", "  (◎_◎)⊃", "  甲甲甲甲", "", ""],
        (MoodLevel::Low, _) => &["", "  (◎ ◎)⊃", "  甲甲甲甲", "", ""],
    }
}

fn dongame_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  (◎ω◎)⊃ﾉ", "  甲甲甲甲", "", ""],
        (Action::Talk, _) => &["", " ﾉ(◎ω◎)⊃", "  甲甲甲甲", "", ""],
        (Action::Play, 0) => &["", "  (◎▽◎)⊃♪", "  甲甲甲甲", "", ""],
        (Action::Play, _) => &["", " ♪(◎▽◎)⊃", "   甲甲甲甲", "", ""],
        (Action::Train, 0) => &["", "  (◎益◎)⊃!!", "  甲甲甲甲", "", ""],
        (Action::Train, _) => &["", "  (◎益◎)⊃", "  甲甲甲甲", "", ""],
        (Action::Relax, 0) => &["", "  (◎_◎)⊃～", "  甲甲甲甲", "", ""],
        (Action::Relax, _) => &["", "  (◎ ◎)⊃～z", "  甲甲甲甲", "", ""],
    }
}

// --- Odayaka type ---

// 7. ヒョロン - Thin and lanky
fn hyoron_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  (˘▽˘)ﾉ♪", "   |  |", "", ""],
        (MoodLevel::High, _) => &["", " ♪ﾉ(˘▽˘)", "    |  |", "", ""],
        (MoodLevel::Normal, 0) => &["", "  (˘ᵕ˘)ﾉ", "   |  |", "", ""],
        (MoodLevel::Normal, _) => &["", "   (˘ᵕ˘)ﾉ", "    |  |", "", ""],
        (MoodLevel::Low, 0) => &["", "  (￣_￣)", "   |  |", "", ""],
        (MoodLevel::Low, _) => &["", "  (￣ ￣)", "   |  |", "", ""],
    }
}

fn hyoron_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  (˘ᵕ˘)ﾉ", "   |  |", "", ""],
        (Action::Talk, _) => &["", " ﾉ(˘ᵕ˘)", "   |  |", "", ""],
        (Action::Play, 0) => &["", "  (˘▽˘)ﾉ♪", "   |  |", "", ""],
        (Action::Play, _) => &["", " ♪ﾉ(˘▽˘)", "    |  |", "", ""],
        (Action::Train, 0) => &["", "  (˘益˘)9", "   |  |", "", ""],
        (Action::Train, _) => &["", " 9(˘益˘)", "   |  |", "", ""],
        (Action::Relax, 0) => &["", "  (˘_˘)～", "   |  |", "", ""],
        (Action::Relax, _) => &["", "  (˘ ˘)～z", "   |  |", "", ""],
    }
}

// 8. フワモン - Cloud-wrapped fluffy monster
fn fuwamon_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " ☁(≧▽≦)☁♪", "  ☁☁☁☁", "", ""],
        (MoodLevel::High, _) => &["", "♪☁(≧▽≦)☁", "  ☁☁☁☁", "", ""],
        (MoodLevel::Normal, 0) => &["", " ☁(˘ω˘)☁", "  ☁☁☁☁", "", ""],
        (MoodLevel::Normal, _) => &["", "  ☁(˘ω˘)☁", "   ☁☁☁☁", "", ""],
        (MoodLevel::Low, 0) => &["", " ☁(￣_￣)☁", "  ☁☁☁☁", "", ""],
        (MoodLevel::Low, _) => &["", " ☁(￣ ￣)☁", "  ☁☁☁☁", "", ""],
    }
}

fn fuwamon_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " ☁(˘ω˘)☁ﾉ", "  ☁☁☁☁", "", ""],
        (Action::Talk, _) => &["", "ﾉ☁(˘ω˘)☁", "  ☁☁☁☁", "", ""],
        (Action::Play, 0) => &["", " ☁(≧▽≦)☁♪", "  ☁☁☁☁", "", ""],
        (Action::Play, _) => &["", "♪☁(≧▽≦)☁", "   ☁☁☁☁", "", ""],
        (Action::Train, 0) => &["", " ☁(≧益≦)☁!", "  ☁☁☁☁", "", ""],
        (Action::Train, _) => &["", " ☁(≧益≦)☁", "  ☁☁☁☁", "", ""],
        (Action::Relax, 0) => &["", " ☁(˘_˘)☁～", "  ☁☁☁☁", "", ""],
        (Action::Relax, _) => &["", " ☁(˘ ˘)☁～z", "  ☁☁☁☁", "", ""],
    }
}

// 9. ユラリ - Jellyfish-like swayer
fn yurari_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " ～(・▽・)～♪", "   ∫∫∫∫∫", "", ""],
        (MoodLevel::High, _) => &["", "♪～(・▽・)～", "   ∫∫∫∫∫", "", ""],
        (MoodLevel::Normal, 0) => &["", " ～(・ᵕ・)～", "   ∫∫∫∫∫", "", ""],
        (MoodLevel::Normal, _) => &["", "  ～(・ᵕ・)～", "    ∫∫∫∫∫", "", ""],
        (MoodLevel::Low, 0) => &["", " ～(・_・)～", "   ∫∫∫∫∫", "", ""],
        (MoodLevel::Low, _) => &["", " ～(・ ・)～", "   ∫∫∫∫∫", "", ""],
    }
}

fn yurari_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " ～(・ᵕ・)～ﾉ", "   ∫∫∫∫∫", "", ""],
        (Action::Talk, _) => &["", "ﾉ～(・ᵕ・)～", "   ∫∫∫∫∫", "", ""],
        (Action::Play, 0) => &["", " ～(・▽・)～♪", "   ∫∫∫∫∫", "", ""],
        (Action::Play, _) => &["", "♪～(・▽・)～", "    ∫∫∫∫∫", "", ""],
        (Action::Train, 0) => &["", " ～(・益・)～!", "   ∫∫∫∫∫", "", ""],
        (Action::Train, _) => &["", " ～(・益・)～", "   ∫∫∫∫∫", "", ""],
        (Action::Relax, 0) => &["", " ～(・_・)～～", "   ∫∫∫∫∫", "", ""],
        (Action::Relax, _) => &["", " ～(・ ・)～～z", "   ∫∫∫∫∫", "", ""],
    }
}

// 10. ネムタ - Always sleepy, pudgy
fn nemuta_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  (˘▽˘)ﾉ!", "   ∪∪∪", "", ""],
        (MoodLevel::High, _) => &["", " !ﾉ(˘▽˘)", "   ∪∪∪", "", ""],
        (MoodLevel::Normal, 0) => &["", "  (˘o˘)zzZ", "   ∪∪∪", "", ""],
        (MoodLevel::Normal, _) => &["", "  (˘o˘)zZ", "    ∪∪∪", "", ""],
        (MoodLevel::Low, 0) => &["", "  (￣o￣)zzZ", "   ∪∪∪", "", ""],
        (MoodLevel::Low, _) => &["", "  (￣ ￣)zZ", "   ∪∪∪", "", ""],
    }
}

fn nemuta_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  (˘o˘)ﾉ", "   ∪∪∪", "", ""],
        (Action::Talk, _) => &["", " ﾉ(˘o˘)", "   ∪∪∪", "", ""],
        (Action::Play, 0) => &["", "  (˘▽˘)ﾉ♪", "   ∪∪∪", "", ""],
        (Action::Play, _) => &["", " ♪ﾉ(˘▽˘)", "   ∪∪∪", "", ""],
        (Action::Train, 0) => &["", "  (˘益˘)9!", "   ∪∪∪", "", ""],
        (Action::Train, _) => &["", " 9(˘益˘)", "   ∪∪∪", "", ""],
        (Action::Relax, 0) => &["", "  (˘_˘)zzZ", "   ∪∪∪", "", ""],
        (Action::Relax, _) => &["", "  (˘ ˘)zzZZ", "   ∪∪∪", "", ""],
    }
}

// 11. ポワン - Dreamy bubble creature
fn powan_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " °(˘▽˘)°♪", "   °○°○", "", ""],
        (MoodLevel::High, _) => &["", "♪°(˘▽˘)°", "   °○°○", "", ""],
        (MoodLevel::Normal, 0) => &["", " °(˘ᵕ˘)°", "   °○°○", "", ""],
        (MoodLevel::Normal, _) => &["", "  °(˘ᵕ˘)°", "    °○°○", "", ""],
        (MoodLevel::Low, 0) => &["", " °(￣_￣)°", "   °○°○", "", ""],
        (MoodLevel::Low, _) => &["", " °(￣ ￣)°", "   °○°○", "", ""],
    }
}

fn powan_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " °(˘ᵕ˘)°ﾉ", "   °○°○", "", ""],
        (Action::Talk, _) => &["", "ﾉ°(˘ᵕ˘)°", "   °○°○", "", ""],
        (Action::Play, 0) => &["", " °(˘▽˘)°♪", "   °○°○", "", ""],
        (Action::Play, _) => &["", "♪°(˘▽˘)°", "    °○°○", "", ""],
        (Action::Train, 0) => &["", " °(˘益˘)°!", "   °○°○", "", ""],
        (Action::Train, _) => &["", " °(˘益˘)°", "   °○°○", "", ""],
        (Action::Relax, 0) => &["", " °(˘_˘)°～", "   °○°○", "", ""],
        (Action::Relax, _) => &["", " °(˘ ˘)°～z", "   °○°○", "", ""],
    }
}

// 12. ホワモコ - Warm fluffy creature with ears
fn howamoko_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " ∩(˘▽˘)∩♪", "  ﾜﾜﾜﾜ", "", ""],
        (MoodLevel::High, _) => &["", "♪∩(˘▽˘)∩", "  ﾜﾜﾜﾜ", "", ""],
        (MoodLevel::Normal, 0) => &["", " ∩(˘ω˘)∩", "  ﾜﾜﾜﾜ", "", ""],
        (MoodLevel::Normal, _) => &["", "  ∩(˘ω˘)∩", "   ﾜﾜﾜﾜ", "", ""],
        (MoodLevel::Low, 0) => &["", " ∩(￣_￣)∩", "  ﾜﾜﾜﾜ", "", ""],
        (MoodLevel::Low, _) => &["", " ∩(￣ ￣)∩", "  ﾜﾜﾜﾜ", "", ""],
    }
}

fn howamoko_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " ∩(˘ω˘)∩ﾉ", "  ﾜﾜﾜﾜ", "", ""],
        (Action::Talk, _) => &["", "ﾉ∩(˘ω˘)∩", "  ﾜﾜﾜﾜ", "", ""],
        (Action::Play, 0) => &["", " ∩(˘▽˘)∩♪", "  ﾜﾜﾜﾜ", "", ""],
        (Action::Play, _) => &["", "♪∩(˘▽˘)∩", "   ﾜﾜﾜﾜ", "", ""],
        (Action::Train, 0) => &["", " ∩(˘益˘)∩!", "  ﾜﾜﾜﾜ", "", ""],
        (Action::Train, _) => &["", " ∩(˘益˘)∩", "  ﾜﾜﾜﾜ", "", ""],
        (Action::Relax, 0) => &["", " ∩(˘_˘)∩～", "  ﾜﾜﾜﾜ", "", ""],
        (Action::Relax, _) => &["", " ∩(˘ ˘)∩～z", "  ﾜﾜﾜﾜ", "", ""],
    }
}

// --- Bouken type ---

// 13. クルル - Spinning wheel creature
fn kururu_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " ◎(・▽・)◎!", "   ○○○○", "", ""],
        (MoodLevel::High, _) => &["", "!◎(・▽・)◎", "   ○○○○", "", ""],
        (MoodLevel::Normal, 0) => &["", " ◎(・∀・)◎", "   ○○○○", "", ""],
        (MoodLevel::Normal, _) => &["", "  ◎(・∀・)◎", "    ○○○○", "", ""],
        (MoodLevel::Low, 0) => &["", " ◎(・_・)◎", "   ○○○○", "", ""],
        (MoodLevel::Low, _) => &["", " ◎(・ ・)◎", "   ○○○○", "", ""],
    }
}

fn kururu_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " ◎(・∀・)◎ﾉ", "   ○○○○", "", ""],
        (Action::Talk, _) => &["", "ﾉ◎(・∀・)◎", "   ○○○○", "", ""],
        (Action::Play, 0) => &["", " ◎(・▽・)◎♪", "   ○○○○", "", ""],
        (Action::Play, _) => &["", "♪◎(・▽・)◎", "    ○○○○", "", ""],
        (Action::Train, 0) => &["", " ◎(・益・)◎!!", "   ○○○○", "", ""],
        (Action::Train, _) => &["", " ◎(・益・)◎", "   ○○○○", "", ""],
        (Action::Relax, 0) => &["", " ◎(・_・)◎～", "   ○○○○", "", ""],
        (Action::Relax, _) => &["", " ◎(・ ・)◎～z", "   ○○○○", "", ""],
    }
}

// 14. トゲたろう - Spiky adventurer
fn togetarou_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " /{＞▽＜}\\!", "  ＞＜＞＜", "", ""],
        (MoodLevel::High, _) => &["", "!/{＞▽＜}\\", "  ＞＜＞＜", "", ""],
        (MoodLevel::Normal, 0) => &["", " /{＞ω＜}\\", "  ＞＜＞＜", "", ""],
        (MoodLevel::Normal, _) => &["", "  /{＞ω＜}\\", "   ＞＜＞＜", "", ""],
        (MoodLevel::Low, 0) => &["", " /{＞_＜}\\", "  ＞＜＞＜", "", ""],
        (MoodLevel::Low, _) => &["", " /{＞ ＜}\\", "  ＞＜＞＜", "", ""],
    }
}

fn togetarou_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " /{＞ω＜}\\ﾉ", "  ＞＜＞＜", "", ""],
        (Action::Talk, _) => &["", "ﾉ/{＞ω＜}\\", "  ＞＜＞＜", "", ""],
        (Action::Play, 0) => &["", " /{＞▽＜}\\♪", "  ＞＜＞＜", "", ""],
        (Action::Play, _) => &["", "♪/{＞▽＜}\\", "   ＞＜＞＜", "", ""],
        (Action::Train, 0) => &["", " /{＞益＜}\\!!", "  ＞＜＞＜", "", ""],
        (Action::Train, _) => &["", " /{＞益＜}\\", "  ＞＜＞＜", "", ""],
        (Action::Relax, 0) => &["", " /{＞_＜}\\～", "  ＞＜＞＜", "", ""],
        (Action::Relax, _) => &["", " /{＞ ＜}\\～z", "  ＞＜＞＜", "", ""],
    }
}

// 15. ハネオ - Winged creature
fn haneo_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " 彡(・▽・)彡♪", "     ∧∧", "", ""],
        (MoodLevel::High, _) => &["", "♪彡(・▽・)彡", "     ∧∧", "", ""],
        (MoodLevel::Normal, 0) => &["", " 彡(・ω・)彡", "     ∧∧", "", ""],
        (MoodLevel::Normal, _) => &["", "  彡(・ω・)彡", "      ∧∧", "", ""],
        (MoodLevel::Low, 0) => &["", " 彡(・_・)彡", "     ∧∧", "", ""],
        (MoodLevel::Low, _) => &["", " 彡(・ ・)彡", "     ∧∧", "", ""],
    }
}

fn haneo_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " 彡(・ω・)彡ﾉ", "     ∧∧", "", ""],
        (Action::Talk, _) => &["", "ﾉ彡(・ω・)彡", "     ∧∧", "", ""],
        (Action::Play, 0) => &["", " 彡(・▽・)彡♪", "     ∧∧", "", ""],
        (Action::Play, _) => &["", "♪彡(・▽・)彡", "      ∧∧", "", ""],
        (Action::Train, 0) => &["", " 彡(・益・)彡!!", "     ∧∧", "", ""],
        (Action::Train, _) => &["", " 彡(・益・)彡", "     ∧∧", "", ""],
        (Action::Relax, 0) => &["", " 彡(・_・)彡～", "     ∧∧", "", ""],
        (Action::Relax, _) => &["", " 彡(・ ・)彡～z", "     ∧∧", "", ""],
    }
}

// 16. ビョーン - Bouncy spring creature
fn byoon_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  (＞◡＜)!", "   ∞∞∞∞", "", ""],
        (MoodLevel::High, _) => &["", "  !(＞◡＜)", "    ∞∞∞∞", "", ""],
        (MoodLevel::Normal, 0) => &["", "  (・◡・)", "   ∞∞∞∞", "", ""],
        (MoodLevel::Normal, _) => &["", "   (・◡・)", "    ∞∞∞∞", "", ""],
        (MoodLevel::Low, 0) => &["", "  (・_・)", "   ∞∞∞∞", "", ""],
        (MoodLevel::Low, _) => &["", "  (・ ・)", "   ∞∞∞∞", "", ""],
    }
}

fn byoon_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  (・◡・)ﾉ", "   ∞∞∞∞", "", ""],
        (Action::Talk, _) => &["", " ﾉ(・◡・)", "   ∞∞∞∞", "", ""],
        (Action::Play, 0) => &["", "  (＞◡＜)♪", "   ∞∞∞∞", "", ""],
        (Action::Play, _) => &["", " ♪(＞◡＜)", "    ∞∞∞∞", "", ""],
        (Action::Train, 0) => &["", "  (＞益＜)!!", "   ∞∞∞∞", "", ""],
        (Action::Train, _) => &["", "  (＞益＜)", "   ∞∞∞∞", "", ""],
        (Action::Relax, 0) => &["", "  (・_・)～", "   ∞∞∞∞", "", ""],
        (Action::Relax, _) => &["", "  (・ ・)～z", "   ∞∞∞∞", "", ""],
    }
}

// 17. ダッシュ - Speed runner with motion lines
fn dashu_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " ≫(°▽°)⊃!", "    ⊂⊃", "", ""],
        (MoodLevel::High, _) => &["", "  ≫(°▽°)⊃!", "     ⊂⊃", "", ""],
        (MoodLevel::Normal, 0) => &["", " ≫(°∀°)⊃", "    ⊂⊃", "", ""],
        (MoodLevel::Normal, _) => &["", "  ≫(°∀°)⊃", "     ⊂⊃", "", ""],
        (MoodLevel::Low, 0) => &["", " ≫(°_°)⊃", "    ⊂⊃", "", ""],
        (MoodLevel::Low, _) => &["", " ≫(° °)⊃", "    ⊂⊃", "", ""],
    }
}

fn dashu_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " ≫(°∀°)⊃ﾉ", "    ⊂⊃", "", ""],
        (Action::Talk, _) => &["", "ﾉ≫(°∀°)⊃", "    ⊂⊃", "", ""],
        (Action::Play, 0) => &["", " ≫(°▽°)⊃♪", "    ⊂⊃", "", ""],
        (Action::Play, _) => &["", "♪≫(°▽°)⊃", "     ⊂⊃", "", ""],
        (Action::Train, 0) => &["", " ≫≫(°益°)⊃!!", "      ⊂⊃", "", ""],
        (Action::Train, _) => &["", " ≫(°益°)⊃", "    ⊂⊃", "", ""],
        (Action::Relax, 0) => &["", " ≫(°_°)⊃～", "    ⊂⊃", "", ""],
        (Action::Relax, _) => &["", " ≫(° °)⊃～z", "    ⊂⊃", "", ""],
    }
}

// 18. グルグル - Spiral spinning creature
fn guruguru_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " @(・▽・)@♪", "   ～～～", "", ""],
        (MoodLevel::High, _) => &["", "♪@(・▽・)@", "   ～～～", "", ""],
        (MoodLevel::Normal, 0) => &["", " @(・ω・)@", "   ～～～", "", ""],
        (MoodLevel::Normal, _) => &["", "  @(・ω・)@", "    ～～～", "", ""],
        (MoodLevel::Low, 0) => &["", " @(・_・)@", "   ～～～", "", ""],
        (MoodLevel::Low, _) => &["", " @(・ ・)@", "   ～～～", "", ""],
    }
}

fn guruguru_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " @(・ω・)@ﾉ", "   ～～～", "", ""],
        (Action::Talk, _) => &["", "ﾉ@(・ω・)@", "   ～～～", "", ""],
        (Action::Play, 0) => &["", " @(・▽・)@♪", "   ～～～", "", ""],
        (Action::Play, _) => &["", "♪@(・▽・)@", "    ～～～", "", ""],
        (Action::Train, 0) => &["", " @(・益・)@!!", "   ～～～", "", ""],
        (Action::Train, _) => &["", " @(・益・)@", "   ～～～", "", ""],
        (Action::Relax, 0) => &["", " @(・_・)@～", "   ～～～", "", ""],
        (Action::Relax, _) => &["", " @(・ ・)@～z", "   ～～～", "", ""],
    }
}

// --- Normal type ---

// 19. ペタ - Flat, spread out on ground
fn peta_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  (´・▽・`)♪", "  ＝＝＝＝", "", ""],
        (MoodLevel::High, _) => &["", " ♪(´・▽・`)", "  ＝＝＝＝", "", ""],
        (MoodLevel::Normal, 0) => &["", "  (´・ω・`)", "  ＝＝＝＝", "", ""],
        (MoodLevel::Normal, _) => &["", "   (´・ω・`)", "   ＝＝＝＝", "", ""],
        (MoodLevel::Low, 0) => &["", "  (´・_・`)", "  ＝＝＝＝", "", ""],
        (MoodLevel::Low, _) => &["", "  (´・ ・`)", "  ＝＝＝＝", "", ""],
    }
}

fn peta_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  (´・ω・`)ﾉ", "  ＝＝＝＝", "", ""],
        (Action::Talk, _) => &["", " ﾉ(´・ω・`)", "  ＝＝＝＝", "", ""],
        (Action::Play, 0) => &["", "  (´・▽・`)♪", "  ＝＝＝＝", "", ""],
        (Action::Play, _) => &["", " ♪(´・▽・`)", "   ＝＝＝＝", "", ""],
        (Action::Train, 0) => &["", "  (´・益・`)9", "  ＝＝＝＝", "", ""],
        (Action::Train, _) => &["", " 9(´・益・`)", "  ＝＝＝＝", "", ""],
        (Action::Relax, 0) => &["", "  (´・_・`)～", "  ＝＝＝＝", "", ""],
        (Action::Relax, _) => &["", "  (´・ ・`)～z", "  ＝＝＝＝", "", ""],
    }
}

// 20. ノホホ - Carefree, relaxed posture
fn nohoho_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  (˘▽˘)ﾉ♪", "   / \\", "", ""],
        (MoodLevel::High, _) => &["", " ♪ﾉ(˘▽˘)", "    / \\", "", ""],
        (MoodLevel::Normal, 0) => &["", "  (˘ω˘)ﾉ", "   / \\", "", ""],
        (MoodLevel::Normal, _) => &["", "   (˘ω˘)ﾉ", "    / \\", "", ""],
        (MoodLevel::Low, 0) => &["", "  (˘_˘)", "   / \\", "", ""],
        (MoodLevel::Low, _) => &["", "  (˘ ˘)", "   / \\", "", ""],
    }
}

fn nohoho_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  (˘ω˘)ﾉ", "   / \\", "", ""],
        (Action::Talk, _) => &["", " ﾉ(˘ω˘)", "   / \\", "", ""],
        (Action::Play, 0) => &["", "  (˘▽˘)ﾉ♪", "   / \\", "", ""],
        (Action::Play, _) => &["", " ♪ﾉ(˘▽˘)", "    / \\", "", ""],
        (Action::Train, 0) => &["", "  (˘益˘)9", "   / \\", "", ""],
        (Action::Train, _) => &["", " 9(˘益˘)", "   / \\", "", ""],
        (Action::Relax, 0) => &["", "  (˘_˘)～", "   / \\", "", ""],
        (Action::Relax, _) => &["", "  (˘ ˘)～z", "   / \\", "", ""],
    }
}

// 21. マジメ - Serious, rigid square posture
fn majime_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  [・▽・]!", "   || ||", "", ""],
        (MoodLevel::High, _) => &["", " ![・▽・]", "   || ||", "", ""],
        (MoodLevel::Normal, 0) => &["", "  [・_・]", "   || ||", "", ""],
        (MoodLevel::Normal, _) => &["", "   [・_・]", "    || ||", "", ""],
        (MoodLevel::Low, 0) => &["", "  [￣_￣]", "   || ||", "", ""],
        (MoodLevel::Low, _) => &["", "  [￣ ￣]", "   || ||", "", ""],
    }
}

fn majime_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  [・_・]ﾉ", "   || ||", "", ""],
        (Action::Talk, _) => &["", " ﾉ[・_・]", "   || ||", "", ""],
        (Action::Play, 0) => &["", "  [・▽・]♪", "   || ||", "", ""],
        (Action::Play, _) => &["", " ♪[・▽・]", "    || ||", "", ""],
        (Action::Train, 0) => &["", "  [・益・]9!", "   || ||", "", ""],
        (Action::Train, _) => &["", " 9[・益・]", "   || ||", "", ""],
        (Action::Relax, 0) => &["", "  [・_・]～", "   || ||", "", ""],
        (Action::Relax, _) => &["", "  [・ ・]～z", "   || ||", "", ""],
    }
}

// 22. フツウ - The most ordinary creature
fn futsuu_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  (・▽・)ﾉ!", "   ∪ ∪", "", ""],
        (MoodLevel::High, _) => &["", " !ﾉ(・▽・)", "    ∪ ∪", "", ""],
        (MoodLevel::Normal, 0) => &["", "  (・ω・)", "   ∪ ∪", "", ""],
        (MoodLevel::Normal, _) => &["", "   (・ω・)", "    ∪ ∪", "", ""],
        (MoodLevel::Low, 0) => &["", "  (・_・)", "   ∪ ∪", "", ""],
        (MoodLevel::Low, _) => &["", "  (・ ・)", "   ∪ ∪", "", ""],
    }
}

fn futsuu_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  (・ω・)ﾉ", "   ∪ ∪", "", ""],
        (Action::Talk, _) => &["", " ﾉ(・ω・)", "   ∪ ∪", "", ""],
        (Action::Play, 0) => &["", "  (・▽・)♪", "   ∪ ∪", "", ""],
        (Action::Play, _) => &["", " ♪(・▽・)", "    ∪ ∪", "", ""],
        (Action::Train, 0) => &["", "  (・益・)9", "   ∪ ∪", "", ""],
        (Action::Train, _) => &["", " 9(・益・)", "   ∪ ∪", "", ""],
        (Action::Relax, 0) => &["", "  (・_・)～", "   ∪ ∪", "", ""],
        (Action::Relax, _) => &["", "  (・ ・)～z", "   ∪ ∪", "", ""],
    }
}

// 23. ナミナミ - Wavy, fluid body
fn naminami_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  (˘▽˘)♪", "  ～～～～", "", ""],
        (MoodLevel::High, _) => &["", " ♪(˘▽˘)", "  ～～～～", "", ""],
        (MoodLevel::Normal, 0) => &["", "  (˘ω˘)", "  ～～～～", "", ""],
        (MoodLevel::Normal, _) => &["", "   (˘ω˘)", "   ～～～～", "", ""],
        (MoodLevel::Low, 0) => &["", "  (˘_˘)", "  ～～～～", "", ""],
        (MoodLevel::Low, _) => &["", "  (˘ ˘)", "  ～～～～", "", ""],
    }
}

fn naminami_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  (˘ω˘)ﾉ", "  ～～～～", "", ""],
        (Action::Talk, _) => &["", " ﾉ(˘ω˘)", "  ～～～～", "", ""],
        (Action::Play, 0) => &["", "  (˘▽˘)♪", "  ～～～～", "", ""],
        (Action::Play, _) => &["", " ♪(˘▽˘)", "   ～～～～", "", ""],
        (Action::Train, 0) => &["", "  (˘益˘)9", "  ～～～～", "", ""],
        (Action::Train, _) => &["", " 9(˘益˘)", "  ～～～～", "", ""],
        (Action::Relax, 0) => &["", "  (˘_˘)～", "  ～～～～", "", ""],
        (Action::Relax, _) => &["", "  (˘ ˘)～z", "  ～～～～", "", ""],
    }
}

// 24. テキトー - Slouchy, casual creature
fn tekitoo_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  (´▽`)ﾉ♪", "   _/\\_", "", ""],
        (MoodLevel::High, _) => &["", " ♪ﾉ(´▽`)", "    _/\\_", "", ""],
        (MoodLevel::Normal, 0) => &["", "  (´ω`)>", "   _/\\_", "", ""],
        (MoodLevel::Normal, _) => &["", "   (´ω`)>", "    _/\\_", "", ""],
        (MoodLevel::Low, 0) => &["", "  (´_`)", "   _/\\_", "", ""],
        (MoodLevel::Low, _) => &["", "  (´ `)", "   _/\\_", "", ""],
    }
}

fn tekitoo_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "  (´ω`)>ﾉ", "   _/\\_", "", ""],
        (Action::Talk, _) => &["", " ﾉ(´ω`)>", "   _/\\_", "", ""],
        (Action::Play, 0) => &["", "  (´▽`)>♪", "   _/\\_", "", ""],
        (Action::Play, _) => &["", " ♪(´▽`)>", "    _/\\_", "", ""],
        (Action::Train, 0) => &["", "  (´益`)>9", "   _/\\_", "", ""],
        (Action::Train, _) => &["", " 9(´益`)>", "   _/\\_", "", ""],
        (Action::Relax, 0) => &["", "  (´_`)>～", "   _/\\_", "", ""],
        (Action::Relax, _) => &["", "  (´ `)>～z", "   _/\\_", "", ""],
    }
}

// --- Wild type ---

// 25. メダマ - Giant eyeball creature
fn medama_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " ◎(⊙▽⊙)◎!", "   ┃┃┃┃", "", ""],
        (MoodLevel::High, _) => &["", "!◎(⊙▽⊙)◎", "   ┃┃┃┃", "", ""],
        (MoodLevel::Normal, 0) => &["", " ◎(⊙_⊙)◎", "   ┃┃┃┃", "", ""],
        (MoodLevel::Normal, _) => &["", "  ◎(⊙_⊙)◎", "    ┃┃┃┃", "", ""],
        (MoodLevel::Low, 0) => &["", " ◎(⊙ ⊙)◎", "   ┃┃┃┃", "", ""],
        (MoodLevel::Low, _) => &["", " ◎(- -)◎", "   ┃┃┃┃", "", ""],
    }
}

fn medama_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " ◎(⊙_⊙)◎ﾉ", "   ┃┃┃┃", "", ""],
        (Action::Talk, _) => &["", "ﾉ◎(⊙_⊙)◎", "   ┃┃┃┃", "", ""],
        (Action::Play, 0) => &["", " ◎(⊙▽⊙)◎♪", "   ┃┃┃┃", "", ""],
        (Action::Play, _) => &["", "♪◎(⊙▽⊙)◎", "    ┃┃┃┃", "", ""],
        (Action::Train, 0) => &["", " ◎(⊙益⊙)◎!!", "   ┃┃┃┃", "", ""],
        (Action::Train, _) => &["", " ◎(⊙益⊙)◎", "   ┃┃┃┃", "", ""],
        (Action::Relax, 0) => &["", " ◎(⊙_⊙)◎～", "   ┃┃┃┃", "", ""],
        (Action::Relax, _) => &["", " ◎(- -)◎～z", "   ┃┃┃┃", "", ""],
    }
}

// 26. ケモノ - Feral beast with fangs
fn kemono_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " 牙(⊙▽⊙)牙!", "   爪爪爪", "", ""],
        (MoodLevel::High, _) => &["", "!牙(⊙▽⊙)牙", "   爪爪爪", "", ""],
        (MoodLevel::Normal, 0) => &["", " 牙(⊙ω⊙)牙", "   爪爪爪", "", ""],
        (MoodLevel::Normal, _) => &["", "  牙(⊙ω⊙)牙", "    爪爪爪", "", ""],
        (MoodLevel::Low, 0) => &["", " 牙(⊙_⊙)牙", "   爪爪爪", "", ""],
        (MoodLevel::Low, _) => &["", " 牙(- -)牙", "   爪爪爪", "", ""],
    }
}

fn kemono_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " 牙(⊙ω⊙)牙ﾉ", "   爪爪爪", "", ""],
        (Action::Talk, _) => &["", "ﾉ牙(⊙ω⊙)牙", "   爪爪爪", "", ""],
        (Action::Play, 0) => &["", " 牙(⊙▽⊙)牙♪", "   爪爪爪", "", ""],
        (Action::Play, _) => &["", "♪牙(⊙▽⊙)牙", "    爪爪爪", "", ""],
        (Action::Train, 0) => &["", " 牙(⊙益⊙)牙!!", "   爪爪爪", "", ""],
        (Action::Train, _) => &["", " 牙(⊙益⊙)牙", "   爪爪爪", "", ""],
        (Action::Relax, 0) => &["", " 牙(⊙_⊙)牙～", "   爪爪爪", "", ""],
        (Action::Relax, _) => &["", " 牙(- -)牙～z", "   爪爪爪", "", ""],
    }
}

// 27. ヌシ - Imposing boss creature with crown
fn nushi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " 王(⊙▽⊙)王!", "  ╠████╣", "", ""],
        (MoodLevel::High, _) => &["", "!王(⊙▽⊙)王", "  ╠████╣", "", ""],
        (MoodLevel::Normal, 0) => &["", " 王(⊙益⊙)王", "  ╠████╣", "", ""],
        (MoodLevel::Normal, _) => &["", "  王(⊙益⊙)王", "   ╠████╣", "", ""],
        (MoodLevel::Low, 0) => &["", " 王(⊙_⊙)王", "  ╠████╣", "", ""],
        (MoodLevel::Low, _) => &["", " 王(- -)王", "  ╠████╣", "", ""],
    }
}

fn nushi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " 王(⊙益⊙)王ﾉ", "  ╠████╣", "", ""],
        (Action::Talk, _) => &["", "ﾉ王(⊙益⊙)王", "  ╠████╣", "", ""],
        (Action::Play, 0) => &["", " 王(⊙▽⊙)王♪", "  ╠████╣", "", ""],
        (Action::Play, _) => &["", "♪王(⊙▽⊙)王", "   ╠████╣", "", ""],
        (Action::Train, 0) => &["", " 王(⊙益⊙)王!!", "  ╠████╣", "", ""],
        (Action::Train, _) => &["", " 王(⊙益⊙)王", "  ╠████╣", "", ""],
        (Action::Relax, 0) => &["", " 王(⊙_⊙)王～", "  ╠████╣", "", ""],
        (Action::Relax, _) => &["", " 王(- -)王～z", "  ╠████╣", "", ""],
    }
}

// 28. カゲ - Shadowy, barely visible creature
fn kage_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " ░(⊙▽⊙)░!", "   ▓▓▓▓", "", ""],
        (MoodLevel::High, _) => &["", "!░(⊙▽⊙)░", "   ▓▓▓▓", "", ""],
        (MoodLevel::Normal, 0) => &["", " ░(⊙_⊙)░", "   ▓▓▓▓", "", ""],
        (MoodLevel::Normal, _) => &["", "  ░(⊙_⊙)░", "    ▓▓▓▓", "", ""],
        (MoodLevel::Low, 0) => &["", " ░(- -)░", "   ▓▓▓▓", "", ""],
        (MoodLevel::Low, _) => &["", " ░(  )░", "   ▓▓▓▓", "", ""],
    }
}

fn kage_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " ░(⊙_⊙)░ﾉ", "   ▓▓▓▓", "", ""],
        (Action::Talk, _) => &["", "ﾉ░(⊙_⊙)░", "   ▓▓▓▓", "", ""],
        (Action::Play, 0) => &["", " ░(⊙▽⊙)░♪", "   ▓▓▓▓", "", ""],
        (Action::Play, _) => &["", "♪░(⊙▽⊙)░", "    ▓▓▓▓", "", ""],
        (Action::Train, 0) => &["", " ░(⊙益⊙)░!!", "   ▓▓▓▓", "", ""],
        (Action::Train, _) => &["", " ░(⊙益⊙)░", "   ▓▓▓▓", "", ""],
        (Action::Relax, 0) => &["", " ░(⊙_⊙)░～", "   ▓▓▓▓", "", ""],
        (Action::Relax, _) => &["", " ░(- -)░～z", "   ▓▓▓▓", "", ""],
    }
}

// 29. ザワザワ - Eerie creature with many appendages
fn zawazawa_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " ψ(⊙▽⊙)ψ!", "  ∫∫∫∫∫", "", ""],
        (MoodLevel::High, _) => &["", "!ψ(⊙▽⊙)ψ", "  ∫∫∫∫∫", "", ""],
        (MoodLevel::Normal, 0) => &["", " ψ(⊙_⊙)ψ", "  ∫∫∫∫∫", "", ""],
        (MoodLevel::Normal, _) => &["", "  ψ(⊙_⊙)ψ", "   ∫∫∫∫∫", "", ""],
        (MoodLevel::Low, 0) => &["", " ψ(- -)ψ", "  ∫∫∫∫∫", "", ""],
        (MoodLevel::Low, _) => &["", " ψ(  )ψ", "  ∫∫∫∫∫", "", ""],
    }
}

fn zawazawa_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " ψ(⊙_⊙)ψﾉ", "  ∫∫∫∫∫", "", ""],
        (Action::Talk, _) => &["", "ﾉψ(⊙_⊙)ψ", "  ∫∫∫∫∫", "", ""],
        (Action::Play, 0) => &["", " ψ(⊙▽⊙)ψ♪", "  ∫∫∫∫∫", "", ""],
        (Action::Play, _) => &["", "♪ψ(⊙▽⊙)ψ", "   ∫∫∫∫∫", "", ""],
        (Action::Train, 0) => &["", " ψ(⊙益⊙)ψ!!", "  ∫∫∫∫∫", "", ""],
        (Action::Train, _) => &["", " ψ(⊙益⊙)ψ", "  ∫∫∫∫∫", "", ""],
        (Action::Relax, 0) => &["", " ψ(⊙_⊙)ψ～", "  ∫∫∫∫∫", "", ""],
        (Action::Relax, _) => &["", " ψ(- -)ψ～z", "  ∫∫∫∫∫", "", ""],
    }
}

// 30. ヒトダマ - Ghost fire, floating will-o-wisp
fn hitodama_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " *(⊙▽⊙)*♪", "   ﾟ.ﾟ.ﾟ", "", ""],
        (MoodLevel::High, _) => &["", "♪*(⊙▽⊙)*", "   ﾟ.ﾟ.ﾟ", "", ""],
        (MoodLevel::Normal, 0) => &["", " *(⊙o⊙)*", "   ﾟ.ﾟ.ﾟ", "", ""],
        (MoodLevel::Normal, _) => &["", "  *(⊙o⊙)*", "    ﾟ.ﾟ.ﾟ", "", ""],
        (MoodLevel::Low, 0) => &["", " *(- -)*", "   ﾟ.ﾟ.ﾟ", "", ""],
        (MoodLevel::Low, _) => &["", " *(  )*", "   ﾟ.ﾟ.ﾟ", "", ""],
    }
}

fn hitodama_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", " *(⊙o⊙)*ﾉ", "   ﾟ.ﾟ.ﾟ", "", ""],
        (Action::Talk, _) => &["", "ﾉ*(⊙o⊙)*", "   ﾟ.ﾟ.ﾟ", "", ""],
        (Action::Play, 0) => &["", " *(⊙▽⊙)*♪", "   ﾟ.ﾟ.ﾟ", "", ""],
        (Action::Play, _) => &["", "♪*(⊙▽⊙)*", "    ﾟ.ﾟ.ﾟ", "", ""],
        (Action::Train, 0) => &["", " *(⊙益⊙)*!!", "   ﾟ.ﾟ.ﾟ", "", ""],
        (Action::Train, _) => &["", " *(⊙益⊙)*", "   ﾟ.ﾟ.ﾟ", "", ""],
        (Action::Relax, 0) => &["", " *(⊙_⊙)*～", "   ﾟ.ﾟ.ﾟ", "", ""],
        (Action::Relax, _) => &["", " *(- -)*～z", "   ﾟ.ﾟ.ﾟ", "", ""],
    }
}
