// NONOS Operating System (AGPL-3.0-or-later)
//! Sound configurations of about a power of two in bytes, each descriptor
//! kept apart so the sweep knows where every one starts.

use super::rng::Rng;

pub const IAD: u8 = 0x0B;
pub const CS_ENDPOINT: u8 = 0x25;
const CLASSES: [u8; 7] = [0x02, 0x0A, 0x09, 0xFF, 0x0B, 0x01, 0x03];
const SUBTYPES: [u8; 7] = [0x00, 0x01, 0x02, 0x06, 0x0F, 0x1A, 0x33];
const SIZES: [u16; 6] = [1, 8, 16, 64, 512, 1024];

/// A sound configuration of 16 to 4096 bytes, its header first.
pub fn config(r: &mut Rng) -> Vec<Vec<u8>> {
    let target = 1usize << r.range(4, 12);
    let mut out = vec![vec![9, 0x02, 0, 0, 0, 1, 0, 0x80, 50]];
    let (mut total, mut number) = (9, 0u8);
    loop {
        let f = function(r, &mut number);
        let len: usize = f.iter().map(Vec::len).sum();
        if total + len > target && out.len() > 1 {
            break;
        }
        total += len;
        out.extend(f);
    }
    out[0][2..4].copy_from_slice(&(total as u16).to_le_bytes());
    out[0][4] = out.iter().filter(|d| d[1] == 0x04 && d[3] == 0).count() as u8;
    out
}

/// One interface (an alternate of the last one time in five), sometimes
/// behind an association, with its class descriptors and endpoints.
fn function(r: &mut Rng, number: &mut u8) -> Vec<Vec<u8>> {
    let (mut f, class) = (Vec::new(), r.pick(&CLASSES));
    let (num, alt) = if *number > 0 && r.one_in(5) { (*number - 1, 1) } else { (*number, 0) };
    *number = number.wrapping_add((alt == 0) as u8);
    f.extend(r.one_in(4).then(|| vec![8, IAD, num, 1, class, 0, 0, 0]));
    let eps = r.below(5);
    f.push(vec![9, 0x04, num, alt, eps as u8, class, r.byte(), r.byte(), 0]);
    for _ in 0..r.below(4) {
        let sub = r.pick(&SUBTYPES);
        let len = r.range(cs_least(sub), cs_least(sub) + 8);
        f.push([vec![len as u8, 0x24, sub], (3..len).map(|_| r.byte()).collect()].concat());
    }
    for e in 0..eps {
        // A hub's first endpoint is its interrupt IN status endpoint.
        let hub = class == 0x09 && e == 0;
        let addr = r.range(1, 15) as u8 | if hub || r.one_in(2) { 0x80 } else { 0 };
        let (kind, size) = (if hub { 3 } else { r.pick(&[2u8, 2, 3, 1]) }, r.pick(&SIZES));
        let size = size.to_le_bytes();
        f.push(vec![7, 0x05, addr, kind, size[0], size[1], r.range(1, 16) as u8]);
        if r.one_in(2) {
            f.push(vec![6, 0x30, r.byte() & 0x0F, 0, 0, 0]);
        }
        if r.one_in(4) {
            f.push(vec![7, CS_ENDPOINT, 1, 0, 0, 0, 0]);
        }
    }
    f
}

/// The least bLength of a CDC functional descriptor by its subtype (CDC
/// 1.2 tables 15, 18, 3 and 16 of the PSTN and ECM subclasses, NCM 1.0
/// table 5-2; as recalled, unverified); 3 for one the sweep does not name.
pub fn cs_least(subtype: u8) -> usize {
    match subtype {
        0x00 | 0x01 | 0x06 => 5,
        0x02 => 4,
        0x0F => 13,
        0x1A => 6,
        _ => 3,
    }
}
