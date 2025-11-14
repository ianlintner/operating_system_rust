#!/bin/bash
# Demo script showing OS features

cat << 'EOF'
===============================================
  Bare Bones Operating System in Rust - Demo
===============================================

This operating system demonstrates:

✓ Bootable x86_64 OS written in Rust
✓ VGA text mode output (80x25 characters)
✓ Keyboard input with PS/2 driver
✓ Interrupt handling (keyboard, timer)
✓ Command-line interface (CLI)
✓ Simple command parsing

Available Commands:
-------------------
  help  - Show available commands
  echo  - Echo back arguments (e.g., "echo Hello World")
  clear - Clear the screen

Build Instructions:
-------------------
  1. cargo bootimage     # Build bootable image
  2. make run-gui        # Run with QEMU GUI
  
Or manually:
  qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-operating_system_rust.bin

The OS will display:
-------------------
  Welcome to Bare Bones OS!
  Type 'help' for available commands
  
  > 

Then you can type commands like:
  > help
  > echo Hello from Rust OS!
  > clear

Technical Details:
------------------
  - No standard library (no_std)
  - Bare metal Rust on x86_64
  - VGA buffer at 0xB8000
  - Interrupt Descriptor Table (IDT)
  - PS/2 keyboard scancode handling
  - Simple ring buffer for keyboard input

File Structure:
--------------
  src/main.rs         - Kernel entry point & shell
  src/vga_buffer.rs   - VGA text mode driver
  src/interrupts.rs   - IDT and interrupt handlers
  src/keyboard.rs     - Keyboard driver
  src/serial.rs       - Serial port output (for debugging)

===============================================
EOF
