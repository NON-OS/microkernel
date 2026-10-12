// NONOS Operating System (AGPL-3.0-or-later)
//! The descriptor walk over random bytes and over QEMU's configurations
//! mutated: nothing panics, and what is read stays inside its own rules.

use crate::desc::{functional, interfaces, union_data, CS_INTERFACE};
use crate::qemu_usb_net::{CONFIG_ECM, CONFIG_RNDIS};

fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}

fn check(raw: &[u8]) {
    for i in interfaces(raw) {
        let p = i.pipes;
        assert!(p.bulk_in == 0 || p.bulk_in & 0x80 != 0, "{p:?}");
        assert_eq!(p.bulk_out & 0x80, 0, "{p:?}");
        assert!(p.max_packet_in <= 0x7FF && p.max_packet_out <= 0x7FF, "{p:?}");
        assert!(p.burst_in <= 15 && p.burst_out <= 15, "{p:?}");
        for d in functional(raw, i.number) {
            assert!(d.len() >= 3 && d[1] == CS_INTERFACE);
        }
        let _ = union_data(raw, i.number);
    }
}

#[test]
fn random_bytes_are_walked_without_a_panic() {
    let mut seed = 0x2545_F491_4F6C_DD1D;
    for _ in 0..20_000 {
        let len = (next(&mut seed) % 300) as usize;
        let raw: Vec<u8> = (0..len).map(|_| next(&mut seed) as u8).collect();
        check(&raw);
    }
}

#[test]
fn qemus_configurations_mutated_are_walked_without_a_panic() {
    let mut seed = 0x9E37_79B9_7F4A_7C15;
    for _ in 0..20_000 {
        let base: &[u8] =
            if next(&mut seed).is_multiple_of(2) { &CONFIG_ECM } else { &CONFIG_RNDIS };
        let mut raw = base.to_vec();
        for _ in 0..1 + next(&mut seed) % 6 {
            let at = (next(&mut seed) as usize) % raw.len();
            raw[at] = next(&mut seed) as u8;
        }
        raw.truncate(1 + (next(&mut seed) as usize) % raw.len());
        check(&raw);
    }
}
