#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
HOOKS_DIR="${REPO_ROOT}/.git/hooks"

mkdir -p "${HOOKS_DIR}"

cat > "${HOOKS_DIR}/pre-commit" << 'EOF'
#!/usr/bin/env bash
set -euo pipefail

echo "[pre-commit] Running strict security and quality gate..."
"$(git rev-parse --show-toplevel)/scripts/check-security.sh"
EOF

chmod +x "${HOOKS_DIR}/pre-commit"
echo "Installed pre-commit hook at ${HOOKS_DIR}/pre-commit"
