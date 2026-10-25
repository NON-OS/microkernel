// NONOS Operating System (AGPL-3.0-or-later)
//! A scene: \_TZ.TZ0 whose _TMP, _CRT, _HOT, _PSV, _AC0, _AC1 and _TZP are
//! methods. Each may take \MX, spins a little (a different count each, so
//! the cuts fall in different places), gives \MX back, and answers.

use crate::enc::*;

#[derive(Clone, Copy, Debug)]
pub enum Ans {
    /// Tenths of a kelvin, or any lie in an integer.
    Int(u64),
    /// Not an integer at all.
    Str,
}

/// One per `SEGS`; None for a trip the zone does not have.
pub type Lies = [Option<Ans>; 7];
pub const SEGS: [&str; 7] = ["_TMP", "_CRT", "_HOT", "_PSV", "_AC0", "_AC1", "_TZP"];

const fn n(v: u64) -> Option<Ans> {
    Some(Ans::Int(v))
}

const SANE: Lies = [n(3332), n(3732), n(3632), n(3532), n(3432), n(3332), n(100)];

/// Every scene: sane, then one lie at a time.
pub fn scenes() -> Vec<Lies> {
    let lie = |i: usize, a: Option<Ans>| {
        let mut l = SANE;
        l[i] = a;
        l
    };
    let mut out = vec![SANE];
    for i in 0..SEGS.len() {
        // Absolute zero, below it as a signed reading, past any bound, not
        // a number.
        for a in [Ans::Int(0), Ans::Int(u64::MAX), Ans::Int(5000), Ans::Str] {
            out.push(lie(i, Some(a)));
        }
    }
    for i in [2, 5, 6] {
        out.push(lie(i, None)); // no _HOT, no _AC1, no _TZP
    }
    out.push(lie(2, Some(Ans::Int(3832)))); // _HOT above _CRT
    out.push(lie(1, Some(Ans::Int(3432)))); // _CRT below _PSV
    out.push(lie(2, Some(Ans::Int(3432)))); // _HOT below _PSV
    out.push(lie(5, Some(Ans::Int(3532)))); // _AC1 above _AC0
    out
}

fn spin(n: u64) -> Vec<u8> {
    let less = e(0x95, &[&local(0), &int(n)]);
    cat(&[&store(&int(0), &local(0)), &pkg(0xA2, &cat(&[&less, &e(0x75, &[&local(0)])]))])
}

pub fn dsdt(lies: &Lies, mutex: bool) -> Vec<u8> {
    let mut body = Vec::new();
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
