<!--
  Target branch: `develop` (`main` only receives releases from `develop`).
  Title: same format as a commit message, e.g. `✨ feat(core): return lines by range`.
  Fibery links the PR from the Relations section below (title and branch are ignored).
-->

## Context

<!-- Why this change? What problem or story does it address? Link the spec section if relevant. -->

## What

<!-- What the PR changes, in a few bullet points. Mention anything reviewers should look at closely. -->

-

## How to test

<!-- Commands or steps to check the change, e.g. `cd core && cargo test -p logviewer-infrastructure`. -->

## Checklist

- [ ] Pull request targets `develop`
- [ ] Branch named `feat/…` or `fix/…` with the Fibery id (e.g. `feat/T-xx-line-index`)
- [ ] Commit messages follow the gitmoji convention below
- [ ] Tests added or updated
- [ ] `cargo fmt`, `cargo clippy -D warnings` and `cargo test` pass in `core/`
- [ ] Layer rules respected (see `docs/architecture.md`)
- [ ] `CHANGELOG.md` updated under *Unreleased*
- [ ] No real log content committed (samples are synthetic)

<!--
  Commit messages: `<gitmoji> <type>(<scope>): <summary>`, imperative, lower case, no final period.
  Scopes: core, domain, application, infrastructure, server, wasm, proto, client, ci, docs.

  ✨ feat(infrastructure): build the line offset index
  🐛 fix(infrastructure): strip the CR of CRLF line endings
  ✅ test(infrastructure): cover empty files and missing trailing newline
  ♻️ refactor(application): split GetLines into its own use case
  ⚡️ perf(infrastructure): read the file by 64 KiB chunks
  📝 docs(adr): add ADR 0003 web core in WASM
  👷 ci: add the WASM build job
  🔧 chore(core): pin the toolchain to 1.100.0
  ⬆️ build(core): upgrade thiserror to 2.0
  🏗️ refactor(core): move the workspace into core/
  🔥 chore: remove the old ci.yml workflow
  💥 feat(proto)!: rename LinesPage to GetLinesResponse

  Full list: https://gitmoji.dev
-->

## Relations

<!--
  Fibery reads this description: every US-<n> (or User Story link) is linked to the PR,
  and the first T-<n> / I-<n> (or Task / Issue link) sets its Task / Issue. Replace xx with the id.
  `Closes #N` closes the GitHub issue mirroring the Fibery task when the PR is merged.
  Remove the lines that do not apply.
-->

- Fibery story: [US-xx](https://stelor.fibery.io/LogViewer/User_Story/xx)
- Fibery task: [T-xx](https://stelor.fibery.io/LogViewer/Task/xx)
- Closes #
- Depends on #
- Follows #
- Related to #
