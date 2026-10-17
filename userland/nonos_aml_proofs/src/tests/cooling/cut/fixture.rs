// NONOS Operating System (AGPL-3.0-or-later)
//! A scene: \_TZ.TZ0 whose _TMP, _CRT, _HOT and _PSV are methods. Each may
//! take \MX, spins a little (a different count each, so the cuts fall in
//! different places), gives \MX back, and returns its answer. _AC0 and
//! _TZP are plain names.

use crate::enc::*;

#[derive(Clone, Copy, Debug)]
pub enum Ans {
    /// Tenths of a kelvin, or any lie in an integer.
    Int(u64),
    /// Not an integer at all.
    Str,
}

/// _TMP, _CRT, _HOT, _PSV; None for a trip the zone does not have.
pub type Lies = [Option<Ans>; 4];
pub const SEGS: [&str; 4] = ["_TMP", "_CRT", "_HOT", "_PSV"];

const SANE: Lies =
    [Some(Ans::Int(3332)), Some(Ans::Int(3732)), Some(Ans::Int(3632)), Some(Ans::Int(3532))];

/// Every scene: sane, then one lie at a time.
pub fn scenes() -> Vec<Lies> {
    let lie = |i: usize, a: Option<Ans>| {
        let mut l = SANE;
        l[i] = a;
        l
    };
    let mut out = vec![SANE];
    for i in 0..4 {
        // Absolute zero, below it as a signed reading, past any bound, not
        // a number.
        for a in [Ans::Int(0), Ans::Int(u64::MAX), Ans::Int(5000), Ans::Str] {
            out.push(lie(i, Some(a)));
        }
    }
    out.push(lie(2, None));
    out.push(lie(2, Some(Ans::Int(3832)))); // _HOT above _CRT
    out.push(lie(1, Some(Ans::Int(3432)))); // _CRT below _PSV
    out.push(lie(2, Some(Ans::Int(3432)))); // _HOT below _PSV
    out
}

fn spin(n: u64) -> Vec<u8> {
    let less = e(0x95, &[&local(0), &int(n)]);
    cat(&[&store(&int(0), &local(0)), &pkg(0xA2, &cat(&[&less, &e(0x75, &[&local(0)])]))])
}

pub fn dsdt(lies: &Lies, mutex: bool) -> Vec<u8> {
    let mut body = cat(&[&name_obj("_AC0", &int(3432)), &name_obj("_TZP", &int(100))]);
    for (i, a) in lies.iter().enumerate() {
        let Some(a) = a else { continue };
        let v = match a {
            Ans::Int(v) => int(*v),
            Ans::Str => string("hot"),
        };
        let take = if mutex { e(0x5B23, &[&name("\\MX"), &[0xFF, 0xFF]]) } else { vec![] };
        let give = if mutex { e(0x5B27, &[&name("\\MX")]) } else { vec![] };
        let m = method(SEGS[i], 0, &cat(&[&take, &spin(1 + i as u64), &give, &ret(&v)]));
        body.extend_from_slice(&m);
    }
    cat(&[&cat(&[&op(0x5B01), &name("\\MX"), &[0]]), &thermal_zone("\\_TZ.TZ0", &body)])
}
