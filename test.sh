#!/bin/bash
# Test script to verify the OS boots in QEMU

set -e

echo "Building the OS..."
cargo bootimage

echo ""
echo "Testing the OS boot (will timeout after 5 seconds)..."
echo "Expected output: Welcome message and command prompt"
echo ""

# Run QEMU with a timeout, serial output, and no display
timeout 5s qemu-system-x86_64 \
    -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-operating_system_rust.bin \
    -serial stdio \
    -display none \
    -no-reboot \
    || true

echo ""
echo "Test complete!"
echo ""
echo "To run the OS interactively with display:"
echo "  make run-gui"
echo "or"
echo "  qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-operating_system_rust.bin"
