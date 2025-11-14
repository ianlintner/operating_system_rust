# Bare Bones Operating System in Rust

A minimal operating system written in Rust that boots and provides a basic command-line interface with an echo program.

## Features

- **Bootable**: Uses bootloader crate for BIOS/UEFI boot support
- **VGA Text Mode**: Simple text output using VGA buffer at 0xB8000
- **Keyboard Input**: Basic PS/2 keyboard driver with interrupt handling
- **Command Line Interface**: Interactive shell with command parsing
- **Echo Command**: Simple echo program to demonstrate command execution
- **Written in Rust**: Maximum use of Rust with minimal assembly (handled by bootloader)

## Available Commands

- `help` - Show available commands
- `echo <text>` - Echo back the provided text
- `clear` - Clear the screen

## Prerequisites

- Rust nightly toolchain
- QEMU (for testing/running the OS)
- cargo-bootimage (for building bootable images)

## Building

1. Install Rust nightly with required components:
```bash
rustup override set nightly
rustup component add rust-src llvm-tools-preview
```

2. Install cargo-bootimage:
```bash
cargo install bootimage
```

3. Build the OS:
```bash
cargo bootimage
```

This will create a bootable disk image at `target/x86_64-unknown-none/debug/bootimage-operating_system_rust.bin`

## Running

Run the OS in QEMU:
```bash
cargo run
```

Or manually with QEMU:
```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-operating_system_rust.bin
```

## Architecture

### Boot Process
1. Bootloader (bootloader crate) loads the kernel
2. Kernel entry point `_start()` initializes the system
3. VGA buffer is set up for text output
4. Interrupt handlers are configured (keyboard, timer)
5. Command line shell starts

### Components

- **main.rs**: Kernel entry point and shell implementation
- **vga_buffer.rs**: VGA text mode driver for screen output
- **interrupts.rs**: Interrupt descriptor table and handlers
- **keyboard.rs**: PS/2 keyboard driver with scancode handling

### Memory Layout

- VGA Text Buffer: `0xB8000`
- Kernel loaded by bootloader into higher half

## Educational Purpose

This OS demonstrates:
- Bare metal Rust programming (no_std)
- Hardware interaction (VGA, keyboard)
- Interrupt handling
- Basic system architecture

Built with AI assistance as an educational demonstration of OS development.