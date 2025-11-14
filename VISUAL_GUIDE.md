# Visual Representation of the OS

When you boot the OS in QEMU, you'll see:

```
┌────────────────────────────────────────────────────────────────────────────────┐
│ Welcome to Bare Bones OS!                                                      │
│ Type 'help' for available commands                                             │
│                                                                                 │
│ >                                                                               │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
└────────────────────────────────────────────────────────────────────────────────┘
```

## Example Session

```
┌────────────────────────────────────────────────────────────────────────────────┐
│ Welcome to Bare Bones OS!                                                      │
│ Type 'help' for available commands                                             │
│                                                                                 │
│ > help                                                                          │
│ Available commands:                                                             │
│   help  - Show this help message                                               │
│   echo  - Echo back the arguments                                              │
│   clear - Clear the screen                                                     │
│ >                                                                               │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
└────────────────────────────────────────────────────────────────────────────────┘
```

```
┌────────────────────────────────────────────────────────────────────────────────┐
│ Welcome to Bare Bones OS!                                                      │
│ Type 'help' for available commands                                             │
│                                                                                 │
│ > echo Hello from Rust OS!                                                     │
│ Hello from Rust OS!                                                            │
│ > echo This is a minimal operating system                                      │
│ This is a minimal operating system                                             │
│ > echo Built with AI assistance                                                │
│ Built with AI assistance                                                       │
│ >                                                                               │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
│                                                                                 │
└────────────────────────────────────────────────────────────────────────────────┘
```

## Features in Action

### Command: `clear`
Clears the entire screen and returns cursor to top

### Command: `echo <text>`
- Splits input by spaces
- Echoes back all arguments after the command
- Supports multiple words

### Command: `help`
- Shows list of available commands
- Brief description of each command

## Keyboard Support

- Full ASCII character input (a-z, A-Z, 0-9, symbols)
- Backspace support
- Enter to execute commands
- PS/2 keyboard scancode translation

## Technical Implementation

### Boot Process
1. **BIOS/UEFI** loads bootloader from MBR
2. **Bootloader** (bootloader crate) loads kernel into memory
3. **Kernel** `_start()` function executes
4. VGA buffer initialized at `0xB8000`
5. Interrupts configured (IDT)
6. Shell starts and waits for input

### Memory Map
- VGA Text Buffer: `0xB8000` (80x25 characters)
- Kernel loaded by bootloader into higher half
- Stack grows downward from kernel

### Interrupts
- IRQ 0: Timer (configured but minimal handling)
- IRQ 1: Keyboard (captures scancodes)
- IDT configured with x86-interrupt handlers
- PIC (8259) configured with offsets 32 and 40
