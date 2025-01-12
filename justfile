check:
	cd bevy_ios_safearea/src && cargo c --target=aarch64-apple-ios
	cd bevy_ios_safearea/src && cargo b
	cd bevy_ios_safearea/src && cargo clippy

publish:
	cd bevy_ios_safearea/ && cargo publish