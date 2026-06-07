.PHONY: help fmt check lint test test-minimal build doc examples audit deny sbom publish-dry-run-independent publish-dry-run-base-dependent publish-dry-run-composed-dependent publish-dry-run-dependent publish-dry-run-focused publish-dry-run-facade release-readiness base-dependent-post-publish-validation composed-dependent-post-publish-validation dependent-post-publish-validation facade-post-publish-validation verify

INDEPENDENT_CRATES := use-currency use-amount use-routing-number use-bank-account use-ach use-bic use-iban
BASE_DEPENDENT_CRATES := use-money use-transaction use-reconciliation
COMPOSED_DEPENDENT_CRATES := use-ledger use-payment use-receipt use-invoice use-bai2
DEPENDENT_CRATES := $(BASE_DEPENDENT_CRATES) $(COMPOSED_DEPENDENT_CRATES)
FOCUSED_CRATES := $(INDEPENDENT_CRATES) $(DEPENDENT_CRATES)
FACADE_CRATE := use-finance

help:
	@printf "%s\n" \
		"help                           Show available repository tasks" \
		"fmt                            Check formatting with rustfmt" \
		"check                          Run cargo check for the workspace" \
		"lint                           Run clippy with warnings denied" \
		"test                           Run workspace tests with all features" \
		"test-minimal                   Run workspace tests with no default features" \
		"build                          Build the workspace with all features" \
		"doc                            Build workspace docs without dependencies" \
		"examples                       Check all examples" \
		"audit                          Run cargo-audit" \
		"deny                           Run cargo-deny" \
		"sbom                           Generate a CycloneDX SBOM for $(FACADE_CRATE)" \
		"publish-dry-run-independent    Wave 1: dry-run independent crates" \
		"publish-dry-run-base-dependent Wave 2: dry-run base dependent crates after Wave 1 is live" \
		"publish-dry-run-composed-dependent Wave 3: dry-run composed dependent crates after Wave 2 is live" \
		"publish-dry-run-dependent      Print staged dependent-crate guidance" \
		"publish-dry-run-focused        Pre-publication Wave 1 readiness path" \
		"publish-dry-run-facade         Dry-run publish $(FACADE_CRATE) after crates.io propagation" \
		"release-readiness              Run the pre-release focused-crate validation path" \
		"base-dependent-post-publish-validation Dry-run Wave 2 after Wave 1 is live" \
		"composed-dependent-post-publish-validation Dry-run Wave 3 after Wave 2 is live" \
		"dependent-post-publish-validation Dry-run Wave 2 and print next-wave guidance" \
		"facade-post-publish-validation Dry-run the facade crate after focused crates are live" \
		"verify                         Run the main workspace validation path"

fmt:
	cargo fmt --all -- --check

check:
	cargo check --workspace --all-features

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace --all-features

test-minimal:
	cargo test --workspace --no-default-features

build:
	cargo build --workspace --all-features

doc:
	cargo doc --workspace --all-features --no-deps

examples:
	cargo check --workspace --all-features --examples

audit:
	cargo audit

deny:
	cargo deny check

sbom:
	cargo cyclonedx --manifest-path crates/$(FACADE_CRATE)/Cargo.toml --all-features --format json --spec-version 1.5 --override-filename sbom.cyclonedx

publish-dry-run-independent:
	@for crate in $(INDEPENDENT_CRATES); do \
		cargo package --allow-dirty --list -p $$crate; \
		cargo publish --dry-run --allow-dirty -p $$crate; \
	done

publish-dry-run-base-dependent:
	@for crate in $(BASE_DEPENDENT_CRATES); do \
		cargo package --allow-dirty --list -p $$crate; \
		cargo publish --dry-run --allow-dirty -p $$crate; \
	done

publish-dry-run-composed-dependent:
	@for crate in $(COMPOSED_DEPENDENT_CRATES); do \
		cargo package --allow-dirty --list -p $$crate; \
		cargo publish --dry-run --allow-dirty -p $$crate; \
	done

publish-dry-run-dependent:
	@printf "%s\n" "Run publish-dry-run-base-dependent after Wave 1 is live: $(BASE_DEPENDENT_CRATES)"
	@printf "%s\n" "Run publish-dry-run-composed-dependent after Wave 2 is live: $(COMPOSED_DEPENDENT_CRATES)"

publish-dry-run-focused: publish-dry-run-independent
	@printf "%s\n" "Deferred Wave 2 dry-runs until Wave 1 crates are live on crates.io: $(BASE_DEPENDENT_CRATES)"
	@printf "%s\n" "Deferred Wave 3 dry-runs until Wave 2 crates are live on crates.io: $(COMPOSED_DEPENDENT_CRATES)"

publish-dry-run-facade:
	cargo package --allow-dirty --list -p $(FACADE_CRATE)
	cargo publish --dry-run --allow-dirty -p $(FACADE_CRATE)

release-readiness: verify examples test-minimal publish-dry-run-focused

base-dependent-post-publish-validation: publish-dry-run-base-dependent

composed-dependent-post-publish-validation: publish-dry-run-composed-dependent

dependent-post-publish-validation: base-dependent-post-publish-validation
	@printf "%s\n" "After publishing and verifying Wave 2, run composed-dependent-post-publish-validation."

facade-post-publish-validation: publish-dry-run-facade

verify: fmt lint test build
