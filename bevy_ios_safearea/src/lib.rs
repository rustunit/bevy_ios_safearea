#![doc = include_str!("../README.md")]

#[cfg(all(
    target_os = "android",
    any(feature = "android-native-activity", feature = "android-game-activity")
))]
mod android;
#[cfg(target_os = "ios")]
mod native;
mod plugin;

pub use plugin::{IosSafeArea, IosSafeAreaPlugin, IosSafeAreaResource};
