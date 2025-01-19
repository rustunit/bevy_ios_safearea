check:
	cd bevy_ios_safearea && cargo c --target=aarch64-apple-ios
	cd bevy_ios_safearea && cargo b
	cd bevy_ios_safearea && cargo clippy
	cd bevy_ios_safearea && cargo doc --workspace --all-features --document-private-items --no-deps

publish:
	cd bevy_ios_safearea/ && cargo publish