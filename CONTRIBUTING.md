# Contributing

## Branch model

- `develop` is the integration branch: every pull request targets it.
- `main` only receives releases, merged from `develop` with **Create a merge commit** (never squash, or `main` and `develop` diverge). The *Only develop into main* check blocks pull requests into `main` from any other branch.
- Work branches start from `develop` and carry the Fibery id: `feat/US-xx-short-name`, `feat/T-xx-short-name` or `fix/I-xx-short-name`.

## Commit messages

`<gitmoji> <type>(<scope>): <summary>`, imperative, lower case, no final period.

Scopes: `core`, `domain`, `application`, `infrastructure`, `server`, `wasm`, `proto`, `client`, `ci`, `docs`.

| Gitmoji | Type | Example |
|---|---|---|
| ✨ | feat | `✨ feat(infrastructure): build the line offset index` |
| 🐛 | fix | `🐛 fix(infrastructure): strip the CR of CRLF line endings` |
| ✅ | test | `✅ test(infrastructure): cover empty files and missing trailing newline` |
| ♻️ | refactor | `♻️ refactor(application): split GetLines into its own use case` |
| ⚡️ | perf | `⚡️ perf(infrastructure): read the file by 64 KiB chunks` |
| 📝 | docs | `📝 docs(adr): add ADR 0003 web core in WASM` |
| 👷 | ci | `👷 ci: add the WASM build job` |
| 🔧 | chore | `🔧 chore(core): pin the toolchain to 1.100.0` |
| ⬆️ | build | `⬆️ build(core): upgrade thiserror to 2.0` |
| 💥 | breaking | `💥 feat(proto)!: rename LinesPage to GetLinesResponse` |

Full list: <https://gitmoji.dev>.

## Pull requests

Fill in [the template](.github/pull_request_template.md). Fibery links the PR to its story and task from the *Relations* section (`US-n`, `T-n`, `I-n`).

Before asking for a review:

- [ ] The PR targets `develop`.
- [ ] Tests are added or updated.
- [ ] `cargo fmt`, `cargo clippy -D warnings` and `cargo test` pass in `core/` (see the [README](README.md#tests-and-quality-checks)).
- [ ] The layer rules of [docs/architecture.md](docs/architecture.md) are respected.
- [ ] `CHANGELOG.md` is updated under *Unreleased*.
- [ ] No real log content is committed: files in `samples/` are synthetic.

The CI (`Core CI success`) must be green before merge.

## Architecture decisions

A significant technical choice gets an ADR in `docs/adr/` (next free number, listed in `docs/adr/README.md`).
