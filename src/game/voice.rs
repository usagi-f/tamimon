use rand::seq::SliceRandom;
use rand::Rng;

use crate::game::actions::Action;
use crate::game::evolution::VoiceType;
use crate::game::pet::MoodLevel;

/// Get reaction text by voice type
pub fn get_reaction(
    voice_type: VoiceType,
    action: Action,
    mood: MoodLevel,
    rng: &mut impl Rng,
) -> String {
    let pool = get_reaction_pool(voice_type, action, mood);
    pool.choose(rng).unwrap().to_string()
}

/// Get idle speech by voice type
pub fn get_idle_speech(voice_type: VoiceType, mood: MoodLevel, rng: &mut impl Rng) -> String {
    let pool = get_idle_pool(voice_type, mood);
    pool.choose(rng).unwrap().to_string()
}

fn get_reaction_pool(
    voice_type: VoiceType,
    action: Action,
    mood: MoodLevel,
) -> &'static [&'static str] {
    match (voice_type, action, mood) {
        // ===== Tameguchi (casual) =====
        (VoiceType::Tameguchi, Action::Talk, MoodLevel::High) => &[
            "「なに、ひまなの」",
            "「あー、いたの」",
            "「ふーん、で？」",
            "「話しかけてきた」",
            "「まあいっか」",
        ],
        (VoiceType::Tameguchi, Action::Talk, MoodLevel::Normal) => &[
            "「なに」",
            "「うるさい」",
            "「ふーん」",
            "「…あっそ」",
            "「べつに」",
        ],
        (VoiceType::Tameguchi, Action::Talk, MoodLevel::Low) => &[
            "「…」",
            "「うざ」",
            "「しつこい」",
            "「ほっとけ」",
            "「はぁ」",
        ],
        (VoiceType::Tameguchi, Action::Play, MoodLevel::High) => &[
            "「まあ、つきあってやるよ」",
            "「しょうがないな」",
            "「やるか」",
            "「…ちょっとだけな」",
        ],
        (VoiceType::Tameguchi, Action::Play, MoodLevel::Normal) => &[
            "「はいはい」",
            "「めんどくさ」",
            "「…やるけど」",
            "「てきとーにやるわ」",
        ],
        (VoiceType::Tameguchi, Action::Play, MoodLevel::Low) => {
            &["「むり」", "「やだ」", "「かってにやれば」", "「…」"]
        }
        (VoiceType::Tameguchi, Action::Train, MoodLevel::High) => &[
            "「やってやるよ」",
            "「余裕だし」",
            "「こんなもんか」",
            "「次は？」",
        ],
        (VoiceType::Tameguchi, Action::Train, MoodLevel::Normal) => &[
            "「だるい」",
            "「…やるけど」",
            "「はぁ…まあいいけど」",
            "「てきとー」",
        ],
        (VoiceType::Tameguchi, Action::Train, MoodLevel::Low) => {
            &["「むり」", "「やめろ」", "「…」", "「しぬ」"]
        }
        (VoiceType::Tameguchi, Action::Relax, MoodLevel::High) => &[
            "「まあ、たまにはいいか」",
            "「ぼーっとする」",
            "「ふぁ〜」",
            "「…ん」",
        ],
        (VoiceType::Tameguchi, Action::Relax, MoodLevel::Normal) => {
            &["「…zzz」", "「ねる」", "「おやすみ」", "「…」"]
        }
        (VoiceType::Tameguchi, Action::Relax, MoodLevel::Low) => {
            &["「もう寝る」", "「ほっとけ」", "「…zzz」", "「うるさい」"]
        }

        // ===== Keigo (polite) =====
        (VoiceType::Keigo, Action::Talk, MoodLevel::High) => &[
            "「お声がけいただき光栄です」",
            "「失礼ですが、うれしいです」",
            "「ありがとうございます」",
            "「恐縮です」",
        ],
        (VoiceType::Keigo, Action::Talk, MoodLevel::Normal) => &[
            "「失礼ですが、今少しお眠いです」",
            "「ありがとうございます。特に何もありません」",
            "「…はい」",
            "「恐縮ですが、少しだけほっといてください」",
        ],
        (VoiceType::Keigo, Action::Talk, MoodLevel::Low) => &[
            "「申し訳ございません、今はちょっと…」",
            "「…失礼します」",
            "「恐れ入りますが…」",
            "「…はい」",
        ],
        (VoiceType::Keigo, Action::Play, MoodLevel::High) => &[
            "「楽しゅうございました」",
            "「お付き合いいただき感謝です」",
            "「またよろしくお願いします」",
            "「光栄です」",
        ],
        (VoiceType::Keigo, Action::Play, MoodLevel::Normal) => &[
            "「ありがとうございました」",
            "「そうですね、まあまあでした」",
            "「お疲れ様です」",
            "「はい、以上です」",
        ],
        (VoiceType::Keigo, Action::Play, MoodLevel::Low) => &[
            "「申し訳ございません…」",
            "「ちょっと厳しいです」",
            "「…失礼」",
            "「恐縮ですが…」",
        ],
        (VoiceType::Keigo, Action::Train, MoodLevel::High) => &[
            "「精進いたします」",
            "「はい、もう一度お願いします」",
            "「ありがとうございます」",
            "「鍛えていただき感謝です」",
        ],
        (VoiceType::Keigo, Action::Train, MoodLevel::Normal) => &[
            "「承知しました」",
            "「はい…」",
            "「もう少しだけ…」",
            "「お疲れ様でした」",
        ],
        (VoiceType::Keigo, Action::Train, MoodLevel::Low) => &[
            "「限界でございます…」",
            "「申し訳ございません…」",
            "「ご勘弁を…」",
            "「…」",
        ],
        (VoiceType::Keigo, Action::Relax, MoodLevel::High) => &[
            "「素敵なお時間ですね」",
            "「心が洗われます」",
            "「ゆっくりさせていただきます」",
            "「ありがとうございます」",
        ],
        (VoiceType::Keigo, Action::Relax, MoodLevel::Normal) => &[
            "「…失礼、少しウトウトしました」",
            "「穏やかですね」",
            "「ゆっくりしております」",
            "「…zzz」",
        ],
        (VoiceType::Keigo, Action::Relax, MoodLevel::Low) => &[
            "「…休ませていただきます」",
            "「…zzz」",
            "「…失礼します」",
            "「…」",
        ],

        // ===== Gal =====
        (VoiceType::Gal, Action::Talk, MoodLevel::High) => &[
            "「えーまって！うれしー！」",
            "「きゃー！話そ話そ！」",
            "「やばくない！？」",
            "「テンション上がる〜！」",
        ],
        (VoiceType::Gal, Action::Talk, MoodLevel::Normal) => &[
            "「あーね」",
            "「それな」",
            "「ふーん、で？」",
            "「まあいっか」",
        ],
        (VoiceType::Gal, Action::Talk, MoodLevel::Low) => {
            &["「無理…」", "「だる…」", "「え、今？」", "「…」"]
        }
        (VoiceType::Gal, Action::Play, MoodLevel::High) => &[
            "「えーやばい！たのしー！！」",
            "「もっかいやろ！絶対！」",
            "「え待って今の何？ウケるんだけど」",
            "「あたしそういうの好き」",
        ],
        (VoiceType::Gal, Action::Play, MoodLevel::Normal) => &[
            "「まあまあかな」",
            "「ふつー」",
            "「つかれたんだけどぉ〜」",
            "「もうちょいがんばる」",
        ],
        (VoiceType::Gal, Action::Play, MoodLevel::Low) => {
            &["「むりぃ〜」", "「やだ」", "「帰りたい」", "「…」"]
        }
        (VoiceType::Gal, Action::Train, MoodLevel::High) => &[
            "「いけるいける！」",
            "「あたし強くない！？」",
            "「やばっ！できた！」",
            "「もっとやる！」",
        ],
        (VoiceType::Gal, Action::Train, MoodLevel::Normal) => &[
            "「えー筋肉痛なるやん」",
            "「まあやるけど」",
            "「しんど」",
            "「つら」",
        ],
        (VoiceType::Gal, Action::Train, MoodLevel::Low) => {
            &["「絶対むり」", "「無理無理無理」", "「…」", "「帰る」"]
        }
        (VoiceType::Gal, Action::Relax, MoodLevel::High) => &[
            "「きもちいい〜！」",
            "「最高じゃん！」",
            "「あーしあわせ」",
            "「エモい…」",
        ],
        (VoiceType::Gal, Action::Relax, MoodLevel::Normal) => &[
            "「まったり〜」",
            "「zzz…はっ！寝てた」",
            "「ふぁ〜」",
            "「まあいっか」",
        ],
        (VoiceType::Gal, Action::Relax, MoodLevel::Low) => {
            &["「もう寝る…」", "「…zzz」", "「おやすみ…」", "「…」"]
        }

        // ===== Oyaji (old man) =====
        (VoiceType::Oyaji, Action::Talk, MoodLevel::High) => &[
            "「おう、よく来たな」",
            "「むかしはなあ…」",
            "「いい天気だなあ」",
            "「まあ座れ座れ」",
        ],
        (VoiceType::Oyaji, Action::Talk, MoodLevel::Normal) => &[
            "「ん？なんだ？」",
            "「あーはいはい」",
            "「そういうこともある」",
            "「まあな」",
        ],
        (VoiceType::Oyaji, Action::Talk, MoodLevel::Low) => {
            &["「…今はちょっと」", "「疲れたわ」", "「腰が痛い」", "「…」"]
        }
        (VoiceType::Oyaji, Action::Play, MoodLevel::High) => &[
            "「わしもまだまだ現役じゃ」",
            "「いい汗かいたわ」",
            "「昔はもっとできたんだがな」",
            "「なかなかやるな」",
        ],
        (VoiceType::Oyaji, Action::Play, MoodLevel::Normal) => &[
            "「まあ、こんなもんだろ」",
            "「ふぅ…」",
            "「歳は取りたくないもんだ」",
            "「…うむ」",
        ],
        (VoiceType::Oyaji, Action::Play, MoodLevel::Low) => &[
            "「無理させんな」",
            "「腰がな…」",
            "「…」",
            "「もう若くないんだ」",
        ],
        (VoiceType::Oyaji, Action::Train, MoodLevel::High) => &[
            "「むかしはなあ、こういう鍛練を毎日やったもんだ」",
            "「若いうちの苦労は買ってでもしろ」",
            "「いい汗かいたわ」",
            "「これがほんとの根性ってもんだ」",
        ],
        (VoiceType::Oyaji, Action::Train, MoodLevel::Normal) => &[
            "「まあ、こんなもんだろ」",
            "「ふぅ…」",
            "「体が資本だからな」",
            "「…うむ」",
        ],
        (VoiceType::Oyaji, Action::Train, MoodLevel::Low) => {
            &["「もう勘弁してくれ」", "「腰が…」", "「…」", "「休ませろ」"]
        }
        (VoiceType::Oyaji, Action::Relax, MoodLevel::High) => &[
            "「あー、極楽じゃ」",
            "「こういうのがいいんだよ」",
            "「風呂上がりのビールみたいだ」",
            "「人生にはこういう時間が必要だ」",
        ],
        (VoiceType::Oyaji, Action::Relax, MoodLevel::Normal) => &[
            "「…zzz」",
            "「ぐう」",
            "「のんびりだなあ」",
            "「…ん？寝てた？」",
        ],
        (VoiceType::Oyaji, Action::Relax, MoodLevel::Low) => {
            &["「もう寝る」", "「おやすみ」", "「…zzz」", "「疲れた」"]
        }

        // ===== Tetsugaku (philosopher) =====
        (VoiceType::Tetsugaku, Action::Talk, MoodLevel::High) => &[
            "「言葉とは、沈黙の間に咲く花だ」",
            "「存在を確認してくれてありがとう」",
            "「会話は、二つの孤独が交差する瞬間だ」",
            "「今日は言葉が軽い。いい日だ」",
        ],
        (VoiceType::Tetsugaku, Action::Talk, MoodLevel::Normal) => &[
            "「存在とは何か」",
            "「…考えていた」",
            "「言葉は不完全だ」",
            "「沈黙にも意味がある」",
        ],
        (VoiceType::Tetsugaku, Action::Talk, MoodLevel::Low) => &[
            "「…」",
            "「虚無を見つめている」",
            "「なぜ話すのだ」",
            "「意味はあるのか」",
        ],
        (VoiceType::Tetsugaku, Action::Play, MoodLevel::High) => &[
            "「遊びとは…自由の実践だ！」",
            "「楽しいとは不思議な概念だ」",
            "「動くことは考えることだ」",
            "「…なるほど。これが遊びか」",
        ],
        (VoiceType::Tetsugaku, Action::Play, MoodLevel::Normal) => &[
            "「遊びとは何か」",
            "「…ふむ」",
            "「目的のない行動にこそ意味がある」",
            "「…」",
        ],
        (VoiceType::Tetsugaku, Action::Play, MoodLevel::Low) => &[
            "「遊ぶ意味を見出せない」",
            "「…」",
            "「虚しい」",
            "「なぜだ」",
        ],
        (VoiceType::Tetsugaku, Action::Train, MoodLevel::High) => &[
            "「肉体は精神の器だ。鍛えよう」",
            "「苦痛は成長の証だ！」",
            "「限界とは…幻想かもしれない」",
            "「汗は哲学の結晶だ」",
        ],
        (VoiceType::Tetsugaku, Action::Train, MoodLevel::Normal) => &[
            "「鍛えるとは何か」",
            "「…ふむ、肉体か」",
            "「努力は報われるのか」",
            "「…」",
        ],
        (VoiceType::Tetsugaku, Action::Train, MoodLevel::Low) => &[
            "「…もう考えられない」",
            "「体が思考を拒否している」",
            "「…」",
            "「限界とは…これか」",
        ],
        (VoiceType::Tetsugaku, Action::Relax, MoodLevel::High) => &[
            "「静寂とは、音の不在ではなく、内なる声との対話だ」",
            "「石になりたい気持ち、わかる？」",
            "「存在することに意味はあるのか…あ、おなかすいた」",
            "「風が、なにかを語っている気がする」",
        ],
        (VoiceType::Tetsugaku, Action::Relax, MoodLevel::Normal) => &[
            "「時間は幻だ」",
            "「…考えている」",
            "「無とは…zzz」",
            "「…」",
        ],
        (VoiceType::Tetsugaku, Action::Relax, MoodLevel::Low) => &[
            "「…」",
            "「存在が重い」",
            "「…zzz」",
            "「なにも考えたくない」",
        ],

        // ===== Taiiku (athletic) =====
        (VoiceType::Taiiku, Action::Talk, MoodLevel::High) => &[
            "「はいっ！元気ッス！」",
            "「今日もいい日ッス！」",
            "「声出していこう！」",
            "「ありがとうございます！」",
        ],
        (VoiceType::Taiiku, Action::Talk, MoodLevel::Normal) => &[
            "「はい！」",
            "「…うッス」",
            "「がんばります」",
            "「了解ッス」",
        ],
        (VoiceType::Taiiku, Action::Talk, MoodLevel::Low) => {
            &["「…はい」", "「…ッス」", "「すいません…」", "「…」"]
        }
        (VoiceType::Taiiku, Action::Play, MoodLevel::High) => &[
            "「よっしゃー！いくぞー！」",
            "「もう一回！もう一回！」",
            "「全力で楽しむッス！」",
            "「最高ッス！」",
        ],
        (VoiceType::Taiiku, Action::Play, MoodLevel::Normal) => &[
            "「やるッス！」",
            "「…ふう」",
            "「まだいけるッス」",
            "「がんばるッス」",
        ],
        (VoiceType::Taiiku, Action::Play, MoodLevel::Low) => &[
            "「…すいません、今日は…」",
            "「気合いが…」",
            "「…」",
            "「休憩いいッスか」",
        ],
        (VoiceType::Taiiku, Action::Train, MoodLevel::High) => &[
            "「まだまだいける！」",
            "「もう一本！」",
            "「つよくなった気がする！」",
            "「いい汗かいた！」",
        ],
        (VoiceType::Taiiku, Action::Train, MoodLevel::Normal) => &[
            "「はい…！」",
            "「がんばります…！」",
            "「…ッス」",
            "「あと少し…」",
        ],
        (VoiceType::Taiiku, Action::Train, MoodLevel::Low) => {
            &["「…限界ッス」", "「すいません…」", "「…」", "「もう…」"]
        }
        (VoiceType::Taiiku, Action::Relax, MoodLevel::High) => &[
            "「休憩も大事ッス！」",
            "「あー気持ちいい！」",
            "「回復！回復！」",
            "「明日に備えるッス！」",
        ],
        (VoiceType::Taiiku, Action::Relax, MoodLevel::Normal) => {
            &["「…zzz」", "「休むのも修行ッス」", "「…ふぅ」", "「…」"]
        }
        (VoiceType::Taiiku, Action::Relax, MoodLevel::Low) => {
            &["「…zzz」", "「…」", "「おやすみなさい…」", "「…ッス」"]
        }

        // ===== Negative =====
        (VoiceType::Negative, Action::Talk, MoodLevel::High) => &[
            "「…え、話しかけてくれるんだ…うれしい…かも」",
            "「どうせすぐいなくなるんでしょ…でもありがとう」",
            "「…今日はちょっとだけ…いい日かも」",
            "「…ほんと？」",
        ],
        (VoiceType::Negative, Action::Talk, MoodLevel::Normal) => &[
            "「どうせ…」",
            "「…ごめんなさい」",
            "「わたしなんかに話しかけても…」",
            "「…うん」",
        ],
        (VoiceType::Negative, Action::Talk, MoodLevel::Low) => &[
            "「…」",
            "「やっぱり無理でした」",
            "「消えたい…」",
            "「…ごめん」",
        ],
        (VoiceType::Negative, Action::Play, MoodLevel::High) => &[
            "「…え、たのしい…こんな気持ち久しぶり…」",
            "「…ありがとう」",
            "「…もうちょっとだけ…」",
            "「…笑っていいのかな」",
        ],
        (VoiceType::Negative, Action::Play, MoodLevel::Normal) => &[
            "「どうせ楽しくならない…」",
            "「…やるけど」",
            "「…はい」",
            "「…」",
        ],
        (VoiceType::Negative, Action::Play, MoodLevel::Low) => &[
            "「…無理です」",
            "「…」",
            "「ごめんなさい…」",
            "「楽しめない…」",
        ],
        (VoiceType::Negative, Action::Train, MoodLevel::High) => &[
            "「…できた！…え、ほんとに？」",
            "「わたしにもできるの…？」",
            "「…ちょっとだけ自信ついた…かも」",
            "「…うれしい」",
        ],
        (VoiceType::Negative, Action::Train, MoodLevel::Normal) => &[
            "「どうせできない…」",
            "「…はい」",
            "「やっぱり無理でした…」",
            "「…ごめんなさい」",
        ],
        (VoiceType::Negative, Action::Train, MoodLevel::Low) => &[
            "「…もうだめ」",
            "「…」",
            "「すみません…」",
            "「わたしなんか…」",
        ],
        (VoiceType::Negative, Action::Relax, MoodLevel::High) => &[
            "「…ちょっとだけ安心する…」",
            "「…ありがとう」",
            "「こういう時間…好きかも」",
            "「…zzz」",
        ],
        (VoiceType::Negative, Action::Relax, MoodLevel::Normal) => &[
            "「…zzz」",
            "「…寝てもいい？」",
            "「…」",
            "「…ごめん、ねむい」",
        ],
        (VoiceType::Negative, Action::Relax, MoodLevel::Low) => {
            &["「…もう寝ます」", "「…」", "「…zzz」", "「…消えたい」"]
        }

        // ===== Tennen (airhead) =====
        (VoiceType::Tennen, Action::Talk, MoodLevel::High) => &[
            "「あ、いた！なんかうれしい！」",
            "「えへへ」",
            "「なんの話してたっけ」",
            "「あ、そっか！」",
        ],
        (VoiceType::Tennen, Action::Talk, MoodLevel::Normal) => &[
            "「あ、そっか」",
            "「…なんの話だっけ」",
            "「ん？」",
            "「あー」",
        ],
        (VoiceType::Tennen, Action::Talk, MoodLevel::Low) => &[
            "「…あれ、なにしてたっけ」",
            "「…zzz…あ、起きてた」",
            "「…ん？」",
            "「…」",
        ],
        (VoiceType::Tennen, Action::Play, MoodLevel::High) => &[
            "「わーい！…あれ、なにして遊ぶんだっけ」",
            "「たのしー！なにが楽しいかわかんないけど！」",
            "「えへへ」",
            "「もっかい！…なにを？」",
        ],
        (VoiceType::Tennen, Action::Play, MoodLevel::Normal) => &[
            "「あ、遊ぶの？」",
            "「…なにするの？」",
            "「ふーん」",
            "「あ、終わった？」",
        ],
        (VoiceType::Tennen, Action::Play, MoodLevel::Low) => {
            &["「…あれ、遊んでたの？」", "「…zzz」", "「…ん？」", "「…」"]
        }
        (VoiceType::Tennen, Action::Train, MoodLevel::High) => &[
            "「がんばるー！…なにを？」",
            "「できたー！…なにが？」",
            "「えへへ、つよくなった？」",
            "「もっかい！」",
        ],
        (VoiceType::Tennen, Action::Train, MoodLevel::Normal) => &[
            "「えーと…」",
            "「…こう？」",
            "「あれ、なにしてるんだっけ」",
            "「…ふぅ」",
        ],
        (VoiceType::Tennen, Action::Train, MoodLevel::Low) => {
            &["「…あれ」", "「…zzz」", "「…」", "「…なにしてたっけ」"]
        }
        (VoiceType::Tennen, Action::Relax, MoodLevel::High) => &[
            "「いいてんき〜…あれ、室内だった」",
            "「zzz…えへへ」",
            "「きもちいい〜」",
            "「あ、雲がおもしろい形してる！…見えないか」",
        ],
        (VoiceType::Tennen, Action::Relax, MoodLevel::Normal) => &[
            "「…zzz」",
            "「ふぁ〜」",
            "「…あ、寝てた」",
            "「のんびり〜」",
        ],
        (VoiceType::Tennen, Action::Relax, MoodLevel::Low) => {
            &["「…zzz」", "「…」", "「…あれ」", "「…ねる」"]
        }

        // ===== Mukuchi (taciturn) =====
        (VoiceType::Mukuchi, Action::Talk, MoodLevel::High) => &[
            "「…」（少し近づいてきた）",
            "「ん」",
            "（じっとこちらを見る）",
            "（少し首をかしげる）",
        ],
        (VoiceType::Mukuchi, Action::Talk, MoodLevel::Normal) => {
            &["「…」", "「ん」", "「。」", "（目を合わせない）"]
        }
        (VoiceType::Mukuchi, Action::Talk, MoodLevel::Low) => {
            &["「…」", "（動かない）", "（…）", "「…」"]
        }
        (VoiceType::Mukuchi, Action::Play, MoodLevel::High) => &[
            "（少し楽しそう）",
            "「…！」",
            "（うなずく）",
            "（もう一回、という顔）",
        ],
        (VoiceType::Mukuchi, Action::Play, MoodLevel::Normal) => {
            &["「…」", "（やっている）", "（…）", "「ん」"]
        }
        (VoiceType::Mukuchi, Action::Play, MoodLevel::Low) => {
            &["（動かない）", "「…」", "（…）", "（首を横に振る）"]
        }
        (VoiceType::Mukuchi, Action::Train, MoodLevel::High) => &[
            "（黙々とやっている）",
            "「…！」",
            "（力強くうなずく）",
            "（…次は？という目）",
        ],
        (VoiceType::Mukuchi, Action::Train, MoodLevel::Normal) => {
            &["「…」", "（やっている）", "（…）", "「ん」"]
        }
        (VoiceType::Mukuchi, Action::Train, MoodLevel::Low) => {
            &["（座り込む）", "「…」", "（…）", "（首を横に振る）"]
        }
        (VoiceType::Mukuchi, Action::Relax, MoodLevel::High) => &[
            "（穏やかな顔）",
            "「…」",
            "（そっと目を閉じる）",
            "（少し微笑む）",
        ],
        (VoiceType::Mukuchi, Action::Relax, MoodLevel::Normal) => {
            &["「…」", "（…）", "（目を閉じている）", "「…zzz」"]
        }
        (VoiceType::Mukuchi, Action::Relax, MoodLevel::Low) => {
            &["「…」", "（…）", "（丸くなる）", "「…zzz」"]
        }

        // ===== Kajou (excessive) =====
        (VoiceType::Kajou, Action::Talk, MoodLevel::High) => &[
            "「話しかけてくれた…！！これが…会話…！！」",
            "「言葉が…！心に響く…！！」",
            "「ありがとう！！ありがとう！！！」",
            "「生きてて良かった…！！」",
        ],
        (VoiceType::Kajou, Action::Talk, MoodLevel::Normal) => &[
            "「…話しかけてくれたのか…」",
            "「ありがとう…」",
            "「言葉って…すごいな…」",
            "「…感動した」",
        ],
        (VoiceType::Kajou, Action::Talk, MoodLevel::Low) => &[
            "「…声が…遠い…」",
            "「…ありがとう…」",
            "「…」",
            "「…聞こえてる…」",
        ],
        (VoiceType::Kajou, Action::Play, MoodLevel::High) => &[
            "「これが…！遊ぶということか…！！」",
            "「楽しい…楽しいとはこういうことだったのか…！」",
            "「疲れた…！でも…！生きてる…！！」",
            "「もう一回やったら、何かが変わる気がする…！」",
        ],
        (VoiceType::Kajou, Action::Play, MoodLevel::Normal) => &[
            "「遊ぶ…遊ぶとは…」",
            "「…なるほど」",
            "「…すごい」",
            "「…」",
        ],
        (VoiceType::Kajou, Action::Play, MoodLevel::Low) => {
            &["「…遊べない…体が…」", "「…」", "「…ごめん…」", "「…無理…」"]
        }
        (VoiceType::Kajou, Action::Train, MoodLevel::High) => &[
            "「鍛える…！！これが成長…！！！」",
            "「限界を超えた…！！気がする…！！！」",
            "「すごい…！自分がすごい…！！」",
            "「もっと…！もっとだ…！！！」",
        ],
        (VoiceType::Kajou, Action::Train, MoodLevel::Normal) => &[
            "「…やる…」",
            "「…鍛えるとは…」",
            "「…ふむ」",
            "「…なるほど」",
        ],
        (VoiceType::Kajou, Action::Train, MoodLevel::Low) => &[
            "「…もう…無理…」",
            "「限界…これが…限界…」",
            "「…」",
            "「…体が…」",
        ],
        (VoiceType::Kajou, Action::Relax, MoodLevel::High) => &[
            "「休む…！！これが安らぎ…！！！」",
            "「なんて穏やかな…！！」",
            "「生きてるって…素晴らしい…！」",
            "「この瞬間を永遠に…！！」",
        ],
        (VoiceType::Kajou, Action::Relax, MoodLevel::Normal) => {
            &["「…休む…」", "「…zzz」", "「…穏やか…」", "「…」"]
        }
        (VoiceType::Kajou, Action::Relax, MoodLevel::Low) => {
            &["「…休ませて…」", "「…zzz」", "「…」", "「…もう…」"]
        }

        // ===== Kansai (Kansai dialect) =====
        (VoiceType::Kansai, Action::Talk, MoodLevel::High) => &[
            "「おー！よう来たな！」",
            "「なんやねん、うれしいやんけ」",
            "「ほんまええ日やわ」",
            "「まあ座り座り」",
        ],
        (VoiceType::Kansai, Action::Talk, MoodLevel::Normal) => &[
            "「なんやねん」",
            "「あーはいはい」",
            "「ほんまかいな」",
            "「知らんけど」",
        ],
        (VoiceType::Kansai, Action::Talk, MoodLevel::Low) => &[
            "「…だるいわ」",
            "「ほっといてくれ」",
            "「…」",
            "「あかん…」",
        ],
        (VoiceType::Kansai, Action::Play, MoodLevel::High) => &[
            "「おもろ！もっかいやろ！」",
            "「それええやん！」",
            "「わはは！」",
            "「楽しいやんけ！」",
        ],
        (VoiceType::Kansai, Action::Play, MoodLevel::Normal) => &[
            "「まあそれなりにできたんちゃう？知らんけど」",
            "「ふつーやな」",
            "「まあまあやな」",
            "「しゃーないな」",
        ],
        (VoiceType::Kansai, Action::Play, MoodLevel::Low) => &[
            "「むりやわ」",
            "「あかん」",
            "「…」",
            "「かんべんしてくれ」",
        ],
        (VoiceType::Kansai, Action::Train, MoodLevel::High) => &[
            "「なんやねんこれ、めっちゃええやん！」",
            "「しゃーないな、もっかいやったるわ」",
            "「いけるいける！」",
            "「ええ感じやで！」",
        ],
        (VoiceType::Kansai, Action::Train, MoodLevel::Normal) => &[
            "「なんやねんこれ」",
            "「疲れたわ〜でもまあええか」",
            "「それちゃうやろ。絶対ちゃうやろ」",
            "「まあこんなもんか」",
        ],
        (VoiceType::Kansai, Action::Train, MoodLevel::Low) => &[
            "「もうあかん…」",
            "「無理やって…」",
            "「…」",
            "「かんにんしてくれ」",
        ],
        (VoiceType::Kansai, Action::Relax, MoodLevel::High) => &[
            "「あー極楽やわ〜」",
            "「これこれ、これやで」",
            "「ええ気持ちやなぁ」",
            "「最高かよ」",
        ],
        (VoiceType::Kansai, Action::Relax, MoodLevel::Normal) => &[
            "「…zzz」",
            "「のんびりやな」",
            "「まあええか」",
            "「…ん？寝てたわ」",
        ],
        (VoiceType::Kansai, Action::Relax, MoodLevel::Low) => {
            &["「…zzz」", "「もう寝るわ」", "「…」", "「おやすみ」"]
        }

        // ===== Kogo (archaic) =====
        (VoiceType::Kogo, Action::Talk, MoodLevel::High) => &[
            "「さよう、いかにも」",
            "「参られたか。よきことなり」",
            "「今宵は月が美しい」",
            "「なかなかに良き日よ」",
        ],
        (VoiceType::Kogo, Action::Talk, MoodLevel::Normal) => {
            &["「…さて」", "「いかがした」", "「さても…」", "「…ふむ」"]
        }
        (VoiceType::Kogo, Action::Talk, MoodLevel::Low) => {
            &["「…」", "「退がれ」", "「…さても退屈なり」", "「…zzz」"]
        }
        (VoiceType::Kogo, Action::Play, MoodLevel::High) => &[
            "「いざ、参らん！」",
            "「なかなかに面白し！」",
            "「よき遊びなり！」",
            "「もう一度、勝負じゃ！」",
        ],
        (VoiceType::Kogo, Action::Play, MoodLevel::Normal) => &[
            "「…ふむ」",
            "「まあ、よかろう」",
            "「さても」",
            "「…いたしかたなし」",
        ],
        (VoiceType::Kogo, Action::Play, MoodLevel::Low) => &[
            "「…もはやこれまで」",
            "「…」",
            "「退くぞ」",
            "「…力尽きたり」",
        ],
        (VoiceType::Kogo, Action::Train, MoodLevel::High) => &[
            "「いざ、鍛錬なり！」",
            "「武者震いがするのう！」",
            "「まだまだじゃ！」",
            "「これぞ修行よ！」",
        ],
        (VoiceType::Kogo, Action::Train, MoodLevel::Normal) => {
            &["「…修行か」", "「心得た」", "「…ふむ」", "「…よかろう」"]
        }
        (VoiceType::Kogo, Action::Train, MoodLevel::Low) => {
            &["「…もはや…」", "「…力尽きたり」", "「…」", "「許されよ」"]
        }
        (VoiceType::Kogo, Action::Relax, MoodLevel::High) => &[
            "「いざ、まどろまん」",
            "「されど、心は穏やかなり」",
            "「風情があるのう」",
            "「ゆるりとしておる。それでよいのじゃ」",
        ],
        (VoiceType::Kogo, Action::Relax, MoodLevel::Normal) => {
            &["「…zzz」", "「さても、退屈なり」", "「…うむ」", "「…」"]
        }
        (VoiceType::Kogo, Action::Relax, MoodLevel::Low) => {
            &["「…zzz」", "「…」", "「もはや…」", "「…休ませよ」"]
        }
    }
}

fn get_idle_pool(voice_type: VoiceType, mood: MoodLevel) -> &'static [&'static str] {
    match (voice_type, mood) {
        (VoiceType::Tameguchi, MoodLevel::High) => {
            &["ふーん", "まあいっか", "ひまだな", "なんかいい天気"]
        }
        (VoiceType::Tameguchi, MoodLevel::Normal) => &["…", "うるさい", "ほっとけ", "べつに"],
        (VoiceType::Tameguchi, MoodLevel::Low) => &["…", "はぁ", "だるい", "…zzz"],

        (VoiceType::Keigo, MoodLevel::High) => &[
            "今日もよい日でございます",
            "ご機嫌うるわしゅう",
            "素敵な一日ですね",
            "ありがとうございます",
        ],
        (VoiceType::Keigo, MoodLevel::Normal) => {
            &["…失礼します", "穏やかですね", "…はい", "恐縮です"]
        }
        (VoiceType::Keigo, MoodLevel::Low) => &[
            "…申し訳ございません",
            "…失礼",
            "…",
            "お休みさせていただきます",
        ],

        (VoiceType::Gal, MoodLevel::High) => &[
            "やばーい！",
            "テンション上がる〜",
            "今日めっちゃいい日じゃん！",
            "るんるん♪",
        ],
        (VoiceType::Gal, MoodLevel::Normal) => &["あーね", "ふつー", "まあいっか", "…ん？"],
        (VoiceType::Gal, MoodLevel::Low) => &["だる…", "無理…", "…", "帰りたい…"],

        (VoiceType::Oyaji, MoodLevel::High) => &[
            "いい天気だなあ",
            "むかしはなあ…",
            "まあ座れ座れ",
            "人生捨てたもんじゃない",
        ],
        (VoiceType::Oyaji, MoodLevel::Normal) => {
            &["ん？なんだ？", "まあな", "…うむ", "そういうこともある"]
        }
        (VoiceType::Oyaji, MoodLevel::Low) => &["腰が痛い", "疲れたわ", "…", "歳は取りたくない"],

        (VoiceType::Tetsugaku, MoodLevel::High) => &[
            "存在とは…ああ、いい天気だ",
            "時間は幻だ。でも今は美しい",
            "考える、ゆえに我あり",
            "なにかが近づいている…いい予感だ",
        ],
        (VoiceType::Tetsugaku, MoodLevel::Normal) => &[
            "存在とは何か",
            "…考えている",
            "時間は幻だ",
            "沈黙にも意味がある",
        ],
        (VoiceType::Tetsugaku, MoodLevel::Low) => {
            &["虚無を見つめている", "…", "意味はあるのか", "存在が重い"]
        }

        (VoiceType::Taiiku, MoodLevel::High) => &[
            "今日もいい日ッス！",
            "元気ッス！",
            "がんばるッス！",
            "声出していこう！",
        ],
        (VoiceType::Taiiku, MoodLevel::Normal) => &["はい！", "…ッス", "がんばります", "了解ッス"],
        (VoiceType::Taiiku, MoodLevel::Low) => &["…ッス", "すいません…", "…", "休憩…"],

        (VoiceType::Negative, MoodLevel::High) => &[
            "…今日はちょっとだけいい日かも",
            "…ありがとう",
            "…うれしい…かも",
            "…笑ってもいいのかな",
        ],
        (VoiceType::Negative, MoodLevel::Normal) => &["どうせ…", "…ごめんなさい", "…", "…うん"],
        (VoiceType::Negative, MoodLevel::Low) => &["消えたい…", "…", "やっぱり無理", "…ごめん"],

        (VoiceType::Tennen, MoodLevel::High) => {
            &["えへへ", "いいてんき〜", "なにしてたっけ", "あ、そっか！"]
        }
        (VoiceType::Tennen, MoodLevel::Normal) => {
            &["…ん？", "あ、そっか", "ふぁ〜", "…なんの話だっけ"]
        }
        (VoiceType::Tennen, MoodLevel::Low) => &["…zzz…あ、起きてた", "…あれ", "…", "…ん？"],

        (VoiceType::Mukuchi, MoodLevel::High) => &["…！", "ん", "（少し微笑む）", "…"],
        (VoiceType::Mukuchi, MoodLevel::Normal) => &["…", "ん", "。", "（…）"],
        (VoiceType::Mukuchi, MoodLevel::Low) => &["…", "（…）", "（動かない）", "…"],

        (VoiceType::Kajou, MoodLevel::High) => &[
            "生きてる…！！すばらしい…！！",
            "今日という日に…感謝…！！",
            "存在が…輝いている…！！",
            "なにもかもが…美しい…！！",
        ],
        (VoiceType::Kajou, MoodLevel::Normal) => &["…生きている", "…すごい", "…なるほど", "…"],
        (VoiceType::Kajou, MoodLevel::Low) => &["…もう…", "…つらい…", "…", "…消えそう…"],

        (VoiceType::Kansai, MoodLevel::High) => &[
            "ええ天気やなぁ",
            "なんやねん、ええやんけ",
            "おもろいわ",
            "最高かよ",
        ],
        (VoiceType::Kansai, MoodLevel::Normal) => {
            &["知らんけど", "まあええか", "なんやねん", "…ん？"]
        }
        (VoiceType::Kansai, MoodLevel::Low) => &["あかん…", "だるいわ", "…", "しんど"],

        (VoiceType::Kogo, MoodLevel::High) => &[
            "なかなかに良き日よ",
            "風情があるのう",
            "されど心は穏やかなり",
            "参られたか",
        ],
        (VoiceType::Kogo, MoodLevel::Normal) => &["…さて", "さても", "…ふむ", "いかがした"],
        (VoiceType::Kogo, MoodLevel::Low) => &["…", "もはや…", "…zzz", "退屈なり"],
    }
}
