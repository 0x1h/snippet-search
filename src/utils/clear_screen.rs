use std::io::{self, Write};

pub fn clear_screen() {
    io::stdout().write_all(b"\x1B[2J\x1B[1;1H").unwrap();
    io::stdout().flush().unwrap();
}
