//! `LogViewer` core library.
//!
//! Hosts the log parsing, indexing and filtering logic. It has no knowledge of
//! the transport layer: the `logviewer-server` crate exposes it over RPC.

/// Version of the core library, as declared in `Cargo.toml`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::VERSION;

    #[test]
    fn version_matches_manifest() {
        assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
    }
}
