#[cfg(target_os = "ios")]
mod native;
mod plugin;

pub use plugin::{IosSafeAreaPlugin, IosSafeAreaResource};
