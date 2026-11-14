#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
export PATH="${HOME}/.cargo/bin:${PATH}"

MIN_COVERAGE=80
TARGET_DIR="${1:-target/coverage}"

echo "=== Checking code coverage (threshold: ${MIN_COVERAGE}%) ==="

if command -v cargo-llvm-cov >/dev/null 2>&1; then
    cargo llvm-cov --workspace --all-targets --lcov --output-path "${TARGET_DIR}/lcov.info"
    cargo llvm-cov report --fail-under-lines "${MIN_COVERAGE}"
elif command -v cargo-tarpaulin >/dev/null 2>&1; then
    cargo tarpaulin --workspace --all-targets --fail-under "${MIN_COVERAGE}" --out Xml --output-dir "${TARGET_DIR}"
else
    echo "[WARN] Neither cargo-llvm-cov nor cargo-tarpaulin found; running regular cargo test"
    cargo test --workspace --all-targets
fi

echo "=== Coverage check complete ==="
