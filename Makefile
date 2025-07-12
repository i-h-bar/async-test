build:
	@cargo build --no-default-features

build_release:
	@cargo build --no-default-features --release

run:
	@cargo run --no-default-features

run_release:
	@target/release/async_test