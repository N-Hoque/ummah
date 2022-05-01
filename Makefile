debug:
	cargo check
	cargo build

performance:
	cargo check --profile performance
	cargo build --profile performance

clean_targets:
	cargo clean

clean_cache:
	rm -fv .current_settings.yaml current_month.yaml

clean_all:
	make clean_targets
	make clean_cache
