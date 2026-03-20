use rand::seq::SliceRandom;
use rand::Rng;

use crate::save::schema::PetData;

/// Random event result
#[allow(dead_code)]
pub struct EventResult {
    pub message: String,
    pub is_death: bool,
    pub survived_accident: bool,
}

/// Positive/neutral event definition
struct PositiveEvent {
    probability: f64,
    message: &'static str,
    apply: fn(&mut PetData, &mut dyn rand::RngCore),
}

const POSITIVE_EVENTS: &[PositiveEvent] = &[
    PositiveEvent {
        probability: 0.15,
        message: "🌈 気分がいい日だったみたい！",
        apply: |pet, _rng| {
            pet.kimochi = (pet.kimochi + 15.0).min(100.0);
        },
    },
    PositiveEvent {
        probability: 0.10,
        message: "🔍 なにかを見つけたみたい！",
        apply: |pet, rng| match rng.gen_range(0u8..3) {
            0 => pet.kimochi = (pet.kimochi + 20.0).min(100.0),
            1 => pet.genki = (pet.genki + 20.0).min(100.0),
            _ => pet.nakayoshi = (pet.nakayoshi + 20.0).min(100.0),
        },
    },
    PositiveEvent {
        probability: 0.08,
        message: "🗺 探検してたみたい！",
        apply: |pet, _rng| {
            pet.genki = (pet.genki + 15.0).min(100.0);
            pet.type_scores.bouken += 1;
        },
    },
    PositiveEvent {
        probability: 0.08,
        message: "☀️ ひなたぼっこしてたみたい！",
        apply: |pet, _rng| {
            pet.kimochi = (pet.kimochi + 10.0).min(100.0);
            pet.nakayoshi = (pet.nakayoshi + 5.0).min(100.0);
        },
    },
    PositiveEvent {
        probability: 0.05,
        message: "💭 不思議な夢を見たみたい！",
        apply: |pet, rng| match rng.gen_range(0u8..3) {
            0 => pet.type_scores.chikara += 1,
            1 => pet.type_scores.odayaka += 1,
            _ => pet.type_scores.bouken += 1,
        },
    },
    PositiveEvent {
        probability: 0.03,
        message: "🤝 誰かと友達になったみたい！",
        apply: |pet, _rng| {
            pet.nakayoshi = (pet.nakayoshi + 25.0).min(100.0);
        },
    },
];

/// Accident messages (cause of death)
const ACCIDENT_MESSAGES: &[&str] = &[
    "⚡ 突然の雷雨。{name}は空を見上げ、そして…",
    "🕳 散歩中に突然消えた。穴の底はとても深かったようだ",
    "⚔️ 見知らぬ強敵に立ち向かっていった。勇敢だった",
    "🌪 大嵐の夜。翌朝、姿が見えなくなっていた",
    "🌌 ある朝、{name}はどこかへ旅立っていた。元気でいるといい",
    "🪨 平らな道で石につまずいた。そのまま帰ってこなかった",
    "🍌 バナナの皮で滑った。どこからバナナの皮が…",
    "😱 自分の影を見て驚き、そのまま走り去って帰ってこなかった",
    "💤 昼寝をしたまま、どうも夢の方が気に入ったらしい",
    "☁️ 雲を眺めながら歩いていたら、気づいたらいなくなっていた",
    "🌿 風で飛んできた葉っぱに包まれた。そのまま転がっていった",
    "👁 散歩中、向こうから来た何かとじっと見つめ合い、一緒に去っていった",
    "🤧 大きなくしゃみをしたら、どこかへ飛んでいった",
    "🪞 はじめて鏡を見た。しばらく見つめた後、鏡の中に入ろうとして消えた",
    "👃 どこからか漂ってきたにおいを追いかけていった。もどってこなかった",
    "💨 理由は不明だが、ある瞬間から全力で走り始め、視界の外へ消えた",
    "🗿 道端の石に話しかけられたらしく、長い時間会話した後、ついていった",
    "⛰️ 坂の上から転がり始めた。かなり遠くまで転がった模様",
    "🌤 お昼寝中にふわふわ浮き始め、そのまま上の方へ消えていった",
    "⛏ なにかを探して穴を掘り続けていた。深すぎて戻れなくなったようだ",
    "🚪 特に理由はないが、気が向いたらしく、どこかへ歩いていった",
];

/// Longevity messages (more likely for pets aged 7+ days)
const LONGEVITY_MESSAGES: &[&str] = &[
    "🌙 {name}はある夜、静かに目を閉じた。穏やかな顔をしていた",
    "☀️ {name}は満足そうにあくびをして、そのまま動かなくなった",
    "🎒 もういい歳だからと言って、朝から荷物をまとめ始めた。行き先は教えてくれなかった",
    "✅ {name}はなにかをやり遂げた顔をしていた。何をやり遂げたかは不明だ",
    "💬 最後に何かひとこと言いかけて、そのまま消えた。聞き取れなかった",
    "🍂 {name}は長生きした。たぶん本人もびっくりしていたと思う",
    "🌇 夕焼けを眺めながら、ゆっくりと遠ざかっていった",
];

/// "Survived" messages
const SURVIVED_MESSAGES: &[&str] = &[
    "💫 危ないところだった…！でも{name}は無事だった！",
    "🍀 {name}はかろうじて助かった…！奇跡だ！",
    "⭐ あぶなかった！でも{name}は帰ってきた！",
];

/// Process random events during startup
/// One roll per hour of elapsed time
pub fn process_offline_events(
    pet: &mut PetData,
    elapsed_ticks: u64,
    rng: &mut impl Rng,
) -> Vec<EventResult> {
    let mut results = Vec::new();

    // No events for eggs
    if pet.stage == 0 {
        return results;
    }

    // One roll per hour elapsed (minimum 1)
    let rolls = (elapsed_ticks / 60).max(1) as usize;

    for _ in 0..rolls {
        // Check for accident first
        if let Some(death_result) = check_accident(pet, rng) {
            let is_death = death_result.is_death;
            results.push(death_result);
            if is_death {
                return results; // Immediately return if dead
            }
        }

        // Roll for positive event
        if let Some(event_result) = roll_positive_event(pet, rng) {
            results.push(event_result);
        }
    }

    results
}

/// Accident check
/// Accident probability per tick = day_age × 0.00347%
fn check_accident(pet: &mut PetData, rng: &mut impl Rng) -> Option<EventResult> {
    let day_age = pet.age_ticks as f64 / 1440.0;
    let accident_prob_per_tick = day_age * 0.0000347;
    // Aggregate accident probability over 1 hour (60 ticks)
    let accident_prob = 1.0 - (1.0 - accident_prob_per_tick).powi(60);

    if rng.gen::<f64>() >= accident_prob {
        return None;
    }

    // Accident occurred! Check for survival
    if !pet.survived_accident {
        let nakayoshi_ratio = pet.nakayoshi / 100.0;
        let survive_prob = nakayoshi_ratio * nakayoshi_ratio * 0.80;

        if rng.gen::<f64>() < survive_prob {
            pet.survived_accident = true;
            let name = pet.display_name();
            let msg_template = SURVIVED_MESSAGES.choose(rng).unwrap();
            let msg = msg_template.replace("{name}", name);
            return Some(EventResult {
                message: msg,
                is_death: false,
                survived_accident: true,
            });
        }
    }

    // Death confirmed
    let name = pet.display_name();

    // At day_age 7+, longevity messages are more likely
    let msg_template = if day_age >= 7.0 && rng.gen::<f64>() < 0.4 {
        LONGEVITY_MESSAGES.choose(rng).unwrap()
    } else {
        ACCIDENT_MESSAGES.choose(rng).unwrap()
    };
    let msg = msg_template.replace("{name}", name);

    Some(EventResult {
        message: msg,
        is_death: true,
        survived_accident: false,
    })
}

/// Roll for positive events
fn roll_positive_event(pet: &mut PetData, rng: &mut impl Rng) -> Option<EventResult> {
    let roll: f64 = rng.gen();
    let mut cumulative = 0.0;

    for event in POSITIVE_EVENTS {
        cumulative += event.probability;
        if roll < cumulative {
            (event.apply)(pet, rng);
            return Some(EventResult {
                message: event.message.to_string(),
                is_death: false,
                survived_accident: false,
            });
        }
    }

    None // Nothing happened
}
