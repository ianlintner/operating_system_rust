#!/bin/bash
# Test Phase 2 features

# Start QEMU in the background with serial output
qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-operating_system_rust.bin -serial stdio -display none &
QEMU_PID=$!

# Wait for OS to boot
sleep 3

# Send commands to test features
echo "help" > /proc/$QEMU_PID/fd/0
sleep 1

echo "ls" > /proc/$QEMU_PID/fd/0
sleep 1

echo "mem" > /proc/$QEMU_PID/fd/0
sleep 1

echo "ps" > /proc/$QEMU_PID/fd/0
sleep 1

echo "run hello.bin" > /proc/$QEMU_PID/fd/0
sleep 1

# Kill QEMU
kill $QEMU_PID 2>/dev/null
wait $QEMU_PID 2>/dev/null

echo "Test complete"
