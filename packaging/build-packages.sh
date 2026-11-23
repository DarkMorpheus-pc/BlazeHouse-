#!/usr/bin/env bash
# Blaze House modern Tauri uygulamasını ISO'ya uygun .deb/.rpm olarak paketler.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
UI_DIR="$PROJECT_ROOT/modern-ui"
OUT_DIR="$PROJECT_ROOT/dist"
FORMAT="${1:-both}"

command -v npm >/dev/null 2>&1 || { echo "HATA: npm bulunamadı"; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo "HATA: cargo bulunamadı"; exit 1; }

case "$FORMAT" in
  deb) BUNDLES="deb" ;;
  rpm) BUNDLES="rpm" ;;
  both) BUNDLES="deb,rpm" ;;
  *) echo "Kullanım: $0 [deb|rpm|both]"; exit 2 ;;
esac

mkdir -p "$OUT_DIR"
cd "$UI_DIR"
npm ci
npm run lint
npm run tauri:build -- --bundles "$BUNDLES"

find "$UI_DIR/src-tauri/target/release/bundle" -type f \( -name '*.deb' -o -name '*.rpm' \) -exec cp -f {} "$OUT_DIR/" \;
echo "Paketler hazır:"
find "$OUT_DIR" -maxdepth 1 -type f \( -name '*.deb' -o -name '*.rpm' \) -print
