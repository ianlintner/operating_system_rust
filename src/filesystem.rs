use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;
use lazy_static::lazy_static;

/// Maximum number of files in the filesystem
const MAX_FILES: usize = 64;

/// Maximum file size (64 KB)
const MAX_FILE_SIZE: usize = 64 * 1024;

/// Represents a file in the filesystem
#[derive(Clone)]
pub struct File {
    pub name: String,
    pub content: Vec<u8>,
    pub size: usize,
}

impl File {
    pub fn new(name: String, content: Vec<u8>) -> Self {
        let size = content.len();
        File { name, content, size }
    }
}

/// Simple in-memory filesystem
pub struct FileSystem {
    files: Vec<File>,
}

impl FileSystem {
    pub const fn new() -> Self {
        FileSystem {
            files: Vec::new(),
        }
    }

    /// Create a new file or overwrite existing one
    pub fn create_file(&mut self, name: String, content: Vec<u8>) -> Result<(), &'static str> {
        if content.len() > MAX_FILE_SIZE {
            return Err("File too large");
        }

        // Remove existing file with same name
        self.files.retain(|f| f.name != name);

        if self.files.len() >= MAX_FILES {
            return Err("Filesystem full");
        }

        self.files.push(File::new(name, content));
        Ok(())
    }

    /// Read a file's content
    pub fn read_file(&self, name: &str) -> Option<&Vec<u8>> {
        self.files.iter()
            .find(|f| f.name == name)
            .map(|f| &f.content)
    }

    /// Delete a file
    pub fn delete_file(&mut self, name: &str) -> Result<(), &'static str> {
        let initial_len = self.files.len();
        self.files.retain(|f| f.name != name);
        
        if self.files.len() == initial_len {
            Err("File not found")
        } else {
            Ok(())
        }
    }

    /// List all files
    pub fn list_files(&self) -> &Vec<File> {
        &self.files
    }

    /// Check if a file exists
    pub fn file_exists(&self, name: &str) -> bool {
        self.files.iter().any(|f| f.name == name)
    }

    /// Get file size
    pub fn file_size(&self, name: &str) -> Option<usize> {
        self.files.iter()
            .find(|f| f.name == name)
            .map(|f| f.size)
    }
}

lazy_static! {
    pub static ref FILESYSTEM: Mutex<FileSystem> = Mutex::new(FileSystem::new());
}

/// Initialize the filesystem with some default programs
pub fn init_filesystem() {
    // Filesystem is ready, programs will be loaded at build time
    // via include_bytes! in the initializer
}

/// Load built-in programs into filesystem
pub fn load_builtin_programs() {
    let mut fs = FILESYSTEM.lock();
    
    // Create a simple "hello" program
    let hello_program: &[u8] = include_bytes!("../programs/hello.bin");
    let mut hello_vec = Vec::new();
    for &byte in hello_program {
        hello_vec.push(byte);
    }
    let _ = fs.create_file(
        String::from("hello.bin"),
        hello_vec
    );
    
    // Create a simple "count" program
    let count_program: &[u8] = include_bytes!("../programs/count.bin");
    let mut count_vec = Vec::new();
    for &byte in count_program {
        count_vec.push(byte);
    }
    let _ = fs.create_file(
        String::from("count.bin"),
        count_vec
    );
}
