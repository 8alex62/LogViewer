# 0003. Web: Core compiled to WASM in a Web Worker, no upload

- Status: accepted
- Date: 2026-11-21 (decision taken on 2026-11-14)
- Supersedes: the earlier upload-based web design (never implemented)

## Context

The web version needs somewhere to process log files. A first design uploaded them to a backend (temporary file storage, sessions, `BeginUpload` / `UploadChunk` / `CompleteUpload` / `CancelUpload`, `CreateSession`, a Docker image).

Logs routinely contain personal or sensitive data: IP addresses, e-mails, tokens, customer data. Users, often non-experts, should never have to send them to a third-party server, and the project should not have to host them (GDPR, ethics). See ADR 0005.

## Decision

Process files **entirely in the user's browser**: the Core is compiled to WebAssembly (`logviewer-wasm`) and runs in a dedicated Web Worker. There is no upload and no backend.

- The user picks a file (file picker or drag and drop); the JS glue posts the browser `File` object to the worker (a reference, not a copy).
- The worker reads it by ranges (`Blob.slice` + `FileReaderSync`) and builds a sparse line index.
- The UI and the worker exchange protobuf `Envelope` messages through `postMessage` (ADR 0002).
- The web app is a static site on GitHub Pages; the custom domain points to it through a DNS `CNAME` record.

Reasons:

1. **Confidentiality**: file content never leaves the browser.
2. **No server to operate**: a static site costs nothing and has no capacity, abuse or patching concerns.
3. **Same Core**: domain, application, parsers and index are compiled for both targets; only the file reader and the transport differ.

## Consequences

- The upload RPCs, sessions and the Docker image are removed from the specs.
- Browser memory is limited (32-bit WASM, about 4 GB) and reading is slower than memory mapping, so very large files work better on desktop. The index must stay sparse in the browser ([Spike 4](https://stelor.fibery.io/LogViewer/Spike/4)).
- How the `File` object reaches the worker (JS glue or Avalonia `StorageProvider`) is still open ([Spike 5](https://stelor.fibery.io/LogViewer/Spike/5)).
- A strict Content Security Policy (`connect-src 'self'`, no third-party scripts) makes sure that even a bug cannot send data elsewhere.
