check:
	cargo c --target=aarch64-apple-ios
	cargo b
	cargo t
	cargo clippy --target=aarch64-apple-ios --all-features --all-targets -- --deny warnings
	cargo clippy
	cargo doc --workspace --all-features --document-private-items --no-deps

publish:
	cargo publish
