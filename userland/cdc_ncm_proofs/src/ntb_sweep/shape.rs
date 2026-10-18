// NONOS Operating System (AGPL-3.0-or-later)
//! The two NTB formats as USB NCM 1.0 3.2 and 3.3 lay them out (as
//! recalled, unverified), restated apart from the driver's format.rs.

/// Lengths and offsets by format: header, NDP fixed part, where the next
/// NDP's index sits in it, one entry, and the two signatures.
pub struct Shape {
    pub wide: bool,
    pub nth_len: usize,
    pub ndp_header: usize,
    pub ndp_next: usize,
    pub entry: usize,
    pub nth_sign: u32,
    pub ndp_sign: u32,
}

pub const N16: Shape = Shape {
    wide: false,
    nth_len: 12,
    ndp_header: 8,
    ndp_next: 6,
    entry: 4,
    nth_sign: 0x484D_434E,
    ndp_sign: 0x304D_434E,
};
pub const N32: Shape = Shape {
    wide: true,
    nth_len: 16,
    ndp_header: 16,
    ndp_next: 8,
    entry: 8,
    nth_sign: 0x686D_636E,
    ndp_sign: 0x306D_636E,
};
