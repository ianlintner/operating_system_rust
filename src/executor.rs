use crate::{print, println};
use alloc::vec::Vec;

/// Simple bytecode instruction set
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Instruction {
    // Basic operations
    Halt = 0x00,
    PrintChar = 0x01,      // Print character from next byte
    PrintString = 0x02,    // Print null-terminated string starting at next byte
    Loop = 0x03,           // Loop: next 2 bytes are count (little-endian), followed by instructions
    
    // Math operations
    Add = 0x10,            // Not implemented yet
    Sub = 0x11,            // Not implemented yet
    
    // System calls
    Syscall = 0xFF,        // System call with next byte as syscall number
}

/// System call numbers
#[repr(u8)]
pub enum Syscall {
    Exit = 0,
    Print = 1,
    Newline = 2,
}

/// Simple bytecode executor
pub struct Executor {
    code: Vec<u8>,
    ip: usize,  // instruction pointer
}

impl Executor {
    pub fn new(code: Vec<u8>) -> Self {
        Executor { code, ip: 0 }
    }

    /// Execute the bytecode program
    pub fn execute(&mut self) -> Result<(), &'static str> {
        while self.ip < self.code.len() {
            let instruction = self.code[self.ip];
            
            match instruction {
                0x00 => { // Halt
                    return Ok(());
                }
                
                0x01 => { // PrintChar
                    self.ip += 1;
                    if self.ip >= self.code.len() {
                        return Err("Unexpected end of program");
                    }
                    let ch = self.code[self.ip] as char;
                    print!("{}", ch);
                    self.ip += 1;
                }
                
                0x02 => { // PrintString
                    self.ip += 1;
                    let start = self.ip;
                    
                    // Find null terminator
                    while self.ip < self.code.len() && self.code[self.ip] != 0 {
                        self.ip += 1;
                    }
                    
                    if self.ip >= self.code.len() {
                        return Err("String not null-terminated");
                    }
                    
                    // Print string
                    let string_bytes = &self.code[start..self.ip];
                    if let Ok(s) = core::str::from_utf8(string_bytes) {
                        print!("{}", s);
                    }
                    
                    self.ip += 1; // Skip null terminator
                }
                
                0x03 => { // Loop
                    self.ip += 1;
                    if self.ip + 1 >= self.code.len() {
                        return Err("Unexpected end of program");
                    }
                    
                    // Read loop count (little-endian 16-bit)
                    let count = self.code[self.ip] as u16 | ((self.code[self.ip + 1] as u16) << 8);
                    self.ip += 2;
                    
                    let loop_start = self.ip;
                    
                    // Find end of loop (next Halt or end of program)
                    let mut depth = 1;
                    let mut loop_end = self.ip;
                    while loop_end < self.code.len() && depth > 0 {
                        if self.code[loop_end] == 0x03 {
                            depth += 1;
                        } else if self.code[loop_end] == 0x00 {
                            depth -= 1;
                        }
                        loop_end += 1;
                    }
                    
                    // Execute loop body
                    for _ in 0..count {
                        self.ip = loop_start;
                        while self.ip < loop_end && self.code[self.ip] != 0x00 {
                            let inner_instruction = self.code[self.ip];
                            match inner_instruction {
                                0x01 => { // PrintChar in loop
                                    self.ip += 1;
                                    if self.ip < self.code.len() {
                                        print!("{}", self.code[self.ip] as char);
                                    }
                                    self.ip += 1;
                                }
                                _ => {
                                    self.ip += 1;
                                }
                            }
                        }
                    }
                    
                    self.ip = loop_end;
                }
                
                0xFF => { // Syscall
                    self.ip += 1;
                    if self.ip >= self.code.len() {
                        return Err("Unexpected end of program");
                    }
                    
                    let syscall_num = self.code[self.ip];
                    match syscall_num {
                        0 => { // Exit
                            return Ok(());
                        }
                        1 => { // Print (next byte is character)
                            self.ip += 1;
                            if self.ip < self.code.len() {
                                print!("{}", self.code[self.ip] as char);
                            }
                            self.ip += 1;
                        }
                        2 => { // Newline
                            println!("");
                            self.ip += 1;
                        }
                        _ => {
                            return Err("Unknown syscall");
                        }
                    }
                }
                
                _ => {
                    return Err("Unknown instruction");
                }
            }
        }
        
        Ok(())
    }
}

/// Execute a program from bytecode
pub fn execute_program(code: Vec<u8>) -> Result<(), &'static str> {
    let mut executor = Executor::new(code);
    executor.execute()
}
