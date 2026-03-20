# Tamimon

**Terminal Monster** — A CLI virtual pet game where your monster grows even while you're away.

[日本語版 README](README_ja.md)

```
─────────────────────────────────────────────
  Koron          3d 14h 22m 08s  ⚖ 84kg
─────────────────────────────────────────────



                   (≧▽≦)ノ
                    ヾ
                   /|



            「 Today is great! 」

─────────────────────────────────────────────
  [T]Talk  [P]Play  [R]Train  [E]Relax
  [A]Album                      [Q]Quit
─────────────────────────────────────────────
```

## What is Tamimon?

Tamimon is a terminal-based idle monster raising game inspired by Digimon and Tamagotchi.

- **Idle-friendly** — Your monster continues to live while the app is closed. Real time passes, stats change, and events happen even when you're away.
- **No death from neglect** — Your monster won't die just because you forgot about it. Death only comes from rare random accidents.
- **Hidden stats** — You can't see your monster's internal stats (happiness, energy, bonding). The only visible number is weight. Everything else is expressed through facial expressions and dialogue.
- **Evolution by playstyle** — How you interact determines which of 120+ species your monster evolves into, using cosine similarity matching. It's hard to aim for a specific evolution — that's part of the fun.
- **Rich ASCII art** — Every species has unique hand-crafted ASCII art with mood variations and action animations.

## Features

- 4-stage evolution system (Egg → Stage 1 → Stage 2 → Stage 3, with rare Stage 4 mutations)
- 120+ unique species across all stages
- 12 distinct personality/voice types with unique dialogue
- Random events and accidents during offline time
- Album (Pokedex-style collection) tracking all monsters you've raised
- Time verification via WorldTimeAPI with local fallback
- Save data stored as JSON at `~/.tamimon/save.json`

## Installation

### Quick install (macOS / Linux)

```bash
curl -sSL https://raw.githubusercontent.com/usagi-f/tamimon/main/install.sh | sh
```

You can set a custom install directory:

```bash
TAMIMON_INSTALL_DIR=~/.local/bin curl -sSL https://raw.githubusercontent.com/usagi-f/tamimon/main/install.sh | sh
```

### From source (requires Rust)

```bash
cargo install --git https://github.com/usagi-f/tamimon.git
```

### Download binary

Pre-built binaries for each release are available on the [Releases](https://github.com/usagi-f/tamimon/releases) page:

| Platform | Binary |
|----------|--------|
| Linux (x86_64) | `tamimon-x86_64-unknown-linux-gnu` |
| macOS (Intel) | `tamimon-x86_64-apple-darwin` |
| macOS (Apple Silicon) | `tamimon-aarch64-apple-darwin` |

## How to Play

```bash
tamimon
```

On first launch, you'll be asked to name your egg. After about an hour, it hatches into a Stage 1 monster.

### Actions

| Key | Action | Effect |
|-----|--------|--------|
| `T` | Talk | Increases bonding and happiness |
| `P` | Play | Increases happiness and energy |
| `R` | Train | Increases energy, reduces weight |
| `E` | Relax | Increases happiness, adds weight |
| `A` | Album | View your monster collection |
| `Q` | Quit | Save and exit |

There are no cooldowns — you can perform actions as often as you like.

### Evolution

Your monster evolves based on how you raise it:

- **Stage 1** (after ~1 hour): Random species from the egg
- **Stage 2** (after ~6 hours): Determined by your action patterns
- **Stage 3** (after ~24 hours): Determined by cosine similarity of your care vector against species vectors
- **Stage 4** (rare mutation): 25% chance per 24-hour check window

The five evolution types are: **Chikara** (power), **Odayaka** (gentle), **Bouken** (adventure), **Normal**, and **Wild**.

### Death & Rebirth

Your monster can only die from random accidents — never from low stats. If your monster dies, it's recorded in your Album and you start fresh with a new egg.

Higher bonding gives your monster a better chance of surviving accidents.

## Supported Platforms

- macOS (Intel and Apple Silicon)
- Linux (x86_64)

## Development

```bash
git clone https://github.com/usagi-f/tamimon.git
cd tamimon
cargo run
```

Run tests:

```bash
cargo test
```

## License

MIT
