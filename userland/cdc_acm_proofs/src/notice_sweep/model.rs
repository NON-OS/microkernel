// NONOS Operating System (AGPL-3.0-or-later)
//! What a host makes of notifications, stated apart from notice.rs (CDC
//! PSTN 1.2, 6.5.4, as recalled, unverified): only SERIAL_STATE (A1h,
//! 20h) about this interface (wIndex), of wLength 2 or more, with its two
//! state bytes there, counts; of it, the seven defined bits. DCD and DSR
//! are levels, as the last state says; the rest are events, each held
//! until the client reads the state, which forgets them.

pub fn bits(n: &[u8], comm: u8) -> Option<u16> {
    if n.len() < 10 || n[0] != 0xA1 || n[1] != 0x20 {
        return None;
    }
    let word = |i: usize| u16::from_le_bytes([n[i], n[i + 1]]);
    (word(4) == comm as u16 && word(6) >= 2).then(|| word(8) & 0x7F)
}

#[derive(Default)]
pub struct Line {
    pub levels: u16,
    pub events: u16,
}

impl Line {
    pub fn seen(&mut self, b: u16) {
        (self.levels, self.events) = (b & 3, self.events | (b & !3));
    }

    pub fn read(&mut self) -> u16 {
        let s = self.levels | self.events;
        self.events = 0;
        s
    }
}
