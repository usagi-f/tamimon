//! Hand-crafted ASCII art for all Stage 1 species (including egg).

use crate::game::actions::Action;
use crate::game::pet::MoodLevel;

/// Returns hand-crafted idle art for a Stage 1 species, or None if not found.
pub fn get_s1_art(species: &str, mood: MoodLevel, frame: usize) -> Option<Vec<String>> {
    let art: &[&str] = match species {
        "たまご" => egg_art(),
        "コロコロ" => korokoro_art(mood, frame),
        "ニョロ" => nyoro_art(mood, frame),
        "フワ" => fuwa_art(mood, frame),
        "ツブ" => tsubu_art(mood, frame),
        "プク" => puku_art(mood, frame),
        "ミジン" => mijin_art(mood, frame),
        "ネロ" => nero_art(mood, frame),
        "ボテ" => bote_art(mood, frame),
        "ピリリ" => piriri_art(mood, frame),
        "モグモ" => mogumo_art(mood, frame),
        _ => return None,
    };
    Some(art.iter().map(|s: &&str| s.to_string()).collect())
}

/// Returns hand-crafted action art for a Stage 1 species, or None if not found.
pub fn get_s1_action_art(species: &str, action: Action, frame: usize) -> Option<Vec<String>> {
    let art: &[&str] = match species {
        "たまご" => egg_art(),
        "コロコロ" => korokoro_action(action, frame),
        "ニョロ" => nyoro_action(action, frame),
        "フワ" => fuwa_action(action, frame),
        "ツブ" => tsubu_action(action, frame),
        "プク" => puku_action(action, frame),
        "ミジン" => mijin_action(action, frame),
        "ネロ" => nero_action(action, frame),
        "ボテ" => bote_action(action, frame),
        "ピリリ" => piriri_action(action, frame),
        "モグモ" => mogumo_action(action, frame),
        _ => return None,
    };
    Some(art.iter().map(|s: &&str| s.to_string()).collect())
}

// ===== Egg Art =====

pub fn egg_art() -> &'static [&'static str] {
    &["", "     ／＼", "    （ ？ ）", "     ￣￣￣", ""]
}

// ===== Stage 1 Species Art (hand-crafted, 1-line each) =====

// 1. コロコロ（口: ω）- 丸いスライム
fn korokoro_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "", "    (≧▽≦)ﾉ", "", ""],
        (MoodLevel::High, _) => &["", "", "   ﾉ(≧▽≦)", "", ""],
        (MoodLevel::Normal, 0) => &["", "", "    (˘ω˘)", "", ""],
        (MoodLevel::Normal, _) => &["", "", "     (˘ω˘)", "", ""],
        (MoodLevel::Low, 0) => &["", "", "    (￣_￣)", "", ""],
        (MoodLevel::Low, _) => &["", "", "    (￣ ￣)", "", ""],
    }
}

// 2. ニョロ（口: へ）- ヘビ/ミミズ
fn nyoro_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "", "  ～～(＞へ＜)！", "", ""],
        (MoodLevel::High, _) => &["", "", "  ！(＞へ＜)～～", "", ""],
        (MoodLevel::Normal, 0) => &["", "", "  ～～(・へ・)", "", ""],
        (MoodLevel::Normal, _) => &["", "", "  (・へ・)～～", "", ""],
        (MoodLevel::Low, 0) => &["", "", "  ～(￣へ￣)", "", ""],
        (MoodLevel::Low, _) => &["", "", "  (￣ ￣)～", "", ""],
    }
}

// 3. フワ（口: ᵕ）- ふわふわ雲
fn fuwa_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "", "  ☁(≧▽≦)☁♪", "", ""],
        (MoodLevel::High, _) => &["", "", "  ♪☁(≧▽≦)☁", "", ""],
        (MoodLevel::Normal, 0) => &["", "", "  ☁(˘ᵕ˘)☁", "", ""],
        (MoodLevel::Normal, _) => &["", "", "   ☁(˘ᵕ˘)☁", "", ""],
        (MoodLevel::Low, 0) => &["", "", "  ☁(￣_￣)☁", "", ""],
        (MoodLevel::Low, _) => &["", "", "   ☁(￣ ￣)☁", "", ""],
    }
}

// 4. ツブ（口: _）- 微小粒
fn tsubu_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "", "    ⊙▽⊙ !", "", ""],
        (MoodLevel::High, _) => &["", "", "   ! ⊙▽⊙", "", ""],
        (MoodLevel::Normal, 0) => &["", "", "    ⊙_⊙", "", ""],
        (MoodLevel::Normal, _) => &["", "", "    ⊙ ⊙", "", ""],
        (MoodLevel::Low, 0) => &["", "", "    -_-", "", ""],
        (MoodLevel::Low, _) => &["", "", "    - -", "", ""],
    }
}

// 5. プク（口: ◡）- ぷくぷく
fn puku_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "", "  (｡＞◡＜｡)ﾉ", "", ""],
        (MoodLevel::High, _) => &["", "", "  ﾉ(｡＞◡＜｡)", "", ""],
        (MoodLevel::Normal, 0) => &["", "", "  (｡・◡・｡)", "", ""],
        (MoodLevel::Normal, _) => &["", "", "   (｡・◡・｡)", "", ""],
        (MoodLevel::Low, 0) => &["", "", "  (｡-◡-｡)", "", ""],
        (MoodLevel::Low, _) => &["", "", "  (｡- -｡)", "", ""],
    }
}

// 6. ミジン（口: ▿）- 極小微生物
fn mijin_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "", "    ＞▿＜ !", "", ""],
        (MoodLevel::High, _) => &["", "", "   ! ＞▿＜", "", ""],
        (MoodLevel::Normal, 0) => &["", "", "    ･▿･", "", ""],
        (MoodLevel::Normal, _) => &["", "", "     ･▿･", "", ""],
        (MoodLevel::Low, 0) => &["", "", "    -▿-", "", ""],
        (MoodLevel::Low, _) => &["", "", "    - -", "", ""],
    }
}

// 7. ネロ（口: ー）- 子猫
fn nero_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "", "  (=^▽^=)ﾉ", "", ""],
        (MoodLevel::High, _) => &["", "", "  ﾉ(=^▽^=)", "", ""],
        (MoodLevel::Normal, 0) => &["", "", "  (=ー=)～", "", ""],
        (MoodLevel::Normal, _) => &["", "", "  (=ー=)～～", "", ""],
        (MoodLevel::Low, 0) => &["", "", "  (=- -)zzZ", "", ""],
        (MoodLevel::Low, _) => &["", "", "  (= =)zzZ", "", ""],
    }
}

// 8. ボテ（口: □）- でっぷり
fn bote_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "", "  《・▽・》ﾉ", "", ""],
        (MoodLevel::High, _) => &["", "", "  ﾉ《・▽・》", "", ""],
        (MoodLevel::Normal, 0) => &["", "", "  《・□・》", "", ""],
        (MoodLevel::Normal, _) => &["", "", "  《・ ・》", "", ""],
        (MoodLevel::Low, 0) => &["", "", "  《・_・》", "", ""],
        (MoodLevel::Low, _) => &["", "", "  《- -》", "", ""],
    }
}

// 9. ピリリ（口: ∀）- 電気
fn piriri_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "", "  ⚡°▽°⚡!", "", ""],
        (MoodLevel::High, _) => &["", "", "  !⚡°▽°⚡", "", ""],
        (MoodLevel::Normal, 0) => &["", "", "  ⚡°∀°⚡", "", ""],
        (MoodLevel::Normal, _) => &["", "", "   ⚡°∀°⚡", "", ""],
        (MoodLevel::Low, 0) => &["", "", "    °_°", "", ""],
        (MoodLevel::Low, _) => &["", "", "    ° °", "", ""],
    }
}

// 10. モグモ（口: 〇）- もぐもぐ耳つき
fn mogumo_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "", "  ∩(°▽°)∩ﾉ", "", ""],
        (MoodLevel::High, _) => &["", "", "  ﾉ∩(°▽°)∩", "", ""],
        (MoodLevel::Normal, 0) => &["", "", "  ∩(°〇°)∩", "", ""],
        (MoodLevel::Normal, _) => &["", "", "   ∩(°〇°)∩", "", ""],
        (MoodLevel::Low, 0) => &["", "", "  ∩(°_°)∩", "", ""],
        (MoodLevel::Low, _) => &["", "", "  ∩(° °)∩", "", ""],
    }
}

// ===== Stage 1 Action Art (hand-crafted, 1-line each) =====

// 1. コロコロ（口: ω）
fn korokoro_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "", "    (˘ω˘)ﾉ", "", ""],
        (Action::Talk, _) => &["", "", "   ﾉ(˘ω˘)", "", ""],
        (Action::Play, 0) => &["", "", "   (≧▽≦)ﾉ♪", "", ""],
        (Action::Play, _) => &["", "", "  ♪ﾉ(≧▽≦)", "", ""],
        (Action::Train, 0) => &["", "", "    (≧ω≦)9", "", ""],
        (Action::Train, _) => &["", "", "   9(≧ω≦)", "", ""],
        (Action::Relax, 0) => &["", "", "    (˘ω˘)～", "", ""],
        (Action::Relax, _) => &["", "", "    (˘ω˘)～z", "", ""],
    }
}

// 2. ニョロ（口: へ）
fn nyoro_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "", "  ～～(・へ・)ﾉ", "", ""],
        (Action::Talk, _) => &["", "", "  ﾉ(・へ・)～～", "", ""],
        (Action::Play, 0) => &["", "", "  ～～(＞へ＜)！♪", "", ""],
        (Action::Play, _) => &["", "", "  ♪！(＞へ＜)～～", "", ""],
        (Action::Train, 0) => &["", "", "  ～～(｀へ´)9", "", ""],
        (Action::Train, _) => &["", "", "  9(｀へ´)～～", "", ""],
        (Action::Relax, 0) => &["", "", "  ～～(˘へ˘)～", "", ""],
        (Action::Relax, _) => &["", "", "  ～～(˘へ˘)～z", "", ""],
    }
}

// 3. フワ（口: ᵕ）
fn fuwa_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "", "  ☁(˘ᵕ˘)ﾉ☁", "", ""],
        (Action::Talk, _) => &["", "", "  ☁ﾉ(˘ᵕ˘)☁", "", ""],
        (Action::Play, 0) => &["", "", "  ☁(≧▽≦)☁♪", "", ""],
        (Action::Play, _) => &["", "", "  ♪☁(≧▽≦)☁", "", ""],
        (Action::Train, 0) => &["", "", "  ☁(≧ᵕ≦)9☁", "", ""],
        (Action::Train, _) => &["", "", "  ☁9(≧ᵕ≦)☁", "", ""],
        (Action::Relax, 0) => &["", "", "  ☁(˘ᵕ˘)☁～", "", ""],
        (Action::Relax, _) => &["", "", "  ☁(˘ᵕ˘)☁～z", "", ""],
    }
}

// 4. ツブ（口: _）
fn tsubu_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "", "    ⊙_⊙ ﾉ", "", ""],
        (Action::Talk, _) => &["", "", "   ﾉ ⊙_⊙", "", ""],
        (Action::Play, 0) => &["", "", "    ⊙▽⊙ !", "", ""],
        (Action::Play, _) => &["", "", "   ! ⊙▽⊙", "", ""],
        (Action::Train, 0) => &["", "", "    ⊙益⊙ !!", "", ""],
        (Action::Train, _) => &["", "", "   !! ⊙益⊙", "", ""],
        (Action::Relax, 0) => &["", "", "    ⊙_⊙ ～", "", ""],
        (Action::Relax, _) => &["", "", "    ⊙_⊙ ～z", "", ""],
    }
}

// 5. プク（口: ◡）
fn puku_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "", "  (｡・◡・｡)ﾉ", "", ""],
        (Action::Talk, _) => &["", "", "  ﾉ(｡・◡・｡)", "", ""],
        (Action::Play, 0) => &["", "", "  (｡＞◡＜｡)♪", "", ""],
        (Action::Play, _) => &["", "", "  ♪(｡＞◡＜｡)", "", ""],
        (Action::Train, 0) => &["", "", "  (｡＞◡＜｡)9", "", ""],
        (Action::Train, _) => &["", "", "  9(｡＞◡＜｡)", "", ""],
        (Action::Relax, 0) => &["", "", "  (｡-◡-｡)～", "", ""],
        (Action::Relax, _) => &["", "", "  (｡-◡-｡)～z", "", ""],
    }
}

// 6. ミジン（口: ▿）
fn mijin_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "", "    ･▿･ ﾉ", "", ""],
        (Action::Talk, _) => &["", "", "   ﾉ ･▿･", "", ""],
        (Action::Play, 0) => &["", "", "    ＞▿＜ !", "", ""],
        (Action::Play, _) => &["", "", "   ! ＞▿＜", "", ""],
        (Action::Train, 0) => &["", "", "    ＞益＜ !!", "", ""],
        (Action::Train, _) => &["", "", "   !! ＞益＜", "", ""],
        (Action::Relax, 0) => &["", "", "    -▿- ～", "", ""],
        (Action::Relax, _) => &["", "", "    -▿- ～z", "", ""],
    }
}

// 7. ネロ（口: ー）
fn nero_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "", "  (=ー=)ﾉ～", "", ""],
        (Action::Talk, _) => &["", "", "  ﾉ(=ー=)～", "", ""],
        (Action::Play, 0) => &["", "", "  (=^▽^=)♪～", "", ""],
        (Action::Play, _) => &["", "", "  ～♪(=^▽^=)", "", ""],
        (Action::Train, 0) => &["", "", "  (=`益´=)9～", "", ""],
        (Action::Train, _) => &["", "", "  ～9(=`益´=)", "", ""],
        (Action::Relax, 0) => &["", "", "  (=ー=)～zzZ", "", ""],
        (Action::Relax, _) => &["", "", "  (= =)～zzZ", "", ""],
    }
}

// 8. ボテ（口: □）
fn bote_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "", "  《・□・》ﾉ", "", ""],
        (Action::Talk, _) => &["", "", "  ﾉ《・□・》", "", ""],
        (Action::Play, 0) => &["", "", "  《・▽・》♪", "", ""],
        (Action::Play, _) => &["", "", "  ♪《・▽・》", "", ""],
        (Action::Train, 0) => &["", "", "  《・益・》9", "", ""],
        (Action::Train, _) => &["", "", "  9《・益・》", "", ""],
        (Action::Relax, 0) => &["", "", "  《・□・》～", "", ""],
        (Action::Relax, _) => &["", "", "  《・ ・》～z", "", ""],
    }
}

// 9. ピリリ（口: ∀）
fn piriri_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "", "  ⚡°∀°⚡ﾉ", "", ""],
        (Action::Talk, _) => &["", "", "  ﾉ⚡°∀°⚡", "", ""],
        (Action::Play, 0) => &["", "", "  ⚡°▽°⚡♪!", "", ""],
        (Action::Play, _) => &["", "", "  !♪⚡°▽°⚡", "", ""],
        (Action::Train, 0) => &["", "", "  ⚡°益°⚡9", "", ""],
        (Action::Train, _) => &["", "", "  9⚡°益°⚡", "", ""],
        (Action::Relax, 0) => &["", "", "    °∀°～", "", ""],
        (Action::Relax, _) => &["", "", "    °∀°～z", "", ""],
    }
}

// 10. モグモ（口: 〇）
fn mogumo_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["", "", "  ∩(°〇°)∩ﾉ", "", ""],
        (Action::Talk, _) => &["", "", "  ﾉ∩(°〇°)∩", "", ""],
        (Action::Play, 0) => &["", "", "  ∩(°▽°)∩♪", "", ""],
        (Action::Play, _) => &["", "", "  ♪∩(°▽°)∩", "", ""],
        (Action::Train, 0) => &["", "", "  ∩(°益°)∩9", "", ""],
        (Action::Train, _) => &["", "", "  9∩(°益°)∩", "", ""],
        (Action::Relax, 0) => &["", "", "  ∩(°〇°)∩～", "", ""],
        (Action::Relax, _) => &["", "", "  ∩(° °)∩～z", "", ""],
    }
}
