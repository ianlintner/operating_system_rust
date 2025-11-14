# Bare Bones Operating System in Rust - Phase 2

A minimal operating system written in Rust that boots and provides a functional command-line interface with the ability to load and execute binary programs.

## Features

### Phase 1 (Complete)
- **Bootable**: Uses bootloader crate for BIOS/UEFI boot support
- **VGA Text Mode**: Simple text output using VGA buffer at 0xB8000
- **Keyboard Input**: Basic PS/2 keyboard driver with interrupt handling
- **Command Line Interface**: Interactive shell with command parsing
- **Basic Commands**: help, echo, clear

### Phase 2 (NEW!)
- **Memory Allocator**: Dynamic memory allocation with 100KB heap
- **Filesystem**: In-memory filesystem for storing and managing files
- **Process Manager**: Process abstraction with state tracking
- **Program Executor**: Custom bytecode interpreter for running programs
- **Binary Execution**: Load and run compiled programs
- **Enhanced Shell**: 8 commands including file operations and program execution
- **Sample Programs**: Built-in hello.bin and count.bin programs

## Available Commands

- `help` - Show available commands
- `echo <text>` - Echo back the provided text
- `clear` - Clear the screen
- `ls` - List all files in the filesystem
- `cat <file>` - Display file contents in hexadecimal
- `run <file>` - Execute a binary program
- `ps` - List active processes
- `mem` - Display memory information
- `write <file> <text>` - Create a text file

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

Or use the Makefile:
```bash
make run-gui  # Run with GUI
make run      # Run with serial output only
```

## Quick Start Example

Once the OS boots, try these commands:
```
> ls
Files:
  hello.bin (18 bytes)
  count.bin (21 bytes)

> run hello.bin
Executing: hello.bin
Hello, World!

> run count.bin
Executing: count.bin
Counting: **********

> write myfile.txt Hello Phase 2
File created: myfile.txt

> ls
Files:
  hello.bin (18 bytes)
  count.bin (21 bytes)
  myfile.txt (13 bytes)
```

## Architecture

### Boot Process
1. Bootloader (bootloader crate) loads the kernel
2. Kernel entry point `_start()` initializes the system
3. Memory allocator is initialized (100KB heap)
4. Filesystem is initialized and programs are loaded
5. VGA buffer is set up for text output
6. Interrupt handlers are configured (keyboard, timer)
7. Command line shell starts

### Components

#### Phase 1 Components
- **main.rs**: Kernel entry point and shell implementation
- **vga_buffer.rs**: VGA text mode driver for screen output
- **interrupts.rs**: Interrupt descriptor table and handlers
- **keyboard.rs**: PS/2 keyboard driver with scancode handling
- **serial.rs**: Serial port driver for debugging

#### Phase 2 Components (NEW!)
- **allocator.rs**: Heap allocator for dynamic memory
- **filesystem.rs**: In-memory filesystem implementation
- **process.rs**: Process management and tracking
- **executor.rs**: Bytecode program executor
- **syscall.rs**: System call interface

### Memory Layout

- VGA Text Buffer: `0xB8000`
- Kernel loaded by bootloader into higher half
- Heap: 100KB static allocation for dynamic memory

## Program Execution

Phase 2 introduces the ability to execute binary programs using a custom bytecode format:

### Bytecode Instructions
- `0x00` - Halt execution
- `0x01` - Print character (next byte)
- `0x02` - Print null-terminated string
- `0x03` - Loop (next 2 bytes = count)
- `0xFF` - System call

See `PHASE2_GUIDE.md` for detailed documentation on creating custom programs.

## Educational Purpose

This OS demonstrates:
- Bare metal Rust programming (no_std)
- Hardware interaction (VGA, keyboard)
- Interrupt handling
- Basic system architecture
- **Dynamic memory allocation** (NEW!)
- **File system concepts** (NEW!)
- **Process management** (NEW!)
- **Program loading and execution** (NEW!)

Built with AI assistance as an educational demonstration of OS development.

## Documentation

- `README.md` - This file, general overview
- `PHASE2_GUIDE.md` - Detailed Phase 2 architecture and usage guide
- `PROJECT_SUMMARY.md` - Complete project summary
- `VISUAL_GUIDE.md` - Visual reference guide