#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;
mod keyboard;
mod interrupts;
mod serial;

/// Entry point for the kernel
#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("Bare Bones OS - Starting...");
    println!("Welcome to Bare Bones OS!");
    serial_println!("VGA buffer initialized");
    println!("Type 'help' for available commands");
    println!("");
    
    // Initialize interrupts and keyboard
    serial_println!("Initializing interrupts...");
    interrupts::init();
    serial_println!("Interrupts initialized");
    
    // Start the command line interface
    serial_println!("Starting command line shell...");
    serial_println!("OS is ready for input!");
    shell::run();
    
    loop {
        x86_64::instructions::hlt();
    }
}

/// Panic handler for the kernel
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("PANIC: {}", info);
    println!("{}", info);
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

mod shell {
    use crate::{print, println};
    use crate::keyboard;
    
    const MAX_INPUT_LEN: usize = 128;
    
    pub fn run() -> ! {
        let mut input_buffer = [0u8; MAX_INPUT_LEN];
        let mut input_len = 0;
        
        print!("> ");
        
        loop {
            if let Some(key) = keyboard::read_key() {
                match key {
                    b'\n' => {
                        println!("");
                        
                        // Process the command
                        if input_len > 0 {
                            let command = core::str::from_utf8(&input_buffer[..input_len])
                                .unwrap_or("");
                            process_command(command);
                            
                            // Clear buffer
                            input_len = 0;
                        }
                        
                        print!("> ");
                    }
                    8 => { // Backspace
                        if input_len > 0 {
                            input_len -= 1;
                            print!("{}", 8 as char);
                        }
                    }
                    c if c >= 32 && c < 127 => { // Printable ASCII
                        if input_len < MAX_INPUT_LEN - 1 {
                            input_buffer[input_len] = c;
                            input_len += 1;
                            print!("{}", c as char);
                        }
                    }
                    _ => {}
                }
            }
            
            x86_64::instructions::hlt();
        }
    }
    
    fn process_command(cmd: &str) {
        let mut parts_vec: [&str; 16] = [""; 16];
        let mut count = 0;
        
        for part in cmd.split(' ') {
            if !part.is_empty() && count < 16 {
                parts_vec[count] = part;
                count += 1;
            }
        }
        
        if count == 0 {
            return;
        }
        
        let command = parts_vec[0];
        
        match command {
            "help" => {
                println!("Available commands:");
                println!("  help  - Show this help message");
                println!("  echo  - Echo back the arguments");
                println!("  clear - Clear the screen");
            }
            "echo" => {
                // Echo all arguments after the command
                for i in 1..count {
                    if i > 1 {
                        print!(" ");
                    }
                    print!("{}", parts_vec[i]);
                }
                println!("");
            }
            "clear" => {
                crate::vga_buffer::clear_screen();
            }
            "" => {}
            _ => {
                println!("Unknown command: {}", command);
                println!("Type 'help' for available commands");
            }
        }
    }
}
