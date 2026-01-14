#!/usr/bin/env bash
# Generates recordings (GIF and MP4) for all core-animation examples.
#
# Usage: ./tools/update-example-screenshots.sh
#
# Outputs are saved to examples/screenshots/
#
# This script:
# 1. Applies a git patch that adds recording support
# 2. Runs all examples with the record feature
# 3. Reverts the patch (leaving working tree clean)
#
# Prerequisites:
# - ImageMagick (brew install imagemagick)
# - ffmpeg (brew install ffmpeg)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CRATE_DIR="$(dirname "$SCRIPT_DIR")"
PATCH_FILE="$SCRIPT_DIR/patches/recording-feature.patch"

cd "$CRATE_DIR"

# Ensure patch file exists
if [ ! -f "$PATCH_FILE" ]; then
    echo "Error: Recording patch not found at $PATCH_FILE"
    echo "This patch is required to add recording support to examples."
    exit 1
fi

# Cleanup function to ensure we always revert the patch
cleanup() {
    echo ""
    echo "Reverting recording patch..."
    git checkout -- . 2>/dev/null || true
    git clean -fd examples/common 2>/dev/null || true
    echo "Working tree restored to clean state."
}

# Trap to ensure cleanup runs even if script fails
trap cleanup EXIT

echo "Generating example recordings for core-animation..."
echo ""

# Apply the recording patch
echo "Applying recording patch..."
git apply "$PATCH_FILE"
echo "Patch applied successfully."
echo ""

# List of examples (hardcoded since they don't have the feature gate in clean state)
EXAMPLES="basic_layers breathing_circle emitter loading_spinner neon_glow particle_images point_burst ripple_rings staggered_dots window_builder"

echo "Examples to record:"
for example in $EXAMPLES; do
    echo "  - $example"
done
echo ""

# Ensure screenshots directory exists
mkdir -p examples/screenshots

# Build once with all features to warm up
echo "Building with recording support..."
cargo build --release --all-features
echo ""

for example in $EXAMPLES; do
    echo "Recording $example..."
    cargo run --release --example "$example" --features record
    echo ""
done

echo "Done! Recordings saved to examples/screenshots/"
echo ""
ls -la examples/screenshots/
