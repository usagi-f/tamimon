# Tamimon

**Terminal Monster** — An idle monster-raising game built with Rust for the terminal

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

## Features

- 4-stage evolution system
- 120+ unique ASCII art species
- Album to track every monster you've raised
- Save data stored at `~/.tamimon/save.json`

## Installation

### Quick install (macOS / Linux)

```bash
curl -sSL https://raw.githubusercontent.com/usagi-f/tamimon/main/install.sh | sh
```

Custom install directory:

```bash
TAMIMON_INSTALL_DIR=~/.local/bin curl -sSL https://raw.githubusercontent.com/usagi-f/tamimon/main/install.sh | sh
```

### From source (requires Rust)

```bash
cargo install --git https://github.com/usagi-f/tamimon.git
```

### Download binary

Pre-built binaries are available on the [Releases](https://github.com/usagi-f/tamimon/releases) page:

| Platform | Binary |
|----------|--------|
| Linux (x86_64) | `tamimon-x86_64-unknown-linux-gnu` |
| macOS (Intel) | `tamimon-x86_64-apple-darwin` |
| macOS (Apple Silicon) | `tamimon-aarch64-apple-darwin` |

## How to Play

```bash
tamimon
```

On first launch, you'll name your egg. After about an hour, it hatches into a Tamimon.

### Actions

| Key | Action | Effect |
|-----|--------|--------|
| `T` | Talk | Increases bonding and happiness |
| `P` | Play | Increases happiness and energy |
| `R` | Train | Increases energy, reduces weight |
| `E` | Relax | Increases happiness, adds weight |
| `A` | Album | View your Tamimon collection |
| `Q` | Quit | Save and exit |

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
