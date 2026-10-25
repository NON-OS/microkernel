// NONOS Operating System (AGPL-3.0-or-later)
//! The two holder checks, About's and the attest service's, against the
//! model over seeded tables, and one list of Admin holders in both.

use super::model::kernel_bits;
use crate::attest_live::caps as attest;
use crate::attest_live::types::{Capsule, Snapshot, MAX_PROCS};
use crate::held_by::*;

const NAMES: [&[u8]; 9] = [
    b"init",
    b"policy",
    b"acpi",
    b"power",
    b"app.install",
    b"tool.install",
    b"app.linux",
    b"driver.i2c_hid0",
    b"prove",
];

#[test]
fn both_checks_count_what_the_model_forbids() {
    let k = kernel_bits();
    let secret = k["InputSource"] | k["InputConsumer"] | k["Keyring"] | k["DeviceSecret"];
    assert_eq!((DEBUG, ADMIN, SECRETS), (k["Debug"], k["Admin"], secret));
    assert_eq!(
        (attest::CAP_DEBUG, attest::CAP_ADMIN, attest::CAP_SECRETS),
        (DEBUG, ADMIN, SECRETS)
    );
    assert_eq!(attest::ADMIN_HOLDERS, ADMIN_HOLDERS, "one list, two readers");
    let (mut x, mut red) = (3u64, [0; 2]);
    for _ in 0..3_000 {
        let mut snap = Snapshot {
            capsules: [const { Capsule { name: [0; 24], name_len: 0, caps: 0 } }; MAX_PROCS],
            count: 0,
        };
        let mut table: Vec<(u64, &[u8])> = Vec::new();
        for slot in 0..((x >> 50) % 12) as usize {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let (name, caps) = (NAMES[(x >> 40) as usize % NAMES.len()], x & ((1 << 40) - 1));
            table.push((caps, name));
            snap.capsules[slot].name[..name.len()].copy_from_slice(name);
            (snap.capsules[slot].name_len, snap.capsules[slot].caps, snap.count) =
                (name.len() as u8, caps, slot + 1);
        }
        let stray =
            table.iter().filter(|(c, n)| c & k["Admin"] != 0 && !ADMIN_HOLDERS.contains(n)).count()
                as u32;
        let leak =
            table.iter().filter(|(c, _)| c & k["Debug"] != 0 && c & secret != 0).count() as u32;
        assert_eq!(beyond(table.iter().copied(), ADMIN, ADMIN_HOLDERS), stray);
        assert_eq!(secret_loggers(table.iter().map(|t| t.0)), leak);
        assert_eq!(snap.holders_beyond(attest::CAP_ADMIN, attest::ADMIN_HOLDERS), stray);
        assert_eq!(snap.holders_both(attest::CAP_DEBUG, attest::CAP_SECRETS), leak);
        red[0] += usize::from(stray > 0);
        red[1] += usize::from(leak > 0);
        x = x.rotate_left(17) ^ 0x5bd1_e995;
    }
    println!("tables with a stray Admin {}, with a secret logger {}", red[0], red[1]);
    assert!(red[0] >= 1_290 && red[1] >= 1_990, "{red:?}");
}
