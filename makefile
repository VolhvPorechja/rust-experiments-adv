.PHONY: build exec_a exec_b

build: ## This target is just a placeholder for convenience
	@:

exec_a: build ## Build and execute target exec_a
	cd exec_a && \
cargo build

exec_b: build ## Build and execute target exec_b
	cd exec_b && \
cargo build