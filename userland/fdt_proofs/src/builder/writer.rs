// NONOS Operating System (AGPL-3.0-or-later)
//! A small FDT writer for blobs no real tool would emit.

const BEGIN: u32 = 1;
const END_NODE: u32 = 2;
const PROP: u32 = 3;
pub const END: u32 = 9;

#[derive(Default)]
pub struct Builder {
    pub(super) structure: Vec<u8>,
    pub(super) strings: Vec<u8>,
    pub rsv: Vec<(u64, u64)>,
    /// Leave the reservation map without its terminator.
    pub rsv_unterminated: bool,
}

impl Builder {
    pub fn token(mut self, tok: u32) -> Self {
        self.structure.extend_from_slice(&tok.to_be_bytes());
        self
    }

    pub fn begin(mut self, name: &str) -> Self {
        self = self.token(BEGIN);
        self.structure.extend_from_slice(name.as_bytes());
        self.structure.push(0);
        self.pad()
    }

    pub fn end(self) -> Self {
        self.token(END_NODE)
    }

    pub fn prop(mut self, name: &str, data: &[u8]) -> Self {
        let off = self.strings.len() as u32;
        self.strings.extend_from_slice(name.as_bytes());
        self.strings.push(0);
        self = self.token(PROP).token(data.len() as u32).token(off);
        self.structure.extend_from_slice(data);
        self.pad()
    }

    pub(super) fn pad(mut self) -> Self {
        while !self.structure.len().is_multiple_of(4) {
            self.structure.push(0);
        }
        self
    }
}
