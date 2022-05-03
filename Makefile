debug:
	cargo check
	cargo build

performance:
	cargo check --profile performance
	cargo build --profile performance

install:
	cargo install --profile performance --path .