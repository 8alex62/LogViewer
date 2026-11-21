//! Platform readers implementing
//! [`LogSource`](logviewer_application::ports::LogSource).

#[cfg(feature = "browser")]
pub mod browser;
#[cfg(feature = "native")]
pub mod native;
