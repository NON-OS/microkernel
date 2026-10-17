// NONOS Operating System (AGPL-3.0-or-later)
//! The clock a terminal runs on, stated apart from clock.rs and
//! entities.rs (USB Audio 2.0 4.7.2, as recalled, unverified). Within the
//! last 2.0 AudioControl interface (class 1, subclass 1, protocol 20h) and
//! until the next interface, class descriptors (24h) of 5 bytes or more
//! are entities: an Input Terminal of 17 bytes names its clock at byte 7,
//! an Output Terminal of 12 at byte 8, a Clock Source of 8 is set by the
//! host when bmControls bits 1:0 are 3, a Clock Selector of 6 with a pin
//! has its first at byte 5. Ids must be distinct and not 0. The terminal's
//! clock is a source, or a selector whose first pin is one.

use super::want::Seen;

enum E {
    T(u8),
    S(bool),
    Sel(u8),
}

pub fn clock(c: &[u8], t: u8) -> Option<Seen> {
    let (mut at, mut on, mut ac, mut ents) = (0, false, 0, Vec::<(u8, E)>::new());
    while at + 2 <= c.len() && c[at] >= 2 && at + c[at] as usize <= c.len() {
        let d = &c[at..at + c[at] as usize];
        at += d.len();
        if d[1] == 0x04 && d.len() >= 9 {
            on = d[5] == 1 && d[6] == 1 && d[7] == 0x20;
            ac = if on { d[2] } else { ac };
            continue;
        }
        if !on || d[1] != 0x24 || d.len() < 5 {
            continue;
        }
        let e = match d[2] {
            0x02 if d.len() >= 17 => E::T(d[7]),
            0x03 if d.len() >= 12 => E::T(d[8]),
            0x0A if d.len() >= 8 => E::S(d[5] & 3 == 3),
            0x0B if d.len() >= 6 && d[4] >= 1 => E::Sel(d[5]),
            _ => continue,
        };
        ents.push((d[3], e));
    }
    let ids: Vec<u8> = ents.iter().map(|e| e.0).collect();
    if ids.iter().enumerate().any(|(i, &x)| x == 0 || ids[..i].contains(&x)) {
        return None;
    }
    let get = |id: u8| ents.iter().find(|e| e.0 == id).map(|e| &e.1);
    let Some(E::T(clock)) = get(t) else { return None };
    match get(*clock)? {
        E::S(p) => Some((ac, *clock, *p, None)),
        E::Sel(first) => match get(*first)? {
            E::S(p) => Some((ac, *first, *p, Some(*clock))),
            _ => None,
        },
        E::T(_) => None,
    }
}
