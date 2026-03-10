.PHONY: build test clean deploy fmt lint

build:
	anchor build

test:
	anchor test

clean:
	anchor clean
	find . -name 'node_modules' -type d -prune -exec rm -rf {} +

deploy:
	anchor deploy --provider.cluster devnet

fmt:
	cargo fmt --all

lint:
	cargo clippy --all-targets -- -D warnings
