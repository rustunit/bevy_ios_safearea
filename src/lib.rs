#![doc = include_str!("../README.md")]

#[cfg(any(target_os = "ios", target_os = "tvos"))]
mod native;
mod plugin;

pub use plugin::{IosSafeArea, IosSafeAreaPlugin, IosSafeAreaResource};
