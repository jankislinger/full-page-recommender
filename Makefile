.PHONY: lint test

format:
	cargo fmt && \
	uv run ruff format

lint:
	cargo fmt && \
	cargo fix --allow-dirty && \
	cargo clippy --fix --allow-dirty && \
	uv run ruff format && \
	uv run ruff check --fix

test:
	cargo test && \
	maturin develop && \
	uv run --all-groups pytest
