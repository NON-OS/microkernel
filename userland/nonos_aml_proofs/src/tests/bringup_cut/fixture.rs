// NONOS Operating System (AGPL-3.0-or-later)
//! A small machine: load-time code (an If and a Store, so a DSDT load can
//! be cut); \_PIC; \_SB._OSC; a host bridge \_SB.PCI0 with _OSC and _PRT; a
//! device with _STA and an _INI that spins; and an EC device with a _REG.
//! Markers to Debug: \_PIC 1, \_SB._OSC 2, _INI 3, _REG 4.

use crate::enc::*;

fn mark(n: u64) -> Vec<u8> {
    store(&int(n), &op(0x5B31))
}

fn spin(n: u64) -> Vec<u8> {
    let less = e(0x95, &[&local(0), &int(n)]);
    cat(&[&store(&int(0), &local(0)), &pkg(0xA2, &cat(&[&less, &e(0x75, &[&local(0)])]))])
}

pub fn dsdt() -> Vec<u8> {
    let prt = package(
        2,
        &[
            package(4, &[int(0xFFFF), int(0), int(0), int(16)]),
            package(4, &[int(0x1_FFFF), int(1), int(0), int(17)]),
        ],
    );
    let dev = cat(&[
        &name_obj("_ADR", &int(0x1_0000)),
        &method("_STA", 0, &ret(&int(0x0F))),
        &method("_INI", 0, &cat(&[&mark(3), &spin(4)])),
    ]);
    let ec = cat(&[&region("ECR", 3, 0, 8), &method("_REG", 2, &cat(&[&spin(2), &mark(4)]))]);
    let pci = cat(&[
        &name_obj("_HID", &string("PNP0A08")),
        &method("_OSC", 4, &ret(&arg(3))),
        &name_obj("_PRT", &prt),
        &device("DEV1", &dev),
        &device("EC0", &ec),
    ]);
    cat(&[
        &name_obj("\\GATE", &int(1)),
        &name_obj("\\LTS", &int(0)),
        &if_(&e(0x93, &[&name("\\GATE"), &int(1)]), &store(&int(5), &name("\\LTS"))),
        &name_obj("\\PICM", &int(0)),
        &method("\\_PIC", 1, &cat(&[&store(&arg(0), &name("\\PICM")), &spin(3), &mark(1)])),
        &method("\\_SB._OSC", 4, &cat(&[&mark(2), &ret(&arg(3))])),
        &device("\\_SB.PCI0", &pci),
    ])
}
