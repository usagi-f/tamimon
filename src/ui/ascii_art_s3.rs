//! Hand-crafted ASCII art for all Stage 3 species.
//!
//! Each species has a unique silhouette distinguishable from all others.

use crate::game::actions::Action;
use crate::game::pet::MoodLevel;

/// Returns hand-crafted idle art for a Stage 3 species, or None if not found.
pub fn get_s3_art(species: &str, mood: MoodLevel, frame: usize) -> Option<Vec<String>> {
    let art: &[&str] = match species {
        // Bouken type
        "ガニ" => gani_art(mood, frame),
        "トビオ" => tobio_art(mood, frame),
        "マルマル" => marumaru_art(mood, frame),
        "ハヤテ" => hayate_art(mood, frame),
        "グルグルン" => gurugurun_art(mood, frame),
        "カゼノコ" => kazenoko_art(mood, frame),
        "ドカーン" => dokaan_art(mood, frame),
        "スイスイ" => suisui_art(mood, frame),
        "サスライ" => sasurai_art(mood, frame),
        "ピカッ" => pikat_art(mood, frame),
        "バサバサ" => basabasa_art(mood, frame),
        "ウロチョロ" => urochoro_art(mood, frame),
        "ゴーゴー" => googoo_art(mood, frame),
        "クモノス" => kumonos_art(mood, frame),
        "ホシゾラ" => hoshizora_art(mood, frame),
        "ブッチギリ" => bucchigiri_art(mood, frame),
        "ワタリ" => watari_art(mood, frame),
        "ヒュー" => hyuu_art(mood, frame),
        "タンケン" => tanken_art(mood, frame),
        "ジェット" => jetto_art(mood, frame),
        // Normal type
        "ノーマル" => noomaru_art(mood, frame),
        "ヘイボン" => heibon_art(mood, frame),
        "タソガレ" => tasogare_art(mood, frame),
        "ニッコリ" => nikkori_art(mood, frame),
        "ダラーン" => daraan_art(mood, frame),
        "キッチリ" => kicchiri_art(mood, frame),
        "ボチボチ" => bochibochi_art(mood, frame),
        "マアマア" => maamaa_art(mood, frame),
        "フニャ" => funya_art(mood, frame),
        "テンテン" => tenten_art(mood, frame),
        "ナァナァ" => naanaa_art(mood, frame),
        "ポツリ" => potsuri_art(mood, frame),
        "ソレナリ" => sorenari_art(mood, frame),
        "ウンウン" => unun_art(mood, frame),
        "チャッカリ" => chakkari_art(mood, frame),
        "ヌルリ" => nururi_art(mood, frame),
        "ヤレヤレ" => yareyare_art(mood, frame),
        "ドッコイ" => dokkoi_art(mood, frame),
        "パッパ" => pappa_art(mood, frame),
        "オットリ" => ottori_art(mood, frame),
        // Chikara type
        "ドドン" => dodon_art(mood, frame),
        "タワーン" => tawaan_art(mood, frame),
        "ゴウケン" => gouken_art(mood, frame),
        "テッカイ" => tekkai_art(mood, frame),
        "ブンブン" => bunbun_art(mood, frame),
        "ガンテツ" => gantetsu_art(mood, frame),
        "ドスコイ" => dosukoi_art(mood, frame),
        "バリバリ" => baribari_art(mood, frame),
        "メガトン" => megaton_art(mood, frame),
        "グランド" => gurando_art(mood, frame),
        "イカヅチ" => ikazuchi_art(mood, frame),
        "ゴリラン" => goriran_art(mood, frame),
        "ダイガン" => daigan_art(mood, frame),
        "ゴロゴロ" => gorogoro_art(mood, frame),
        "カチワリ" => kachiwari_art(mood, frame),
        "テツジン" => tetsujin_art(mood, frame),
        "ドゴン" => dogon_art(mood, frame),
        "バンカー" => bankaa_art(mood, frame),
        "マッスル" => massuru_art(mood, frame),
        "イワオ" => iwao_art(mood, frame),
        // Odayaka type
        "ながれもん" => nagaremon_art(mood, frame),
        "フワリン" => fuwarin_art(mood, frame),
        "モコモコ" => mokomoko_art(mood, frame),
        "ネンネ" => nenne_art(mood, frame),
        "ポヨン" => poyon_art(mood, frame),
        "スヤスヤ" => suyasuya_art(mood, frame),
        "カスミ" => kasumi_art(mood, frame),
        "ノドカ" => nodoka_art(mood, frame),
        "ユメミ" => yumemi_art(mood, frame),
        "ボンヤリ" => bonyari_art(mood, frame),
        "ヒラタ" => hirata_art(mood, frame),
        "コロリン" => kororin_art(mood, frame),
        "ムニャ" => munya_art(mood, frame),
        "マッタリ" => mattari_art(mood, frame),
        "ホワワ" => howawa_art(mood, frame),
        "シズカ" => shizuka_art(mood, frame),
        "モグモグ" => mogumogu_art(mood, frame),
        "トロン" => toron_art(mood, frame),
        "ユッタリ" => yuttari_art(mood, frame),
        "ソヨカゼ" => soyokaze_art(mood, frame),
        // Wild type
        "ヤミノメ" => yaminome_art(mood, frame),
        "オオヌシ" => oonushi_art(mood, frame),
        "バケモノ" => bakemono_art(mood, frame),
        "ユウレイ" => yuurei_art(mood, frame),
        "ヤセイジ" => yaseiji_art(mood, frame),
        "シンエン" => shinen_art(mood, frame),
        "ノラクロ" => norakuro_art(mood, frame),
        "モノノケ" => mononoke_art(mood, frame),
        "クライ" => kurai_art(mood, frame),
        "アヤシイ" => ayashii_art(mood, frame),
        "ムジナ" => mujina_art(mood, frame),
        "ヌエ" => nue_art(mood, frame),
        "カマイタチ" => kamaitachi_art(mood, frame),
        "ドロドロ" => dorodoro_art(mood, frame),
        "ヒノタマ" => hinotama_art(mood, frame),
        "フルエ" => furue_art(mood, frame),
        "ケダマ" => kedama_art(mood, frame),
        "シノビ" => shinobi_art(mood, frame),
        "ジゴク" => jigoku_art(mood, frame),
        "ムゲン" => mugen_art(mood, frame),
        _ => return None,
    };
    Some(art.iter().map(|s: &&str| s.to_string()).collect())
}

/// Returns hand-crafted action art for a Stage 3 species, or None if not found.
pub fn get_s3_action_art(species: &str, action: Action, frame: usize) -> Option<Vec<String>> {
    let art: &[&str] = match species {
        // Bouken type
        "ガニ" => gani_action(action, frame),
        "トビオ" => tobio_action(action, frame),
        "マルマル" => marumaru_action(action, frame),
        "ハヤテ" => hayate_action(action, frame),
        "グルグルン" => gurugurun_action(action, frame),
        "カゼノコ" => kazenoko_action(action, frame),
        "ドカーン" => dokaan_action(action, frame),
        "スイスイ" => suisui_action(action, frame),
        "サスライ" => sasurai_action(action, frame),
        "ピカッ" => pikat_action(action, frame),
        "バサバサ" => basabasa_action(action, frame),
        "ウロチョロ" => urochoro_action(action, frame),
        "ゴーゴー" => googoo_action(action, frame),
        "クモノス" => kumonos_action(action, frame),
        "ホシゾラ" => hoshizora_action(action, frame),
        "ブッチギリ" => bucchigiri_action(action, frame),
        "ワタリ" => watari_action(action, frame),
        "ヒュー" => hyuu_action(action, frame),
        "タンケン" => tanken_action(action, frame),
        "ジェット" => jetto_action(action, frame),
        // Normal type
        "ノーマル" => noomaru_action(action, frame),
        "ヘイボン" => heibon_action(action, frame),
        "タソガレ" => tasogare_action(action, frame),
        "ニッコリ" => nikkori_action(action, frame),
        "ダラーン" => daraan_action(action, frame),
        "キッチリ" => kicchiri_action(action, frame),
        "ボチボチ" => bochibochi_action(action, frame),
        "マアマア" => maamaa_action(action, frame),
        "フニャ" => funya_action(action, frame),
        "テンテン" => tenten_action(action, frame),
        "ナァナァ" => naanaa_action(action, frame),
        "ポツリ" => potsuri_action(action, frame),
        "ソレナリ" => sorenari_action(action, frame),
        "ウンウン" => unun_action(action, frame),
        "チャッカリ" => chakkari_action(action, frame),
        "ヌルリ" => nururi_action(action, frame),
        "ヤレヤレ" => yareyare_action(action, frame),
        "ドッコイ" => dokkoi_action(action, frame),
        "パッパ" => pappa_action(action, frame),
        "オットリ" => ottori_action(action, frame),
        // Chikara type
        "ドドン" => dodon_action(action, frame),
        "タワーン" => tawaan_action(action, frame),
        "ゴウケン" => gouken_action(action, frame),
        "テッカイ" => tekkai_action(action, frame),
        "ブンブン" => bunbun_action(action, frame),
        "ガンテツ" => gantetsu_action(action, frame),
        "ドスコイ" => dosukoi_action(action, frame),
        "バリバリ" => baribari_action(action, frame),
        "メガトン" => megaton_action(action, frame),
        "グランド" => gurando_action(action, frame),
        "イカヅチ" => ikazuchi_action(action, frame),
        "ゴリラン" => goriran_action(action, frame),
        "ダイガン" => daigan_action(action, frame),
        "ゴロゴロ" => gorogoro_action(action, frame),
        "カチワリ" => kachiwari_action(action, frame),
        "テツジン" => tetsujin_action(action, frame),
        "ドゴン" => dogon_action(action, frame),
        "バンカー" => bankaa_action(action, frame),
        "マッスル" => massuru_action(action, frame),
        "イワオ" => iwao_action(action, frame),
        // Odayaka type
        "ながれもん" => nagaremon_action(action, frame),
        "フワリン" => fuwarin_action(action, frame),
        "モコモコ" => mokomoko_action(action, frame),
        "ネンネ" => nenne_action(action, frame),
        "ポヨン" => poyon_action(action, frame),
        "スヤスヤ" => suyasuya_action(action, frame),
        "カスミ" => kasumi_action(action, frame),
        "ノドカ" => nodoka_action(action, frame),
        "ユメミ" => yumemi_action(action, frame),
        "ボンヤリ" => bonyari_action(action, frame),
        "ヒラタ" => hirata_action(action, frame),
        "コロリン" => kororin_action(action, frame),
        "ムニャ" => munya_action(action, frame),
        "マッタリ" => mattari_action(action, frame),
        "ホワワ" => howawa_action(action, frame),
        "シズカ" => shizuka_action(action, frame),
        "モグモグ" => mogumogu_action(action, frame),
        "トロン" => toron_action(action, frame),
        "ユッタリ" => yuttari_action(action, frame),
        "ソヨカゼ" => soyokaze_action(action, frame),
        // Wild type
        "ヤミノメ" => yaminome_action(action, frame),
        "オオヌシ" => oonushi_action(action, frame),
        "バケモノ" => bakemono_action(action, frame),
        "ユウレイ" => yuurei_action(action, frame),
        "ヤセイジ" => yaseiji_action(action, frame),
        "シンエン" => shinen_action(action, frame),
        "ノラクロ" => norakuro_action(action, frame),
        "モノノケ" => mononoke_action(action, frame),
        "クライ" => kurai_action(action, frame),
        "アヤシイ" => ayashii_action(action, frame),
        "ムジナ" => mujina_action(action, frame),
        "ヌエ" => nue_action(action, frame),
        "カマイタチ" => kamaitachi_action(action, frame),
        "ドロドロ" => dorodoro_action(action, frame),
        "ヒノタマ" => hinotama_action(action, frame),
        "フルエ" => furue_action(action, frame),
        "ケダマ" => kedama_action(action, frame),
        "シノビ" => shinobi_action(action, frame),
        "ジゴク" => jigoku_action(action, frame),
        "ムゲン" => mugen_action(action, frame),
        _ => return None,
    };
    Some(art.iter().map(|s: &&str| s.to_string()).collect())
}

// ============================================================
// CHIKARA TYPE Stage 3 Species
// ============================================================

// ============================================================
// CHIKARA TYPE Stage 3 Species
// ============================================================

// --- ドドン (dodon) - Giant taiko drum, NO limbs, vibrates ---
fn dodon_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[")))◎◎◎(((", " ))◎◎◎(( !", "  )◎◎◎(  ", "", ""],
        (MoodLevel::High, _) => &[" ))◎◎◎(( ", ")))◎◎◎(((♪", " ))◎◎◎(( ", "", ""],
        (MoodLevel::Normal, 0) => &[" ))◎◎◎(( ", "  ◎◎◎◎◎  ", " ))◎◎◎(( ", "", ""],
        (MoodLevel::Normal, _) => &["  )◎◎◎(  ", "  ◎◎◎◎◎  ", "  )◎◎◎(  ", "", ""],
        (MoodLevel::Low, 0) => &["   ◎◎◎   ", "  ◎◎◎◎◎  ", "   ◎◎◎   ", "", ""],
        (MoodLevel::Low, _) => &["   ◎◎◎   ", "  ◎◎◎◎◎  ", "   ...    ", "", ""],
    }
}
fn dodon_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[")))◎◎◎(((", " DOOM!!   ", ")))◎◎◎(((", "", ""],
        (Action::Talk, _) => &[" ))◎◎◎(( ", "   BOOM!! ", " ))◎◎◎(( ", "", ""],
        (Action::Play, 0) => &["~))◎◎◎((~", " ♪◎◎◎◎◎♪ ", "~))◎◎◎((~", "", ""],
        (Action::Play, _) => &[" ♪)◎◎◎(♪ ", "~◎◎◎◎◎◎◎~", " ♪)◎◎◎(♪ ", "", ""],
        (Action::Train, 0) => &["»»»◎◎◎«««", " »»◎◎◎«« ", "  »◎◎◎«  ", "", ""],
        (Action::Train, _) => &["  »◎◎◎«  ", " »»◎◎◎«« ", "»»»◎◎◎«««", "", ""],
        (Action::Relax, 0) => &["   ◎◎◎   ", "  ◎◎◎◎◎  ", "  ~~~~~   ", "", ""],
        (Action::Relax, _) => &["   ◎◎◎   ", "  ◎◎◎◎◎  ", "   ~~~    ", "", ""],
    }
}

// --- タワーン (tawaan) - Tall stack, ONE eye on top ---
fn tawaan_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["    [◉]!", "    [█]  ", "   [███] ", "", ""],
        (MoodLevel::High, _) => &["   ![◉]  ", "    [█]  ", "   [███] ", "", ""],
        (MoodLevel::Normal, 0) => &["    [◉]  ", "    [█]  ", "   [███] ", "", ""],
        (MoodLevel::Normal, _) => &["    [◉]  ", "    [█]  ", "   [███] ", "", ""],
        (MoodLevel::Low, 0) => &["    [.]  ", "    [█]  ", "  [█████]", "", ""],
        (MoodLevel::Low, _) => &["   [.]   ", "   [██]  ", "  [█████]", "", ""],
    }
}
fn tawaan_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["    [◉]~ ", "    [█]  ", "   [███] ", "", ""],
        (Action::Talk, _) => &["   ~[◉]  ", "    [█]  ", "   [███] ", "", ""],
        (Action::Play, 0) => &["   [◉]♪  ", "   [██]  ", "   [███] ", "", ""],
        (Action::Play, _) => &["  ♪[◉]   ", "   [██]  ", "   [███] ", "", ""],
        (Action::Train, 0) => &["    [◉]↑ ", "   [███] ", "  [█████]", "", ""],
        (Action::Train, _) => &["   ↑[◉]  ", "   [███] ", "  [█████]", "", ""],
        (Action::Relax, 0) => &["    [‐]  ", "    [█]  ", "   [███] ", "", ""],
        (Action::Relax, _) => &["    [‐]  ", "    [█]  ", "   [███] ", "", ""],
    }
}

// --- ゴウケン (gouken) - IS a giant fist, no face ---
fn gouken_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ╦╦╦╦╦! ", " ║█████║ ", "  ╚═══╝  ", "", ""],
        (MoodLevel::High, _) => &[" !╦╦╦╦╦  ", " ║█████║ ", "  ╚═══╝  ", "", ""],
        (MoodLevel::Normal, 0) => &["  ╦╦╦╦╦  ", " ║█████║ ", "  ╚═══╝  ", "", ""],
        (MoodLevel::Normal, _) => &["  ╦╦╦╦╦  ", " ║█████║ ", "  ╚═══╝  ", "", ""],
        (MoodLevel::Low, 0) => &["  ╦╦╦╦╦  ", " ║.....║ ", "  ╚───╝  ", "", ""],
        (MoodLevel::Low, _) => &["  ╦╦╦╦   ", " ║.....║ ", "  ╚───╝  ", "", ""],
    }
}
fn gouken_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ╦╦╦╦╦  ", " ║█████║~", "  ╚═══╝  ", "", ""],
        (Action::Talk, _) => &["  ╦╦╦╦╦  ", "~║█████║ ", "  ╚═══╝  ", "", ""],
        (Action::Play, 0) => &["  ╦╦╦╦╦♪ ", " ║█████║ ", "  ╚═══╝  ", "", ""],
        (Action::Play, _) => &[" ♪╦╦╦╦╦  ", " ║█████║ ", "  ╚═══╝  ", "", ""],
        (Action::Train, 0) => &["  ╦╦╦╦╦>>", " ║█████║ ", "  ╚═══╝  ", "", ""],
        (Action::Train, _) => &[">>╦╦╦╦╦  ", " ║█████║ ", "  ╚═══╝  ", "", ""],
        (Action::Relax, 0) => &["  ╦╦╦╦╦  ", " ║ ... ║ ", "  ╚───╝  ", "", ""],
        (Action::Relax, _) => &["  ╦╦╦╦╦  ", " ║  .  ║ ", "  ╚───╝  ", "", ""],
    }
}

// --- テッカイ (tekkai) - Tank/pillbox, single visor slit ---
fn tekkai_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ┌══════┐", " │═◄══►═│!", " └██████┘", "", ""],
        (MoodLevel::High, _) => &[" ┌══════┐", "!│═◄══►═│", " └██████┘", "", ""],
        (MoodLevel::Normal, 0) => &[" ┌══════┐", " │══════│", " └██████┘", "", ""],
        (MoodLevel::Normal, _) => &[" ┌══════┐", " │══════│", " └██████┘", "", ""],
        (MoodLevel::Low, 0) => &[" ┌──────┐", " │══──══│", " └██████┘", "", ""],
        (MoodLevel::Low, _) => &[" ┌──────┐", " │═──══─│", " └██████┘", "", ""],
    }
}
fn tekkai_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ┌══════┐", " │══════│~", " └██████┘", "", ""],
        (Action::Talk, _) => &[" ┌══════┐", "~│══════│", " └██████┘", "", ""],
        (Action::Play, 0) => &[" ┌══════┐♪", " │═◄══►═│", " └██████┘", "", ""],
        (Action::Play, _) => &["♪┌══════┐", " │═►══◄═│", " └██████┘", "", ""],
        (Action::Train, 0) => &[" ┌══════┐", " │»»»»»»│", " └██████┘»", "", ""],
        (Action::Train, _) => &[" ┌══════┐", " │»»»»»»│»", " └██████┘", "", ""],
        (Action::Relax, 0) => &[" ┌──────┐", " │══──══│", " └██████┘", "", ""],
        (Action::Relax, _) => &[" ┌──────┐", " │══──══│", " └██████┘", "", ""],
    }
}

// --- ブンブン (bunbun) - Wrecking ball on chain ---
fn bunbun_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["⊙-⊙-⊙\\  ", "      ●  ", "     /●\\ ", "", ""],
        (MoodLevel::High, _) => &["  /⊙-⊙-⊙ ", "  ●      ", " /●\\     ", "", ""],
        (MoodLevel::Normal, 0) => &["⊙-⊙-⊙   ", "     |   ", "     ●   ", "", ""],
        (MoodLevel::Normal, _) => &["⊙-⊙-⊙   ", "     |   ", "     ●   ", "", ""],
        (MoodLevel::Low, 0) => &["⊙-⊙-⊙   ", "     |   ", "     .   ", "", ""],
        (MoodLevel::Low, _) => &["⊙-⊙-⊙   ", "      \\  ", "       . ", "", ""],
    }
}
fn bunbun_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["⊙-⊙-⊙~  ", "     |   ", "     ●   ", "", ""],
        (Action::Talk, _) => &["⊙-⊙-⊙   ", "     |~  ", "     ●   ", "", ""],
        (Action::Play, 0) => &["⊙-⊙-⊙\\  ", "      ●♪ ", "         ", "", ""],
        (Action::Play, _) => &["  /⊙-⊙-⊙ ", " ♪●      ", "         ", "", ""],
        (Action::Train, 0) => &["⊙-⊙-⊙\\  ", "      ●>>", "     *** ", "", ""],
        (Action::Train, _) => &["  /⊙-⊙-⊙ ", "<<●      ", " ***     ", "", ""],
        (Action::Relax, 0) => &["⊙-⊙-⊙   ", "     |   ", "     ~   ", "", ""],
        (Action::Relax, _) => &["⊙-⊙-⊙   ", "     |   ", "     ~   ", "", ""],
    }
}

// --- ガンテツ (gantetsu) - Cracked boulder, ONE eye in crack ---
fn gantetsu_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" /▓▓▓▓▓\\", " ▓╲◉╱▓▓▓!", " \\▓▓▓▓▓/", "", ""],
        (MoodLevel::High, _) => &[" /▓▓▓▓▓\\!", "!▓╲◉╱▓▓▓", " \\▓▓▓▓▓/", "", ""],
        (MoodLevel::Normal, 0) => &[" /▓▓▓▓▓\\", " ▓╲◉╱▓▓▓", " \\▓▓▓▓▓/", "", ""],
        (MoodLevel::Normal, _) => &[" /▓▓▓▓▓\\", " ▓╲●╱▓▓▓", " \\▓▓▓▓▓/", "", ""],
        (MoodLevel::Low, 0) => &[" /▓▓▓▓▓\\", " ▓╲.╱▓▓▓", " \\▓▓▓▓▓/", "", ""],
        (MoodLevel::Low, _) => &[" .▓▓▓▓▓.", " ▓╲.╱▓▓▓", " .▓▓▓▓▓.", "", ""],
    }
}
fn gantetsu_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" /▓▓▓▓▓\\", " ▓╲◉╱▓▓▓~", " \\▓▓▓▓▓/", "", ""],
        (Action::Talk, _) => &[" /▓▓▓▓▓\\", "~▓╲◉╱▓▓▓", " \\▓▓▓▓▓/", "", ""],
        (Action::Play, 0) => &[" /▓▓▓▓▓\\♪", " ▓╲◉╱▓▓▓", " \\▓▓▓▓▓/", "", ""],
        (Action::Play, _) => &["♪/▓▓▓▓▓\\", " ▓╲◉╱▓▓▓", " \\▓▓▓▓▓/", "", ""],
        (Action::Train, 0) => &[" /▓▓▓▓▓\\", " ▓╲◉╱▓▓▓", " \\▓▓▓▓▓/»", "", ""],
        (Action::Train, _) => &[" /▓▓▓▓▓\\", " ▓╲◉╱▓▓▓", "«\\▓▓▓▓▓/", "", ""],
        (Action::Relax, 0) => &[" /▓▓▓▓▓\\", " ▓╲─╱▓▓▓", " \\▓▓▓▓▓/", "", ""],
        (Action::Relax, _) => &[" /▓▓▓▓▓\\", " ▓╲─╱▓▓▓", " \\▓▓▓▓▓/", "", ""],
    }
}

// --- ドスコイ (dosukoi) - Massive wide body, tiny head ---
fn dosukoi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["     .   ", "  /█████\\!", " /███████\\", "", ""],
        (MoodLevel::High, _) => &["     .   ", "!/█████\\ ", " /███████\\", "", ""],
        (MoodLevel::Normal, 0) => &["     .   ", "  /█████\\", " /███████\\", "", ""],
        (MoodLevel::Normal, _) => &["     .   ", "  /█████\\", " /███████\\", "", ""],
        (MoodLevel::Low, 0) => &["     .   ", "  /█████\\", " /████████\\", "", ""],
        (MoodLevel::Low, _) => &["    .    ", "  /█████\\", "/█████████\\", "", ""],
    }
}
fn dosukoi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["     .~  ", "  /█████\\", " /███████\\", "", ""],
        (Action::Talk, _) => &["    ~.   ", "  /█████\\", " /███████\\", "", ""],
        (Action::Play, 0) => &["     .♪  ", "  /█████\\", " /███████\\", "", ""],
        (Action::Play, _) => &["    ♪.   ", " /███████\\", " /███████\\", "", ""],
        (Action::Train, 0) => &["     .   ", " </█████\\>", " /███████\\", "", ""],
        (Action::Train, _) => &["     .   ", " >/█████\\<", " /███████\\", "", ""],
        (Action::Relax, 0) => &["     .   ", "  /█████\\", "/█████████\\", "", ""],
        (Action::Relax, _) => &["     .   ", "  /█████\\", "/█████████\\", "", ""],
    }
}

// --- バリバリ (baribari) - Electric zigzag, no face ---
fn baribari_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["*╱╲╱╲╱╲*", " ╲*╱╲*╱ ", "*╱╲╱╲╱╲*", "", ""],
        (MoodLevel::High, _) => &[" ╲╱╲╱╲╱*", "*╱*╲╱*╲ ", " ╲╱╲╱╲╱*", "", ""],
        (MoodLevel::Normal, 0) => &[" ╱╲╱╲╱╲ ", " ╲ ╱╲ ╱ ", " ╱╲╱╲╱╲ ", "", ""],
        (MoodLevel::Normal, _) => &["  ╲╱╲╱╲ ", "  ╱╲ ╱╲ ", "  ╲╱╲╱╲ ", "", ""],
        (MoodLevel::Low, 0) => &["  ╱╲╱╲  ", "  ╲ ╱   ", "  ╱╲    ", "", ""],
        (MoodLevel::Low, _) => &["   ╲╱╲  ", "   ╱╲   ", "   ╲╱   ", "", ""],
    }
}
fn baribari_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["*╱╲╱╲╱╲~", " ╲ ╱╲ ╱ ", " ╱╲╱╲╱╲ ", "", ""],
        (Action::Talk, _) => &[" ╱╲╱╲╱╲ ", "~╲ ╱╲ ╱*", " ╱╲╱╲╱╲ ", "", ""],
        (Action::Play, 0) => &["♪╱╲╱╲╱╲ ", " ╲*╱╲*╱♪", " ╱╲╱╲╱╲ ", "", ""],
        (Action::Play, _) => &[" ╱╲╱╲╱╲♪", "♪╲*╱╲*╱ ", " ╱╲╱╲╱╲ ", "", ""],
        (Action::Train, 0) => &["*╱╲╱╲╱╲*", "*╲*╱╲*╱*", "*╱╲╱╲╱╲*", "", ""],
        (Action::Train, _) => &["**╲╱╲╱╲**", "**╱╲╱╲╱**", "**╲╱╲╱╲**", "", ""],
        (Action::Relax, 0) => &["  ╱╲╱╲  ", "  ╲ ╱╲  ", "  ╱╲╱╲  ", "", ""],
        (Action::Relax, _) => &["  ╱╲╱╲  ", "  ╲ ╱╲  ", "   ╱╲   ", "", ""],
    }
}

// --- メガトン (megaton) - Mushroom cloud shape, no face ---
fn megaton_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["(((███)))!", "   ███   ", "   |||   ", "", ""],
        (MoodLevel::High, _) => &["!(((███)))", "   ███   ", "   |||   ", "", ""],
        (MoodLevel::Normal, 0) => &[" ((███)) ", "   ██    ", "   ||    ", "", ""],
        (MoodLevel::Normal, _) => &[" ((███)) ", "   ██    ", "   ||    ", "", ""],
        (MoodLevel::Low, 0) => &["  (███)  ", "   █     ", "   |     ", "", ""],
        (MoodLevel::Low, _) => &["  .███.  ", "   █     ", "   |     ", "", ""],
    }
}
fn megaton_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ((███))~", "   ██    ", "   ||    ", "", ""],
        (Action::Talk, _) => &["~((███)) ", "   ██    ", "   ||    ", "", ""],
        (Action::Play, 0) => &[" ((███))♪", "   ██    ", "  ♪||    ", "", ""],
        (Action::Play, _) => &["♪((███)) ", "   ██    ", "   ||♪   ", "", ""],
        (Action::Train, 0) => &["(((███)))", "  ████   ", "   |||   ", "", ""],
        (Action::Train, _) => &["((██████))", "  █████  ", "   ||||  ", "", ""],
        (Action::Relax, 0) => &["  (███)  ", "   ██    ", "   ||    ", "", ""],
        (Action::Relax, _) => &["  (███)  ", "   ██    ", "   ..    ", "", ""],
    }
}

// --- グランド (gurando) - Mountain/landmass, trees on slopes ---
fn gurando_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["    /\\!  ", "  /△▲△\\ ", " /△▲▲▲△\\", "", ""],
        (MoodLevel::High, _) => &["   !/\\   ", "  /▲△▲\\ ", " /▲△△△▲\\", "", ""],
        (MoodLevel::Normal, 0) => &["    /\\   ", "  /△ △\\  ", " /△  △△\\", "", ""],
        (MoodLevel::Normal, _) => &["    /\\   ", "  / △ \\  ", " /△ △ △\\", "", ""],
        (MoodLevel::Low, 0) => &["    /\\   ", "   / \\   ", "  / . \\  ", "", ""],
        (MoodLevel::Low, _) => &["    ..   ", "   / \\   ", "  / . \\  ", "", ""],
    }
}
fn gurando_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["    /\\~  ", "  /△ △\\  ", " /△  △△\\", "", ""],
        (Action::Talk, _) => &["   ~/\\   ", "  /△ △\\  ", " /△  △△\\", "", ""],
        (Action::Play, 0) => &["   ♪/\\   ", "  /△▲△\\  ", " /△▲▲▲△\\", "", ""],
        (Action::Play, _) => &["    /\\♪  ", "  /▲△▲\\  ", " /▲△△△▲\\", "", ""],
        (Action::Train, 0) => &["    /\\   ", "  /█▲█\\  ", " /████▲█\\", "", ""],
        (Action::Train, _) => &["    /\\   ", "  /▲█▲\\  ", " /▲█████\\", "", ""],
        (Action::Relax, 0) => &["    /\\   ", "  / ~ \\  ", " / ~~~ \\", "", ""],
        (Action::Relax, _) => &["    /\\   ", "  / ~ \\  ", " / ~~~ \\", "", ""],
    }
}

// --- イカヅチ (ikazuchi) - Thunder drum with lightning ---
fn ikazuchi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["↯┌────┐↯", " │⚡⚡⚡│ ", "↯└────┘↯", "", ""],
        (MoodLevel::High, _) => &[" ┌────┐ ", "↯│⚡⚡⚡│↯", " └────┘ ", "", ""],
        (MoodLevel::Normal, 0) => &[" ┌────┐ ", " │⚡ ⚡│ ", " └────┘ ", "", ""],
        (MoodLevel::Normal, _) => &[" ┌────┐ ", " │ ⚡ │ ", " └────┘ ", "", ""],
        (MoodLevel::Low, 0) => &[" ┌────┐ ", " │ .  │ ", " └────┘ ", "", ""],
        (MoodLevel::Low, _) => &[" ┌────┐ ", " │  . │ ", " └────┘ ", "", ""],
    }
}
fn ikazuchi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ┌────┐~", " │⚡⚡⚡│ ", " └────┘ ", "", ""],
        (Action::Talk, _) => &["~┌────┐ ", " │⚡⚡⚡│ ", " └────┘ ", "", ""],
        (Action::Play, 0) => &["♪┌────┐ ", " │⚡♪⚡│ ", " └────┘♪", "", ""],
        (Action::Play, _) => &[" ┌────┐♪", " │♪⚡♪│ ", "♪└────┘ ", "", ""],
        (Action::Train, 0) => &["↯┌────┐↯", "↯│⚡⚡⚡│↯", "↯└────┘↯", "", ""],
        (Action::Train, _) => &["⚡┌────┐⚡", " │↯↯↯↯│ ", "⚡└────┘⚡", "", ""],
        (Action::Relax, 0) => &[" ┌────┐ ", " │ ~~ │ ", " └────┘ ", "", ""],
        (Action::Relax, _) => &[" ┌────┐ ", " │ ~  │ ", " └────┘ ", "", ""],
    }
}

// --- ゴリラン (goriran) - Massive gorilla silhouette ---
fn goriran_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ╔██╗!  ", " ╔████╗  ", "██╝  ╚██ ", "", ""],
        (MoodLevel::High, _) => &[" !╔██╗   ", " ╔████╗  ", "██╝  ╚██ ", "", ""],
        (MoodLevel::Normal, 0) => &["  ╔██╗   ", " ╔████╗  ", " █╝  ╚█  ", "", ""],
        (MoodLevel::Normal, _) => &["  ╔██╗   ", " ╔████╗  ", " █╝  ╚█  ", "", ""],
        (MoodLevel::Low, 0) => &["  ┌██┐   ", " ┌████┐  ", " █    █  ", "", ""],
        (MoodLevel::Low, _) => &["  ┌██┐   ", " ┌████┐  ", " █.  .█  ", "", ""],
    }
}
fn goriran_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ╔██╗~  ", " ╔████╗  ", " █╝  ╚█  ", "", ""],
        (Action::Talk, _) => &["  ╔██╗   ", " ╔████╗~ ", " █╝  ╚█  ", "", ""],
        (Action::Play, 0) => &["  ╔██╗♪  ", " ╔████╗  ", "██    ██ ", "", ""],
        (Action::Play, _) => &[" ♪╔██╗   ", " ╔████╗  ", " ██  ██  ", "", ""],
        (Action::Train, 0) => &["  ╔██╗   ", "╔██████╗ ", "██╝  ╚██!", "", ""],
        (Action::Train, _) => &["  ╔██╗!  ", "╔██████╗ ", "██╝  ╚██ ", "", ""],
        (Action::Relax, 0) => &["  ╔██╗   ", " ╔████╗  ", " █~  ~█  ", "", ""],
        (Action::Relax, _) => &["  ╔██╗   ", " ╔████╗  ", " █~  ~█  ", "", ""],
    }
}

// --- ダイガン (daigan) - Giant diamond/crystal ---
fn daigan_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ╱╲*   ", " ╱◇◇╲   ", " ╲◇◇╱!  ", "", ""],
        (MoodLevel::High, _) => &["  *╱╲   ", "  ╱◇◇╲  ", "  !╲◇◇╱ ", "", ""],
        (MoodLevel::Normal, 0) => &["   ╱╲   ", "  ╱◇◇╲  ", "  ╲◇◇╱  ", "", ""],
        (MoodLevel::Normal, _) => &["   ╱╲   ", "  ╱◇◇╲  ", "  ╲◇◇╱  ", "", ""],
        (MoodLevel::Low, 0) => &["   ╱╲   ", "  ╱..╲  ", "  ╲..╱  ", "", ""],
        (MoodLevel::Low, _) => &["   ╱╲   ", "  ╱. ╲  ", "  ╲ .╱  ", "", ""],
    }
}
fn daigan_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["   ╱╲   ", "  ╱◇◇╲~ ", "  ╲◇◇╱  ", "", ""],
        (Action::Talk, _) => &["   ╱╲   ", " ~╱◇◇╲  ", "  ╲◇◇╱  ", "", ""],
        (Action::Play, 0) => &["  ♪╱╲   ", "  ╱◇◇╲  ", "  ╲◇◇╱♪ ", "", ""],
        (Action::Play, _) => &["   ╱╲♪  ", "  ╱◇◇╲  ", " ♪╲◇◇╱  ", "", ""],
        (Action::Train, 0) => &["  *╱╲*  ", " *╱◇◇╲* ", " *╲◇◇╱* ", "", ""],
        (Action::Train, _) => &["   ╱╲   ", " *╱◇◇╲* ", "  ╲◇◇╱  ", "", ""],
        (Action::Relax, 0) => &["   ╱╲   ", "  ╱~~╲  ", "  ╲~~╱  ", "", ""],
        (Action::Relax, _) => &["   ╱╲   ", "  ╱~ ╲  ", "  ╲ ~╱  ", "", ""],
    }
}

// --- ゴロゴロ (gorogoro) - Rolling boulder, NO face ---
fn gorogoro_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ████>> ", " ██████>>", "...████  ", "", ""],
        (MoodLevel::High, _) => &[">>████   ", ">>██████ ", "   ████..", "", ""],
        (MoodLevel::Normal, 0) => &["   ████  ", "  ██████ ", "  .████. ", "", ""],
        (MoodLevel::Normal, _) => &["   ████  ", "  ██████ ", " . ████ .", "", ""],
        (MoodLevel::Low, 0) => &["   ████  ", "  ██████ ", "  ██████ ", "", ""],
        (MoodLevel::Low, _) => &["   ████  ", "  ██████ ", "  ██████ ", "", ""],
    }
}
fn gorogoro_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["   ████~ ", "  ██████ ", "  .████. ", "", ""],
        (Action::Talk, _) => &["  ~████  ", "  ██████ ", "  .████. ", "", ""],
        (Action::Play, 0) => &["  ████>>♪", " ██████>>", "...████  ", "", ""],
        (Action::Play, _) => &["♪<<████  ", " <<██████", "   ████..", "", ""],
        (Action::Train, 0) => &["  ████>>>", " ██████>>", "...████..", "", ""],
        (Action::Train, _) => &["<<<████  ", "<<<██████", "...████..", "", ""],
        (Action::Relax, 0) => &["   ████  ", "  ██████ ", "  ~~~~~~ ", "", ""],
        (Action::Relax, _) => &["   ████  ", "  ██████ ", "   ~~~~  ", "", ""],
    }
}

// --- カチワリ (kachiwari) - Axe blade IS the body ---
fn kachiwari_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  \\███/! ", "   \\█/   ", "    |    ", "", ""],
        (MoodLevel::High, _) => &[" !\\███/  ", "   \\█/   ", "    |    ", "", ""],
        (MoodLevel::Normal, 0) => &["  \\███/  ", "   \\█/   ", "    |    ", "", ""],
        (MoodLevel::Normal, _) => &["  \\███/  ", "   \\█/   ", "    |    ", "", ""],
        (MoodLevel::Low, 0) => &["  \\██/   ", "   \\█/   ", "    |    ", "", ""],
        (MoodLevel::Low, _) => &["   \\█/   ", "   \\█/   ", "    .    ", "", ""],
    }
}
fn kachiwari_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  \\███/~ ", "   \\█/   ", "    |    ", "", ""],
        (Action::Talk, _) => &[" ~\\███/  ", "   \\█/   ", "    |    ", "", ""],
        (Action::Play, 0) => &["  \\███/♪ ", "   \\█/   ", "   ♪|    ", "", ""],
        (Action::Play, _) => &[" ♪\\███/  ", "   \\█/   ", "    |♪   ", "", ""],
        (Action::Train, 0) => &["  \\███/  ", "   \\█/   ", "    |▼   ", "", ""],
        (Action::Train, _) => &["  \\███/  ", "   \\█/▼  ", "    |    ", "", ""],
        (Action::Relax, 0) => &["  \\███/  ", "   \\█/   ", "    ~    ", "", ""],
        (Action::Relax, _) => &["  \\███/  ", "   \\█/   ", "    ~    ", "", ""],
    }
}

// --- テツジン (tetsujin) - Mech suit, visor slit ---
fn tetsujin_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ┌[═══]┐!", " ├█████┤ ", " └┤   ├┘ ", "", ""],
        (MoodLevel::High, _) => &["!┌[═══]┐ ", " ├█████┤ ", " └┤   ├┘ ", "", ""],
        (MoodLevel::Normal, 0) => &[" ┌[═══]┐ ", " ├█████┤ ", " └┤   ├┘ ", "", ""],
        (MoodLevel::Normal, _) => &[" ┌[═══]┐ ", " ├█████┤ ", " └┤   ├┘ ", "", ""],
        (MoodLevel::Low, 0) => &[" ┌[───]┐ ", " ├█████┤ ", " └┤   ├┘ ", "", ""],
        (MoodLevel::Low, _) => &[" ┌[──-]┐ ", " ├█████┤ ", " └┤ . ├┘ ", "", ""],
    }
}
fn tetsujin_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ┌[═══]┐ ", " ├█████┤~", " └┤   ├┘ ", "", ""],
        (Action::Talk, _) => &[" ┌[═══]┐ ", "~├█████┤ ", " └┤   ├┘ ", "", ""],
        (Action::Play, 0) => &[" ┌[═══]┐♪", " ├█████┤ ", " └┤ ♪ ├┘ ", "", ""],
        (Action::Play, _) => &["♪┌[═══]┐ ", " ├█████┤ ", " └┤♪  ├┘ ", "", ""],
        (Action::Train, 0) => &[" ┌[═══]┐ ", "»├█████┤«", " └┤   ├┘ ", "", ""],
        (Action::Train, _) => &[" ┌[═══]┐ ", "«├█████┤»", " └┤   ├┘ ", "", ""],
        (Action::Relax, 0) => &[" ┌[───]┐ ", " ├█████┤ ", " └┤ ~ ├┘ ", "", ""],
        (Action::Relax, _) => &[" ┌[───]┐ ", " ├█████┤ ", " └┤~  ├┘ ", "", ""],
    }
}

// --- ドゴン (dogon) - Totem pole, THREE stacked faces ---
fn dogon_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["   [◉◉]! ", "   [▼▼]  ", "   [◆◆]  ", "", ""],
        (MoodLevel::High, _) => &["  ![◉◉]  ", "   [▼▼]  ", "   [◆◆]  ", "", ""],
        (MoodLevel::Normal, 0) => &["   [◉◉]  ", "   [▽▽]  ", "   [◇◇]  ", "", ""],
        (MoodLevel::Normal, _) => &["   [●●]  ", "   [▽▽]  ", "   [◇◇]  ", "", ""],
        (MoodLevel::Low, 0) => &["   [..]  ", "   [..]  ", "   [..]  ", "", ""],
        (MoodLevel::Low, _) => &["   [. ]  ", "   [ .]  ", "   [..]  ", "", ""],
    }
}
fn dogon_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["   [◉◉]  ", "   [▽▽]~ ", "   [◇◇]  ", "", ""],
        (Action::Talk, _) => &["   [◉◉]  ", "  ~[▽▽]  ", "   [◇◇]  ", "", ""],
        (Action::Play, 0) => &["  ♪[◉◉]  ", "   [▼▼]♪ ", "   [◆◆]  ", "", ""],
        (Action::Play, _) => &["   [◉◉]♪ ", "  ♪[▼▼]  ", "   [◆◆]  ", "", ""],
        (Action::Train, 0) => &["   [◉◉]  ", "  »[▼▼]« ", "   [◆◆]  ", "", ""],
        (Action::Train, _) => &["   [◉◉]  ", "  «[▼▼]» ", "   [◆◆]  ", "", ""],
        (Action::Relax, 0) => &["   [──]  ", "   [──]  ", "   [──]  ", "", ""],
        (Action::Relax, _) => &["   [──]  ", "   [──]  ", "   [──]  ", "", ""],
    }
}

// --- バンカー (bankaa) - Bunker/pillbox, gun slit eye ---
fn bankaa_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["╔════════╗", "║▒▒═►═▒▒║!", "╚════════╝", "", ""],
        (MoodLevel::High, _) => &["╔════════╗", "!║▒▒═►═▒▒║", "╚════════╝", "", ""],
        (MoodLevel::Normal, 0) => &["╔════════╗", "║▒▒═══▒▒║", "╚════════╝", "", ""],
        (MoodLevel::Normal, _) => &["╔════════╗", "║▒▒═══▒▒║", "╚════════╝", "", ""],
        (MoodLevel::Low, 0) => &["╔────────╗", "║  ═──═  ║", "╚────────╝", "", ""],
        (MoodLevel::Low, _) => &["╔────────╗", "║  ──═─  ║", "╚────────╝", "", ""],
    }
}
fn bankaa_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["╔════════╗", "║▒▒═══▒▒║~", "╚════════╝", "", ""],
        (Action::Talk, _) => &["╔════════╗", "~║▒▒═══▒▒║", "╚════════╝", "", ""],
        (Action::Play, 0) => &["╔════════╗♪", "║▒▒═══▒▒║", "╚════════╝", "", ""],
        (Action::Play, _) => &["♪╔════════╗", "║▒▒═══▒▒║", "╚════════╝", "", ""],
        (Action::Train, 0) => &["╔════════╗", "║▒═►►►═▒║", "╚════════╝", "", ""],
        (Action::Train, _) => &["╔════════╗", "║▒═»»»═▒║", "╚════════╝", "", ""],
        (Action::Relax, 0) => &["╔════════╗", "║  ════  ║", "╚════════╝", "", ""],
        (Action::Relax, _) => &["╔════════╗", "║  ═──═  ║", "╚════════╝", "", ""],
    }
}

// --- マッスル (massuru) - Whole creature IS a flexing arm ---
fn massuru_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ╔█╗!   ", " ╔███╗   ", " ║██╝    ", "", ""],
        (MoodLevel::High, _) => &[" !╔█╗    ", " ╔███╗   ", "  ║██╝   ", "", ""],
        (MoodLevel::Normal, 0) => &["  ╔█╗    ", " ╔███╗   ", " ║██╝    ", "", ""],
        (MoodLevel::Normal, _) => &["  ╔█╗    ", " ╔███╗   ", "  ║██╝   ", "", ""],
        (MoodLevel::Low, 0) => &["  ┌█┐    ", " ┌██┐    ", " ║█╝     ", "", ""],
        (MoodLevel::Low, _) => &["  ┌█┐    ", " ┌██┐    ", "  ║█.    ", "", ""],
    }
}
fn massuru_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ╔█╗~   ", " ╔███╗   ", " ║██╝    ", "", ""],
        (Action::Talk, _) => &["  ╔█╗    ", " ╔███╗~  ", " ║██╝    ", "", ""],
        (Action::Play, 0) => &["  ╔█╗♪   ", " ╔███╗   ", " ║██╝    ", "", ""],
        (Action::Play, _) => &[" ♪╔█╗    ", " ╔███╗   ", "  ║██╝♪  ", "", ""],
        (Action::Train, 0) => &["  ╔██╗   ", " ╔████╗! ", " ║███╝   ", "", ""],
        (Action::Train, _) => &["  ╔██╗!  ", " ╔████╗  ", " ║███╝   ", "", ""],
        (Action::Relax, 0) => &["  ╔█╗    ", " ┌██┐    ", " ║█~     ", "", ""],
        (Action::Relax, _) => &["  ╔█╗    ", " ┌██┐    ", "  ║~     ", "", ""],
    }
}

// --- イワオ (iwao) - Stacked boulders golem, no clear face ---
fn iwao_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["   (██)! ", "  (████) ", " (██████)", "", ""],
        (MoodLevel::High, _) => &["  !(██)  ", "  (████) ", " (██████)", "", ""],
        (MoodLevel::Normal, 0) => &["   (██)  ", "  (████) ", " (██████)", "", ""],
        (MoodLevel::Normal, _) => &["   (██)  ", "  (████) ", "  (█████)", "", ""],
        (MoodLevel::Low, 0) => &["   .██.  ", "  .████. ", " .██████.", "", ""],
        (MoodLevel::Low, _) => &["    ██   ", "  .████. ", " .██████.", "", ""],
    }
}
fn iwao_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["   (██)~ ", "  (████) ", " (██████)", "", ""],
        (Action::Talk, _) => &["  ~(██)  ", "  (████) ", " (██████)", "", ""],
        (Action::Play, 0) => &["  ♪(██)  ", "  (████) ", " (██████)♪", "", ""],
        (Action::Play, _) => &["   (██)♪ ", "  (████) ", "♪(██████)", "", ""],
        (Action::Train, 0) => &["   (██)  ", " »(████)«", " (██████)", "", ""],
        (Action::Train, _) => &["   (██)  ", " «(████)»", " (██████)", "", ""],
        (Action::Relax, 0) => &["   (██)  ", "  (████) ", " (██████)~", "", ""],
        (Action::Relax, _) => &["   (██)  ", "  (████) ", "~(██████)", "", ""],
    }
}

// ============================================================
// BOUKEN TYPE Stage 3 Species
// ============================================================

// --- ガニ (gani) - Crab: pincers, shell, legs, NO face ---
fn gani_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["＞ ▓▓▓▓▓ ＜", "  ▓▓▓▓▓▓▓ ", " ⅃⌐⅃⌐⅃⌐⅃⌐", "", ""],
        (MoodLevel::High, _) => &[" ＞▓▓▓▓▓＜!", "  ▓▓▓▓▓▓▓ ", " ⌐⅃⌐⅃⌐⅃⌐⅃", "", ""],
        (MoodLevel::Normal, 0) => &["＞ ▓▓▓ ＜", "  ▓▓▓▓▓ ", "  ⅃⌐⅃⌐ ", "", ""],
        (MoodLevel::Normal, _) => &[" ＞▓▓▓＜ ", "  ▓▓▓▓▓ ", "  ⌐⅃⌐⅃ ", "", ""],
        (MoodLevel::Low, 0) => &[">  ▓▓  < ", "   ▓▓▓  ", "   ⅃ ⌐  ", "", ""],
        (MoodLevel::Low, _) => &[" > ▓▓ <  ", "   ▓▓▓  ", "   ⅃ ⌐  ", "", ""],
    }
}
fn gani_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["＞ ▓▓▓ ＜♪", "  ▓▓▓▓▓  ", "  ⅃⌐⅃⌐  ", "", ""],
        (Action::Talk, _) => &["♫＞▓▓▓＜ ", "  ▓▓▓▓▓  ", "  ⌐⅃⌐⅃  ", "", ""],
        (Action::Play, 0) => &[" ＞▓▓▓＜~ ", "  ▓▓▓▓▓  ", " ~⅃⌐⅃⌐~ ", "", ""],
        (Action::Play, _) => &["~＞▓▓▓＜  ", "  ▓▓▓▓▓  ", " ~⌐⅃⌐⅃~ ", "", ""],
        (Action::Train, 0) => &["＞＞▓▓▓＜＜!", " ▓▓▓▓▓▓▓ ", " ⅃⌐⅃⌐⅃⌐⅃⌐", "", ""],
        (Action::Train, _) => &["!＞＞▓▓▓＜＜", " ▓▓▓▓▓▓▓ ", " ⌐⅃⌐⅃⌐⅃⌐⅃", "", ""],
        (Action::Relax, 0) => &[">  ▓▓  < ", "   ▓▓▓ z ", "   ⅃ ⌐  ", "", ""],
        (Action::Relax, _) => &[" > ▓▓ <  ", "   ▓▓▓ zZ", "   ⅃ ⌐  ", "", ""],
    }
}

// --- トビオ (tobio) - Dragon-bird: wings, body, neck+head, tail ---
fn tobio_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["彡彡 v~~> ", " 彡▓▓▓▓▓ ", "   ╰━━~  ", "", ""],
        (MoodLevel::High, _) => &[" 彡彡v~~>!", "  彡▓▓▓▓▓", "   ╰━━~  ", "", ""],
        (MoodLevel::Normal, 0) => &[" 彡 v~~> ", "  彡▓▓▓  ", "   ╰━~   ", "", ""],
        (MoodLevel::Normal, _) => &["  彡v~~> ", "  彡▓▓▓  ", "   ╰━~   ", "", ""],
        (MoodLevel::Low, 0) => &["   v~>   ", "   ▓▓▓   ", "   ╰~    ", "", ""],
        (MoodLevel::Low, _) => &["   v~>   ", "   ▓▓▓   ", "    ╰~   ", "", ""],
    }
}
fn tobio_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" 彡 v~~>♪", "  彡▓▓▓  ", "   ╰━~   ", "", ""],
        (Action::Talk, _) => &[" 彡 v~~>♫", "  彡▓▓▓  ", "   ╰━~   ", "", ""],
        (Action::Play, 0) => &["彡彡~v~~> ", " 彡▓▓▓▓▓~", "   ╰━━~  ", "", ""],
        (Action::Play, _) => &[" ~彡彡v~~>", "  彡▓▓▓▓▓", "    ╰━━~ ", "", ""],
        (Action::Train, 0) => &["彡彡彡v~~>!", " 彡▓▓▓▓▓▓", "   ╰━━━~ ", "", ""],
        (Action::Train, _) => &["!彡彡彡v~~>", " 彡▓▓▓▓▓▓ ", "   ╰━━━~  ", "", ""],
        (Action::Relax, 0) => &["   v~>  z", "   ▓▓▓   ", "   ╰~    ", "", ""],
        (Action::Relax, _) => &["   v~> zZ", "   ▓▓▓   ", "    ╰~   ", "", ""],
    }
}

// --- マルマル (marumaru) - Armored ball: ONE eye peeking through crack ---
fn marumaru_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ║▓▓▓▓║ ", "  ║◎▓▓▓║!", "  ║▓▓▓▓║ ", "", ""],
        (MoodLevel::High, _) => &["  ║▓▓▓▓║ ", " !║▓▓▓◎║ ", "  ║▓▓▓▓║ ", "", ""],
        (MoodLevel::Normal, 0) => &["  ║▓▓▓▓║ ", "  ║◎▓▓▓║ ", "  ║▓▓▓▓║ ", "", ""],
        (MoodLevel::Normal, _) => &["  ║▓▓▓▓║ ", "  ║▓▓◎▓║ ", "  ║▓▓▓▓║ ", "", ""],
        (MoodLevel::Low, 0) => &["  ║▓▓▓▓║ ", "  ║.▓▓▓║ ", "  ║▓▓▓▓║ ", "", ""],
        (MoodLevel::Low, _) => &["  ║▓▓▓▓║ ", "  ║▓▓▓.║ ", "  ║▓▓▓▓║ ", "", ""],
    }
}
fn marumaru_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ║▓▓▓▓║ ", "  ║◎▓▓▓║♪", "  ║▓▓▓▓║ ", "", ""],
        (Action::Talk, _) => &["  ║▓▓▓▓║ ", "♫║▓▓◎▓║ ", "  ║▓▓▓▓║ ", "", ""],
        (Action::Play, 0) => &[" ~║▓▓▓▓║~", "  ║◎▓▓▓║ ", " ~║▓▓▓▓║~", "", ""],
        (Action::Play, _) => &["~ ║▓▓▓▓║ ", "  ║▓▓▓◎║ ", "  ║▓▓▓▓║~", "", ""],
        (Action::Train, 0) => &["  ║▓▓▓▓║!", "  ║◎▓▓▓║!", "  ║▓▓▓▓║!", "", ""],
        (Action::Train, _) => &[" !║▓▓▓▓║ ", " !║▓▓▓◎║ ", " !║▓▓▓▓║ ", "", ""],
        (Action::Relax, 0) => &["  ║▓▓▓▓║ ", "  ║.▓▓▓║z", "  ║▓▓▓▓║ ", "", ""],
        (Action::Relax, _) => &["  ║▓▓▓▓║ ", "  ║▓▓▓.║ ", "  ║▓▓▓▓║z", "", ""],
    }
}

// --- ハヤテ (hayate) - Swift Wolf: lean, speed lines, fanged head ---
fn hayate_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["≫≫ /▽\\  ", "≫ ▓▓▓▓▓▓!", "    ╰╯╰╯ ", "", ""],
        (MoodLevel::High, _) => &[" ≫≫/▽\\ !", " ≫▓▓▓▓▓▓ ", "    ╰╯╰╯ ", "", ""],
        (MoodLevel::Normal, 0) => &[" ≫ /▽\\  ", "  ▓▓▓▓▓  ", "   ╰╯╰╯  ", "", ""],
        (MoodLevel::Normal, _) => &["  ≫/▽\\  ", "  ▓▓▓▓▓  ", "   ╰╯╰╯  ", "", ""],
        (MoodLevel::Low, 0) => &["   /▽\\   ", "   ▓▓▓   ", "   ╰ ╰   ", "", ""],
        (MoodLevel::Low, _) => &["   /▽\\   ", "   ▓▓▓   ", "    ╰ ╰  ", "", ""],
    }
}
fn hayate_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ≫ /▽\\♪ ", "  ▓▓▓▓▓  ", "   ╰╯╰╯  ", "", ""],
        (Action::Talk, _) => &[" ≫ /▽\\♫ ", "  ▓▓▓▓▓  ", "   ╰╯╰╯  ", "", ""],
        (Action::Play, 0) => &["≫≫/▽\\ ~ ", " ▓▓▓▓▓▓~ ", "   ╰╯╰╯  ", "", ""],
        (Action::Play, _) => &[" ~≫≫/▽\\ ", "  ~▓▓▓▓▓▓", "    ╰╯╰╯ ", "", ""],
        (Action::Train, 0) => &["≫≫≫/▽\\!!", "≫▓▓▓▓▓▓▓ ", "   ╰╯╰╯  ", "", ""],
        (Action::Train, _) => &["!!≫≫≫/▽\\", " ≫▓▓▓▓▓▓▓", "    ╰╯╰╯ ", "", ""],
        (Action::Relax, 0) => &["   /▽\\ z ", "   ▓▓▓   ", "   ╰ ╰   ", "", ""],
        (Action::Relax, _) => &["   /▽\\ zZ", "   ▓▓▓   ", "    ╰ ╰  ", "", ""],
    }
}

// --- グルグルン (gurugurun) - Nautilus: spiral shell, ONE eye on stalk, tentacle ---
fn gurugurun_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ◎─┐    ", " @@@@@@ ! ", "   ~~~~   ", "", ""],
        (MoodLevel::High, _) => &["   ┌─◎   ", " !@@@@@@  ", "   ~~~~   ", "", ""],
        (MoodLevel::Normal, 0) => &["  ◎─┐    ", "  @@@@@   ", "    ~~~   ", "", ""],
        (MoodLevel::Normal, _) => &["   ┌─◎   ", "   @@@@@  ", "   ~~~    ", "", ""],
        (MoodLevel::Low, 0) => &["  .─┐    ", "   @@@    ", "    ~     ", "", ""],
        (MoodLevel::Low, _) => &["  .─┐    ", "   @@@    ", "     ~    ", "", ""],
    }
}
fn gurugurun_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ◎─┐ ♪  ", "  @@@@@   ", "    ~~~   ", "", ""],
        (Action::Talk, _) => &["   ┌─◎♫  ", "   @@@@@  ", "   ~~~    ", "", ""],
        (Action::Play, 0) => &["  ◎─┐~   ", " ~@@@@@~  ", "   ~~~~~  ", "", ""],
        (Action::Play, _) => &["  ~┌─◎   ", "  ~@@@@@~ ", "  ~~~~~   ", "", ""],
        (Action::Train, 0) => &["  ◎─┐  ! ", " @@@@@@@! ", "   ~~~~~~ ", "", ""],
        (Action::Train, _) => &[" !  ┌─◎  ", " !@@@@@@@ ", " ~~~~~~   ", "", ""],
        (Action::Relax, 0) => &["  .─┐  z ", "   @@@    ", "    ~     ", "", ""],
        (Action::Relax, _) => &["  .─┐  zZ", "   @@@    ", "     ~    ", "", ""],
    }
}

// --- カゼノコ (kazenoko) - Tornado with THREE scattered eyes ---
fn kazenoko_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ◎ ∧ ◎  ", " ◎/|||\\  ", "  /|||\\\\ !", "", ""],
        (MoodLevel::High, _) => &["   ∧◎    ", " ◎/|||\\◎!", " /|||\\\\  ", "", ""],
        (MoodLevel::Normal, 0) => &["  ◎∧     ", "  /||\\◎  ", "  /||\\   ", "", ""],
        (MoodLevel::Normal, _) => &["   ∧ ◎   ", " ◎/||\\   ", "  /||\\   ", "", ""],
        (MoodLevel::Low, 0) => &["   ∧     ", "  /|\\◎   ", "  /|\\    ", "", ""],
        (MoodLevel::Low, _) => &["   ∧     ", " ◎/|\\    ", "   /|\\   ", "", ""],
    }
}
fn kazenoko_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ◎∧  ♪  ", "  /||\\◎  ", "  /||\\   ", "", ""],
        (Action::Talk, _) => &[" ♫ ∧ ◎   ", " ◎/||\\   ", "  /||\\   ", "", ""],
        (Action::Play, 0) => &[" ~◎∧◎ ~  ", " ~/|||\\◎ ", "  /|||\\~ ", "", ""],
        (Action::Play, _) => &["~ ◎∧  ◎~ ", " ◎/|||\\~ ", " ~/|||\\  ", "", ""],
        (Action::Train, 0) => &[" !◎ ∧ ◎! ", " ◎/||||\\◎", "  /||||\\!", "", ""],
        (Action::Train, _) => &["!◎  ∧  ◎!", " ◎/||||\\◎", " !/||||\\", "", ""],
        (Action::Relax, 0) => &["   ∧  z  ", "  /|\\◎   ", "  /|\\    ", "", ""],
        (Action::Relax, _) => &["   ∧  zZ ", " ◎/|\\    ", "   /|\\   ", "", ""],
    }
}

// --- ドカーン (dokaan) - Round bomb: NO face, fuse on top ---
fn dokaan_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ※※※ *  ", " ●●●●●●  ", "  ●●●●●  ", "", ""],
        (MoodLevel::High, _) => &["  * ※※※  ", "  ●●●●●● ", "  ●●●●●  ", "", ""],
        (MoodLevel::Normal, 0) => &["   ※※    ", "  ●●●●●  ", "  ●●●●●  ", "", ""],
        (MoodLevel::Normal, _) => &["    ※※   ", "  ●●●●●  ", "  ●●●●●  ", "", ""],
        (MoodLevel::Low, 0) => &["    .    ", "   ●●●   ", "   ●●●   ", "", ""],
        (MoodLevel::Low, _) => &["    .    ", "   ●●●   ", "   ●●●   ", "", ""],
    }
}
fn dokaan_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["   ※※ ♪  ", "  ●●●●● ♫", "  ●●●●●  ", "", ""],
        (Action::Talk, _) => &["  ♪※※    ", " ♫●●●●●  ", "  ●●●●●  ", "", ""],
        (Action::Play, 0) => &["  ~※※※~  ", " ~●●●●●~ ", "  ●●●●●  ", "", ""],
        (Action::Play, _) => &[" ~※※※~   ", "  ~●●●●●~", "  ●●●●●  ", "", ""],
        (Action::Train, 0) => &[" ※※※ !!  ", " ●●●●●●●!", "  ●●●●●● ", "", ""],
        (Action::Train, _) => &["!! ※※※   ", "!●●●●●●● ", " ●●●●●●  ", "", ""],
        (Action::Relax, 0) => &["    .  z ", "   ●●●   ", "   ●●●   ", "", ""],
        (Action::Relax, _) => &["    .  zZ", "   ●●●   ", "   ●●●   ", "", ""],
    }
}

// --- スイスイ (suisui) - Manta Ray: flat diamond body, long tail, NO eyes ---
fn suisui_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ╱▓▓▓▓╲ ", " ╱▓▓▓▓▓▓╲", "━━╱~~    ", "", ""],
        (MoodLevel::High, _) => &[" ╱▓▓▓▓╲ !", "╱▓▓▓▓▓▓╲ ", " ━━╱~~   ", "", ""],
        (MoodLevel::Normal, 0) => &["  ╱▓▓▓╲  ", " ╱▓▓▓▓▓╲ ", "  ━━╱~~  ", "", ""],
        (MoodLevel::Normal, _) => &["  ╱▓▓▓╲  ", " ╱▓▓▓▓▓╲ ", "   ━━╱~~ ", "", ""],
        (MoodLevel::Low, 0) => &["   ╱▓╲   ", "  ╱▓▓▓╲  ", "   ━╱~   ", "", ""],
        (MoodLevel::Low, _) => &["   ╱▓╲   ", "  ╱▓▓▓╲  ", "    ━╱~  ", "", ""],
    }
}
fn suisui_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ╱▓▓▓╲♪ ", " ╱▓▓▓▓▓╲ ", "  ━━╱~~  ", "", ""],
        (Action::Talk, _) => &["♫╱▓▓▓╲  ", " ╱▓▓▓▓▓╲ ", "   ━━╱~~ ", "", ""],
        (Action::Play, 0) => &[" ~╱▓▓▓╲~ ", " ╱▓▓▓▓▓╲ ", "  ━━╱~~~~", "", ""],
        (Action::Play, _) => &["~ ╱▓▓▓╲ ~", " ╱▓▓▓▓▓╲ ", "~~~~━━╱~~", "", ""],
        (Action::Train, 0) => &["  ╱▓▓▓▓╲!", " ╱▓▓▓▓▓▓╲", " !━━━╱~~ ", "", ""],
        (Action::Train, _) => &["!╱▓▓▓▓╲  ", "╱▓▓▓▓▓▓╲!", "  ━━━╱~~ ", "", ""],
        (Action::Relax, 0) => &["   ╱▓╲  z", "  ╱▓▓▓╲  ", "   ━╱~   ", "", ""],
        (Action::Relax, _) => &["   ╱▓╲ zZ", "  ╱▓▓▓╲  ", "    ━╱~  ", "", ""],
    }
}

// --- サスライ (sasurai) - Cloaked Wanderer: hood, ONE glowing dot inside ---
fn sasurai_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ╱△╲  ! ", "  │• │   ", "  ▓▓▓▓   ", "", ""],
        (MoodLevel::High, _) => &["  ╱△╲ !  ", "  │ •│   ", "  ▓▓▓▓   ", "", ""],
        (MoodLevel::Normal, 0) => &["  ╱△╲    ", "  │• │   ", "  ▓▓▓▓   ", "", ""],
        (MoodLevel::Normal, _) => &["  ╱△╲    ", "  │ •│   ", "  ▓▓▓▓   ", "", ""],
        (MoodLevel::Low, 0) => &["  ╱△╲    ", "  │. │   ", "   ▓▓▓   ", "", ""],
        (MoodLevel::Low, _) => &["  ╱△╲    ", "  │ .│   ", "   ▓▓▓   ", "", ""],
    }
}
fn sasurai_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ╱△╲ ♪  ", "  │• │   ", "  ▓▓▓▓   ", "", ""],
        (Action::Talk, _) => &[" ♫╱△╲    ", "  │ •│   ", "  ▓▓▓▓   ", "", ""],
        (Action::Play, 0) => &["  ╱△╲~   ", "  │• │~  ", " ~▓▓▓▓   ", "", ""],
        (Action::Play, _) => &["  ~╱△╲   ", " ~│ •│   ", "  ▓▓▓▓~  ", "", ""],
        (Action::Train, 0) => &["  ╱△╲ !! ", "  │◉ │   ", "  ▓▓▓▓▓  ", "", ""],
        (Action::Train, _) => &["!!╱△╲    ", "  │ ◉│   ", "  ▓▓▓▓▓  ", "", ""],
        (Action::Relax, 0) => &["  ╱△╲ z  ", "  │. │   ", "   ▓▓▓   ", "", ""],
        (Action::Relax, _) => &["  ╱△╲ zZ ", "  │ .│   ", "   ▓▓▓   ", "", ""],
    }
}

// --- ピカッ (pikat) - Lightning bolt shape, NO face ---
fn pikat_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["╲╲  * ╱╱ ", " ╲⚡⚡⚡╱ !", "  ╲╲ ╱╱  ", "", ""],
        (MoodLevel::High, _) => &[" ╲╲ * ╱╱!", "  ╲⚡⚡⚡╱ ", " ╲╲  ╱╱  ", "", ""],
        (MoodLevel::Normal, 0) => &[" ╲╲  ╱╱  ", "  ╲⚡⚡╱  ", "  ╲╲╱╱   ", "", ""],
        (MoodLevel::Normal, _) => &["  ╲╲ ╱╱  ", "  ╲⚡⚡╱  ", "   ╲╲╱╱  ", "", ""],
        (MoodLevel::Low, 0) => &["   ╲ ╱   ", "    ⚡    ", "   ╲ ╱   ", "", ""],
        (MoodLevel::Low, _) => &["   ╲╱    ", "    ⚡    ", "    ╲╱   ", "", ""],
    }
}
fn pikat_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ╲╲  ╱╱♪ ", "  ╲⚡⚡╱  ", "  ╲╲╱╱   ", "", ""],
        (Action::Talk, _) => &["♫╲╲ ╱╱   ", "  ╲⚡⚡╱  ", "   ╲╲╱╱  ", "", ""],
        (Action::Play, 0) => &["~╲╲  ╱╱~ ", " ~╲⚡⚡╱~ ", " ~╲╲╱╱~  ", "", ""],
        (Action::Play, _) => &[" ~╲╲ ╱╱~ ", "  ~╲⚡⚡╱~", "  ~╲╲╱╱~ ", "", ""],
        (Action::Train, 0) => &["╲╲  * ╱╱!", " ╲⚡⚡⚡╱! ", " ╲╲  ╱╱ !", "", ""],
        (Action::Train, _) => &["!╲╲ * ╱╱ ", "!╲⚡⚡⚡╱  ", "! ╲╲ ╱╱  ", "", ""],
        (Action::Relax, 0) => &["   ╲ ╱ z ", "    ⚡    ", "   ╲ ╱   ", "", ""],
        (Action::Relax, _) => &["   ╲╱  zZ", "    ⚡    ", "    ╲╱   ", "", ""],
    }
}

// --- バサバサ (basabasa) - Moth: huge wings flap, tiny body, antennae ---
fn basabasa_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  Y   Y  ", "}▓▓▓▓▓▓{ ", " }▓▓▓▓{  ", "", ""],
        (MoodLevel::High, _) => &["  Y   Y !", " }▓▓▓▓▓▓{", "  }▓▓▓▓{ ", "", ""],
        (MoodLevel::Normal, 0) => &["  Y   Y  ", " }▓▓▓▓{  ", "  }▓▓{   ", "", ""],
        (MoodLevel::Normal, _) => &["  Y   Y  ", "  }▓▓▓▓{ ", "  }▓▓{   ", "", ""],
        (MoodLevel::Low, 0) => &["  Y  Y   ", "  }▓▓{   ", "   }▓{   ", "", ""],
        (MoodLevel::Low, _) => &["  Y  Y   ", "   }▓▓{  ", "   }▓{   ", "", ""],
    }
}
fn basabasa_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  Y   Y♪ ", " }▓▓▓▓{  ", "  }▓▓{   ", "", ""],
        (Action::Talk, _) => &["♫Y   Y   ", "  }▓▓▓▓{ ", "  }▓▓{   ", "", ""],
        (Action::Play, 0) => &["  Y~  Y  ", "}▓▓▓▓▓▓{~", " }▓▓▓▓{  ", "", ""],
        (Action::Play, _) => &[" ~Y  Y~  ", "~}▓▓▓▓▓▓{", "  }▓▓▓▓{ ", "", ""],
        (Action::Train, 0) => &["  Y   Y !!", "}▓▓▓▓▓▓▓{", " }▓▓▓▓▓{ ", "", ""],
        (Action::Train, _) => &["!!Y   Y  ", "{▓▓▓▓▓▓▓}", " {▓▓▓▓▓} ", "", ""],
        (Action::Relax, 0) => &["  Y  Y z ", "  }▓▓{   ", "   }▓{   ", "", ""],
        (Action::Relax, _) => &["  Y  Y zZ", "   }▓▓{  ", "   }▓{   ", "", ""],
    }
}

// --- ウロチョロ (urochoro) - Centipede: long segmented horizontal ---
fn urochoro_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["v═══════>!", "║║║║║║║║ ", "         ", "", ""],
        (MoodLevel::High, _) => &["!v═══════>", " ║║║║║║║║", "         ", "", ""],
        (MoodLevel::Normal, 0) => &[" v══════> ", " ║║║║║║  ", "         ", "", ""],
        (MoodLevel::Normal, _) => &["  v══════>", "  ║║║║║║ ", "         ", "", ""],
        (MoodLevel::Low, 0) => &["  v════>  ", "   ║║║║  ", "         ", "", ""],
        (MoodLevel::Low, _) => &["   v════> ", "   ║║║║  ", "         ", "", ""],
    }
}
fn urochoro_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" v══════>♪", " ║║║║║║  ", "         ", "", ""],
        (Action::Talk, _) => &["♫v══════> ", "  ║║║║║║ ", "         ", "", ""],
        (Action::Play, 0) => &["~v══════>~", " ║║║║║║  ", "         ", "", ""],
        (Action::Play, _) => &[" ~v══════>", "  ~║║║║║║", "         ", "", ""],
        (Action::Train, 0) => &["v════════>!!", "║║║║║║║║║", "         ", "", ""],
        (Action::Train, _) => &["!!v════════>", " ║║║║║║║║║", "          ", "", ""],
        (Action::Relax, 0) => &["  v════> z", "   ║║║║  ", "         ", "", ""],
        (Action::Relax, _) => &["   v════>zZ", "   ║║║║  ", "         ", "", ""],
    }
}

// --- ゴーゴー (googoo) - Racing Wheel: circular, spokes, spins ---
fn googoo_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["≫╭─◎─╮  ", "  │╳╳╳│≫ ", "  ╰───╯  ", "", ""],
        (MoodLevel::High, _) => &["  ╭─◎─╮≫!", " ≫│╳╳╳│  ", "  ╰───╯  ", "", ""],
        (MoodLevel::Normal, 0) => &["  ╭─◎─╮  ", "  │╲╱╲│  ", "  ╰───╯  ", "", ""],
        (MoodLevel::Normal, _) => &["  ╭─◎─╮  ", "  │╱╲╱│  ", "  ╰───╯  ", "", ""],
        (MoodLevel::Low, 0) => &["  ╭─◎─╮  ", "  │ . │  ", "  ╰───╯  ", "", ""],
        (MoodLevel::Low, _) => &["  ╭─◎─╮  ", "  │  . │  ", "  ╰───╯  ", "", ""],
    }
}
fn googoo_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ╭─◎─╮♪ ", "  │╲╱╲│  ", "  ╰───╯  ", "", ""],
        (Action::Talk, _) => &["♫╭─◎─╮   ", "  │╱╲╱│  ", "  ╰───╯  ", "", ""],
        (Action::Play, 0) => &["≫╭─◎─╮~  ", " ~│╳╳╳│  ", "  ╰───╯≫ ", "", ""],
        (Action::Play, _) => &["  ~╭─◎─╮≫", "  │╳╳╳│~ ", " ≫╰───╯  ", "", ""],
        (Action::Train, 0) => &["≫≫╭─◎─╮!!", "  │╳╳╳│≫≫", "  ╰───╯  ", "", ""],
        (Action::Train, _) => &["!!╭─◎─╮≫≫", " ≫≫│╳╳╳│ ", "   ╰───╯ ", "", ""],
        (Action::Relax, 0) => &["  ╭─◎─╮ z", "  │ . │  ", "  ╰───╯  ", "", ""],
        (Action::Relax, _) => &["  ╭─◎─╮zZ", "  │  . │  ", "  ╰───╯  ", "", ""],
    }
}

// --- クモノス (kumonos) - Spider: 8 legs radiating from center body ---
fn kumonos_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["╲ │ ╱  ! ", " (▓▓▓)   ", "╱ │ ╲    ", "", ""],
        (MoodLevel::High, _) => &[" !╲│ ╱   ", "  (▓▓▓)  ", "  ╱ │╲   ", "", ""],
        (MoodLevel::Normal, 0) => &[" ╲ │ ╱   ", "  (▓▓)   ", " ╱ │ ╲   ", "", ""],
        (MoodLevel::Normal, _) => &["  ╲│ ╱   ", "  (▓▓)   ", "  ╱│ ╲   ", "", ""],
        (MoodLevel::Low, 0) => &["  ╲│╱    ", "   (▓)   ", "  ╱│╲    ", "", ""],
        (MoodLevel::Low, _) => &["   ╲│╱   ", "   (▓)   ", "   ╱│╲   ", "", ""],
    }
}
fn kumonos_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ╲ │ ╱ ♪ ", "  (▓▓)   ", " ╱ │ ╲   ", "", ""],
        (Action::Talk, _) => &["♫╲│ ╱    ", "  (▓▓)   ", "  ╱│ ╲   ", "", ""],
        (Action::Play, 0) => &[" ╲~│~╱   ", " ~(▓▓)~  ", " ╱~│~╲   ", "", ""],
        (Action::Play, _) => &["  ~╲│╱~  ", "  ~(▓▓)~ ", "  ~╱│╲~  ", "", ""],
        (Action::Train, 0) => &["╲ │ ╱  !!", " (▓▓▓▓) !", "╱ │ ╲    ", "", ""],
        (Action::Train, _) => &["!!╲│ ╱   ", "! (▓▓▓▓) ", "  ╱│ ╲   ", "", ""],
        (Action::Relax, 0) => &["  ╲│╱  z ", "   (▓)   ", "  ╱│╲    ", "", ""],
        (Action::Relax, _) => &["   ╲│╱ zZ", "   (▓)   ", "   ╱│╲   ", "", ""],
    }
}

// --- ホシゾラ (hoshizora) - Constellation: dots connected by lines ---
fn hoshizora_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["✦──✦──✦ !", " ╲  │ ╱  ", "  ✦─✦    ", "", ""],
        (MoodLevel::High, _) => &["!✦──✦──✦ ", "  ╲ │╱   ", "   ✦─✦   ", "", ""],
        (MoodLevel::Normal, 0) => &[" ✦──✦    ", "  ╲ │    ", "   ✦─✦   ", "", ""],
        (MoodLevel::Normal, _) => &["    ✦──✦ ", "    │ ╱  ", "   ✦─✦   ", "", ""],
        (MoodLevel::Low, 0) => &["  ✦  ✦   ", "   ╲╱    ", "    ✦    ", "", ""],
        (MoodLevel::Low, _) => &["   ✦  ✦  ", "    ╲╱   ", "    ✦    ", "", ""],
    }
}
fn hoshizora_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ✦──✦  ♪ ", "  ╲ │    ", "   ✦─✦   ", "", ""],
        (Action::Talk, _) => &["♫ ✦──✦   ", "   │ ╱   ", "   ✦─✦   ", "", ""],
        (Action::Play, 0) => &[" ✦~~✦~~✦ ", " ~╲ │ ╱~ ", "  ✦~✦    ", "", ""],
        (Action::Play, _) => &["  ✦~~✦~~✦", "  ~╲│╱~  ", "   ✦~✦   ", "", ""],
        (Action::Train, 0) => &["✦──✦──✦!!", " ╲  │ ╱ !", "  ✦─✦─✦  ", "", ""],
        (Action::Train, _) => &["!!✦──✦──✦", " !╲ │ ╱  ", "  ✦─✦─✦  ", "", ""],
        (Action::Relax, 0) => &["  ✦  ✦ z ", "   ╲╱    ", "    ✦    ", "", ""],
        (Action::Relax, _) => &["   ✦  ✦zZ", "    ╲╱   ", "    ✦    ", "", ""],
    }
}

// --- ブッチギリ (bucchigiri) - Rhino: charging horn, big body, dust ---
fn bucchigiri_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["▷▷ [▓▓▓]!", "   [▓▓▓▓]", "  *╰╯╰╯ *", "", ""],
        (MoodLevel::High, _) => &["!▷▷[▓▓▓] ", "  [▓▓▓▓] ", " * ╰╯╰╯* ", "", ""],
        (MoodLevel::Normal, 0) => &[" ▷ [▓▓▓] ", "   [▓▓▓] ", "   ╰╯╰╯  ", "", ""],
        (MoodLevel::Normal, _) => &["  ▷[▓▓▓] ", "   [▓▓▓] ", "   ╰╯╰╯  ", "", ""],
        (MoodLevel::Low, 0) => &["  > [▓▓] ", "    [▓▓] ", "    ╰ ╰  ", "", ""],
        (MoodLevel::Low, _) => &["   >[▓▓] ", "    [▓▓] ", "    ╰ ╰  ", "", ""],
    }
}
fn bucchigiri_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ▷ [▓▓▓]♪", "   [▓▓▓] ", "   ╰╯╰╯  ", "", ""],
        (Action::Talk, _) => &["♫▷ [▓▓▓] ", "   [▓▓▓] ", "   ╰╯╰╯  ", "", ""],
        (Action::Play, 0) => &[" ▷~[▓▓▓]~", "  ~[▓▓▓] ", "  ~╰╯╰╯  ", "", ""],
        (Action::Play, _) => &["~▷ [▓▓▓] ", "   [▓▓▓]~", "   ╰╯╰╯~ ", "", ""],
        (Action::Train, 0) => &["▷▷▷[▓▓▓]!!", "  [▓▓▓▓▓]", " **╰╯╰╯**", "", ""],
        (Action::Train, _) => &["!!▷▷▷[▓▓▓]", "  [▓▓▓▓▓] ", " ** ╰╯╰╯**", "", ""],
        (Action::Relax, 0) => &["  > [▓▓] z", "    [▓▓] ", "    ╰ ╰  ", "", ""],
        (Action::Relax, _) => &["   >[▓▓]zZ", "    [▓▓] ", "    ╰ ╰  ", "", ""],
    }
}

// --- ワタリ (watari) - Crane: long beak, small body, VERY long legs ---
fn watari_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" >─(▓) ! ", "    |||  ", "    |||  ", "", ""],
        (MoodLevel::High, _) => &[" ! >─(▓) ", "    |||  ", "    |||  ", "", ""],
        (MoodLevel::Normal, 0) => &["  >─(▓)  ", "    |||  ", "    |||  ", "", ""],
        (MoodLevel::Normal, _) => &["  >─(▓)  ", "    ||│  ", "    ||│  ", "", ""],
        (MoodLevel::Low, 0) => &["  >─(▓)  ", "    ||   ", "    ||   ", "", ""],
        (MoodLevel::Low, _) => &["   >─(▓) ", "     ||  ", "     ||  ", "", ""],
    }
}
fn watari_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  >─(▓)♪ ", "    |||  ", "    |||  ", "", ""],
        (Action::Talk, _) => &["♫ >─(▓)  ", "    |||  ", "    |||  ", "", ""],
        (Action::Play, 0) => &[" ~>─(▓)  ", "   ~|||  ", "    |||  ", "", ""],
        (Action::Play, _) => &["  >─(▓)~ ", "    |||~ ", "    |||  ", "", ""],
        (Action::Train, 0) => &[">>──(▓)!!", "    ||||!", "    |||| ", "", ""],
        (Action::Train, _) => &["!!>>──(▓)", " !  ||||", "    |||| ", "", ""],
        (Action::Relax, 0) => &["  >─(▓) z", "    ||   ", "    ||   ", "", ""],
        (Action::Relax, _) => &["  >─(▓)zZ", "     ||  ", "     ||  ", "", ""],
    }
}

// --- ヒュー (hyuu) - Arrow/Missile: pointed, triple body lines, trail ---
fn hyuu_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["─▷▷═══── ", " ─▷═══── ", "─▷▷═══── ", "", ""],
        (MoodLevel::High, _) => &[" ─▷▷═══──", "  ─▷═══──", " ─▷▷═══──", "", ""],
        (MoodLevel::Normal, 0) => &[" ─▷═══── ", "  ─▷═══  ", " ─▷═══── ", "", ""],
        (MoodLevel::Normal, _) => &["  ─▷═══──", "  ─▷═══  ", "  ─▷═══──", "", ""],
        (MoodLevel::Low, 0) => &["  ─▷══   ", "   ▷══   ", "  ─▷══   ", "", ""],
        (MoodLevel::Low, _) => &["   ─▷══  ", "   ▷══   ", "   ─▷══  ", "", ""],
    }
}
fn hyuu_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ─▷═══♪  ", "  ─▷═══  ", " ─▷═══── ", "", ""],
        (Action::Talk, _) => &["♫─▷═══── ", "  ─▷═══  ", "  ─▷═══──", "", ""],
        (Action::Play, 0) => &[" ~▷═══──~", " ~─▷═══~ ", " ~▷═══──~", "", ""],
        (Action::Play, _) => &["~─▷═══── ", " ~─▷═══~ ", "~─▷═══── ", "", ""],
        (Action::Train, 0) => &["─▷▷▷════!", " ─▷▷═════", "─▷▷▷════!", "", ""],
        (Action::Train, _) => &["!─▷▷▷════", " ─▷▷═════", "!─▷▷▷════", "", ""],
        (Action::Relax, 0) => &["  ─▷══  z", "   ▷══   ", "  ─▷══   ", "", ""],
        (Action::Relax, _) => &["   ─▷══zZ", "   ▷══   ", "   ─▷══  ", "", ""],
    }
}

// --- タンケン (tanken) - Hermit Crab: shell with gear on top, legs ---
fn tanken_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ☆  !   ", " {▓▓▓▓▓} ", "  ⌐⅃⌐⅃  ", "", ""],
        (MoodLevel::High, _) => &["!  ☆     ", " {▓▓▓▓▓} ", "  ⅃⌐⅃⌐  ", "", ""],
        (MoodLevel::Normal, 0) => &["   ☆     ", "  {▓▓▓}  ", "   ⌐⅃   ", "", ""],
        (MoodLevel::Normal, _) => &["   ☆     ", "  {▓▓▓}  ", "   ⅃⌐   ", "", ""],
        (MoodLevel::Low, 0) => &["   .     ", "  {▓▓}   ", "   ⌐⅃   ", "", ""],
        (MoodLevel::Low, _) => &["   .     ", "   {▓▓}  ", "   ⅃⌐   ", "", ""],
    }
}
fn tanken_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["   ☆  ♪  ", "  {▓▓▓}  ", "   ⌐⅃   ", "", ""],
        (Action::Talk, _) => &["  ♫☆     ", "  {▓▓▓}  ", "   ⅃⌐   ", "", ""],
        (Action::Play, 0) => &["  ~☆~    ", " ~{▓▓▓}~ ", "  ~⌐⅃~  ", "", ""],
        (Action::Play, _) => &["   ~☆~   ", "  ~{▓▓▓}~", "   ~⅃⌐~ ", "", ""],
        (Action::Train, 0) => &["  ☆☆  !! ", " {▓▓▓▓▓}!", "  ⌐⅃⌐⅃  ", "", ""],
        (Action::Train, _) => &["!!☆☆     ", "!{▓▓▓▓▓} ", "  ⅃⌐⅃⌐  ", "", ""],
        (Action::Relax, 0) => &["   .  z  ", "  {▓▓}   ", "   ⌐⅃   ", "", ""],
        (Action::Relax, _) => &["   .  zZ ", "   {▓▓}  ", "   ⅃⌐   ", "", ""],
    }
}

// --- ジェット (jetto) - Rocket: vertical, body, exhaust below ---
fn jetto_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["   /▓\\  !", "  [▓▓▓]  ", " ※炎炎※  ", "", ""],
        (MoodLevel::High, _) => &["  !/▓\\   ", "  [▓▓▓]  ", "  ※炎炎※ ", "", ""],
        (MoodLevel::Normal, 0) => &["   /▓\\   ", "  [▓▓▓]  ", "   ※炎※  ", "", ""],
        (MoodLevel::Normal, _) => &["   /▓\\   ", "  [▓▓▓]  ", "  ※炎※   ", "", ""],
        (MoodLevel::Low, 0) => &["   /▓\\   ", "   [▓]   ", "    .    ", "", ""],
        (MoodLevel::Low, _) => &["   /▓\\   ", "   [▓]   ", "    .    ", "", ""],
    }
}
fn jetto_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["   /▓\\ ♪ ", "  [▓▓▓]  ", "   ※炎※  ", "", ""],
        (Action::Talk, _) => &[" ♫ /▓\\   ", "  [▓▓▓]  ", "  ※炎※   ", "", ""],
        (Action::Play, 0) => &["  ~/▓\\~  ", " ~[▓▓▓]~ ", "  ※炎炎※ ", "", ""],
        (Action::Play, _) => &[" ~ /▓\\ ~ ", "  ~[▓▓▓]~", " ※炎炎※  ", "", ""],
        (Action::Train, 0) => &["   /▓\\ !!", "  [▓▓▓▓]!", " ※炎炎炎※", "", ""],
        (Action::Train, _) => &["!! /▓\\   ", "! [▓▓▓▓] ", " ※炎炎炎※", "", ""],
        (Action::Relax, 0) => &["   /▓\\  z", "   [▓]   ", "    .    ", "", ""],
        (Action::Relax, _) => &["   /▓\\ zZ", "   [▓]   ", "    .    ", "", ""],
    }
}

// ============================================================
// NORMAL TYPE Stage 3 Species
// ============================================================

// --- ノーマル (noomaru) - Plain blocks, simplest shape ---
fn noomaru_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["         ", "  (≧▽≦)ﾉ", "         ", "", ""],
        (MoodLevel::High, _) => &["         ", " ﾉ(≧▽≦)  ", "         ", "", ""],
        (MoodLevel::Normal, 0) => &["         ", "  (・ω・) ", "         ", "", ""],
        (MoodLevel::Normal, _) => &["         ", "   (・ω・)", "         ", "", ""],
        (MoodLevel::Low, 0) => &["         ", "  (￣ω￣) ", "         ", "", ""],
        (MoodLevel::Low, _) => &["         ", "   (￣ω￣)", "         ", "", ""],
    }
}
fn noomaru_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["         ", "  (・ω・)♪", "         ", "", ""],
        (Action::Talk, _) => &["♫        ", "  (・ω・) ", "         ", "", ""],
        (Action::Play, 0) => &["         ", " ﾉ(≧▽≦)＼", "         ", "", ""],
        (Action::Play, _) => &["         ", "  (≧▽≦)ﾉ ", "         ", "", ""],
        (Action::Train, 0) => &["         ", " (＞ω＜)！", "         ", "", ""],
        (Action::Train, _) => &["         ", "！(＞ω＜) ", "         ", "", ""],
        (Action::Relax, 0) => &["         ", "  (-ω-)  ", "   zzz   ", "", ""],
        (Action::Relax, _) => &["         ", "  (-ω-) z", "    zZ   ", "", ""],
    }
}

// --- ヘイボン (heibon) - Asymmetric oval with ONE tiny eye ---
fn heibon_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" .~~-.,  ", "(  ·    )!", " '-.~~'  ", "", ""],
        (MoodLevel::High, _) => &["  .~~-., ", "!(    · )", "  '-.~~' ", "", ""],
        (MoodLevel::Normal, 0) => &[" .~~-.,  ", "(  ·    )", " '-.~~'  ", "", ""],
        (MoodLevel::Normal, _) => &[" .~~-.,  ", "(    ·  )", " '-.~~'  ", "", ""],
        (MoodLevel::Low, 0) => &["  .~-.,  ", " (  .  ) ", "  '-~'   ", "", ""],
        (MoodLevel::Low, _) => &["  .~-.,  ", " ( .   ) ", "  '-~'   ", "", ""],
    }
}
fn heibon_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" .~~-., ♪", "(  ·    )", " '-.~~'  ", "", ""],
        (Action::Talk, _) => &["♫.~~-.,  ", "(    ·  )", " '-.~~'  ", "", ""],
        (Action::Play, 0) => &[" .~~-.,~ ", "( ·     )", " '-.~~'  ", "", ""],
        (Action::Play, _) => &["~.~~-.,  ", "(     · )", " '-.~~'  ", "", ""],
        (Action::Train, 0) => &[" .~~-.,! ", "(  ·    )!", " '-.~~'  ", "", ""],
        (Action::Train, _) => &["!.~~-.,  ", "!(   · ) ", " '-.~~'  ", "", ""],
        (Action::Relax, 0) => &["  .~-.,  ", " (  .  )z", "  '-~'   ", "", ""],
        (Action::Relax, _) => &["  .~-.,  ", " ( .   ) ", "  '-~' zZ", "", ""],
    }
}

// --- タソガレ (tasogare) - Constellation of dots and stars ---
fn tasogare_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[".:*:.*:.*:", " *:.*:.:* ", ".:*:.*:.*:", "", ""],
        (MoodLevel::High, _) => &["*:.*:.:*:.", " .:*:.*:* ", "*:.*:.:*:.", "", ""],
        (MoodLevel::Normal, 0) => &[" .:*:.*:. ", "  *:.:*   ", " .:*:.*:. ", "", ""],
        (MoodLevel::Normal, _) => &[" *:.:*:.  ", "  .:*:.   ", " *:.:*:.  ", "", ""],
        (MoodLevel::Low, 0) => &["  .:..:   ", "   :.:    ", "  .:..:   ", "", ""],
        (MoodLevel::Low, _) => &["   .:..   ", "  .:.     ", "   .:..   ", "", ""],
    }
}
fn tasogare_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" .:*:.*:.♪", "  *:.:*   ", " .:*:.*:. ", "", ""],
        (Action::Talk, _) => &["♫.:*:.*:. ", "  *:.:*   ", " .:*:.*:. ", "", ""],
        (Action::Play, 0) => &[".:*:.*:.*:", " *:.*:.:* ", ".:*:.*:.*:", "", ""],
        (Action::Play, _) => &["*:.*:.:*:.", " .:*:.*:* ", "*:.*:.:*:.", "", ""],
        (Action::Train, 0) => &["*:*:.*:.*:", " *:.*:.:*!", ".:*:.*:.*:", "", ""],
        (Action::Train, _) => &["!:*:.*:.*:", " *:.*:.:* ", "*:*:.*:.*:", "", ""],
        (Action::Relax, 0) => &["  .:..: z ", "   :.:    ", "  .:..:   ", "", ""],
        (Action::Relax, _) => &["  .:..:   ", "   :.: zZ ", "  .:..:   ", "", ""],
    }
}

// --- ニッコリ (nikkori) - Bright smile face ---
fn nikkori_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["         ", "  (≧▽≦)！", "         ", "", ""],
        (MoodLevel::High, _) => &["         ", " (≧▽≦)   ", "         ", "", ""],
        (MoodLevel::Normal, 0) => &["         ", "  (＾▽＾) ", "         ", "", ""],
        (MoodLevel::Normal, _) => &["         ", "   (＾▽＾)", "         ", "", ""],
        (MoodLevel::Low, 0) => &["         ", "  (・_・) ", "         ", "", ""],
        (MoodLevel::Low, _) => &["         ", "   (・_・)", "         ", "", ""],
    }
}
fn nikkori_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["         ", "  (＾▽＾)♪", "         ", "", ""],
        (Action::Talk, _) => &["♫        ", "  (＾▽＾) ", "         ", "", ""],
        (Action::Play, 0) => &["         ", " ﾉ(≧▽≦)＼", "         ", "", ""],
        (Action::Play, _) => &["         ", "  (≧▽≦)ﾉ ", "         ", "", ""],
        (Action::Train, 0) => &["         ", " (＾▽＾)！ ", "         ", "", ""],
        (Action::Train, _) => &["         ", "！(＾▽＾)  ", "         ", "", ""],
        (Action::Relax, 0) => &["         ", "  (＾ω＾)z", "         ", "", ""],
        (Action::Relax, _) => &["       zZ", "  (＾ω＾) ", "         ", "", ""],
    }
}

// --- ダラーン (daraan) - Slack/lazy face ---
fn daraan_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["         ", "  (ーωー)~", "         ", "", ""],
        (MoodLevel::High, _) => &["         ", "  ~(ーωー)", "         ", "", ""],
        (MoodLevel::Normal, 0) => &["         ", "  (ーωー) ", "         ", "", ""],
        (MoodLevel::Normal, _) => &["         ", "   (ーωー)", "         ", "", ""],
        (MoodLevel::Low, 0) => &["         ", "  (ー_ー) ", "         ", "", ""],
        (MoodLevel::Low, _) => &["         ", "   (ー_ー)", "         ", "", ""],
    }
}
fn daraan_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["         ", "  (ーωー)♪", "         ", "", ""],
        (Action::Talk, _) => &["♫        ", "  (ーωー) ", "         ", "", ""],
        (Action::Play, 0) => &["         ", "  (ーωー)~", "    ~    ", "", ""],
        (Action::Play, _) => &["    ~    ", "  ~(ーωー)", "         ", "", ""],
        (Action::Train, 0) => &["         ", " (ーωー)！ ", "         ", "", ""],
        (Action::Train, _) => &["         ", "！(ーωー)  ", "         ", "", ""],
        (Action::Relax, 0) => &["         ", "  (ー_ー) ", "   zzz   ", "", ""],
        (Action::Relax, _) => &["         ", "  (ー_ー)z", "    zZ   ", "", ""],
    }
}

// --- キッチリ (kicchiri) - Perfect geometric square with one dot ---
fn kicchiri_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["┌────────┐", "│   ·    │", "└────────┘", "", ""],
        (MoodLevel::High, _) => &["┌────────┐", "│    ·   │", "└────────┘", "", ""],
        (MoodLevel::Normal, 0) => &[" ┌──────┐", " │  ·   │", " └──────┘", "", ""],
        (MoodLevel::Normal, _) => &[" ┌──────┐", " │  ·   │", " └──────┘", "", ""],
        (MoodLevel::Low, 0) => &["  ┌────┐ ", "  │ .  │ ", "  └────┘ ", "", ""],
        (MoodLevel::Low, _) => &["  ┌────┐ ", "  │  . │ ", "  └────┘ ", "", ""],
    }
}
fn kicchiri_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ┌──────┐♪", " │  ·   │ ", " └──────┘ ", "", ""],
        (Action::Talk, _) => &["♫┌──────┐ ", " │  ·   │ ", " └──────┘ ", "", ""],
        (Action::Play, 0) => &[" ┌──────┐ ", "~│  ·   │~", " └──────┘ ", "", ""],
        (Action::Play, _) => &[" ┌──────┐ ", " │  ·   │ ", "~└──────┘~", "", ""],
        (Action::Train, 0) => &["┌────────┐!", "│   ·    │ ", "└────────┘ ", "", ""],
        (Action::Train, _) => &["!┌────────┐", " │    ·   │", " └────────┘", "", ""],
        (Action::Relax, 0) => &["  ┌────┐ ", "  │ .  │z", "  └────┘ ", "", ""],
        (Action::Relax, _) => &["  ┌────┐  ", "  │  . │  ", "  └────┘zZ", "", ""],
    }
}

// --- ボチボチ (bochibochi) - Two blobs connected by line ---
fn bochibochi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["         ", "(o)━━━(o)!", "         ", "", ""],
        (MoodLevel::High, _) => &["         ", "!(o)━━━(o)", "         ", "", ""],
        (MoodLevel::Normal, 0) => &["         ", " (o)━━(o) ", "         ", "", ""],
        (MoodLevel::Normal, _) => &["         ", " (o)━━(o) ", "         ", "", ""],
        (MoodLevel::Low, 0) => &["         ", "  (.)━(.) ", "         ", "", ""],
        (MoodLevel::Low, _) => &["         ", " (.)━(.)  ", "         ", "", ""],
    }
}
fn bochibochi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["        ♪", " (o)━━(o) ", "         ", "", ""],
        (Action::Talk, _) => &["♫        ", " (o)━━(o) ", "         ", "", ""],
        (Action::Play, 0) => &["         ", "(o)~━━~(o)", "         ", "", ""],
        (Action::Play, _) => &["         ", "~(o)━━(o)~", "         ", "", ""],
        (Action::Train, 0) => &["    !    ", "(o)━━━━(o)", "         ", "", ""],
        (Action::Train, _) => &["         ", "(o)━━━━(o)", "    !    ", "", ""],
        (Action::Relax, 0) => &["         ", " (.)━(.)z", "         ", "", ""],
        (Action::Relax, _) => &["       zZ", " (.)━(.) ", "         ", "", ""],
    }
}

// --- マアマア (maamaa) - Wavy amoeba, no features ---
fn maamaa_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ~-__-~  ", " {      }", " ~-__-~  ", "", ""],
        (MoodLevel::High, _) => &["  ~-__-~ ", "{      } ", "  ~-__-~ ", "", ""],
        (MoodLevel::Normal, 0) => &["  ~-__-~ ", "  {    } ", "  ~-__-~ ", "", ""],
        (MoodLevel::Normal, _) => &["  ~-__-~ ", "  {    } ", "  ~-__-~ ", "", ""],
        (MoodLevel::Low, 0) => &["   ~__~  ", "   {  }  ", "   ~__~  ", "", ""],
        (MoodLevel::Low, _) => &["   ~__~  ", "   {  }  ", "   ~__~  ", "", ""],
    }
}
fn maamaa_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ~-__-~♪", "  {    } ", "  ~-__-~ ", "", ""],
        (Action::Talk, _) => &["♫ ~-__-~ ", "  {    } ", "  ~-__-~ ", "", ""],
        (Action::Play, 0) => &[" ~-__-~  ", "~{      }~", " ~-__-~  ", "", ""],
        (Action::Play, _) => &["  ~-__-~ ", " ~{    }~ ", "  ~-__-~ ", "", ""],
        (Action::Train, 0) => &[" ~-__-~ !", " {      }", " ~-__-~  ", "", ""],
        (Action::Train, _) => &["! ~-__-~ ", "{      } ", " ~-__-~  ", "", ""],
        (Action::Relax, 0) => &["   ~__~  ", "   {  } z", "   ~__~  ", "", ""],
        (Action::Relax, _) => &["   ~__~ zZ", "   {  }  ", "   ~__~  ", "", ""],
    }
}

// --- フニャ (funya) - Soft/squished face ---
fn funya_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["   ^^    ", "  (´▽｀)ﾉ", "         ", "", ""],
        (MoodLevel::High, _) => &["    ^^   ", " ﾉ(´▽｀)  ", "         ", "", ""],
        (MoodLevel::Normal, 0) => &["         ", "  (´ω｀)  ", "         ", "", ""],
        (MoodLevel::Normal, _) => &["         ", "   (´ω｀) ", "         ", "", ""],
        (MoodLevel::Low, 0) => &["         ", "  (´_｀)  ", "         ", "", ""],
        (MoodLevel::Low, _) => &["         ", "   (´_｀) ", "         ", "", ""],
    }
}
fn funya_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["         ", "  (´ω｀)♪ ", "         ", "", ""],
        (Action::Talk, _) => &["♫        ", "  (´ω｀)  ", "         ", "", ""],
        (Action::Play, 0) => &["   ^^    ", "  (´▽｀)ﾉ ", "         ", "", ""],
        (Action::Play, _) => &["    ^^   ", " ﾉ(´▽｀)  ", "         ", "", ""],
        (Action::Train, 0) => &["         ", " (´▽｀)！  ", "         ", "", ""],
        (Action::Train, _) => &["         ", "！(´▽｀)   ", "         ", "", ""],
        (Action::Relax, 0) => &["         ", "  (´_｀) z", "         ", "", ""],
        (Action::Relax, _) => &["       zZ", "  (´_｀)  ", "         ", "", ""],
    }
}

// --- テンテン (tenten) - Made entirely of dots ---
fn tenten_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["·:·:·:·:·", " :·:·:·: ", "·:·:·:·:·", "", ""],
        (MoodLevel::High, _) => &[":·:·:·:·:", "·:·:·:·:·", ":·:·:·:·:", "", ""],
        (MoodLevel::Normal, 0) => &[" ·:·:·:· ", "  :·:·:  ", " ·:·:·:· ", "", ""],
        (MoodLevel::Normal, _) => &[" :·:·:·  ", "  ·:·:   ", " :·:·:·  ", "", ""],
        (MoodLevel::Low, 0) => &["  · : ·  ", "   : :   ", "  · : ·  ", "", ""],
        (MoodLevel::Low, _) => &["  : · :  ", "   · ·   ", "  : · :  ", "", ""],
    }
}
fn tenten_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ·:·:·:·♪", "  :·:·:  ", " ·:·:·:· ", "", ""],
        (Action::Talk, _) => &["♫·:·:·:· ", "  :·:·:  ", " ·:·:·:· ", "", ""],
        (Action::Play, 0) => &["·:·:·:·:·", " :·:·:·: ", "·:·:·:·:·", "", ""],
        (Action::Play, _) => &[":·:·:·:·:", "·:·:·:·:·", ":·:·:·:·:", "", ""],
        (Action::Train, 0) => &["·:·:·:·:·!", " :·:·:·:  ", "·:·:·:·:·!", "", ""],
        (Action::Train, _) => &["!:·:·:·:·:", " ·:·:·:·: ", "!:·:·:·:·:", "", ""],
        (Action::Relax, 0) => &["  · : · z", "   : :   ", "  · : ·  ", "", ""],
        (Action::Relax, _) => &["  : · : zZ", "   · ·   ", "  : · :  ", "", ""],
    }
}

// --- ナァナァ (naanaa) - Leaning parallelogram ---
fn naanaa_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  /‾‾‾/  ", " /   /   ", "/___/    ", "", ""],
        (MoodLevel::High, _) => &["   /‾‾‾/ ", "  /   /  ", " /___/   ", "", ""],
        (MoodLevel::Normal, 0) => &["  /‾‾/   ", " /  /    ", "/‾‾/     ", "", ""],
        (MoodLevel::Normal, _) => &["   /‾‾/  ", "  /  /   ", " /‾‾/    ", "", ""],
        (MoodLevel::Low, 0) => &[" /‾/     ", "/  /     ", "/__/     ", "", ""],
        (MoodLevel::Low, _) => &["/‾/      ", "/  /     ", "/__/     ", "", ""],
    }
}
fn naanaa_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  /‾‾/ ♪ ", " /  /    ", "/‾‾/     ", "", ""],
        (Action::Talk, _) => &["♫ /‾‾/   ", " /  /    ", "/‾‾/     ", "", ""],
        (Action::Play, 0) => &["  /‾‾‾/~ ", " /   /   ", "/___/    ", "", ""],
        (Action::Play, _) => &[" ~/‾‾‾/  ", "  /   /  ", " /___/   ", "", ""],
        (Action::Train, 0) => &["  /‾‾‾/! ", " /   /   ", "/___/    ", "", ""],
        (Action::Train, _) => &["!/‾‾‾/   ", " /   /   ", "/___/    ", "", ""],
        (Action::Relax, 0) => &[" /‾/   z ", "/  /     ", "/__/     ", "", ""],
        (Action::Relax, _) => &[" /‾/  zZ ", "/  /     ", "/__/     ", "", ""],
    }
}

// --- ポツリ (potsuri) - Solitary/lonely face ---
fn potsuri_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["         ", "  (；ω；) ", "   ...   ", "", ""],
        (MoodLevel::High, _) => &["         ", "   (；ω；)", "   ...   ", "", ""],
        (MoodLevel::Normal, 0) => &["         ", "  (；_；) ", "   ...   ", "", ""],
        (MoodLevel::Normal, _) => &["         ", "   (；_；)", "   ...   ", "", ""],
        (MoodLevel::Low, 0) => &["         ", "  (T_T)  ", "   ...   ", "", ""],
        (MoodLevel::Low, _) => &["         ", "   (T_T) ", "   ...   ", "", ""],
    }
}
fn potsuri_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["         ", "  (；_；)♪", "   ...   ", "", ""],
        (Action::Talk, _) => &["♫        ", "  (；_；) ", "   ...   ", "", ""],
        (Action::Play, 0) => &["         ", " (；ω；)ﾉ ", "   ~!    ", "", ""],
        (Action::Play, _) => &["         ", "  (；ω；) ", "    ~!   ", "", ""],
        (Action::Train, 0) => &["         ", " (；；)！  ", "   ...   ", "", ""],
        (Action::Train, _) => &["         ", "！(；；)   ", "   ...   ", "", ""],
        (Action::Relax, 0) => &["         ", "  (T_T) z", "   ...   ", "", ""],
        (Action::Relax, _) => &["       zZ", "  (T_T)  ", "   ...   ", "", ""],
    }
}

// --- ソレナリ (sorenari) - Pentagon shape ---
fn sorenari_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  /‾‾‾\\  ", " /     \\ ", " \\_____/ ", "", ""],
        (MoodLevel::High, _) => &["  /‾‾‾\\  ", " /     \\!", " \\_____/ ", "", ""],
        (MoodLevel::Normal, 0) => &["  /‾‾‾\\  ", " /     \\ ", " \\_____/ ", "", ""],
        (MoodLevel::Normal, _) => &["  /‾‾‾\\  ", " /     \\ ", " \\_____/ ", "", ""],
        (MoodLevel::Low, 0) => &["   /‾\\   ", "  /   \\  ", "  \\___/  ", "", ""],
        (MoodLevel::Low, _) => &["   /‾\\   ", "  /   \\  ", "  \\___/  ", "", ""],
    }
}
fn sorenari_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  /‾‾‾\\ ♪", " /     \\ ", " \\_____/ ", "", ""],
        (Action::Talk, _) => &["♫/‾‾‾\\   ", " /     \\ ", " \\_____/ ", "", ""],
        (Action::Play, 0) => &[" ~/‾‾‾\\~ ", " /     \\ ", " \\_____/ ", "", ""],
        (Action::Play, _) => &["  /‾‾‾\\  ", "~/     \\~", " \\_____/ ", "", ""],
        (Action::Train, 0) => &["  /‾‾‾\\ !", " /     \\ ", " \\_____/ ", "", ""],
        (Action::Train, _) => &["! /‾‾‾\\  ", " /     \\ ", " \\_____/ ", "", ""],
        (Action::Relax, 0) => &["   /‾\\  z", "  /   \\  ", "  \\___/  ", "", ""],
        (Action::Relax, _) => &["   /‾\\ zZ", "  /   \\  ", "  \\___/  ", "", ""],
    }
}

// --- ウンウン (unun) - Mushroom cap on stalk ---
fn unun_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[".=========.", "    |||    ", "    |||    ", "", ""],
        (MoodLevel::High, _) => &[" .=======. ", "    |||    ", "    |||    ", "", ""],
        (MoodLevel::Normal, 0) => &[" .=======. ", "    |||    ", "    |||    ", "", ""],
        (MoodLevel::Normal, _) => &[" .=======. ", "    |||    ", "    |||    ", "", ""],
        (MoodLevel::Low, 0) => &["  .=====.  ", "    ||     ", "    ||     ", "", ""],
        (MoodLevel::Low, _) => &["  .=====.  ", "     ||    ", "     ||    ", "", ""],
    }
}
fn unun_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" .=======.♪", "    |||    ", "    |||    ", "", ""],
        (Action::Talk, _) => &["♫.=======. ", "    |||    ", "    |||    ", "", ""],
        (Action::Play, 0) => &[".=========.", "   ~|||~   ", "    |||    ", "", ""],
        (Action::Play, _) => &[" .=======. ", "    |||    ", "   ~|||~   ", "", ""],
        (Action::Train, 0) => &[".=========.!", "    |||     ", "    |||     ", "", ""],
        (Action::Train, _) => &["!.=========.", "     |||    ", "     |||    ", "", ""],
        (Action::Relax, 0) => &["  .=====. z", "    ||     ", "    ||     ", "", ""],
        (Action::Relax, _) => &["  .=====.  ", "    ||  zZ ", "    ||     ", "", ""],
    }
}

// --- チャッカリ (chakkari) - Clever/cunning face ---
fn chakkari_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["         ", " (^ω^)ﾉ  ", "         ", "", ""],
        (MoodLevel::High, _) => &["         ", "  (^ω^)！ ", "         ", "", ""],
        (MoodLevel::Normal, 0) => &["         ", "  (^ω^)  ", "         ", "", ""],
        (MoodLevel::Normal, _) => &["         ", "   (^ω^) ", "         ", "", ""],
        (MoodLevel::Low, 0) => &["         ", "  (・_・) ", "         ", "", ""],
        (MoodLevel::Low, _) => &["         ", "   (・_・)", "         ", "", ""],
    }
}
fn chakkari_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["         ", "  (^ω^)♪ ", "         ", "", ""],
        (Action::Talk, _) => &["♫        ", "  (^ω^)  ", "         ", "", ""],
        (Action::Play, 0) => &["         ", " (^ω^)~  ", "         ", "", ""],
        (Action::Play, _) => &["         ", "  ~(^ω^) ", "         ", "", ""],
        (Action::Train, 0) => &["         ", " (^ω^)！  ", "         ", "", ""],
        (Action::Train, _) => &["         ", "！(^ω^)   ", "         ", "", ""],
        (Action::Relax, 0) => &["         ", "  (・_・)z", "         ", "", ""],
        (Action::Relax, _) => &["       zZ", "  (・_・) ", "         ", "", ""],
    }
}

// --- ヌルリ (nururi) - Slug with slimy trail ---
fn nururi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["         ", " <====3  ", " ~~~~~~  ", "", ""],
        (MoodLevel::High, _) => &["         ", "  <====3 ", "  ~~~~~~ ", "", ""],
        (MoodLevel::Normal, 0) => &["         ", "  <===3  ", "  ~~~~   ", "", ""],
        (MoodLevel::Normal, _) => &["         ", "  <===3  ", "   ~~~~  ", "", ""],
        (MoodLevel::Low, 0) => &["         ", "   <=3   ", "   ~~    ", "", ""],
        (MoodLevel::Low, _) => &["         ", "   <=3   ", "    ~~   ", "", ""],
    }
}
fn nururi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["        ♪", "  <===3  ", "  ~~~~   ", "", ""],
        (Action::Talk, _) => &["♫        ", "  <===3  ", "  ~~~~   ", "", ""],
        (Action::Play, 0) => &["         ", " ~<===3~ ", "  ~~~~   ", "", ""],
        (Action::Play, _) => &["         ", "~ <===3  ", "  ~~~~   ", "", ""],
        (Action::Train, 0) => &["        !", " <====3  ", " ~~~~~~  ", "", ""],
        (Action::Train, _) => &["!        ", "  <====3 ", "  ~~~~~~ ", "", ""],
        (Action::Relax, 0) => &["       z ", "   <=3   ", "   ~~    ", "", ""],
        (Action::Relax, _) => &["      zZ ", "   <=3   ", "    ~~   ", "", ""],
    }
}

// --- ヤレヤレ (yareyare) - Weary/exasperated face ---
fn yareyare_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["         ", " (￣▽￣)ﾉ ", "  ﾔﾚﾔﾚ   ", "", ""],
        (MoodLevel::High, _) => &["         ", "  (￣▽￣)  ", "   ﾔﾚﾔﾚ  ", "", ""],
        (MoodLevel::Normal, 0) => &["         ", "  (￣ω￣) ", "   ...   ", "", ""],
        (MoodLevel::Normal, _) => &["         ", "   (￣ω￣)", "   ...   ", "", ""],
        (MoodLevel::Low, 0) => &["  -   -  ", "  (ー_ー) ", "   ...   ", "", ""],
        (MoodLevel::Low, _) => &["   -   - ", "   (ー_ー)", "   ...   ", "", ""],
    }
}
fn yareyare_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["         ", "  (￣ω￣)♪", "   ...   ", "", ""],
        (Action::Talk, _) => &["♫        ", "  (￣ω￣) ", "   ...   ", "", ""],
        (Action::Play, 0) => &["         ", " (￣▽￣)ﾉ ", "  ﾔﾚﾔﾚ   ", "", ""],
        (Action::Play, _) => &["         ", "  (￣▽￣)  ", "   ﾔﾚﾔﾚ  ", "", ""],
        (Action::Train, 0) => &["         ", " (￣□￣)！ ", "  ﾔﾚﾔﾚ   ", "", ""],
        (Action::Train, _) => &["         ", "！(￣□￣)  ", "   ﾔﾚﾔﾚ  ", "", ""],
        (Action::Relax, 0) => &["  -   -  ", "  (ー_ー)z", "   ...   ", "", ""],
        (Action::Relax, _) => &["   -  -  ", "  (ー_ー) ", "   ...zZ ", "", ""],
    }
}

// --- ドッコイ (dokkoi) - Off-balance, tilted shape ---
fn dokkoi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["   __/   ", "  |  /   ", "  |_/    ", "", ""],
        (MoodLevel::High, _) => &["  \\__    ", "  \\  |   ", "   \\_|   ", "", ""],
        (MoodLevel::Normal, 0) => &["   __/   ", "  |  /   ", "  |_/    ", "", ""],
        (MoodLevel::Normal, _) => &["  \\__    ", "  \\  |   ", "   \\_|   ", "", ""],
        (MoodLevel::Low, 0) => &["  __/    ", "  | /    ", "  |_/    ", "", ""],
        (MoodLevel::Low, _) => &["   \\__   ", "   \\ |   ", "   \\_|   ", "", ""],
    }
}
fn dokkoi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["   __/ ♪ ", "  |  /   ", "  |_/    ", "", ""],
        (Action::Talk, _) => &[" ♫\\__    ", "  \\  |   ", "   \\_|   ", "", ""],
        (Action::Play, 0) => &["  ~__/~  ", "  |  /   ", "  |_/    ", "", ""],
        (Action::Play, _) => &["  ~\\__~  ", "  \\  |   ", "   \\_|   ", "", ""],
        (Action::Train, 0) => &["   __/ ! ", "  |  /   ", "  |_/    ", "", ""],
        (Action::Train, _) => &[" !\\__    ", "  \\  |   ", "   \\_|   ", "", ""],
        (Action::Relax, 0) => &["  __/  z ", "  | /    ", "  |_/    ", "", ""],
        (Action::Relax, _) => &["   \\__ zZ", "   \\ |   ", "   \\_|   ", "", ""],
    }
}

// --- パッパ (pappa) - Tiny compact bird ---
fn pappa_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  >>==>  ", "    ^^   ", "         ", "", ""],
        (MoodLevel::High, _) => &["   >>==> ", "     ^^  ", "         ", "", ""],
        (MoodLevel::Normal, 0) => &["   >==>  ", "    ^^   ", "         ", "", ""],
        (MoodLevel::Normal, _) => &["  >==>   ", "   ^^    ", "         ", "", ""],
        (MoodLevel::Low, 0) => &["   >=>   ", "    ^^   ", "         ", "", ""],
        (MoodLevel::Low, _) => &["   >=>   ", "   ^^    ", "         ", "", ""],
    }
}
fn pappa_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  >==> ♪ ", "   ^^    ", "         ", "", ""],
        (Action::Talk, _) => &["♫ >==>   ", "   ^^    ", "         ", "", ""],
        (Action::Play, 0) => &["  >>==>~ ", "    ^^   ", "         ", "", ""],
        (Action::Play, _) => &[" ~>>==>  ", "    ^^   ", "         ", "", ""],
        (Action::Train, 0) => &[" >>===>! ", "    ^^   ", "         ", "", ""],
        (Action::Train, _) => &["!>>===>  ", "    ^^   ", "         ", "", ""],
        (Action::Relax, 0) => &["   >=> z ", "    ^^   ", "         ", "", ""],
        (Action::Relax, _) => &["   >=> zZ", "   ^^    ", "         ", "", ""],
    }
}

// --- オットリ (ottori) - Gentle/easygoing face ---
fn ottori_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["         ", " (˘▽˘)ﾉ  ", "         ", "", ""],
        (MoodLevel::High, _) => &["         ", "  (˘▽˘)！ ", "         ", "", ""],
        (MoodLevel::Normal, 0) => &["         ", "  (˘ω˘)  ", "         ", "", ""],
        (MoodLevel::Normal, _) => &["         ", "   (˘ω˘) ", "         ", "", ""],
        (MoodLevel::Low, 0) => &["         ", "  (˘_˘)  ", "         ", "", ""],
        (MoodLevel::Low, _) => &["         ", "   (˘_˘) ", "         ", "", ""],
    }
}
fn ottori_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["         ", "  (˘ω˘)♪ ", "         ", "", ""],
        (Action::Talk, _) => &["♫        ", "  (˘ω˘)  ", "         ", "", ""],
        (Action::Play, 0) => &["         ", " (˘▽˘)ﾉ  ", "         ", "", ""],
        (Action::Play, _) => &["         ", "  ﾉ(˘▽˘) ", "         ", "", ""],
        (Action::Train, 0) => &["         ", " (˘▽˘)！  ", "         ", "", ""],
        (Action::Train, _) => &["         ", "！(˘▽˘)   ", "         ", "", ""],
        (Action::Relax, 0) => &["         ", "  (˘_˘) z", "         ", "", ""],
        (Action::Relax, _) => &["       zZ", "  (˘_˘)  ", "         ", "", ""],
    }
}

// ============================================================
// ODAYAKA TYPE Stage 3 Species
// ============================================================

// --- ながれもん (nagaremon) - Jellyfish: dome and tentacles ---
fn nagaremon_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ╭───╮  ", "  ╰┬┬┬╯  ", "   ∫∫∫   ", "", ""],
        (MoodLevel::High, _) => &["  ╭───╮  ", "  ╰┬┬┬╯  ", "  ∫ ∫ ∫  ", "", ""],
        (MoodLevel::Normal, 0) => &["  ╭───╮  ", "  ╰┬┬┬╯  ", "   ∫∫∫   ", "", ""],
        (MoodLevel::Normal, _) => &["  ╭───╮  ", "  ╰┬┬┬╯  ", "  ∫ ∫ ∫  ", "", ""],
        (MoodLevel::Low, 0) => &["   ╭─╮   ", "   ╰┬╯   ", "    ∫    ", "", ""],
        (MoodLevel::Low, _) => &["   ╭─╮   ", "   ╰┬╯   ", "   ∫     ", "", ""],
    }
}
fn nagaremon_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ╭───╮ ♪", "  ╰┬┬┬╯  ", "   ∫∫∫   ", "", ""],
        (Action::Talk, _) => &["♫ ╭───╮  ", "  ╰┬┬┬╯  ", "  ∫ ∫ ∫  ", "", ""],
        (Action::Play, 0) => &[" ~╭───╮~ ", "  ╰┬┬┬╯  ", "  ∫ ∫ ∫  ", "", ""],
        (Action::Play, _) => &["  ╭───╮  ", " ~╰┬┬┬╯~ ", "   ∫∫∫   ", "", ""],
        (Action::Train, 0) => &["  ╭───╮ !", "  ╰┬┬┬╯  ", "  ∫∫∫∫∫  ", "", ""],
        (Action::Train, _) => &["! ╭───╮  ", "  ╰┬┬┬╯  ", "  ∫∫∫∫∫  ", "", ""],
        (Action::Relax, 0) => &["   ╭─╮ z ", "   ╰┬╯   ", "    ∫    ", "", ""],
        (Action::Relax, _) => &["   ╭─╮ zZ", "   ╰┬╯   ", "   ∫     ", "", ""],
    }
}

// --- フワリン (fuwarin) - Balloon with single eye ---
fn fuwarin_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["   .-.   ", "  ( ○ )  ", "    |    ", "", ""],
        (MoodLevel::High, _) => &["    .-. ", "   ( ○ ) ", "     |   ", "", ""],
        (MoodLevel::Normal, 0) => &["   .-.   ", "  ( ○ )  ", "    |    ", "", ""],
        (MoodLevel::Normal, _) => &["  .-.    ", " ( ○ )   ", "   |     ", "", ""],
        (MoodLevel::Low, 0) => &["   .-.   ", "  ( . )  ", "    |    ", "", ""],
        (MoodLevel::Low, _) => &["  .-.    ", " ( . )   ", "   |     ", "", ""],
    }
}
fn fuwarin_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["   .-. ♪ ", "  ( ○ )  ", "    |    ", "", ""],
        (Action::Talk, _) => &[" ♫ .-.   ", "  ( ○ )  ", "    |    ", "", ""],
        (Action::Play, 0) => &["  ~.-.~  ", "  ( ○ )  ", "    |    ", "", ""],
        (Action::Play, _) => &["   .-.   ", " ~( ○ )~ ", "    |    ", "", ""],
        (Action::Train, 0) => &["   .-. ! ", "  ( ○ )  ", "    |    ", "", ""],
        (Action::Train, _) => &[" ! .-.   ", "  ( ○ )  ", "    |    ", "", ""],
        (Action::Relax, 0) => &["   .-. z ", "  ( . )  ", "    |    ", "", ""],
        (Action::Relax, _) => &["   .-.zZ ", "  ( . )  ", "    |    ", "", ""],
    }
}

// --- モコモコ (mokomoko) - Cloud Sheep with wool ---
fn mokomoko_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["(~~~)~~) ", " (~~~~)  ", "  || ||  ", "", ""],
        (MoodLevel::High, _) => &[" (~~(~~~)", "  (~~~~) ", "  || ||  ", "", ""],
        (MoodLevel::Normal, 0) => &[" (~~~)~) ", "  (~~~)  ", "  || ||  ", "", ""],
        (MoodLevel::Normal, _) => &[" (~(~~~) ", "  (~~~)  ", "  || ||  ", "", ""],
        (MoodLevel::Low, 0) => &["  (~~)~  ", "  (~~)   ", "   | |   ", "", ""],
        (MoodLevel::Low, _) => &["  ~(~~)  ", "  (~~)   ", "   | |   ", "", ""],
    }
}
fn mokomoko_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" (~~~)~)♪", "  (~~~)  ", "  || ||  ", "", ""],
        (Action::Talk, _) => &["♫(~~~)~) ", "  (~~~)  ", "  || ||  ", "", ""],
        (Action::Play, 0) => &["(~~~)~~)~", " (~~~~)  ", " ~|| ||~ ", "", ""],
        (Action::Play, _) => &["~(~~(~~~)", "  (~~~~) ", " ~|| ||~ ", "", ""],
        (Action::Train, 0) => &["(~~~)~~)!", " (~~~~)  ", "  || ||  ", "", ""],
        (Action::Train, _) => &["!(~~(~~~)", "  (~~~~) ", "  || ||  ", "", ""],
        (Action::Relax, 0) => &["  (~~)~ z", "  (~~)   ", "   | |   ", "", ""],
        (Action::Relax, _) => &["  ~(~~)zZ", "  (~~)   ", "   | |   ", "", ""],
    }
}

// --- ネンネ (nenne) - Wrapped cocoon, NO features ---
fn nenne_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["         ", "  (-ω-)  ", "   zzZ   ", "", ""],
        (MoodLevel::High, _) => &["         ", "  (-ω-)  ", "  zzZ    ", "", ""],
        (MoodLevel::Normal, 0) => &["         ", "  (-_-)  ", "    zZ   ", "", ""],
        (MoodLevel::Normal, _) => &["         ", "  (-_-)  ", "   zZ    ", "", ""],
        (MoodLevel::Low, 0) => &["         ", "  (-_-)  ", "     z   ", "", ""],
        (MoodLevel::Low, _) => &["         ", "  (-_-)  ", "    z    ", "", ""],
    }
}
fn nenne_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["         ", "  (-ω-)♪ ", "   zZ    ", "", ""],
        (Action::Talk, _) => &["♫        ", "  (-ω-)  ", "   zZ    ", "", ""],
        (Action::Play, 0) => &["         ", "  (-ω-)  ", "  ~zZ~   ", "", ""],
        (Action::Play, _) => &["         ", "  (-ω-)  ", "   ~zZ   ", "", ""],
        (Action::Train, 0) => &["         ", " (-ω-)！  ", "   zZ    ", "", ""],
        (Action::Train, _) => &["         ", "！(-ω-)   ", "   zZ    ", "", ""],
        (Action::Relax, 0) => &["         ", "  (-_-)  ", "   zzZ   ", "", ""],
        (Action::Relax, _) => &["         ", "  (-_-)  ", "  zzZ    ", "", ""],
    }
}

// --- ポヨン (poyon) - Wobbling pudding, no face ---
fn poyon_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ╭──╮   ", " ╱    ╲  ", " ╰══════╯", "", ""],
        (MoodLevel::High, _) => &["   ╭──╮  ", "  ╱    ╲ ", " ╰══════╯", "", ""],
        (MoodLevel::Normal, 0) => &["  ╭──╮   ", " ╱    ╲  ", " ╰════╯  ", "", ""],
        (MoodLevel::Normal, _) => &["   ╭──╮  ", "  ╱    ╲ ", "  ╰════╯ ", "", ""],
        (MoodLevel::Low, 0) => &["   ╭─╮   ", "  ╱   ╲  ", "  ╰══╯   ", "", ""],
        (MoodLevel::Low, _) => &["   ╭─╮   ", "  ╱   ╲  ", "  ╰══╯   ", "", ""],
    }
}
fn poyon_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ╭──╮  ♪", " ╱    ╲  ", " ╰════╯  ", "", ""],
        (Action::Talk, _) => &["♫ ╭──╮   ", " ╱    ╲  ", " ╰════╯  ", "", ""],
        (Action::Play, 0) => &["  ╭──╮   ", "~╱    ╲~ ", " ╰══════╯", "", ""],
        (Action::Play, _) => &["   ╭──╮  ", " ╱    ╲  ", "~╰══════╯~", "", ""],
        (Action::Train, 0) => &["  ╭──╮  !", " ╱    ╲  ", " ╰══════╯", "", ""],
        (Action::Train, _) => &["! ╭──╮   ", " ╱    ╲  ", " ╰══════╯", "", ""],
        (Action::Relax, 0) => &["   ╭─╮ z ", "  ╱   ╲  ", "  ╰══╯   ", "", ""],
        (Action::Relax, _) => &["   ╭─╮zZ ", "  ╱   ╲  ", "  ╰══╯   ", "", ""],
    }
}

// --- スヤスヤ (suyasuya) - Sound asleep face under moon ---
fn suyasuya_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["    ☽    ", "  (-ω-)  ", "  zzZZ   ", "", ""],
        (MoodLevel::High, _) => &["   ☽     ", "  (-ω-)  ", "  zzZ    ", "", ""],
        (MoodLevel::Normal, 0) => &["    ☽    ", "  (-_-)  ", "    zZ   ", "", ""],
        (MoodLevel::Normal, _) => &["   ☽     ", "  (-_-)  ", "   zZ    ", "", ""],
        (MoodLevel::Low, 0) => &["    ☽    ", "  (._.)  ", "    z    ", "", ""],
        (MoodLevel::Low, _) => &["   ☽     ", "  (._.)  ", "   z     ", "", ""],
    }
}
fn suyasuya_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["    ☽   ♪", "  (-_-)  ", "    zZ   ", "", ""],
        (Action::Talk, _) => &["♫   ☽    ", "  (-_-)  ", "    zZ   ", "", ""],
        (Action::Play, 0) => &["  ~☽~    ", "  (-ω-)  ", "  zzZ    ", "", ""],
        (Action::Play, _) => &["   ☽~    ", "  (-ω-)  ", "  zzZZ   ", "", ""],
        (Action::Train, 0) => &["    ☽  ! ", "  (-ω-)  ", "  zzZ    ", "", ""],
        (Action::Train, _) => &[" !  ☽    ", "  (-ω-)  ", "  zzZ    ", "", ""],
        (Action::Relax, 0) => &["    ☽   z", "  (-_-)  ", "   zzZ   ", "", ""],
        (Action::Relax, _) => &["   ☽  zZ ", "  (-_-)  ", "  zzZ    ", "", ""],
    }
}

// --- カスミ (kasumi) - Mist/fog gradient ---
fn kasumi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ░▒▓▒░  ", " ░▒███▒░ ", "  ░▒▓▒░  ", "", ""],
        (MoodLevel::High, _) => &[" ░▒▓▒░   ", "░▒███▒░  ", " ░▒▓▒░   ", "", ""],
        (MoodLevel::Normal, 0) => &["  ░▒▓▒░  ", "  ░██▒░  ", "  ░▒▓▒░  ", "", ""],
        (MoodLevel::Normal, _) => &["  ░▒▓▒░  ", "  ░██▒░  ", "   ░▒░   ", "", ""],
        (MoodLevel::Low, 0) => &["   ░▒░   ", "   ░█░   ", "   ░▒░   ", "", ""],
        (MoodLevel::Low, _) => &["   ░░░   ", "   ░░░   ", "    ░    ", "", ""],
    }
}
fn kasumi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ░▒▓▒░ ♪", "  ░██▒░  ", "  ░▒▓▒░  ", "", ""],
        (Action::Talk, _) => &["♫ ░▒▓▒░  ", "  ░██▒░  ", "  ░▒▓▒░  ", "", ""],
        (Action::Play, 0) => &["  ░▒▓▒░  ", " ░▒███▒░ ", "  ░▒▓▒░  ", "", ""],
        (Action::Play, _) => &[" ░▒▓▒░   ", "░▒███▒░  ", " ░▒▓▒░   ", "", ""],
        (Action::Train, 0) => &["  ░▒▓▒░ !", " ░▒███▒░ ", "  ░▒▓▒░  ", "", ""],
        (Action::Train, _) => &["! ░▒▓▒░  ", " ░▒███▒░ ", "  ░▒▓▒░  ", "", ""],
        (Action::Relax, 0) => &["   ░▒░  z", "   ░█░   ", "   ░▒░   ", "", ""],
        (Action::Relax, _) => &["   ░▒░ zZ", "   ░█░   ", "   ░▒░   ", "", ""],
    }
}

// --- ノドカ (nodoka) - Lotus flower on water ---
fn nodoka_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["   ✿✿   ", "  ❀✿✿❀  ", "～～～～～～～", "", ""],
        (MoodLevel::High, _) => &["  ✿✿    ", " ❀✿✿❀   ", "～～～～～～～", "", ""],
        (MoodLevel::Normal, 0) => &["   ✿    ", "  ❀✿❀   ", "～～～～～～ ", "", ""],
        (MoodLevel::Normal, _) => &["    ✿   ", "   ❀✿❀  ", " ～～～～～～", "", ""],
        (MoodLevel::Low, 0) => &["   .    ", "  .✿.   ", "～～～～～  ", "", ""],
        (MoodLevel::Low, _) => &["        ", "   ✿    ", " ～～～～～ ", "", ""],
    }
}
fn nodoka_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["   ✿   ♪", "  ❀✿❀   ", "～～～～～～ ", "", ""],
        (Action::Talk, _) => &[" ♫ ✿    ", "  ❀✿❀   ", "～～～～～～ ", "", ""],
        (Action::Play, 0) => &["  ~✿✿~  ", "  ❀✿✿❀  ", "～～～～～～～", "", ""],
        (Action::Play, _) => &["   ✿✿   ", " ~❀✿✿❀~ ", "～～～～～～～", "", ""],
        (Action::Train, 0) => &["   ✿✿  !", "  ❀✿✿❀  ", "～～～～～～～", "", ""],
        (Action::Train, _) => &["!  ✿✿   ", "  ❀✿✿❀  ", "～～～～～～～", "", ""],
        (Action::Relax, 0) => &["   .   z", "  .✿.   ", "～～～～～  ", "", ""],
        (Action::Relax, _) => &["      zZ", "   ✿    ", " ～～～～～ ", "", ""],
    }
}

// --- ユメミ (yumemi) - Dreamer with thought bubbles ---
fn yumemi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["     °° ○", "  (~~)   ", "         ", "", ""],
        (MoodLevel::High, _) => &["    °° ○ ", "  (~~)   ", "         ", "", ""],
        (MoodLevel::Normal, 0) => &["      °° ", "  (~~)   ", "         ", "", ""],
        (MoodLevel::Normal, _) => &["     °°  ", "  (~~)   ", "         ", "", ""],
        (MoodLevel::Low, 0) => &["      °  ", "  (~~)   ", "         ", "", ""],
        (MoodLevel::Low, _) => &["     °   ", "  (~~)   ", "         ", "", ""],
    }
}
fn yumemi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["      °°♪", "  (~~)   ", "         ", "", ""],
        (Action::Talk, _) => &["♫    °°  ", "  (~~)   ", "         ", "", ""],
        (Action::Play, 0) => &["    °° ○ ", " ~(~~)~  ", "         ", "", ""],
        (Action::Play, _) => &["   ~°° ○ ", "  (~~)   ", "         ", "", ""],
        (Action::Train, 0) => &["    °°○ !", "  (~~)   ", "         ", "", ""],
        (Action::Train, _) => &["!   °°○  ", "  (~~)   ", "         ", "", ""],
        (Action::Relax, 0) => &["      ° z", "  (~~)   ", "         ", "", ""],
        (Action::Relax, _) => &["     ° zZ", "  (~~)   ", "         ", "", ""],
    }
}

// --- ボンヤリ (bonyari) - Hazy doubled outline ---
fn bonyari_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[".:╭──╮:. ", ":(    ): ", "':╰──╯:' ", "", ""],
        (MoodLevel::High, _) => &[" .:╭──╮:.", " :(    ):", " ':╰──╯:'", "", ""],
        (MoodLevel::Normal, 0) => &[".:╭──╮:. ", ":(    ): ", "':╰──╯:' ", "", ""],
        (MoodLevel::Normal, _) => &[" .:╭──╮:.", " :(    ):", " ':╰──╯:'", "", ""],
        (MoodLevel::Low, 0) => &[" .╭─╮.   ", " (   )   ", " .╰─╯.   ", "", ""],
        (MoodLevel::Low, _) => &["  .╭─╮.  ", "  (   )  ", "  .╰─╯.  ", "", ""],
    }
}
fn bonyari_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[".:╭──╮:.♪", ":(    ): ", "':╰──╯:' ", "", ""],
        (Action::Talk, _) => &["♫.:╭──╮:.", " :(    ):", " ':╰──╯:'", "", ""],
        (Action::Play, 0) => &["~:╭──╮:~.", ":(    ): ", "':╰──╯:' ", "", ""],
        (Action::Play, _) => &[" .:╭──╮:.", "~:(    ):~", " ':╰──╯:'", "", ""],
        (Action::Train, 0) => &[".:╭──╮:.!", ":(    ): ", "':╰──╯:' ", "", ""],
        (Action::Train, _) => &["!.:╭──╮:.", " :(    ):", " ':╰──╯:'", "", ""],
        (Action::Relax, 0) => &[" .╭─╮.  z", " (   )   ", " .╰─╯.   ", "", ""],
        (Action::Relax, _) => &["  .╭─╮. zZ", "  (   )  ", "  .╰─╯.  ", "", ""],
    }
}

// --- ヒラタ (hirata) - Paper-thin, alternates front/side ---
fn hirata_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["═════════", "         ", "         ", "", ""],
        (MoodLevel::High, _) => &["    |    ", "    |    ", "    |    ", "", ""],
        (MoodLevel::Normal, 0) => &[" ═══════ ", "         ", "         ", "", ""],
        (MoodLevel::Normal, _) => &["    |    ", "    |    ", "         ", "", ""],
        (MoodLevel::Low, 0) => &["  ═════  ", "         ", "         ", "", ""],
        (MoodLevel::Low, _) => &["    |    ", "         ", "         ", "", ""],
    }
}
fn hirata_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ═══════♪", "         ", "         ", "", ""],
        (Action::Talk, _) => &["♫   |    ", "    |    ", "         ", "", ""],
        (Action::Play, 0) => &["~═══════~", "         ", "         ", "", ""],
        (Action::Play, _) => &["   ~|~   ", "    |    ", "    |    ", "", ""],
        (Action::Train, 0) => &["═════════!", "         ", "         ", "", ""],
        (Action::Train, _) => &["!   |    ", "    |    ", "    |    ", "", ""],
        (Action::Relax, 0) => &["  ═════ z", "         ", "         ", "", ""],
        (Action::Relax, _) => &["    |  zZ", "         ", "         ", "", ""],
    }
}

// --- コロリン (kororin) - Perfect sphere that rolls ---
fn kororin_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ╭──╮    ", " (    )  ", " ╰──╯    ", "", ""],
        (MoodLevel::High, _) => &["    ╭──╮ ", "   (    )", "    ╰──╯ ", "", ""],
        (MoodLevel::Normal, 0) => &["  ╭──╮   ", "  (    ) ", "  ╰──╯   ", "", ""],
        (MoodLevel::Normal, _) => &["   ╭──╮  ", "  (    ) ", "   ╰──╯  ", "", ""],
        (MoodLevel::Low, 0) => &["  ╭──╮   ", "  (    ) ", "  ╰──╯   ", "", ""],
        (MoodLevel::Low, _) => &["  ╭──╮   ", "  (    ) ", "  ╰──╯   ", "", ""],
    }
}
fn kororin_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ╭──╮  ♪", "  (    ) ", "  ╰──╯   ", "", ""],
        (Action::Talk, _) => &["♫ ╭──╮   ", "  (    ) ", "  ╰──╯   ", "", ""],
        (Action::Play, 0) => &[" ╭──╮    ", "~(    )  ", " ╰──╯    ", "", ""],
        (Action::Play, _) => &["    ╭──╮ ", "   (    )~", "    ╰──╯ ", "", ""],
        (Action::Train, 0) => &[" ╭──╮   !", " (    )  ", " ╰──╯    ", "", ""],
        (Action::Train, _) => &["!   ╭──╮ ", "   (    )", "    ╰──╯ ", "", ""],
        (Action::Relax, 0) => &["  ╭──╮  z", "  (    ) ", "  ╰──╯   ", "", ""],
        (Action::Relax, _) => &["  ╭──╮ zZ", "  (    ) ", "  ╰──╯   ", "", ""],
    }
}

// --- ムニャ (munya) - Faint wisp of fireflies ---
fn munya_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" .oO°   ", "  (-_-)  ", " .oO°    ", "", ""],
        (MoodLevel::High, _) => &["  .oO°  ", "  (-_-)  ", "  .oO°   ", "", ""],
        (MoodLevel::Normal, 0) => &[" .o     ", "  (-_-)  ", "   .o    ", "", ""],
        (MoodLevel::Normal, _) => &["  .o    ", "  (-_-)  ", "    .o   ", "", ""],
        (MoodLevel::Low, 0) => &["    .    ", "  (-_-)  ", "     .   ", "", ""],
        (MoodLevel::Low, _) => &["     .   ", "  (-_-)  ", "    .    ", "", ""],
    }
}
fn munya_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" .o     ♪", "  (-_-)  ", "   .o    ", "", ""],
        (Action::Talk, _) => &["♫.o      ", "  (-_-)  ", "   .o    ", "", ""],
        (Action::Play, 0) => &[" .oO°   ", " ~(-_-)~ ", " .oO°    ", "", ""],
        (Action::Play, _) => &["  .oO°  ", " ~(-_-)~ ", "  .oO°   ", "", ""],
        (Action::Train, 0) => &[" .oO°！  ", "  (-_-)  ", " .oO°    ", "", ""],
        (Action::Train, _) => &["！.oO°   ", "  (-_-)  ", " .oO°    ", "", ""],
        (Action::Relax, 0) => &["    .   z", "  (-_-)  ", "     .   ", "", ""],
        (Action::Relax, _) => &["    . zZ ", "  (-_-)  ", "    .    ", "", ""],
    }
}

// --- マッタリ (mattari) - Melted puddle ---
fn mattari_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["    ^    ", "  ~~~~~~ ", " ~~~~~~~~", "", ""],
        (MoodLevel::High, _) => &["   ^     ", " ~~~~~~  ", "~~~~~~~~~", "", ""],
        (MoodLevel::Normal, 0) => &["         ", "  ~~~~~~ ", " ~~~~~~~~", "", ""],
        (MoodLevel::Normal, _) => &["         ", " ~~~~~~  ", "~~~~~~~~~", "", ""],
        (MoodLevel::Low, 0) => &["         ", "         ", "~~~~~~~~~", "", ""],
        (MoodLevel::Low, _) => &["         ", "         ", " ~~~~~~~~", "", ""],
    }
}
fn mattari_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["        ♪", "  ~~~~~~ ", " ~~~~~~~~", "", ""],
        (Action::Talk, _) => &["♫        ", " ~~~~~~  ", "~~~~~~~~~", "", ""],
        (Action::Play, 0) => &["   ~~    ", " ~~~~~~~ ", "~~~~~~~~~", "", ""],
        (Action::Play, _) => &["    ~~   ", "  ~~~~~~ ", " ~~~~~~~~", "", ""],
        (Action::Train, 0) => &["    ^  ! ", "  ~~~~~~ ", " ~~~~~~~~", "", ""],
        (Action::Train, _) => &[" !  ^    ", " ~~~~~~  ", "~~~~~~~~~", "", ""],
        (Action::Relax, 0) => &["       z ", "         ", "~~~~~~~~~", "", ""],
        (Action::Relax, _) => &["      zZ ", "         ", " ~~~~~~~~", "", ""],
    }
}

// --- ホワワ (howawa) - Dandelion puff with floating seeds ---
fn howawa_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["° °  ° ° ", " °✿✿°   ", "   |     ", "", ""],
        (MoodLevel::High, _) => &[" ° °  ° °", "  °✿✿°   ", "    |    ", "", ""],
        (MoodLevel::Normal, 0) => &["  °  °   ", "  °✿✿°   ", "    |    ", "", ""],
        (MoodLevel::Normal, _) => &["   °  °  ", "   °✿✿°  ", "    |    ", "", ""],
        (MoodLevel::Low, 0) => &["         ", "   °✿°   ", "    |    ", "", ""],
        (MoodLevel::Low, _) => &["    °    ", "   °✿°   ", "    |    ", "", ""],
    }
}
fn howawa_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  °  °  ♪", "  °✿✿°   ", "    |    ", "", ""],
        (Action::Talk, _) => &["♫ °  °   ", "  °✿✿°   ", "    |    ", "", ""],
        (Action::Play, 0) => &["° °  ° ° ", " °✿✿°~   ", "   |     ", "", ""],
        (Action::Play, _) => &[" ° °  ° °", "  ~°✿✿°  ", "    |    ", "", ""],
        (Action::Train, 0) => &["° °  ° °!", " °✿✿°    ", "   |     ", "", ""],
        (Action::Train, _) => &["! ° °  ° ", "  °✿✿°   ", "    |    ", "", ""],
        (Action::Relax, 0) => &["        z", "   °✿°   ", "    |    ", "", ""],
        (Action::Relax, _) => &["       zZ", "   °✿°   ", "    |    ", "", ""],
    }
}

// --- シズカ (shizuka) - Still water with tiny ripple ---
fn shizuka_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["    *    ", "═════════", "    ╨    ", "", ""],
        (MoodLevel::High, _) => &["   *     ", "═════════", "     ╨   ", "", ""],
        (MoodLevel::Normal, 0) => &["         ", " ═══════ ", "    ╨    ", "", ""],
        (MoodLevel::Normal, _) => &["         ", " ═══════ ", "   ╨     ", "", ""],
        (MoodLevel::Low, 0) => &["         ", " ═══════ ", "         ", "", ""],
        (MoodLevel::Low, _) => &["         ", " ═══════ ", "         ", "", ""],
    }
}
fn shizuka_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["        ♪", " ═══════ ", "    ╨    ", "", ""],
        (Action::Talk, _) => &["♫        ", " ═══════ ", "   ╨     ", "", ""],
        (Action::Play, 0) => &["   ~     ", " ═══════ ", "    ╨    ", "", ""],
        (Action::Play, _) => &["     ~   ", " ═══════ ", "   ╨     ", "", ""],
        (Action::Train, 0) => &["    * ! ", "═════════", "    ╨    ", "", ""],
        (Action::Train, _) => &[" !  *    ", "═════════", "     ╨   ", "", ""],
        (Action::Relax, 0) => &["       z ", " ═══════ ", "         ", "", ""],
        (Action::Relax, _) => &["      zZ ", " ═══════ ", "         ", "", ""],
    }
}

// --- モグモグ (mogumogu) - Tea kettle with steam ---
fn mogumogu_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ♨♨♨   ", " ╭━┓╮   ", " ╰━━╯   ", "", ""],
        (MoodLevel::High, _) => &[" ♨ ♨ ♨  ", " ╭━┓╮   ", " ╰━━╯   ", "", ""],
        (MoodLevel::Normal, 0) => &["   ♨♨   ", " ╭━┓╮   ", " ╰━━╯   ", "", ""],
        (MoodLevel::Normal, _) => &["  ♨ ♨   ", " ╭━┓╮   ", " ╰━━╯   ", "", ""],
        (MoodLevel::Low, 0) => &["    ♨   ", " ╭━┓╮   ", " ╰━━╯   ", "", ""],
        (MoodLevel::Low, _) => &["   ♨    ", " ╭━┓╮   ", " ╰━━╯   ", "", ""],
    }
}
fn mogumogu_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["   ♨♨  ♪", " ╭━┓╮   ", " ╰━━╯   ", "", ""],
        (Action::Talk, _) => &[" ♫ ♨♨   ", " ╭━┓╮   ", " ╰━━╯   ", "", ""],
        (Action::Play, 0) => &["  ♨♨♨~  ", " ╭━┓╮   ", " ╰━━╯   ", "", ""],
        (Action::Play, _) => &[" ~♨ ♨ ♨ ", " ╭━┓╮   ", " ╰━━╯   ", "", ""],
        (Action::Train, 0) => &[" ♨♨♨♨ ! ", " ╭━┓╮   ", " ╰━━╯   ", "", ""],
        (Action::Train, _) => &["!♨♨♨♨   ", " ╭━┓╮   ", " ╰━━╯   ", "", ""],
        (Action::Relax, 0) => &["    ♨  z", " ╭━┓╮   ", " ╰━━╯   ", "", ""],
        (Action::Relax, _) => &["   ♨  zZ", " ╭━┓╮   ", " ╰━━╯   ", "", ""],
    }
}

// --- トロン (toron) - Snail with spiral shell ---
fn toron_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["         ", " (`_`)ﾉ  ", "         ", "", ""],
        (MoodLevel::High, _) => &["         ", "  (`_`)！ ", "         ", "", ""],
        (MoodLevel::Normal, 0) => &["         ", "  (`_`)  ", "         ", "", ""],
        (MoodLevel::Normal, _) => &["         ", "   (`_`) ", "         ", "", ""],
        (MoodLevel::Low, 0) => &["         ", "  (. _.) ", "   zzz   ", "", ""],
        (MoodLevel::Low, _) => &["         ", "   (. _.)", "   zzz   ", "", ""],
    }
}
fn toron_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["         ", "  (`_`)♪ ", "         ", "", ""],
        (Action::Talk, _) => &["♫        ", "  (`_`)  ", "         ", "", ""],
        (Action::Play, 0) => &["         ", " (`_`)ﾉ  ", "         ", "", ""],
        (Action::Play, _) => &["         ", "  (`_`)ﾉ ", "         ", "", ""],
        (Action::Train, 0) => &["         ", " (`_`)！  ", "         ", "", ""],
        (Action::Train, _) => &["         ", "！(`_`)   ", "         ", "", ""],
        (Action::Relax, 0) => &["         ", "  (. _.)z", "   zzz   ", "", ""],
        (Action::Relax, _) => &["       zZ", "  (. _.) ", "   zzz   ", "", ""],
    }
}

// --- ユッタリ (yuttari) - Cat-arch, stretching ---
fn yuttari_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["   /\\    ", "  /  \\   ", " =    =  ", "", ""],
        (MoodLevel::High, _) => &["    /\\   ", "   /  \\  ", "  =    = ", "", ""],
        (MoodLevel::Normal, 0) => &["   /\\    ", "  /  \\   ", " =    =  ", "", ""],
        (MoodLevel::Normal, _) => &["   /\\    ", "  /  \\   ", " =    =  ", "", ""],
        (MoodLevel::Low, 0) => &["   __    ", "  /  \\   ", " =    =  ", "", ""],
        (MoodLevel::Low, _) => &["   __    ", "  /  \\   ", " =    =  ", "", ""],
    }
}
fn yuttari_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["   /\\  ♪ ", "  /  \\   ", " =    =  ", "", ""],
        (Action::Talk, _) => &[" ♫ /\\    ", "  /  \\   ", " =    =  ", "", ""],
        (Action::Play, 0) => &["   /\\~   ", "  /  \\   ", "~=    =~ ", "", ""],
        (Action::Play, _) => &["  ~/\\    ", "  /  \\   ", " =    =  ", "", ""],
        (Action::Train, 0) => &["   /\\  ! ", "  /  \\   ", " =    =  ", "", ""],
        (Action::Train, _) => &[" ! /\\    ", "  /  \\   ", " =    =  ", "", ""],
        (Action::Relax, 0) => &["   __  z ", "  /  \\   ", " =    =  ", "", ""],
        (Action::Relax, _) => &["   __ zZ ", "  /  \\   ", " =    =  ", "", ""],
    }
}

// --- ソヨカゼ (soyokaze) - Half-open clam with pearl ---
fn soyokaze_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ╭────╮  ", " ⊂ ◎  ⊃  ", " ╰────╯  ", "", ""],
        (MoodLevel::High, _) => &["  ╭────╮ ", "  ⊂  ◎ ⊃ ", "  ╰────╯ ", "", ""],
        (MoodLevel::Normal, 0) => &["  ╭──╮   ", "  ⊂◎ ⊃  ", "  ╰──╯   ", "", ""],
        (MoodLevel::Normal, _) => &["  ╭──╮   ", "  ⊂ ◎⊃  ", "  ╰──╯   ", "", ""],
        (MoodLevel::Low, 0) => &["  ╭──╮   ", "  ⊂.⊃    ", "  ╰──╯   ", "", ""],
        (MoodLevel::Low, _) => &["  ╭──╮   ", "  ⊂.⊃    ", "  ╰──╯   ", "", ""],
    }
}
fn soyokaze_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ╭──╮  ♪", "  ⊂◎ ⊃  ", "  ╰──╯   ", "", ""],
        (Action::Talk, _) => &["♫ ╭──╮   ", "  ⊂ ◎⊃  ", "  ╰──╯   ", "", ""],
        (Action::Play, 0) => &[" ╭────╮~ ", " ⊂ ◎  ⊃  ", " ╰────╯  ", "", ""],
        (Action::Play, _) => &["~╭────╮  ", " ⊂  ◎ ⊃  ", " ╰────╯  ", "", ""],
        (Action::Train, 0) => &[" ╭────╮ !", " ⊂ ◎  ⊃  ", " ╰────╯  ", "", ""],
        (Action::Train, _) => &["! ╭────╮ ", " ⊂  ◎ ⊃  ", " ╰────╯  ", "", ""],
        (Action::Relax, 0) => &["  ╭──╮ z ", "  ⊂.⊃    ", "  ╰──╯   ", "", ""],
        (Action::Relax, _) => &["  ╭──╮zZ ", "  ⊂.⊃    ", "  ╰──╯   ", "", ""],
    }
}

// ============================================================
// WILD TYPE Stage 3 Species (first 4)
// ============================================================

// --- ヤミノメ (yaminome) - Dark Eye: giant eye in darkness ---
fn yaminome_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ░░░░░  ", "  ░◉░░░  ", "  ░░░░░  ", "", ""],
        (MoodLevel::High, _) => &["  ░░░░░  ", "  ░░░◉░  ", "  ░░░░░  ", "", ""],
        (MoodLevel::Normal, 0) => &["  ░░░░░  ", "  ░◉░░░  ", "  ░░░░░  ", "", ""],
        (MoodLevel::Normal, _) => &["  ░░░░░  ", "  ░░◉░░  ", "  ░░░░░  ", "", ""],
        (MoodLevel::Low, 0) => &["  ░░░░░  ", "  ░·░░░  ", "  ░░░░░  ", "", ""],
        (MoodLevel::Low, _) => &["  ░░░░░  ", "  ░░·░░  ", "  ░░░░░  ", "", ""],
    }
}
fn yaminome_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ░░░░░ ♪", "  ░◉░░░  ", "  ░░░░░  ", "", ""],
        (Action::Talk, _) => &["♫ ░░░░░  ", "  ░░◉░░  ", "  ░░░░░  ", "", ""],
        (Action::Play, 0) => &[" ~░░░░░~ ", "  ░◉░░░  ", "  ░░░░░  ", "", ""],
        (Action::Play, _) => &["  ░░░░░  ", " ~░░░◉░~ ", "  ░░░░░  ", "", ""],
        (Action::Train, 0) => &["  ░░░░░ !", "  ░◉░░░  ", "  ░░░░░  ", "", ""],
        (Action::Train, _) => &["! ░░░░░  ", "  ░░░◉░  ", "  ░░░░░  ", "", ""],
        (Action::Relax, 0) => &["  ░░░░░ z", "  ░·░░░  ", "  ░░░░░  ", "", ""],
        (Action::Relax, _) => &["  ░░░░░zZ", "  ░░·░░  ", "  ░░░░░  ", "", ""],
    }
}

// --- オオヌシ (oonushi) - Great Lord: crowned imposing figure ---
fn oonushi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["    ♔    ", " ╔████╗  ", " ╚┤  ├╝  ", "", ""],
        (MoodLevel::High, _) => &["    ♔    ", "  ╔████╗ ", "  ╚┤  ├╝ ", "", ""],
        (MoodLevel::Normal, 0) => &["    ♔    ", " ╔████╗  ", " ╚┤  ├╝  ", "", ""],
        (MoodLevel::Normal, _) => &["    ♔    ", " ╔████╗  ", " ╚┤  ├╝  ", "", ""],
        (MoodLevel::Low, 0) => &["    ♔    ", " ╔██╗    ", " ╚┤├╝    ", "", ""],
        (MoodLevel::Low, _) => &["    ♔    ", "  ╔██╗   ", "  ╚┤├╝   ", "", ""],
    }
}
fn oonushi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["    ♔   ♪", " ╔████╗  ", " ╚┤  ├╝  ", "", ""],
        (Action::Talk, _) => &[" ♫  ♔    ", " ╔████╗  ", " ╚┤  ├╝  ", "", ""],
        (Action::Play, 0) => &["   ~♔~   ", " ╔████╗  ", " ╚┤  ├╝  ", "", ""],
        (Action::Play, _) => &["    ♔    ", "~╔████╗~ ", " ╚┤  ├╝  ", "", ""],
        (Action::Train, 0) => &["    ♔   !", " ╔████╗  ", " ╚┤  ├╝  ", "", ""],
        (Action::Train, _) => &["!   ♔    ", "  ╔████╗ ", "  ╚┤  ├╝ ", "", ""],
        (Action::Relax, 0) => &["    ♔  z ", " ╔██╗    ", " ╚┤├╝    ", "", ""],
        (Action::Relax, _) => &["    ♔ zZ ", "  ╔██╗   ", "  ╚┤├╝   ", "", ""],
    }
}

// --- バケモノ (bakemono) - Shapeless horror with reaching limbs ---
fn bakemono_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["╱╲╱╲╱╲  ", " ████▓▓ ", " ╲╱╲╱╲╱ ", "", ""],
        (MoodLevel::High, _) => &[" ╲╱╲╱╲╱ ", "  ▓▓████ ", "╱╲╱╲╱╲  ", "", ""],
        (MoodLevel::Normal, 0) => &[" ╱╲╱╲   ", "  ████   ", "  ╲╱╲╱  ", "", ""],
        (MoodLevel::Normal, _) => &["  ╲╱╲╱  ", "   ████  ", " ╱╲╱╲   ", "", ""],
        (MoodLevel::Low, 0) => &["  ╱╲    ", "  ██    ", "  ╲╱    ", "", ""],
        (MoodLevel::Low, _) => &["   ╲╱   ", "   ██   ", "   ╱╲   ", "", ""],
    }
}
fn bakemono_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ╱╲╱╲  ♪", "  ████   ", "  ╲╱╲╱  ", "", ""],
        (Action::Talk, _) => &["♫╲╱╲╱   ", "  ████   ", " ╱╲╱╲   ", "", ""],
        (Action::Play, 0) => &["╱╲╱╲╱╲~ ", " ████▓▓ ", " ╲╱╲╱╲╱ ", "", ""],
        (Action::Play, _) => &["~╲╱╲╱╲╱ ", "  ▓▓████ ", "╱╲╱╲╱╲  ", "", ""],
        (Action::Train, 0) => &["╱╲╱╲╱╲ !", " ████▓▓  ", " ╲╱╲╱╲╱ ", "", ""],
        (Action::Train, _) => &["!╲╱╲╱╲╱ ", "  ▓▓████ ", "╱╲╱╲╱╲  ", "", ""],
        (Action::Relax, 0) => &["  ╱╲   z", "  ██    ", "  ╲╱    ", "", ""],
        (Action::Relax, _) => &["   ╲╱ zZ", "   ██   ", "   ╱╲   ", "", ""],
    }
}

// --- ユウレイ (yuurei) - Ghost: upper body, no legs, wispy ---
fn yuurei_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ╭──╮   ", "  │  │   ", "  ~~ ~ ~ ", "", ""],
        (MoodLevel::High, _) => &["   ╭──╮  ", "   │  │  ", "   ~ ~~ ~", "", ""],
        (MoodLevel::Normal, 0) => &["  ╭──╮   ", "  │  │   ", "  ~~ ~ ~ ", "", ""],
        (MoodLevel::Normal, _) => &["  ╭──╮   ", "  │  │   ", "   ~ ~~ ~", "", ""],
        (MoodLevel::Low, 0) => &["  ╭──╮   ", "  │  │   ", "   ~ ~   ", "", ""],
        (MoodLevel::Low, _) => &["  ╭──╮   ", "  │  │   ", "  ~ ~    ", "", ""],
    }
}
fn yuurei_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ╭──╮  ♪", "  │  │   ", "  ~~ ~ ~ ", "", ""],
        (Action::Talk, _) => &["♫ ╭──╮   ", "  │  │   ", "   ~ ~~ ~", "", ""],
        (Action::Play, 0) => &[" ~╭──╮~  ", "  │  │   ", "  ~~ ~ ~ ", "", ""],
        (Action::Play, _) => &["  ╭──╮   ", " ~│  │~  ", "   ~ ~~ ~", "", ""],
        (Action::Train, 0) => &["  ╭──╮  !", "  │  │   ", "  ~~ ~ ~ ", "", ""],
        (Action::Train, _) => &["! ╭──╮   ", "  │  │   ", "   ~ ~~ ~", "", ""],
        (Action::Relax, 0) => &["  ╭──╮  z", "  │  │   ", "   ~ ~   ", "", ""],
        (Action::Relax, _) => &["  ╭──╮ zZ", "  │  │   ", "  ~ ~    ", "", ""],
    }
}
// --- ヤセイジ (yaseiji) - Feral wolf: bristling fur, fanged jaw, hunched ---
fn yaseiji_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ///▲▲\\\\\\!", " {  ◉  }>>", " \\\\▼▼▼///", "", ""],
        (MoodLevel::High, _) => &[" ///▲▲\\\\\\♪", "  { ◉  }>>", " \\\\▼▼▼///", "", ""],
        (MoodLevel::Normal, 0) => &[" //▲▲\\\\", " { ◉   }", " \\\\▼▼//", "", ""],
        (MoodLevel::Normal, _) => &[" //▲▲\\\\", "  {  ◉ }", "  \\\\▼▼//", "", ""],
        (MoodLevel::Low, 0) => &["  /▲\\", " {   }", "  \\▼/", "", ""],
        (MoodLevel::Low, _) => &["  /▲\\", " {    }", "   \\▼/", "", ""],
    }
}
fn yaseiji_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" //▲▲\\\\", " { ◉  }ﾉ", " \\\\▼▼//", "", ""],
        (Action::Talk, _) => &[" //▲▲\\\\", "ﾉ{  ◉ }", " \\\\▼▼//", "", ""],
        (Action::Play, 0) => &["♪///▲▲\\\\\\", " { ◉ ◉ }~", " \\\\▼▼▼///", "", ""],
        (Action::Play, _) => &[" ///▲▲\\\\\\♪", " ~{ ◉ ◉ }", " \\\\▼▼▼///", "", ""],
        (Action::Train, 0) => &[" ///▲▲\\\\\\!!", " {◉ ▼▼ ◉}", " \\\\▼▼▼▼///", "", ""],
        (Action::Train, _) => &["!!///▲▲\\\\\\", " {◉ ▼▼ ◉}", " \\\\▼▼▼▼///", "", ""],
        (Action::Relax, 0) => &["  /▲▲\\", " {  -  }~", "  \\\\__//", "", ""],
        (Action::Relax, _) => &["  /▲▲\\", " {  -  }zzZ", "  \\\\__//", "", ""],
    }
}

// --- シンエン (shinen) - Deep void: rectangle of darkness, faint shape inside ---
fn shinen_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ████████!", " ██ ・ ███", " ████████", "", ""],
        (MoodLevel::High, _) => &[" ████████♪", " ███ ・ ██", " ████████", "", ""],
        (MoodLevel::Normal, 0) => &[" ████████", " ███. ███", " ████████", "", ""],
        (MoodLevel::Normal, _) => &[" ████████", " ██ .████", " ████████", "", ""],
        (MoodLevel::Low, 0) => &[" ▓▓▓▓▓▓▓▓", " ▓▓▓▓▓▓▓▓", " ▓▓▓▓▓▓▓▓", "", ""],
        (MoodLevel::Low, _) => &[" ▓▓▓▓▓▓▓▓", " ▓▓▓ ▓▓▓▓", " ▓▓▓▓▓▓▓▓", "", ""],
    }
}
fn shinen_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &[" ████████", " ██ ・..██", " ████████", "", ""],
        (Action::Talk, _) => &[" ████████", " ██..・ ██", " ████████", "", ""],
        (Action::Play, 0) => &["♪████████", " ██ ~ ~ ██", " ████████", "", ""],
        (Action::Play, _) => &[" ████████♪", " ██ ~ ~ ██", " ████████", "", ""],
        (Action::Train, 0) => &[" ████████!!", " ██◉  ◉██", " ████████", "", ""],
        (Action::Train, _) => &["!!████████", " ██ ◉◉ ██", " ████████", "", ""],
        (Action::Relax, 0) => &[" ▓▓▓▓▓▓▓▓", " ▓▓▓▓▓▓▓▓~", " ▓▓▓▓▓▓▓▓", "", ""],
        (Action::Relax, _) => &[" ▓▓▓▓▓▓▓▓", " ▓▓▓▓▓▓▓▓zzZ", " ▓▓▓▓▓▓▓▓", "", ""],
    }
}

// --- ノラクロ (norakuro) - Shadow cat: arched back, slit-eye, tail up ---
fn norakuro_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ∧ ∧  ┃!", " (◈)~~┛", " ╰━╯", "", ""],
        (MoodLevel::High, _) => &["  ∧ ∧  ┃♪", "  (◈)~~┛", "  ╰━╯", "", ""],
        (MoodLevel::Normal, 0) => &["  ∧_∧  ┃", " (◈) ~┛", " ╰─╯", "", ""],
        (MoodLevel::Normal, _) => &["  ∧_∧ ┃", "  (◈)~┛", "  ╰─╯", "", ""],
        (MoodLevel::Low, 0) => &["  n_n", " ( )  _/", "  ╰╯", "", ""],
        (MoodLevel::Low, _) => &["  n n", "  ( ) _/", "  ╰╯", "", ""],
    }
}
fn norakuro_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ∧_∧  ┃", " (◈)~┛ﾉ", " ╰─╯", "", ""],
        (Action::Talk, _) => &["  ∧_∧  ┃", "ﾉ(◈) ~┛", "  ╰─╯", "", ""],
        (Action::Play, 0) => &["♪ ∧ ∧ ┃~", " (◈)~~┛", "  ╰━╯", "", ""],
        (Action::Play, _) => &["  ∧ ∧ ┃~♪", " (◈)~~┛", " ╰━╯", "", ""],
        (Action::Train, 0) => &["!!∧ ∧ ┃┃", " (◈)━━┛", " ╰━━╯", "", ""],
        (Action::Train, _) => &["  ∧ ∧┃┃!!", " (◈)━━┛", "  ╰━━╯", "", ""],
        (Action::Relax, 0) => &["  n_n  _/", " ( -)~~", "  ╰─╯ ~", "", ""],
        (Action::Relax, _) => &["  n_n  _/", " ( -)~zzZ", "  ╰─╯", "", ""],
    }
}

// --- モノノケ (mononoke) - Japanese ghost: upper body fades, NO legs, wispy ---
fn mononoke_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ╭ o o╮ !", " ╭┤    ├╮", "  ~~ ~~ ~~", "", ""],
        (MoodLevel::High, _) => &["  ╭ o o╮ ♪", "  ╭┤   ├╮", " ~~ ~~ ~~", "", ""],
        (MoodLevel::Normal, 0) => &["  ╭ .  .╮", "  │     │", "  ~~~ ~~~", "", ""],
        (MoodLevel::Normal, _) => &["  ╭.  . ╮", "  │     │", "   ~~~ ~~", "", ""],
        (MoodLevel::Low, 0) => &["  ╭     ╮", "  :     :", "   ~ ~ ~", "", ""],
        (MoodLevel::Low, _) => &["  ╭     ╮", "  :     :", "  ~ ~ ~", "", ""],
    }
}
fn mononoke_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ╭ .  .╮", "  │  o  │ﾉ", "  ~~~ ~~~", "", ""],
        (Action::Talk, _) => &["  ╭ .  .╮", "ﾉ│  o  │", "  ~~~ ~~~", "", ""],
        (Action::Play, 0) => &["♪╭ o o╮", " ╭┤ ~ ├╮", "  ~~ ~~ ~~", "", ""],
        (Action::Play, _) => &["  ╭ o o╮♪", "  ╭┤~ ├╮", " ~~ ~~ ~~", "", ""],
        (Action::Train, 0) => &["  ╭ ◉ ◉╮!!", " ╭┤ ▼▼ ├╮", " ~~~ ~~~~~", "", ""],
        (Action::Train, _) => &["!!╭ ◉ ◉╮", "  ╭┤▼▼ ├╮", "  ~~~ ~~~~", "", ""],
        (Action::Relax, 0) => &["  ╭ - - ╮~", "  │     │", "  ~~~ ~~~", "", ""],
        (Action::Relax, _) => &["  ╭ -  -╮", "  │     │zzZ", "   ~~ ~~~", "", ""],
    }
}

// --- クライ (kurai) - Shadow blob consuming light, amorphous darkness ---
fn kurai_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[".:░▓██████▓░!", "  ░▓████▓░", " .:░▓██▓░:.", "", ""],
        (MoodLevel::High, _) => &[".:░▓██████▓░♪", " ░▓████▓░", ".:░▓██▓░:.", "", ""],
        (MoodLevel::Normal, 0) => &["  ░▓████▓░", "  ░▓██▓░", "   ░▓▓░", "", ""],
        (MoodLevel::Normal, _) => &["   ░▓████▓░", "  ░▓██▓░", "  ░▓▓░", "", ""],
        (MoodLevel::Low, 0) => &["   ░▓▓░", "   ░▓░", "    ░", "", ""],
        (MoodLevel::Low, _) => &["    ░▓░", "   ░▓░", "   ░", "", ""],
    }
}
fn kurai_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ░▓████▓░", "  ░▓██▓░ ﾉ", "   ░▓▓░", "", ""],
        (Action::Talk, _) => &["  ░▓████▓░", "ﾉ ░▓██▓░", "   ░▓▓░", "", ""],
        (Action::Play, 0) => &["♪░▓████▓░", " ░▓██▓░~", "  ░▓▓░", "", ""],
        (Action::Play, _) => &["  ░▓████▓░♪", "  ~░▓██▓░", "   ░▓▓░", "", ""],
        (Action::Train, 0) => &[".:░▓██████▓░!!", " ░▓██████▓░", ".:░▓████▓░:.", "", ""],
        (Action::Train, _) => &["!!░▓██████▓░:.", " ░▓██████▓░", ".:░▓████▓░:.", "", ""],
        (Action::Relax, 0) => &["   ░▓▓▓░~", "   ░▓▓░", "    ░░", "", ""],
        (Action::Relax, _) => &["   ░▓▓▓░", "   ░▓▓░ zzZ", "    ░░", "", ""],
    }
}

// --- アヤシイ (ayashii) - Shifting shape: different each frame, unstable ---
fn ayashii_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ?{◇□△}?!", " ?<¿¿¿>?", " ?{△□◇}?", "", ""],
        (MoodLevel::High, _) => &[" ?<△◇□>?♪", " ?{!?!}?", " ?<□◇△>?", "", ""],
        (MoodLevel::Normal, 0) => &["  <□?△>", "  {???}", "  <△?□>", "", ""],
        (MoodLevel::Normal, _) => &["  {△?□}", "  <???>", "  {□?△}", "", ""],
        (MoodLevel::Low, 0) => &["   <??>", "   {?}", "   <?>", "", ""],
        (MoodLevel::Low, _) => &["   {??}", "   <?>", "   {?}", "", ""],
    }
}
fn ayashii_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  <□?△>", "  {???}ﾉ", "  <△?□>", "", ""],
        (Action::Talk, _) => &["  {△?□}", " ﾉ<???>", "  {□?△}", "", ""],
        (Action::Play, 0) => &["♪{◇□△◇}", " <¿!?!¿>", " {◇△□◇}", "", ""],
        (Action::Play, _) => &[" <◇□△◇>♪", " {!¿?¿!}", " <◇△□◇>", "", ""],
        (Action::Train, 0) => &[" !{◇□△□◇}!", " !<¿!¿!¿>!", " !{◇△□△◇}!", "", ""],
        (Action::Train, _) => &["!<◇□△□◇>!!", " !{¿!¿!¿}!", " !<◇△□△◇>!", "", ""],
        (Action::Relax, 0) => &["   {~?~}~", "   <?.?>", "   {.?.}", "", ""],
        (Action::Relax, _) => &["   <~?~>", "   {?.?}zzZ", "   <.?.>", "", ""],
    }
}

// --- ムジナ (mujina) - Faceless creature: smooth face, NO features, creepy ---
fn mujina_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ╭━━━╮", "  ┃    ┃!", "  ╰┳━┳╯", "", ""],
        (MoodLevel::High, _) => &["  ╭━━━╮♪", "  ┃    ┃", "  ╰┳━┳╯", "", ""],
        (MoodLevel::Normal, 0) => &["  ╭───╮", "  │    │", "  ╰┬─┬╯", "", ""],
        (MoodLevel::Normal, _) => &["  ╭───╮", "  │    │", "  ╰┬─┬╯", "", ""],
        (MoodLevel::Low, 0) => &["  ╭╌╌╌╮", "  ┊    ┊", "  ╰┬─┬╯", "", ""],
        (MoodLevel::Low, _) => &["  ╭╌╌╌╮", "  ┊    ┊", "   ┬ ┬", "", ""],
    }
}
fn mujina_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ╭───╮", "  │    │ﾉ", "  ╰┬─┬╯", "", ""],
        (Action::Talk, _) => &["  ╭───╮", "ﾉ│    │", "  ╰┬─┬╯", "", ""],
        (Action::Play, 0) => &["♪╭━━━╮", "  ┃    ┃~", "  ╰┳━┳╯", "", ""],
        (Action::Play, _) => &["  ╭━━━╮♪", " ~┃    ┃", "  ╰┳━┳╯", "", ""],
        (Action::Train, 0) => &["  ╭━━━━━╮!!", "  ┃      ┃", "  ╰┳━━━┳╯", "", ""],
        (Action::Train, _) => &["!!╭━━━━━╮", "  ┃      ┃", "  ╰┳━━━┳╯", "", ""],
        (Action::Relax, 0) => &["  ╭╌╌╌╮~", "  ┊    ┊", "  ╰┬─┬╯", "", ""],
        (Action::Relax, _) => &["  ╭╌╌╌╮", "  ┊    ┊zzZ", "   ┬ ┬", "", ""],
    }
}

// --- ヌエ (nue) - Chimera: different animal parts assembled wrong ---
fn nue_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ▼猿▼蛇▼!", " ┣虎╋鬼┫", " ≪≪尾≫≫", "", ""],
        (MoodLevel::High, _) => &[" ▼蛇▼猿▼♪", " ┣鬼╋虎┫", "  ≪≪尾≫≫", "", ""],
        (MoodLevel::Normal, 0) => &["  ▼猿▼蛇", " ┣虎╋鬼┫", "  ≪尾≫", "", ""],
        (MoodLevel::Normal, _) => &["  蛇▼猿▼", " ┣鬼╋虎┫", "   ≪尾≫", "", ""],
        (MoodLevel::Low, 0) => &["   ▼..▼", "  ┣╋┫", "   ≪≫", "", ""],
        (MoodLevel::Low, _) => &["   ▼ ▼", "   ┣╋┫", "   ≪≫", "", ""],
    }
}
fn nue_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ▼猿▼蛇", " ┣虎╋鬼┫ﾉ", "  ≪尾≫", "", ""],
        (Action::Talk, _) => &["  蛇▼猿▼", "ﾉ┣鬼╋虎┫", "   ≪尾≫", "", ""],
        (Action::Play, 0) => &["♪▼猿▼蛇▼", " ┣虎╋鬼┫~", " ≪≪尾≫≫", "", ""],
        (Action::Play, _) => &[" ▼蛇▼猿▼♪", " ~┣鬼╋虎┫", "  ≪≪尾≫≫", "", ""],
        (Action::Train, 0) => &[" ▼猿▼蛇▼!!", " ┣虎╋╋鬼┫", " ≪≪≪尾≫≫≫", "", ""],
        (Action::Train, _) => &["!!▼蛇▼猿▼", " ┣鬼╋╋虎┫", " ≪≪≪尾≫≫≫", "", ""],
        (Action::Relax, 0) => &["  ▼..▼~", "  ┣╋┫", "  ≪尾≫", "", ""],
        (Action::Relax, _) => &["  ▼ ▼", "  ┣╋┫ zzZ", "  ≪尾≫", "", ""],
    }
}

// --- カマイタチ (kamaitachi) - Slash marks in air, creature barely visible ---
fn kamaitachi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ╱  ╲╱ !", " ╲╱  ╲╱", "  ╲  ╱", "", ""],
        (MoodLevel::High, _) => &["  ╱╲  ╱♪", " ╱  ╲╱", " ╲╱  ╲", "", ""],
        (MoodLevel::Normal, 0) => &["  ╱ ╲", "   ╲╱", "  ╱  ╲", "", ""],
        (MoodLevel::Normal, _) => &["   ╱╲", "  ╲  ╱", "   ╲╱", "", ""],
        (MoodLevel::Low, 0) => &["   /", "    \\", "   /", "", ""],
        (MoodLevel::Low, _) => &["    \\", "   /", "", "", ""],
    }
}
fn kamaitachi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ╱ ╲ ﾉ", "   ╲╱", "  ╱  ╲", "", ""],
        (Action::Talk, _) => &["ﾉ  ╱╲", "  ╲  ╱", "   ╲╱", "", ""],
        (Action::Play, 0) => &["♪╱ ╲╱ ╲", " ╲╱  ╲╱", "  ╱ ╲╱", "", ""],
        (Action::Play, _) => &[" ╲╱ ╲╱♪", " ╱  ╱╲", " ╲╱ ╲", "", ""],
        (Action::Train, 0) => &["╱╲╱╲╱╲╱!!", " ╲╱╲╱╲╱", "╱╲╱╲╱╲╱", "", ""],
        (Action::Train, _) => &["!!╲╱╲╱╲╱╲", "  ╱╲╱╲╱╲", " ╲╱╲╱╲╱╲", "", ""],
        (Action::Relax, 0) => &["   / ~", "    \\", "   /", "", ""],
        (Action::Relax, _) => &["    \\", "   / zzZ", "", "", ""],
    }
}

// --- ドロドロ (dorodoro) - Dripping sludge, eye-dots floating in goo ---
fn dorodoro_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ∿●∿●∿●∿!", " ∿∿●∿∿●∿∿", " _●_∿_●__", "", ""],
        (MoodLevel::High, _) => &["  ●∿●∿●∿♪", " ∿∿●∿●∿∿", " __●∿_●__", "", ""],
        (MoodLevel::Normal, 0) => &["  ∿●∿∿●∿", " ∿∿∿●∿∿∿", "  __∿∿__", "", ""],
        (MoodLevel::Normal, _) => &["  ∿∿●∿●∿", " ∿●∿∿∿∿∿", "  __∿∿__", "", ""],
        (MoodLevel::Low, 0) => &["   ∿ ● ∿", "  ∿∿∿∿∿", "   _∿__", "", ""],
        (MoodLevel::Low, _) => &["   ∿●  ∿", "  ∿∿∿∿∿", "    _∿_", "", ""],
    }
}
fn dorodoro_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ∿●∿∿●∿", " ∿∿∿●∿∿∿ﾉ", "  __∿∿__", "", ""],
        (Action::Talk, _) => &["  ∿∿●∿●∿", "ﾉ∿●∿∿∿∿∿", "  __∿∿__", "", ""],
        (Action::Play, 0) => &["♪∿●∿●∿●∿", " ∿∿●∿∿●∿~", "  _●_∿_●_", "", ""],
        (Action::Play, _) => &[" ∿●∿●∿●∿♪", " ~∿∿●∿●∿∿", "  _●_∿_●_", "", ""],
        (Action::Train, 0) => &["∿●∿●∿●∿●!!", " ∿●∿●∿●∿●", " _●_●∿●_●_", "", ""],
        (Action::Train, _) => &["!!●∿●∿●∿●∿", "  ●∿●∿●∿●", " _●_●∿●_●_", "", ""],
        (Action::Relax, 0) => &["   ∿ ● ∿~", "  ∿∿∿∿∿", "   _∿__", "", ""],
        (Action::Relax, _) => &["   ∿●  ∿", "  ∿∿∿∿∿zzZ", "    _∿_", "", ""],
    }
}

// --- ヒノタマ (hinotama) - Floating flame sphere, no solid body ---
fn hinotama_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["  ※*※*※!", " *※(※)※*", "  ~※*※~", "", ""],
        (MoodLevel::High, _) => &[" *※*※*♪", "  ※*(※)*※", " ~*※*~", "", ""],
        (MoodLevel::Normal, 0) => &["   ※*※", "  *(※)*", "   ~*~", "", ""],
        (MoodLevel::Normal, _) => &["   *※*", "  ※(※)※", "   ~※~", "", ""],
        (MoodLevel::Low, 0) => &["    *", "   (※)", "    ~", "", ""],
        (MoodLevel::Low, _) => &["    ※", "   (※)", "    ~", "", ""],
    }
}
fn hinotama_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["   ※*※", "  *(※)* ﾉ", "   ~*~", "", ""],
        (Action::Talk, _) => &["   *※*", "ﾉ ※(※)※", "   ~※~", "", ""],
        (Action::Play, 0) => &["♪ ※*※*※", " *※(※)※*~", "  ~※*※~", "", ""],
        (Action::Play, _) => &["  *※*※*♪", " ~※*(※)*※", "  ~*※*~", "", ""],
        (Action::Train, 0) => &["※*※*※*※!!", " *※*※※*※*", " ※~※*※~※", "", ""],
        (Action::Train, _) => &["!!*※*※*※*", "  ※*※※*※*※", " *~*※*~*", "", ""],
        (Action::Relax, 0) => &["    *~", "   (※)", "    ~", "", ""],
        (Action::Relax, _) => &["    ※", "   (※) zzZ", "    ~", "", ""],
    }
}

// --- フルエ (furue) - Vibrating creature: double/triple outline, shaking ---
fn furue_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ╔╔╔══╗╗╗!", "  ║║    ║║", " ╚╚╚══╝╝╝", "", ""],
        (MoodLevel::High, _) => &["╔╔╔══╗╗╗♪", " ║║    ║║", "╚╚╚══╝╝╝", "", ""],
        (MoodLevel::Normal, 0) => &["  ╔╔══╗╗", "  ║║  ║║", "  ╚╚══╝╝", "", ""],
        (MoodLevel::Normal, _) => &[" ╔╔══╗╗", "  ║║  ║║", "  ╚╚══╝╝", "", ""],
        (MoodLevel::Low, 0) => &["   ╔══╗", "   ║  ║", "   ╚══╝", "", ""],
        (MoodLevel::Low, _) => &["   ╔══╗", "   ║  ║", "   ╚══╝", "", ""],
    }
}
fn furue_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ╔╔══╗╗", "  ║║  ║║ﾉ", "  ╚╚══╝╝", "", ""],
        (Action::Talk, _) => &[" ╔╔══╗╗", " ﾉ║║  ║║", "  ╚╚══╝╝", "", ""],
        (Action::Play, 0) => &["♪╔╔╔══╗╗╗", "  ║║    ║║~", " ╚╚╚══╝╝╝", "", ""],
        (Action::Play, _) => &[" ╔╔╔══╗╗╗♪", " ~║║    ║║", "  ╚╚╚══╝╝╝", "", ""],
        (Action::Train, 0) => &["╔╔╔╔══╗╗╗╗!!", " ║║║    ║║║", "╚╚╚╚══╝╝╝╝", "", ""],
        (Action::Train, _) => &["!!╔╔╔╔══╗╗╗╗", "  ║║║    ║║║", " ╚╚╚╚══╝╝╝╝", "", ""],
        (Action::Relax, 0) => &["   ╔══╗~", "   ║  ║", "   ╚══╝", "", ""],
        (Action::Relax, _) => &["   ╔══╗", "   ║  ║ zzZ", "   ╚══╝", "", ""],
    }
}

// --- ケダマ (kedama) - Ball of fur, messy hair everywhere, tiny dots barely visible ---
fn kedama_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" ≋≋≋≋≋≋≋!", " ≋≋ ..\u{a0}≋≋≋", " ≋≋≋≋≋≋≋", "", ""],
        (MoodLevel::High, _) => &["  ≋≋≋≋≋≋≋♪", " ≋≋≋.. ≋≋", "  ≋≋≋≋≋≋≋", "", ""],
        (MoodLevel::Normal, 0) => &["  ≋≋≋≋≋≋", " ≋≋ .. ≋≋", "  ≋≋≋≋≋≋", "", ""],
        (MoodLevel::Normal, _) => &["  ≋≋≋≋≋≋", "  ≋≋.. ≋≋", "  ≋≋≋≋≋≋", "", ""],
        (MoodLevel::Low, 0) => &["   ≋≋≋≋", "  ≋≋  ≋≋", "   ≋≋≋≋", "", ""],
        (MoodLevel::Low, _) => &["   ≋≋≋≋", "   ≋≋ ≋≋", "   ≋≋≋≋", "", ""],
    }
}
fn kedama_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  ≋≋≋≋≋≋", " ≋≋ .. ≋≋ﾉ", "  ≋≋≋≋≋≋", "", ""],
        (Action::Talk, _) => &["  ≋≋≋≋≋≋", "ﾉ≋≋..≋≋", "  ≋≋≋≋≋≋", "", ""],
        (Action::Play, 0) => &["♪≋≋≋≋≋≋≋", " ≋≋ .. ≋≋~", " ≋≋≋≋≋≋≋", "", ""],
        (Action::Play, _) => &[" ≋≋≋≋≋≋≋♪", " ~≋≋.. ≋≋", " ≋≋≋≋≋≋≋", "", ""],
        (Action::Train, 0) => &["≋≋≋≋≋≋≋≋≋!!", " ≋≋≋..≋≋≋≋", "≋≋≋≋≋≋≋≋≋", "", ""],
        (Action::Train, _) => &["!!≋≋≋≋≋≋≋≋≋", "  ≋≋≋..≋≋≋≋", " ≋≋≋≋≋≋≋≋≋", "", ""],
        (Action::Relax, 0) => &["   ≋≋≋≋~", "  ≋≋  ≋≋", "   ≋≋≋≋", "", ""],
        (Action::Relax, _) => &["   ≋≋≋≋", "  ≋≋  ≋≋zzZ", "   ≋≋≋≋", "", ""],
    }
}

// --- シノビ (shinobi) - Shadow with blade, mostly invisible, weapon glints ---
fn shinobi_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &["        ━━★!", "    ░ ░", "   ░░░░", "", ""],
        (MoodLevel::High, _) => &["  ★━━    ♪", "     ░ ░", "    ░░░░", "", ""],
        (MoodLevel::Normal, 0) => &["       ━★", "    ░ ░", "    ░░░", "", ""],
        (MoodLevel::Normal, _) => &["   ★━", "     ░ ░", "    ░░░", "", ""],
        (MoodLevel::Low, 0) => &["       ─", "     ░", "    ░░", "", ""],
        (MoodLevel::Low, _) => &["   ─", "    ░", "    ░░", "", ""],
    }
}
fn shinobi_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["       ━★", "    ░ ░ ﾉ", "    ░░░", "", ""],
        (Action::Talk, _) => &["   ★━", "  ﾉ  ░ ░", "    ░░░", "", ""],
        (Action::Play, 0) => &["♪     ━━★~", "    ░ ░", "   ░░░░", "", ""],
        (Action::Play, _) => &["  ~★━━   ♪", "    ░ ░", "   ░░░░", "", ""],
        (Action::Train, 0) => &["      ━━━★!!", "    ░ ░", "   ░░░░░", "", ""],
        (Action::Train, _) => &["!!★━━━", "      ░ ░", "    ░░░░░", "", ""],
        (Action::Relax, 0) => &["       ─~", "     ░", "    ░░", "", ""],
        (Action::Relax, _) => &["   ─", "    ░  zzZ", "    ░░", "", ""],
    }
}

// --- ジゴク (jigoku) - Oni/demon mask: horns on top, fanged wide mouth ---
fn jigoku_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" Y╔══════╗Y!", " ║◉▼▼▼▼◉║", " ╚══════╝", "", ""],
        (MoodLevel::High, _) => &["Y╔══════╗Y♪", "  ║◉▼▼▼▼◉║", "  ╚══════╝", "", ""],
        (MoodLevel::Normal, 0) => &["  Y╔════╗Y", "  ║◉▼▼◉║", "  ╚════╝", "", ""],
        (MoodLevel::Normal, _) => &[" Y╔════╗Y", "  ║◉▼▼◉║", "  ╚════╝", "", ""],
        (MoodLevel::Low, 0) => &["  y╔══╗y", "  ║ __ ║", "  ╚══╝", "", ""],
        (MoodLevel::Low, _) => &["  y╔══╗y", "  ║ __ ║", "   ╚══╝", "", ""],
    }
}
fn jigoku_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  Y╔════╗Y", "  ║◉▼▼◉║ﾉ", "  ╚════╝", "", ""],
        (Action::Talk, _) => &["  Y╔════╗Y", "ﾉ║◉▼▼◉║", "  ╚════╝", "", ""],
        (Action::Play, 0) => &["♪Y╔══════╗Y", " ║◉▼▼▼▼◉║~", " ╚══════╝", "", ""],
        (Action::Play, _) => &[" Y╔══════╗Y♪", " ~║◉▼▼▼▼◉║", "  ╚══════╝", "", ""],
        (Action::Train, 0) => &["Y╔════════╗Y!!", " ║◉▼▼▼▼▼▼◉║", " ╚════════╝", "", ""],
        (Action::Train, _) => &["!!Y╔════════╗Y", "  ║◉▼▼▼▼▼▼◉║", "  ╚════════╝", "", ""],
        (Action::Relax, 0) => &["  y╔══╗y~", "  ║ __ ║", "  ╚══╝", "", ""],
        (Action::Relax, _) => &["  y╔══╗y", "  ║ __ ║zzZ", "   ╚══╝", "", ""],
    }
}

// --- ムゲン (mugen) - Ouroboros: snake eating tail, eternal loop ---
fn mugen_art(mood: MoodLevel, frame: usize) -> &'static [&'static str] {
    match (mood, frame % 2) {
        (MoodLevel::High, 0) => &[" >=>=>=>=>!", " <=<  >=>", " >=>=>=<=<", "", ""],
        (MoodLevel::High, _) => &["  <=<=<=<=<♪", "  >=>  <=<", "  <=<=>=>=", "", ""],
        (MoodLevel::Normal, 0) => &["  >=>=>=>", "  <=   =>", "  >=>=><=", "", ""],
        (MoodLevel::Normal, _) => &["  <=<=<=<", "  =>   <=", "  <=>=>=>", "", ""],
        (MoodLevel::Low, 0) => &["   >=>=>", "   =   =", "   <=<=<", "", ""],
        (MoodLevel::Low, _) => &["   <=<=<", "   =   =", "   >=>=>", "", ""],
    }
}
fn mugen_action(action: Action, frame: usize) -> &'static [&'static str] {
    match (action, frame % 2) {
        (Action::Talk, 0) => &["  >=>=>=> ﾉ", "  <=   =>", "  >=>=><=", "", ""],
        (Action::Talk, _) => &["ﾉ <=<=<=<", "  =>   <=", "  <=>=>=>", "", ""],
        (Action::Play, 0) => &["♪>=>=>=>=>", " <=<  >=>~", " >=>=>=<=<", "", ""],
        (Action::Play, _) => &[" <=<=<=<=<♪", " ~>=>  <=<", " <=<=>=>=", "", ""],
        (Action::Train, 0) => &[">=>=>=>=>=>!!", " <=<=  >=>=>", " >=>=>=><=<=", "", ""],
        (Action::Train, _) => &["!!<=<=<=<=<=<", "  >=>=  <=<=", "  <=<=<=>=>=", "", ""],
        (Action::Relax, 0) => &["   >=>=>~", "   =   =", "   <=<=<", "", ""],
        (Action::Relax, _) => &["   <=<=<", "   =   = zzZ", "   >=>=>", "", ""],
    }
}
