# git-tagger

`git-tagger` is a fast, lightweight Terminal User Interface (TUI) for visualizing Git tags and comparing changes between them. Built with Rust using `ratatui` and `git2-rs`.

## Features

- **Tag Visualization**: View all tags in your repository with their creation dates.
- **Environment Highlights**: Automatically color-codes tags (Green for `-staging`, Red for `-prod` or `-production`).
- **Flexible Sorting**: Sort tags chronologically (Commit Date) or by Version (SemVer).
- **Comparison Mode**: Quickly see which commits are missing between two tags.
- **Lightweight**: No heavy dependencies; works directly in your terminal.

## Installation

### From Source
Ensure you have [Rust and Cargo installed](https://rustup.rs/).

```bash
git clone https://github.com/your-username/git-tagger.git
cd git-tagger
cargo install --path .
```

This will install the `git-tagger` binary to your `~/.cargo/bin` directory.

## Usage

Run `git-tagger` from any directory that is a Git repository:

```bash
git-tagger
```

### Controls

| Key | Action |
|-----|--------|
| `j` / `Down` | Move selection down |
| `k` / `Up` | Move selection up |
| `c` | **Compare Mode**: Select a base tag, then select a target tag to see commits between them |
| `d` | Sort by **Date** (newest first) |
| `s` | Sort by **SemVer** (highest version first) |
| `q` | Quit |

## License

MIT
