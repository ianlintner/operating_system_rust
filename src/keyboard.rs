use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
        Keyboard::new(
            ScancodeSet1::new(),
            layouts::Us104Key,
            HandleControl::Ignore
        )
    );
    
    static ref KEY_BUFFER: Mutex<KeyBuffer> = Mutex::new(KeyBuffer::new());
}

struct KeyBuffer {
    buffer: [u8; 256],
    read_pos: usize,
    write_pos: usize,
}

impl KeyBuffer {
    const fn new() -> Self {
        KeyBuffer {
            buffer: [0; 256],
            read_pos: 0,
            write_pos: 0,
        }
    }

    fn push(&mut self, key: u8) {
        let next_write = (self.write_pos + 1) % self.buffer.len();
        if next_write != self.read_pos {
            self.buffer[self.write_pos] = key;
            self.write_pos = next_write;
        }
    }

    fn pop(&mut self) -> Option<u8> {
        if self.read_pos == self.write_pos {
            None
        } else {
            let key = self.buffer[self.read_pos];
            self.read_pos = (self.read_pos + 1) % self.buffer.len();
            Some(key)
        }
    }
}

pub fn handle_scancode(scancode: u8) {
    let mut keyboard = KEYBOARD.lock();
    
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    let mut buffer = KEY_BUFFER.lock();
                    buffer.push(character as u8);
                }
                DecodedKey::RawKey(_key) => {
                    // Handle special keys if needed
                }
            }
        }
    }
}

pub fn read_key() -> Option<u8> {
    let mut buffer = KEY_BUFFER.lock();
    buffer.pop()
}
