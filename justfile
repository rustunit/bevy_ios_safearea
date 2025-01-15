check:
	cd bevy_ios_safearea/src && cargo c --target=aarch64-apple-ios
	cd bevy_ios_safearea/src && cargo b
	cd bevy_ios_safearea/src && cargo clippy --workspace --all-features --all-targets -- -D warnings

publish:
	cd bevy_ios_safearea/ && cargo publish