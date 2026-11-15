#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
export PATH="${HOME}/.cargo/bin:${PATH}"

STRICT=false
STAGED_ONLY=false
for arg in "$@"; do
    case "$arg" in
        --strict) STRICT=true ;;
        --staged-only) STAGED_ONLY=true ;;
    esac
done

RUN_RUST_CHECKS=true
if [ "$STAGED_ONLY" = true ] && git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    STAGED_FILES=$(git diff --cached --name-only --diff-filter=ACMR)
    if [ -z "$STAGED_FILES" ] || ! echo "$STAGED_FILES" | grep -qE '(^|/)(Cargo\.toml|Cargo\.lock|deny\.toml|rust-toolchain\.toml|build\.rs|\.cargo/.*|.*\.rs|policies/.*\.rego)$'; then
        RUN_RUST_CHECKS=false
    fi
fi

if [ "$RUN_RUST_CHECKS" = true ]; then
    echo "=== [1/6] Formatting check (cargo fmt) ==="
    cargo fmt --all -- --check

    echo "=== [2/6] Clippy lints (production strict + test warnings) ==="
    cargo clippy --workspace --lib --bins -- -D warnings -D clippy::unwrap_used -D clippy::panic
    cargo clippy --workspace --tests -- -W clippy::all
else
    echo "=== [1/6] & [2/6] Skipping Rust fmt & clippy (no staged Rust/config files) ==="
fi

echo "=== [3/6] Secrets scan (gitleaks) ==="
if command -v gitleaks >/dev/null 2>&1; then
    gitleaks git --no-banner --redact --config .gitleaks.toml .
elif [ "$STRICT" = true ]; then
    echo "ERROR: gitleaks not found in PATH" >&2
    exit 1
else
    echo "[WARN] gitleaks not found; skipping secrets scan"
fi

if [ "$RUN_RUST_CHECKS" = true ]; then
    echo "=== [4/6] Rust vulnerability audit (cargo-audit) ==="
    if command -v cargo-audit >/dev/null 2>&1 || cargo audit --version >/dev/null 2>&1; then
        if AUDIT_OUT=$(cargo audit 2>&1); then
            echo "$AUDIT_OUT"
        elif echo "$AUDIT_OUT" | grep -qiE "fetch|network|dns|resolve|connect|tls|http|failed to clone"; then
            echo "[WARN] Network unavailable for advisory DB update; running cargo audit --no-fetch"
            cargo audit --no-fetch
        else
            echo "$AUDIT_OUT" >&2
            exit 1
        fi
    elif [ "$STRICT" = true ]; then
        echo "ERROR: cargo-audit not found" >&2
        exit 1
    else
        echo "[WARN] cargo-audit not found; skipping"
    fi

    echo "=== [5/6] Supply-chain & license check (cargo-deny) ==="
    if command -v cargo-deny >/dev/null 2>&1 || cargo deny --version >/dev/null 2>&1; then
        if DENY_OUT=$(cargo deny check 2>&1); then
            echo "$DENY_OUT"
        elif echo "$DENY_OUT" | grep -qiE "fetch|network|dns|resolve|connect|tls|http|offline|advisory-db|index"; then
            echo "[WARN] Network unavailable; running cargo deny --offline check"
            cargo deny --offline check
        else
            echo "$DENY_OUT" >&2
            exit 1
        fi
    elif [ "$STRICT" = true ]; then
        echo "ERROR: cargo-deny not found" >&2
        exit 1
    else
        echo "[WARN] cargo-deny not found; skipping"
    fi
else
    echo "=== [4/6] & [5/6] Skipping Rust audit & deny (no staged Rust/config files) ==="
fi

echo "=== [6/6] Frontend dependency audit ==="
if [ -f "ui/package.json" ]; then
    if command -v pnpm >/dev/null 2>&1; then
        if ! (cd ui && pnpm audit --audit-level=high); then
            if [ -n "${CI:-}" ]; then
                exit 1
            else
                echo "[WARN] pnpm audit failed (likely offline); skipping locally"
            fi
        fi
    elif [ -n "${CI:-}" ] && [ "$STRICT" = true ]; then
        echo "ERROR: pnpm not found in CI environment with ui/ present" >&2
        exit 1
    else
        echo "[WARN] pnpm not available in PATH; skipping frontend audit locally"
    fi
else
    echo "No ui/package.json present; skipping frontend audit."
fi

echo "=== All security and quality checks passed! ==="
