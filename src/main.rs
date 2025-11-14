#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;

mod vga_buffer;
mod keyboard;
mod interrupts;
mod serial;
mod allocator;
mod filesystem;
mod process;
mod executor;
mod syscall;

/// Entry point for the kernel
#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("Bare Bones OS Phase 2 - Starting...");
    println!("Welcome to Bare Bones OS - Phase 2!");
    serial_println!("VGA buffer initialized");
    
    // Initialize memory allocator
    serial_println!("Initializing heap allocator...");
    allocator::init_heap();
    serial_println!("Heap allocator initialized");
    
    // Initialize filesystem
    serial_println!("Initializing filesystem...");
    filesystem::init_filesystem();
    filesystem::load_builtin_programs();
    serial_println!("Filesystem initialized");
    
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

/// Out of memory handler
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

mod shell {
    use crate::{print, println, filesystem, executor, process};
    use crate::keyboard;
    use alloc::string::String;
    use alloc::vec::Vec;
    
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
                println!("  help       - Show this help message");
                println!("  echo       - Echo back the arguments");
                println!("  clear      - Clear the screen");
                println!("  ls         - List files in filesystem");
                println!("  cat <file> - Display file contents (hex)");
                println!("  run <file> - Execute a binary program");
                println!("  ps         - List running processes");
                println!("  mem        - Show memory info");
                println!("  write <file> <text> - Create a text file");
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
            "ls" => {
                let fs = filesystem::FILESYSTEM.lock();
                let files = fs.list_files();
                if files.is_empty() {
                    println!("No files in filesystem");
                } else {
                    println!("Files:");
                    for file in files {
                        println!("  {} ({} bytes)", file.name, file.size);
                    }
                }
            }
            "cat" => {
                if count < 2 {
                    println!("Usage: cat <filename>");
                } else {
                    let filename = parts_vec[1];
                    let fs = filesystem::FILESYSTEM.lock();
                    if let Some(content) = fs.read_file(filename) {
                        println!("File: {} ({} bytes)", filename, content.len());
                        print!("Content (hex): ");
                        for (i, byte) in content.iter().enumerate() {
                            if i > 0 && i % 16 == 0 {
                                println!("");
                                print!("               ");
                            }
                            print!("{:02x} ", byte);
                        }
                        println!("");
                    } else {
                        println!("File not found: {}", filename);
                    }
                }
            }
            "run" => {
                if count < 2 {
                    println!("Usage: run <filename>");
                } else {
                    let filename = parts_vec[1];
                    
                    // Clone the content before releasing the lock
                    let content_option = {
                        let fs = filesystem::FILESYSTEM.lock();
                        fs.read_file(filename).map(|c| c.clone())
                    };
                    
                    if let Some(content) = content_option {
                        println!("Executing: {}", filename);
                        
                        // Create a process
                        let mut pm = process::PROCESS_MANAGER.lock();
                        let mut name = String::new();
                        for c in filename.chars() {
                            name.push(c);
                        }
                        match pm.create_process(name, content.clone()) {
                            Ok(pid) => {
                                pm.set_current(pid);
                                drop(pm);
                                
                                // Execute the program
                                match executor::execute_program(content.clone()) {
                                    Ok(_) => {
                                        println!("");
                                        let mut pm = process::PROCESS_MANAGER.lock();
                                        let _ = pm.terminate_process(pid);
                                    }
                                    Err(e) => {
                                        println!("");
                                        println!("Execution error: {}", e);
                                        let mut pm = process::PROCESS_MANAGER.lock();
                                        let _ = pm.terminate_process(pid);
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Failed to create process: {}", e);
                            }
                        }
                    } else {
                        println!("File not found: {}", filename);
                    }
                }
            }
            "ps" => {
                let pm = process::PROCESS_MANAGER.lock();
                let processes = pm.list_processes();
                if processes.is_empty() {
                    println!("No active processes");
                } else {
                    println!("Active processes:");
                    for proc in processes {
                        println!("  PID {} - {} ({:?})", proc.pid, proc.name, proc.state);
                    }
                }
            }
            "mem" => {
                println!("Memory Information:");
                println!("  Heap start: 0x{:x}", crate::allocator::get_heap_start());
                println!("  Heap size:  {} KB", crate::allocator::HEAP_SIZE / 1024);
            }
            "write" => {
                if count < 3 {
                    println!("Usage: write <filename> <text>");
                } else {
                    let filename = parts_vec[1];
                    let mut text = String::new();
                    for i in 2..count {
                        if i > 2 {
                            text.push(' ');
                        }
                        text.push_str(parts_vec[i]);
                    }
                    
                    let mut fs = filesystem::FILESYSTEM.lock();
                    let mut fname = String::new();
                    for c in filename.chars() {
                        fname.push(c);
                    }
                    let mut content_vec = Vec::new();
                    for &byte in text.as_bytes() {
                        content_vec.push(byte);
                    }
                    match fs.create_file(fname, content_vec) {
                        Ok(_) => println!("File created: {}", filename),
                        Err(e) => println!("Error: {}", e),
                    }
                }
            }
            "" => {}
            _ => {
                println!("Unknown command: {}", command);
                println!("Type 'help' for available commands");
            }
        }
    }
}
