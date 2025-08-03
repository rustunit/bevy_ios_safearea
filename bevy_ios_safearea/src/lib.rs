#![doc = include_str!("../README.md")]

#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "ios")]
mod native;
mod plugin;

pub use plugin::{IosSafeArea, IosSafeAreaPlugin, IosSafeAreaResource};
