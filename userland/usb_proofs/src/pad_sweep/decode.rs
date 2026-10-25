// NONOS Operating System (AGPL-3.0-or-later)
//! What a host makes of gamepad reports, stated apart from decode.rs and
//! values.rs (HID 1.11 5.8 and 6.2.2.7, as recalled, unverified). A report
//! starts with its ID when the device numbers them; an empty one is
//! nothing. Each field of that report whose bits all came is read, little-
//! endian from its bit offset, sign-extended when its minimum is under 0.
//! A button is down when non-zero. An axis is scaled from its logical
//! range onto -32767..32767, clamped into it (0 for an empty or inverted
//! range); the hat gives eight directions over its range, -1 outside it.
//! An axis is told when its value changes, and the first time; buttons
//! that changed are told after, lowest first.

use super::writer::F;

pub fn read(data: &[u8], f: &F) -> Option<i64> {
    let (off, size) = (f.3 as usize, f.4 as usize);
    if off + size > data.len() * 8 {
        return None;
    }
    let mut v = 0i64;
    for b in 0..size {
        v |= ((data[(off + b) / 8] >> ((off + b) % 8) & 1) as i64) << b;
    }
    Some(if f.5 < 0 && v >> (size - 1) & 1 == 1 { v - (1 << size) } else { v })
}

pub fn value(v: i64, f: &F) -> i32 {
    let (min, max) = (f.5 as i64, f.6 as i64);
    if f.1 == 0x39 {
        return if v < min || v > max { -1 } else { ((v - min) * 8 / (max - min + 1)) as i32 };
    }
    if max <= min {
        return 0;
    }
    ((v.clamp(min, max) - min) * 65534 / (max - min) - 32767) as i32
}

#[derive(Default)]
pub struct Pad {
    pub buttons: u32,
    pub axes: [Option<i32>; 10],
}

/// The events (usage or button, value) one report makes.
pub fn feed(p: &mut Pad, ids: bool, fields: &[F], raw: &[u8]) -> Vec<(bool, u16, i32)> {
    let (id, data) = match (ids, raw.split_first()) {
        (true, Some((&id, rest))) => (id, rest),
        (true, None) => return vec![],
        (false, _) => (0, raw),
    };
    let (mut out, mut buttons) = (vec![], p.buttons);
    for f in fields.iter().filter(|f| f.2 == id) {
        let Some(v) = read(data, f) else { continue };
        if f.0 == 9 {
            let bit = 1u32 << (f.1 - 1);
            buttons = if v != 0 { buttons | bit } else { buttons & !bit };
            continue;
        }
        let x = value(v, f);
        if p.axes[(f.1 - 0x30) as usize] != Some(x) {
            p.axes[(f.1 - 0x30) as usize] = Some(x);
            out.push((false, f.1, x));
        }
    }
    for n in 0..32 {
        if (buttons ^ p.buttons) >> n & 1 == 1 {
            out.push((true, n as u16 + 1, (buttons >> n & 1) as i32));
        }
    }
    p.buttons = buttons;
    out
}
