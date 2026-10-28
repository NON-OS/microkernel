// NONOS Operating System (AGPL-3.0-or-later)

//! Numbers as the console shows them, built without allocating.

/// `n` in decimal, into `out`; the digits written.
pub fn decimal(mut n: u32, out: &mut [u8; 10]) -> &[u8] {
    let mut i = out.len();
    loop {
        i -= 1;
        out[i] = b'0' + (n % 10) as u8;
        n /= 10;
        if n == 0 {
            return &out[i..];
        }
    }
}

/// `v` as four lower-case hex digits.
pub fn hex16(v: u16) -> [u8; 4] {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    core::array::from_fn(|i| DIGITS[((v >> (12 - i * 4)) & 0xF) as usize])
}
