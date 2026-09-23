default:
	@echo 'Targets:'
	@echo '  test-default   Run tests with default settings.'
	@echo '  test-verbose   Run tests with verbose settings.'
	@echo '  doc            Generate documentation.'

test-default:
	cd practical-astronomy-rust-tests; cargo test

test-verbose:
	cd practical-astronomy-rust-tests; cargo test -v -- --nocapture

doc:
	rm -rf docs/rust; cd practical-astronomy-rust; cargo doc --document-private-items --target-dir ../docs/rust
