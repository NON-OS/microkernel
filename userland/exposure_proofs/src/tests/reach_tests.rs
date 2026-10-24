// NONOS Operating System (AGPL-3.0-or-later)
//! Each reach verdict against the model, over every single bit, every pair
//! and seeded masks; and the names against the kernel's bits.

use super::model::{any, kernel_bits, masks};
use crate::command::builtin::audit::names::names;
use crate::command::builtin::audit::reach::{bit, reaches, through, DEVICES, NETWORK};
use crate::command::builtin::audit::reach::{AREAS, STORAGE};
use crate::command::builtin::cap_names::CAP_NAMES;

#[test]
fn every_verdict_is_the_kernels_gate() {
    let k = kernel_bits();
    let (mut yes, mut all) = ([0; 3], 0);
    for m in masks() {
        let want = [
            any(&k, m, &["Network"]),
            any(&k, m, &["FileSystem", "StoreWrite", "StreamImport"]),
            any(&k, m, &["Driver", "Mmio", "Irq", "Dma", "Pio"]),
        ];
        let got = [reaches(m, &NETWORK), reaches(m, &STORAGE), reaches(m, &DEVICES)];
        assert_eq!(got, want, "mask {m:#x}");
        for (i, area) in AREAS.iter().enumerate() {
            assert_eq!(through(m, area).is_empty(), !want[i]);
            yes[i] += usize::from(want[i]);
        }
        all += 1;
    }
    for alone in ["Admin", "Hardware", "DeviceEnum", "IPC"] {
        assert!(AREAS.iter().all(|a| !reaches(k[alone], a)), "{alone} alone reaches nothing");
    }
    println!("{all} masks; network {}, storage {}, devices {}", yes[0], yes[1], yes[2]);
    assert!(all >= 4_800 && yes[0] >= 1_600 && yes[1] >= 2_900 && yes[2] >= 3_250, "{yes:?}");
}

#[test]
fn the_names_are_the_kernels_bits() {
    let k = kernel_bits();
    for area in &AREAS {
        for c in area.caps {
            assert_eq!(bit(c), k[core::str::from_utf8(c).unwrap()], "{c:?}");
        }
    }
    for (i, n) in CAP_NAMES.iter().enumerate() {
        assert_eq!(k[core::str::from_utf8(n).unwrap()], 1 << i, "{n:?}");
    }
    let m = k["Network"] | k["Dma"] | k["IPC"];
    assert_eq!(names(m), [b"Network".to_vec(), b"IPC".to_vec(), b"Dma".to_vec()]);
}
