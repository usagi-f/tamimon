# Tamimon 仕様書 01 - 概要・技術・構成

## 1. プロジェクト概要

### ゲームコンセプト
CLIで動作するデジモン・ポケモン風の育成放置ゲーム。
プレイヤーは仮想のモンスターを育て、アプリを閉じている間もリアル時間に応じてゲームが進行する。

**基本方針：「放置してても大丈夫」**
仕事中も裏で起動しておけるカジュアルさを重視する。
お世話をサボっても死なない。ケアの内容は「どんな姿に進化するか」に影響するが、死因にはならない。
モンスターの死は稀な「事故」によってのみ発生し、プレイヤーにとって予測不能な運ゲー的演出として機能する。
目標は「より良い進化を目指すこと」と「どんな事故が起きるかを楽しむこと」。

### ゲームタイトル
**`Tamimon`**（Terminal Monster → ターミナル + モンスター → Tamimon）

---

## 2. 技術スタック

| 用途 | ライブラリ / ツール | 理由 |
|------|------|------|
| 言語 | **Rust** | 単一バイナリで配布可能、依存関係ゼロ、クロスコンパイル対応 |
| TUI | **ratatui + crossterm** | リッチなターミナルUI（カラー・レイアウト・アスキーアート） |
| HTTP | **reqwest** (async) | WorldTimeAPI呼び出し |
| 非同期ランタイム | **tokio** | reqwestのasync対応 |
| シリアライズ | **serde + serde_json** | セーブデータのJSON入出力 |
| 日時処理 | **chrono** | タイムスタンプ計算 |
| エラーハンドリング | **anyhow** | エラー伝播の簡略化 |
| 乱数 | **rand** | イベント発生確率 |
| CLIパース | **clap** | コマンドライン引数・サブコマンド管理 |

対応環境: **macOS / Linux のみ**

---

## 3. ディレクトリ構成

```
tamimon/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── .github/
│   └── workflows/
│       └── release.yml          # クロスコンパイル & リリース自動化
├── src/
│   ├── main.rs                  # エントリーポイント、CLIパース
│   ├── app.rs                   # アプリケーションループ（TUI制御）
│   ├── game/
│   │   ├── mod.rs
│   │   ├── pet.rs               # Petステート・ステータス管理
│   │   ├── evolution.rs         # 進化ロジック
│   │   ├── time.rs              # 経過時間計算・WorldTimeAPI
│   │   ├── events.rs            # ランダムイベント
│   │   └── actions.rs           # プレイヤーアクション処理
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── main_screen.rs       # メイン画面レイアウト
│   │   ├── ascii_art.rs         # キャラクターアスキーアート
│   │   └── album.rs             # 図鑑画面
│   └── save/
│       ├── mod.rs
│       └── schema.rs            # SaveDataの構造体定義
└── assets/
    ├── ascii/                   # アスキーアートテキストファイル（各進化段階）
    └── reactions/               # 口調タイプ別リアクションテキスト
```

---

## 4. 実装フェーズ

### Phase 1: コア機能（MVP）
- [ ] セーブデータの読み書き
- [ ] WorldTimeAPIによる経過時間計算
- [ ] ステータス変化計算（オフライン経過分）
- [ ] 基本アクション（話しかける / あそぶ / 特訓 / まったり）
- [ ] シンプルなTUI表示（ratatuiレイアウト）
- [ ] アスキーアート表示（Stage1のみ）

### Phase 2: ゲームプレイ充実
- [ ] 進化・死亡システム
- [ ] ランダムイベント
- [ ] 全進化段階のアスキーアート・アニメーション
- [ ] 図鑑（アルバム）
- [ ] 口調タイプ別リアクションテキスト整備

### Phase 3: 品質・配布
- [ ] GitHub Actions でクロスコンパイル & リリース
- [ ] インストールシェルスクリプト（curl | sh）
- [ ] README（インストール方法・遊び方）
- [ ] エラーハンドリングの整備（ネットワーク障害時など）

---

## 5. 配布・インストール方法（計画）

### macOS / Linux
```bash
curl -sSL https://raw.githubusercontent.com/username/tamimon/main/install.sh | sh
```

### cargo（Rustユーザー向け）
```bash
cargo install tamimon
```

GitHub Releases に以下のバイナリを配置:
- `tamimon-x86_64-unknown-linux-gnu`
- `tamimon-x86_64-apple-darwin`
- `tamimon-aarch64-apple-darwin` (Apple Silicon)

---

## 6. スコープ確定事項

| 項目 | 決定内容 |
|-----|---------|
| 命名機能 | あり（たまご孵化時に任意のニックネームを入力） |
| 同時育成 | なし。常に1匹のみ |
| シーズナルイベント | 実装しない |
| 対戦・交流機能 | 実装しない |
| BGM / SE | 実装しない |
| 対応OS | macOS / Linux のみ |
