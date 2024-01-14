##	:
##	make cargo-*
##	:
cargo-help:### 	cargo-help
	@awk 'BEGIN {FS = ":.*?###"} /^[a-zA-Z_-]+:.*?###/ {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
##:cargo-help
##	:
cargo-build:### 	cargo build
##:cargo-build
## 	make cargo-build q=1
##	:
	#@. $(HOME)/.cargo/env || true
##	:
	@RUST_BACKTRACE=all cargo b $(QUIET)
##	:
cargo-install:### 	cargo install --path .
##:cargo-install
#@. $(HOME)/.cargo/env || true
	#@cargo install --path $(PWD)
	@cargo install --locked --path $(PWD)
	@$(MAKE) cargo-install-gnostr-db-cli

cargo-install-gnostr-db-cli:### 	cargo install gnostr-db-cli
	cd gnostr-db-cli && cargo install --path .

##	:
cargo-b-release:cargo-build-release
cargo-build-release:### 	cargo-build-release
##:cargo-build-release
## 	make cargo-build-release q=1
	#@. $(HOME)/.cargo/env || true
##	cargo b --profile=<release-with-debug>
	@cargo b $(QUIET) --profile=$(PROFILE)

##	:
cargo-br:cargo-build-release### 	cargo-br
##:cargo-br
## 	make cargo-br q=$(QUIET)

##	:
cargo-check:### 	cargo-check
##:cargo-check
## cargo c
	#@. $(HOME)/.cargo/env || true
	@cargo c

##	:
cargo-bench:### 	cargo-bench
##:cargo-bench
## cargo b
	#@. $(HOME)/.cargo/env || true
	@cargo bench

##	:
cargo-test:### 	cargo-test
##:cargo-test
## cargo t
	#@. $(HOME)/.cargo/env || true
	@cargo test

##	:
cargo-profile-dev:### 	cargo-profile-dev
##:cargo-profile-dev
## cargo b --release profile=dev
	#@. $(HOME)/.cargo/env || true
	$(MAKE) cargo-br profile=dev && ./target/debug/gnostr-db nost README.md
	$(MAKE) cargo-br profile=dev && ./target/debug/gnostr-db to poem.txt
##	:
cargo-profile-release:### 	cargo-profile-release
##:cargo-profile-release
## cargo b --release profile=release
	#@. $(HOME)/.cargo/env || true
	$(MAKE) cargo-br profile=release && ./target/release/gnostr-db nost README.md
	$(MAKE) cargo-br profile=release && ./target/release/gnostr-db to poem.txt

##	:
cargo-report:### 	cargo-report
##:cargo-report
	#@. $(HOME)/.cargo/env || true
	cargo report future-incompatibilities --id 1

##	:
cargo-doc:### 	cargo-doc
##:cargo-doc
	@cargo doc --no-deps --open
##	:

# vim: set noexpandtab:
# vim: set setfiletype make
