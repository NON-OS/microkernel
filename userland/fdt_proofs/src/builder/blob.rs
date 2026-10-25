// NONOS Operating System (AGPL-3.0-or-later)
use super::writer::Builder;

impl Builder {
    /// Header, reservation map, structure, strings; FDT_END is the caller's.
    pub fn finish(self) -> Vec<u8> {
        let mut rsv: Vec<u8> = Vec::new();
        for (a, s) in self.rsv.iter().copied() {
            rsv.extend_from_slice(&a.to_be_bytes());
            rsv.extend_from_slice(&s.to_be_bytes());
        }
        if !self.rsv_unterminated {
            rsv.extend_from_slice(&[0; 16]);
        }
        let off_struct = 40 + rsv.len();
        let off_strings = off_struct + self.structure.len();
        let total = off_strings + self.strings.len();
        let fields = [0xd00d_feed, total, off_struct, off_strings, 40, 17, 16, 0];
        let mut out: Vec<u8> = fields.iter().flat_map(|&f| (f as u32).to_be_bytes()).collect();
        out.extend_from_slice(&(self.strings.len() as u32).to_be_bytes());
        out.extend_from_slice(&(self.structure.len() as u32).to_be_bytes());
        out.extend(rsv);
        out.extend(self.structure);
        out.extend(self.strings);
        out
    }
}
