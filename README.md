<p align="center">
  <img src="docs/media/logo.svg" width="96" height="96" alt="">
</p>

<h1 align="center">git-tagger</h1>

<p align="center">
  A terminal UI for git tags: see exactly which commits shipped between two releases.
</p>

![git-tagger comparing the last production tag with the newest staging tag](docs/media/demo.gif)

If you tag releases per environment (`v1.4.0-staging`, `v1.3.0-prod`, …),
the question you keep asking is *"what's on staging that isn't in production
yet?"* git-tagger answers it in two keystrokes: mark the production tag, move
to the staging tag, and you get the commit list between them.

## Features

- **Every tag at a glance**, with its date, commit, author and annotation.
- **Compare mode:** pick a base tag and a target tag to list the commits
  between them.
- **Environment colours:** `-staging` tags in green, `-prod` / `-production`
  in red, or your own patterns via [`git-tagger.toml`](#configuration).
- **Two sort orders:** commit date (newest first) or SemVer (highest first).
- **Local and read-only:** a single binary built on `ratatui` and `git2`. It
  reads your repository and never fetches, pushes or writes.

## Install

You need [Rust and Cargo](https://rustup.rs/).

```bash
cargo install --git https://github.com/stephenbreen/git-tagger
```

Or from a clone:

```bash
git clone https://github.com/stephenbreen/git-tagger.git
cd git-tagger
cargo install --path .
```

Either way the `git-tagger` binary lands in `~/.cargo/bin`.

## Usage

Run it from the root of a git repository:

```bash
git-tagger
```

| Key          | Action                                                        |
| ------------ | ------------------------------------------------------------- |
| `j` / `Down` | Move selection down                                           |
| `k` / `Up`   | Move selection up                                             |
| `c`          | Mark the base tag, then press again on a target tag to compare. Press once more to leave compare mode |
| `d`          | Sort by commit date (newest first)                            |
| `s`          | Sort by SemVer (highest first)                                |
| `q`          | Quit                                                          |

## Configuration

Drop a `git-tagger.toml` next to where you run it to choose your own
highlight colours. The first pattern a tag name contains wins:

```toml
highlights = [
  { pattern = "-staging", color = "green" },
  { pattern = "-prod", color = "red" },
  { pattern = "-rc", color = "yellow" },
]
```

Colours: `red`, `green`, `blue`, `yellow`, `magenta`, `cyan`, `white`,
`black`. A config file replaces the defaults, so list every pattern you want.

## Development

```bash
cargo run      # from the root of any git repo with tags
cargo test
```

The demo GIF is recorded with [VHS](https://github.com/charmbracelet/vhs)
against a throwaway repo (`docs/make-demo-repo.sh`), so it can be re-recorded
after UI changes:

```bash
cargo build --release && vhs docs/demo.tape
```

## License

MIT, see [LICENSE](LICENSE).

---

<sub>Made by [Stephen Breen](https://stephenbreen.github.io).</sub>
