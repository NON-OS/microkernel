// NONOS Operating System (AGPL-3.0-or-later)
//! wTotalLength told wrong: the configuration read is cut where the
//! device's own count and the control buffer allow, and every descriptor
//! wholly inside what came is found. A device or configuration header
//! whose bLength is short of its fixed size is no device at all.

use crate::desc::walk;
use crate::found::fetch;
use crate::mock::{descriptors, MockBus};
use crate::sweep::build::config;
use crate::sweep::rng::{rounds, Rng};
use crate::xhci::CONTROL_MAX;

const DEVICE: [u8; 18] = [18, 1, 0, 2, 0, 0, 0, 64, 0x34, 0x12, 0x78, 0x56, 0, 1, 0, 0, 0, 1];

#[test]
fn a_lying_total_cuts_the_read_and_never_loses_a_whole_descriptor() {
    let mut told = [0u32; 6];
    for seed in 1..rounds(20_000) {
        let mut r = Rng::new(seed);
        let built = config(&mut r);
        let mut raw = built.concat();
        let have = raw.len();
        let (kind, mut device) = (r.below(6), DEVICE);
        let total = match kind {
            0 | 4 | 5 => have,
            1 => r.range(9, have - 1),
            2 => r.range(have + 1, 65_535),
            _ => r.range(0, 8),
        };
        raw[2..4].copy_from_slice(&(total as u16).to_le_bytes());
        match kind {
            4 => raw[0] = r.range(0, 8) as u8,
            5 => device[0] = r.range(0, 17) as u8,
            _ => {}
        }
        let config = raw.clone();
        let mut bus = MockBus::new(Box::new(move |call| {
            descriptors(call, &device, &[&config], &[]).ok_or(-32)
        }));
        told[kind] += 1;
        if kind >= 4 {
            assert_eq!(fetch(&mut bus, 1).err(), Some(-5), "seed {seed}: kind {kind}");
            continue;
        }
        let got = fetch(&mut bus, 1).unwrap().configs.remove(0);
        let cut = total.clamp(9, CONTROL_MAX).min(have);
        assert_eq!(got, raw[..cut], "seed {seed}: total {total} of {have}");
        let mut at = 0;
        let whole: Vec<&[u8]> = built
            .iter()
            .take_while(|d| {
                at += d.len();
                at <= cut
            })
            .map(Vec::as_slice)
            .collect();
        let found: Vec<&[u8]> = walk(&got).collect();
        assert_eq!(found[1..], whole[1..], "seed {seed}: total {total} of {have}");
    }
    for (n, what) in told.iter().zip(["true", "short", "long", "under 9", "header", "device"]) {
        assert!(cfg!(miri) || *n >= 3_000, "a {what} total was told only {n} times");
    }
}
