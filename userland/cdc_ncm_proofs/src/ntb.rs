// NONOS Operating System (AGPL-3.0-or-later)
//! NTB16s built field by field from USB NCM 1.0 section 3: the NTH16
//! (3.2.1: dwSignature "NCMH", wHeaderLength 12, wSequence, wBlockLength,
//! wNdpIndex) and NDP16s (3.3.1: dwSignature, wLength, wNextNdpIndex, then
//! wDatagramIndex/wDatagramLength pairs ended by a zero pair). The setters
//! write any value, so a test can make a block lie about itself.

pub const NCMH: u32 = 0x484D_434E;
pub const NCM0: u32 = 0x304D_434E;
/// "NCM1": an NDP16 whose datagrams carry a CRC.
pub const NCM1: u32 = 0x314D_434E;

pub struct Ntb(pub Vec<u8>);

impl Ntb {
    /// `len` bytes, the NTH16 claiming `len` as its block length.
    pub fn new(len: usize, ndp: u16) -> Self {
        let mut b = Ntb(vec![0; len]);
        b.u32(0, NCMH).u16(4, 12).u16(6, 0).u16(8, len as u16).u16(10, ndp);
        b
    }

    /// An NDP16 at `at` with `entries` and the zero entry that ends them.
    pub fn ndp(&mut self, at: usize, sign: u32, next: u16, entries: &[(u16, u16)]) -> &mut Self {
        let len = 8 + 4 * (entries.len() + 1);
        self.u32(at, sign).u16(at + 4, len as u16).u16(at + 6, next);
        for (i, (index, length)) in entries.iter().enumerate() {
            self.u16(at + 8 + 4 * i, *index).u16(at + 10 + 4 * i, *length);
        }
        self
    }

    /// `len` bytes of `fill` at `at`: a datagram the tests can tell apart.
    pub fn datagram(&mut self, at: usize, len: usize, fill: u8) -> &mut Self {
        self.0[at..at + len].fill(fill);
        self
    }

    pub fn u16(&mut self, at: usize, v: u16) -> &mut Self {
        self.0[at..at + 2].copy_from_slice(&v.to_le_bytes());
        self
    }

    pub fn u32(&mut self, at: usize, v: u32) -> &mut Self {
        self.0[at..at + 4].copy_from_slice(&v.to_le_bytes());
        self
    }
}

/// The datagrams `datagrams` finds in `raw`, as (index, length).
pub fn found(raw: &[u8], rx_max: usize) -> Vec<(usize, usize)> {
    let mut q = std::collections::VecDeque::new();
    crate::ncm::ntb_in::datagrams(raw, rx_max, &mut q);
    q.into_iter().collect()
}
