//! `LogViewer` web entry point.
//!
//! Compiled to `wasm32-unknown-unknown` and loaded in a dedicated Web Worker.
//! The UI and the worker exchange protobuf `Envelope` messages through
//! `postMessage` (see `proto/logviewer/v1/worker.proto`). This is the
//! composition root for the web: it picks the `browser` infrastructure.
//!
//! The envelope dispatch is not wired yet (walking skeleton).

use wasm_bindgen::prelude::wasm_bindgen;

// Composition root: every layer is linked; the imports go once real code uses them.
use logviewer_application as _;
use logviewer_domain as _;
use logviewer_infrastructure as _;

/// Version of the Core, so the JS glue can check it loaded the right module.
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

#[cfg(test)]
mod tests {
    #[test]
    fn version_matches_manifest() {
        assert_eq!(super::version(), env!("CARGO_PKG_VERSION"));
    }
}
