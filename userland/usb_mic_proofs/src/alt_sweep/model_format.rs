// NONOS Operating System (AGPL-3.0-or-later)
//! Format Type I as model.rs reads it: of two-byte subframes and 16 bits;
//! for 1.0 in 8 bytes or more holding every rate its count names, 48 kHz
//! listed or within its range (count 0), and its channel count taken; for
//! 2.0 in 6 bytes. Any other leaves the channel count as it was.

pub fn format(v: u8, d: &[u8], channels: u8) -> (bool, u8) {
    let rate =
        |i: usize| d.get(i..i + 3).map(|b| b[0] as u32 | (b[1] as u32) << 8 | (b[2] as u32) << 16);
    let n = d.get(7).map_or(0, |&n| if n == 0 { 2 } else { n as usize });
    let ok = match v {
        1 if d.len() >= 8 && d[3] == 1 && d[5] == 2 && d[6] == 16 && d.len() >= 8 + 3 * n => {
            match d[7] {
                0 => rate(8).unwrap() <= 48_000 && 48_000 <= rate(11).unwrap(),
                _ => (0..n).any(|i| rate(8 + 3 * i) == Some(48_000)),
            }
        }
        2 => d.len() >= 6 && d[3] == 1 && d[4] == 2 && d[5] == 16,
        _ => false,
    };
    (ok, if ok && v == 1 { d[4] } else { channels })
}
