# 0007. No macOS desktop build; macOS users use the web app

- Status: accepted
- Date: 2026-11-21 (decision taken on 2026-11-14)

## Context

Shipping a macOS desktop application properly requires an Apple developer account (paid yearly) for code signing and notarization; unsigned apps are blocked by Gatekeeper. The project has no such account. Meanwhile the web app runs the same Core in any modern browser, macOS included (ADR 0003).

## Decision

- **Desktop builds target Windows and Linux only.** macOS users use the web app.
- **CI** runs fmt, clippy and tests on Linux and Windows, plus a `wasm32-unknown-unknown` build of the web Core. There is no macOS job.
- First Windows releases ship unsigned (SmartScreen shows "More info" then "Run anyway"); signing will go through a free route (SignPath Foundation or the Microsoft Store), tracked in [Trust the Windows installer](https://stelor.fibery.io/LogViewer/User_Story/56).

## Consequences

- Lower cost and fewer CI minutes; no Apple-specific packaging to maintain.
- macOS-specific issues in the native reader (memory mapping, file watching) are not caught; this is acceptable because no macOS binary is shipped.
- macOS users get the browser limits (memory, no live tailing) described in ADR 0003.
- Revisit if an Apple developer account becomes available or if macOS users ask for desktop-only features such as live tailing.
