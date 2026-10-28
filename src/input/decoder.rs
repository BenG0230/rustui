#[derive(Debug)]
pub struct KeyEvent {
    pub code: KeyCode,
}

#[derive(Debug)]
pub enum KeyCode {
    Char(char),
    Space,
    Enter,
    Escape,
    Up,
    Down,
    Left,
    Right,
}

pub(super) struct Decoder {
    bytes: Vec<u8>,
}

impl Decoder {
    pub const fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    pub fn push_bytes(&mut self, buf: &[u8]) {
        self.bytes.extend_from_slice(buf);
    }

    pub fn get_events(&mut self) -> Vec<KeyEvent> {
        let mut events: Vec<KeyEvent> = Vec::new();

        let mut byte_iter = self.bytes.iter().peekable();

        while let Some(byte) = byte_iter.next() {
            match byte {
                b'a'..=b'z' => events.push(KeyEvent {
                    code: KeyCode::Char(*byte as char),
                }),
                b'A'..=b'Z' => events.push(KeyEvent {
                    code: KeyCode::Char(*byte as char),
                }),
                b'0'..=b'9' => events.push(KeyEvent {
                    code: KeyCode::Char(*byte as char),
                }),
                13 => events.push(KeyEvent {
                    code: KeyCode::Enter,
                }),
                27 => match byte_iter.peek() {
                    Some(b'[') => {
                        // escape sequence :O
                        byte_iter.next();
                        match byte_iter.next() {
                            Some(b'A') => events.push(KeyEvent { code: KeyCode::Up }),
                            Some(b'B') => events.push(KeyEvent {
                                code: KeyCode::Down,
                            }),
                            Some(b'C') => events.push(KeyEvent {
                                code: KeyCode::Right,
                            }),
                            Some(b'D') => events.push(KeyEvent {
                                code: KeyCode::Left,
                            }),
                            _ => {}
                        }
                    }
                    _ => events.push(KeyEvent {
                        code: KeyCode::Escape,
                    }),
                },
                32 => events.push(KeyEvent {
                    code: KeyCode::Space,
                }),
                _ => print!("{} ", byte),
            }
        }

        self.bytes.drain(..); // Done with these bytes 

        events
    }
}
