[working-directory: 'bevy_ios_safearea']
check:
	cargo c --target=aarch64-apple-ios
	cargo b
	cargo t
	cargo clippy
	cargo doc --workspace --all-features --document-private-items --no-deps

[working-directory: 'bevy_ios_safearea']
publish:
	cargo publish
