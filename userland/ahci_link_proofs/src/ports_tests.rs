// NONOS Operating System (AGPL-3.0-or-later)
//! Which ports the walk covers, which hold a talking device, how a port is
//! powered up, and what the HBA reset must give back. The values are the
//! ones real HBAs report: QEMU's ICH9, Intel Gemini Lake (8086:31E3), the
//! sparse PI of a PCH with ports fused off, and AMD FCH.

use crate::constants::regs::{CMD_POD, CMD_SUD};
use crate::controller::ports::{device_present, effective_pi, port_count, spin_up_bits};
use crate::controller::restore::restore_writes;
use crate::controller::window::ports_in_window;

/// CAP with NP (ports less one) and the given extra bits.
fn cap(np: u32, bits: u32) -> u32 {
    (np & 0x1f) | bits
}

#[test]
fn the_named_controllers_cover_their_ports() {
    let named = [
        (cap(5, 0), 0x3f, 6, "QEMU ICH9: six ports, all implemented"),
        (cap(1, 0), 0x3, 2, "Gemini Lake: two ports"),
        (cap(1, 0), 0x5, 3, "two ports, the second at index 2"),
        (cap(3, 0), 0x31, 6, "four ports with port 1-3 fused off, 4 and 5 present"),
        (cap(0, 0), 0x80, 8, "NP says one port; PI puts it at 7"),
        (cap(5, 0), 0x8000_0000, 32, "a lone port 31"),
        (cap(31, 0), 0x1, 32, "NP says 32 ports, PI one"),
        (cap(3, 0), 0, 4, "PI never written: the first NP + 1 ports"),
    ];
    for (c, pi, want, what) in named {
        assert_eq!(port_count(c, pi), want, "{what}: CAP {c:#x} PI {pi:#x}");
    }
}

#[test]
fn every_implemented_port_is_inside_the_count() {
    let mut s = 0x9E37_79B9_7F4A_7C15u64;
    for _ in 0..100_000 {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        let c = s as u32;
        let pi = (s >> 32) as u32;
        let n = u32::from(port_count(c, pi));
        let walked = effective_pi(c, pi);
        assert!((1..=32).contains(&n), "CAP {c:#x} PI {pi:#x}: count {n}");
        assert!(n > (c & 0x1f), "CAP {c:#x} PI {pi:#x}: fewer than NP + 1");
        for i in 0..32 {
            if walked & (1 << i) != 0 {
                assert!(i < n, "CAP {c:#x} PI {pi:#x}: port {i} outside the count {n}");
            }
        }
        assert_ne!(walked, 0, "CAP {c:#x}: nothing walked");
    }
}

#[test]
fn a_zero_pi_falls_back_to_np_and_a_set_one_is_kept() {
    assert_eq!(effective_pi(cap(0, 0), 0), 0x1);
    assert_eq!(effective_pi(cap(5, 0), 0), 0x3f);
    assert_eq!(effective_pi(cap(31, 0), 0), u32::MAX);
    assert_eq!(effective_pi(cap(5, 0), 0x24), 0x24);
}

#[test]
fn small_abar_windows_reach_the_ports_they_hold() {
    // AMD FCH 1 KiB ABAR: ports 0 to 5; Intel PCH 2 KiB: ports 0 to 13.
    assert_eq!(ports_in_window(0x400), 0x3f);
    assert_eq!(ports_in_window(0x800), 0x3fff);
    // The smallest ABAR the driver takes holds port 0 alone.
    assert_eq!(ports_in_window(0x180), 0x1);
    // A sparse PI on a 2 KiB ABAR: port 15 is out of reach, never touched.
    assert_eq!(effective_pi(cap(1, 0), 0x8001) & ports_in_window(0x800), 0x1);
}

#[test]
fn a_talking_device_in_any_live_power_state_is_present() {
    for ipm in [1u32, 2, 6, 8] {
        assert!(device_present(ipm << 8 | 0x3), "IPM {ipm}");
        assert!(device_present(ipm << 8 | 0x3 | 0x20), "IPM {ipm}, Gen2 speed");
    }
    // DET 3 with IPM 0 (not present) or reserved, or any other DET, is not.
    for ssts in [0x003u32, 0x303, 0x403, 0x113 & !0x2, 0x101, 0x100, 0x104, 0x0, 0x601] {
        assert!(!device_present(ssts), "SSTS {ssts:#x}");
    }
}

#[test]
fn spin_up_sets_sud_and_pod_only_where_writable() {
    const CPD: u32 = 1 << 20;
    assert_eq!(spin_up_bits(cap(1, 0)), CMD_SUD);
    assert_eq!(spin_up_bits(cap(1, CPD)), CMD_SUD | CMD_POD);
    // Staggered spin-up (CAP.SSS) alone changes nothing: SUD is set either way.
    assert_eq!(spin_up_bits(cap(1, 1 << 27)), CMD_SUD);
}

#[test]
fn the_reset_gives_back_only_what_it_changed() {
    const SSS: u32 = 1 << 27;
    // Untouched: nothing written.
    assert_eq!(restore_writes(cap(1, SSS), 0x3, cap(1, SSS), 0x3), (None, None));
    // An Intel PCH whose HR cleared PI and CAP.SSS: both written back.
    assert_eq!(restore_writes(cap(1, SSS), 0x3, cap(1, 0), 0), (Some(cap(1, SSS)), Some(0x3)));
    // PI the firmware never set is nothing to restore.
    assert_eq!(restore_writes(cap(3, 0), 0, cap(3, 0), 0xf), (None, None));
    // PI changed on its own.
    assert_eq!(restore_writes(cap(3, 0), 0x5, cap(3, 0), 0xf), (None, Some(0x5)));
}
