#!/usr/bin/env bash
# Geriye dönük uyumluluk: modern Tauri paketleyicisini kullanır.
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$SCRIPT_DIR/build-packages.sh" "${1:-both}"
