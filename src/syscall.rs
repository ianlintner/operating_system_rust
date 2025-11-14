/// System call interface for user programs
use crate::{print, println};

/// System call numbers
#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum SyscallNumber {
    Exit = 0,
    Write = 1,
    Read = 2,
    Open = 3,
    Close = 4,
}

/// Execute a system call
pub fn syscall(number: usize, arg1: usize, arg2: usize, _arg3: usize) -> isize {
    match number {
        0 => { // Exit
            // In a real OS, this would terminate the process
            0
        }
        1 => { // Write
            // arg1: file descriptor (1 = stdout)
            // arg2: pointer to data
            // In simplified version, we just print
            if arg1 == 1 {
                if arg2 < 128 {
                    print!("{}", arg2 as u8 as char);
                    1
                } else {
                    -1
                }
            } else {
                -1
            }
        }
        2 => { // Read
            // Not implemented yet
            -1
        }
        _ => {
            -1
        }
    }
}

/// Helper function to print a string via syscall interface
pub fn sys_write(fd: usize, data: &str) -> isize {
    if fd == 1 {
        print!("{}", data);
        data.len() as isize
    } else {
        -1
    }
}

/// Helper function to print newline
pub fn sys_newline() {
    println!("");
}
