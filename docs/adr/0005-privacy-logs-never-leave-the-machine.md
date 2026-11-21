# 0005. Privacy: log files never leave the user's machine

- Status: accepted
- Date: 2026-11-21

## Context

Log files are among the most sensitive data a developer or an operator handles: IP addresses, e-mails, user names, tokens, stack traces with customer data. A log viewer is trusted with all of it. Even metadata (file names, paths, filters, bookmarks, recent files) can reveal what a user investigates.

## Decision

Privacy is a product principle that every feature must respect:

1. **No upload, no backend, no telemetry**: log content is only read on the user's machine, by the desktop Core or by the browser worker (ADR 0003). Nothing is sent to any server, analytics or crash service.
2. **Nothing about the user's files is persisted by default**, even locally: file names, paths, filters, presets, bookmarks and recent files are forgotten when the session ends.
3. **Persistence is opt-in**: a setting, off by default, shown with a red warning box that explains what is stored and the risk. Turning it off deletes everything that was stored.
4. **Only non-sensitive preferences are always stored**: theme, text size, language, tutorial seen.
5. **Defence in depth on the web**: strict Content Security Policy (`connect-src 'self'`, no third-party scripts), HTTPS only.

## Consequences

- Features such as recent files, bookmarks or filter presets must work without persistence, and only persist through the opt-in storage.
- Update checks against GitHub Releases (desktop) can be turned off; they never send log data.
- Test data and bug reports must be synthetic: `samples/` only holds anonymized or generated logs, and the issue template asks not to attach real logs.
- No usage analytics: product decisions rely on direct feedback rather than tracking.
