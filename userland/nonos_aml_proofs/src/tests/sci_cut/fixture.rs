// NONOS Operating System (AGPL-3.0-or-later)
//! A scene's AML: \_GPE._L01 (level) or \_GPE._E02 (edge), the same body:
//! maybe take \MX, notify \_SB.DV1 (0x80), say 1, spin, notify \_SB.DV2
//! (0x81), say 2, maybe give \MX back. Each device has HND, which the
//! drain runs for a delivery: it spins and DV1's notifies \_SB.DV3 (0x90).

use crate::enc::*;

pub struct Scene {
    pub level: bool,
    pub spin: u64,
    pub mutex: bool,
}

pub fn scene(seed: u64) -> Scene {
    Scene {
        level: seed.is_multiple_of(2),
        spin: 1 + seed / 2 % 5,
        mutex: (seed / 10).is_multiple_of(2),
    }
}

fn spin(n: u64) -> Vec<u8> {
    let less = e(0x95, &[&local(0), &int(n)]);
    cat(&[&store(&int(0), &local(0)), &pkg(0xA2, &cat(&[&less, &e(0x75, &[&local(0)])]))])
}

fn say(n: u64) -> Vec<u8> {
    store(&int(n), &op(0x5B31))
}

pub fn dsdt(sc: &Scene) -> Vec<u8> {
    let notify = |d: &str, v: u64| e(0x86, &[&name(d), &int(v)]);
    let take = if sc.mutex { e(0x5B23, &[&name("\\MX"), &[0xFF, 0xFF]]) } else { Vec::new() };
    let give = if sc.mutex { e(0x5B27, &[&name("\\MX")]) } else { Vec::new() };
    let body = cat(&[
        &take,
        &notify("\\_SB.DV1", 0x80),
        &say(1),
        &spin(sc.spin),
        &notify("\\_SB.DV2", 0x81),
        &say(2),
        &give,
    ]);
    let gpe = if sc.level { "\\_GPE._L01" } else { "\\_GPE._E02" };
    let hnd = |more: &[u8]| method("HND", 0, &cat(&[&spin(2), more]));
    cat(&[
        &cat(&[&op(0x5B01), &name("\\MX"), &[0]]),
        &device("\\_SB.DV1", &hnd(&cat(&[&notify("\\_SB.DV3", 0x90), &say(5)]))),
        &device("\\_SB.DV2", &hnd(&[])),
        &device("\\_SB.DV3", &hnd(&[])),
        &method(gpe, 0, &body),
    ])
}
