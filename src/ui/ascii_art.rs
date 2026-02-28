use crate::game::actions::Action;
use crate::game::pet::MoodLevel;

pub fn egg_art() -> &'static [&'static str] {
    &[
        "",
        "      ＿＿",
        "    （　　　）",
        "     ￣￣￣",
        "",
    ]
}

pub fn get_art(species: &str, mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match species {
        "たまご" => egg_art(),
        "コロコロ" => korokoro_art(mood, frame),
        "ニョロ" => nyoro_art(mood, frame),
        "フワ" => fuwa_art(mood, frame),
        "ツブ" => tsubu_art(mood, frame),
        "プク" => puku_art(mood, frame),
        "ミジン" => mijin_art(mood, frame),
        "ネロ" => nero_art(mood, frame),
        "ボテ" => bote_art(mood, frame),
        _ => egg_art(),
    }
}

pub fn get_action_art(species: &str, action: Action) -> &'static [&'static str] {
    match (species, action) {
        (_, Action::Talk) => &[
            "",
            "    (●'◡'●)ノ",
            "     ヾ|",
            "     /|",
            "",
        ],
        (_, Action::Play) => &[
            "      \\(≧▽≦)/",
            "        |",
            "       / \\",
            "",
        ],
        (_, Action::Train) => &[
            "",
            "    (｀・ω・´)9",
            "      ヾ|",
            "      /|",
            "",
        ],
        (_, Action::Relax) => &[
            "",
            "",
            "    _(˘ω˘)_",
            "   zzZ",
            "",
        ],
    }
}

pub fn get_idle_speech(species: &str, mood: MoodLevel) -> &'static [&'static str] {
    match (species, mood) {
        ("たまご", _) => &[
            "（ぴくっ…ぴくっ…）",
            "（もぞもぞ…）",
            "（かたかた…）",
        ],
        (_, MoodLevel::High) => &[
            "今日もげんきだよ！",
            "うれしい！",
            "なんかいい日！",
            "るんるん♪",
        ],
        (_, MoodLevel::Normal) => &[
            "…ぼーっとしてた",
            "なんかいい日",
            "ふぁ〜",
            "…ん？",
        ],
        (_, MoodLevel::Low) => &[
            "…",
            "ねむい",
            "…べつに",
            "（ぼんやり）",
        ],
    }
}

// --- Stage 1 Species Art ---

fn korokoro_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "    (≧▽≦)ノ", "     ヾ|", "     /|", ""],
        (MoodLevel::High, _) => &["", "     (≧▽≦)", "      |ノ", "      |\\", ""],
        (MoodLevel::Normal, 0) => &["", "    (˘ω˘)", "      |", "     / \\", ""],
        (MoodLevel::Normal, _) => &["", "    (˘─˘)", "      |", "     / \\", ""],
        (MoodLevel::Low, 0) => &["", "    (￣_￣)", "      |", "     / \\", ""],
        (MoodLevel::Low, _) => &["", "    (￣ ￣)", "      |", "     / \\", ""],
    }
}

fn nyoro_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", " ≋(＞ω＜)≋", "", ""],
        (MoodLevel::High, _) => &["", "≋(＞ω＜)≋ ", "", ""],
        (MoodLevel::Normal, 0) => &["", " ≋(・ω・)", "", ""],
        (MoodLevel::Normal, _) => &["", " ≋(・─・)", "", ""],
        (MoodLevel::Low, 0) => &["", " ≋(￣_￣)", "", ""],
        (MoodLevel::Low, _) => &["", "≋(￣ ￣) ", "", ""],
    }
}

fn fuwa_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "  ୧(˶≧▽≦)୨", "", ""],
        (MoodLevel::High, _) => &["", " ୧(˶≧▽≦)୨ ", "", ""],
        (MoodLevel::Normal, 0) => &["", "  ୧(˶˘ᵕ˘)", "", ""],
        (MoodLevel::Normal, _) => &["", "  ୧(˶˘─˘)", "", ""],
        (MoodLevel::Low, 0) => &["", "  ୧(˶￣_￣)", "", ""],
        (MoodLevel::Low, _) => &["", "  ୧(˶￣ ￣)", "", ""],
    }
}

fn tsubu_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "    ⊙▽⊙", "", ""],
        (MoodLevel::High, _) => &["", "    ⊙▽⊙ !", "", ""],
        (MoodLevel::Normal, 0) => &["", "    ⊙_⊙", "", ""],
        (MoodLevel::Normal, _) => &["", "    ⊙─⊙", "", ""],
        (MoodLevel::Low, 0) => &["", "    ⊙_⊙", "", ""],
        (MoodLevel::Low, _) => &["", "    -_-", "", ""],
    }
}

fn puku_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "   (｡>ω<｡)", "", ""],
        (MoodLevel::High, _) => &["", "   (｡>ω<｡)ノ", "", ""],
        (MoodLevel::Normal, 0) => &["", "   (｡･ω･｡)", "", ""],
        (MoodLevel::Normal, _) => &["", "   (｡･─･｡)", "", ""],
        (MoodLevel::Low, 0) => &["", "   (｡-ω-｡)", "", ""],
        (MoodLevel::Low, _) => &["", "   (｡- -｡)", "", ""],
    }
}

fn mijin_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "    ＞ω＜", "", ""],
        (MoodLevel::High, _) => &["", "   ＞ω＜ !", "", ""],
        (MoodLevel::Normal, 0) => &["", "    ･ω･", "", ""],
        (MoodLevel::Normal, _) => &["", "    ･─･", "", ""],
        (MoodLevel::Low, 0) => &["", "    -ω-", "", ""],
        (MoodLevel::Low, _) => &["", "    - -", "", ""],
    }
}

fn nero_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "   (^ω^)", "", ""],
        (MoodLevel::High, _) => &["", "   (^ω^)ノ", "", ""],
        (MoodLevel::Normal, 0) => &["", "   (=ω=)", "", ""],
        (MoodLevel::Normal, _) => &["", "   (=─=)", "", ""],
        (MoodLevel::Low, 0) => &["", "   (-ω-) zzZ", "", ""],
        (MoodLevel::Low, _) => &["", "   (- -) zzZ", "", ""],
    }
}

fn bote_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["", "   (・▽・)", "", ""],
        (MoodLevel::High, _) => &["", "   (・▽・)ノ", "", ""],
        (MoodLevel::Normal, 0) => &["", "   (・●・)", "", ""],
        (MoodLevel::Normal, _) => &["", "   (・─・)", "", ""],
        (MoodLevel::Low, 0) => &["", "   (・_・)", "", ""],
        (MoodLevel::Low, _) => &["", "   (・ ・)", "", ""],
    }
}
