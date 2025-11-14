use alloc::vec::Vec;
use spin::Mutex;
use lazy_static::lazy_static;

/// Process ID type
pub type Pid = usize;

/// Maximum number of processes
const MAX_PROCESSES: usize = 16;

/// Process states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Ready,
    Running,
    Terminated,
}

/// Represents a process in the system
#[derive(Clone)]
pub struct Process {
    pub pid: Pid,
    pub name: alloc::string::String,
    pub state: ProcessState,
    pub code: Vec<u8>,
    pub entry_point: usize,
    pub stack_pointer: usize,
    pub instruction_pointer: usize,
}

impl Process {
    pub fn new(pid: Pid, name: alloc::string::String, code: Vec<u8>) -> Self {
        Process {
            pid,
            name,
            state: ProcessState::Ready,
            code,
            entry_point: 0,
            stack_pointer: 0,
            instruction_pointer: 0,
        }
    }
}

/// Process manager to handle process lifecycle
pub struct ProcessManager {
    processes: Vec<Process>,
    next_pid: Pid,
    current_pid: Option<Pid>,
}

impl ProcessManager {
    pub const fn new() -> Self {
        ProcessManager {
            processes: Vec::new(),
            next_pid: 1,
            current_pid: None,
        }
    }

    /// Create a new process
    pub fn create_process(&mut self, name: alloc::string::String, code: Vec<u8>) -> Result<Pid, &'static str> {
        if self.processes.len() >= MAX_PROCESSES {
            return Err("Process table full");
        }

        let pid = self.next_pid;
        self.next_pid += 1;

        let process = Process::new(pid, name, code);
        self.processes.push(process);

        Ok(pid)
    }

    /// Get a process by PID
    pub fn get_process(&self, pid: Pid) -> Option<&Process> {
        self.processes.iter().find(|p| p.pid == pid)
    }

    /// Get a mutable process by PID
    pub fn get_process_mut(&mut self, pid: Pid) -> Option<&mut Process> {
        self.processes.iter_mut().find(|p| p.pid == pid)
    }

    /// Terminate a process
    pub fn terminate_process(&mut self, pid: Pid) -> Result<(), &'static str> {
        if let Some(process) = self.get_process_mut(pid) {
            process.state = ProcessState::Terminated;
            Ok(())
        } else {
            Err("Process not found")
        }
    }

    /// List all active processes
    pub fn list_processes(&self) -> Vec<&Process> {
        self.processes.iter()
            .filter(|p| p.state != ProcessState::Terminated)
            .collect()
    }

    /// Set current running process
    pub fn set_current(&mut self, pid: Pid) {
        self.current_pid = Some(pid);
        if let Some(process) = self.get_process_mut(pid) {
            process.state = ProcessState::Running;
        }
    }

    /// Get current running process
    pub fn get_current(&self) -> Option<Pid> {
        self.current_pid
    }

    /// Clean up terminated processes
    pub fn cleanup(&mut self) {
        self.processes.retain(|p| p.state != ProcessState::Terminated);
    }
}

lazy_static! {
    pub static ref PROCESS_MANAGER: Mutex<ProcessManager> = Mutex::new(ProcessManager::new());
}
