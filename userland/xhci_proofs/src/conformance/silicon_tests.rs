// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! What real controllers do that QEMU's does not: an extended capability
//! list far into BAR0, separate USB 2 and USB 3 port ranges, USB 3 ports
//! that enable themselves, PORTSC bits that act when written back, a page
//! size register, 32-bit-only DMA, and 64-byte contexts.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use nonos_devmodel::{run, FakeBar};

use super::devices::host_controller;
use super::model::{controller, op_base, CAP_LEN, LEGACY, MAPPED, OS_OWNED, PORT1, XECP_WORDS};
use crate::constants::{
    HCCPARAMS1, PORTSC_CCS, PORTSC_CHANGE_BITS, PORTSC_PED, PORTSC_PLS_SHIFT, PORTSC_PP, PORTSC_PR,
    PORTSC_PRC, PORTSC_WPR, USBSTS,
};
use crate::contexts::{
    copy_slot_context, ep0_max_packet, ep0_needs_descriptor, interrupt_interval,
    max_packet_for_speed, with_entries, write_configure_endpoint_input, write_evaluate_ep0_input,
    EndpointConfig,
};
use crate::controller::{
    aligned_page, legacy_handoff, port_action, reset, reset_port, PortAction, Scratchpads,
};
use crate::dma::{below_4g, DmaPool};
use crate::error::XhciError;
use crate::regs::cap::PortProtocols;
use crate::regs::op::{page_bytes, portsc_neutral, usbsts_clear};

const USBSTS_PCD: u32 = 1 << 4;
const USBSTS_EINT: u32 = 1 << 3;
const USBSTS_HSE: u32 = 1 << 2;

/// A capability list: `(offset, dw0, dw2)`, each pointing at the next.
fn lay_out_caps(bar: &FakeBar, caps: &[(usize, u32, u32)]) {
    for (i, &(off, id_and_rev, dw2)) in caps.iter().enumerate() {
        let next = caps.get(i + 1).map_or(0, |n| ((n.0 - off) / 4) as u32);
        bar.present32(off, id_and_rev | (next << 8));
        bar.present32(off + 8, dw2);
    }
}

fn supported_protocol(major: u32) -> u32 {
    2 | (major << 24)
}

#[test]
fn port_protocols_split_usb2_and_usb3_ranges_as_an_intel_pch_does() {
    // GLK-like: USB 2 on ports 1 to 8, USB 3 on 9 to 15.
    let bar = controller(XECP_WORDS);
    lay_out_caps(
        &bar,
        &[
            (LEGACY, supported_protocol(2), 1 | (8 << 8)),
            (LEGACY + 0x20, supported_protocol(3), 9 | (7 << 8)),
        ],
    );
    let ports = PortProtocols::read(bar.base(), MAPPED);
    for port in 1..=8 {
        assert_eq!(ports.major(port), 2, "port {port}");
        assert!(!ports.is_usb3(port));
    }
    for port in 9..=15 {
        assert!(ports.is_usb3(port), "port {port}");
    }
    assert_eq!(ports.major(16), 0, "a port no capability names is unknown");
}

#[test]
fn the_capability_walk_stays_inside_the_mapped_window() {
    let hcc = (0x100u32 << 16) | 1;
    let reads = std::cell::RefCell::new(Vec::new());
    // A first capability inside, pointing at one that ends past the window.
    let window = 0x408;
    let read = |off: u64| {
        reads.borrow_mut().push(off);
        match off {
            0x400 => supported_protocol(2) | (1 << 8),
            0x408 => 1 | (2 << 8),
            _ => 0xDEAD_BEEF,
        }
    };
    let ports = PortProtocols::parse(hcc, window + 12, read);
    assert_eq!(ports.major(1), 2);
    assert!(reads.borrow().iter().all(|&o| o + 4 <= window + 12), "{:x?}", reads.borrow());
    // The same list with the window ending inside the first capability
    // reads nothing at all.
    reads.borrow_mut().clear();
    let ports = PortProtocols::parse(hcc, 0x404, read);
    assert_eq!(ports.major(1), 0);
    assert!(reads.borrow().is_empty());
}

#[test]
fn a_capability_list_that_loops_ends() {
    let hcc = (0x100u32 << 16) | 1;
    // Next pointer zero would end it; a pointer back to itself cannot, as
    // the offset only grows, so loop through a run of pointers of one.
    let ports = PortProtocols::parse(hcc, u64::MAX, |_| supported_protocol(3) | (1 << 8));
    // Every read gives the same capability: port 2 (dw2 bits 7:0), one port.
    assert_eq!(ports.major(2), 3);
    assert_eq!(ports.major(0), 0, "port 0 is never a port");
}

#[test]
fn the_legacy_capability_is_found_behind_the_protocol_capabilities() {
    let bar = controller(XECP_WORDS);
    let legacy = LEGACY + 0x40;
    lay_out_caps(&bar, &[(LEGACY, supported_protocol(2), 1 | (2 << 8)), (legacy, 1, 0)]);
    bar.present32(legacy + 4, 0);
    legacy_handoff(bar.base(), MAPPED);
    assert_ne!(bar.wrote32(legacy) & OS_OWNED, 0, "the OS asked for the controller");
}

#[test]
fn a_legacy_capability_past_the_mapping_is_not_touched() {
    let bar = controller(XECP_WORDS);
    bar.present32(LEGACY, 1);
    bar.present32(LEGACY + 4, 0xFFFF_FFFF);
    // Mapped short of the capability's second dword.
    legacy_handoff(bar.base(), LEGACY as u64 + 4);
    assert_eq!(bar.wrote32(LEGACY), 1, "nothing was written");
    assert_eq!(bar.wrote32(LEGACY + 4), 0xFFFF_FFFF);
}

#[test]
fn the_neutral_portsc_never_writes_back_an_action_bit() {
    let all = u32::MAX;
    let n = portsc_neutral(all);
    assert_eq!(n & PORTSC_PED, 0, "a one in PED disables the port");
    assert_eq!(n & PORTSC_PR, 0);
    assert_eq!(n & PORTSC_WPR, 0);
    assert_eq!(n & PORTSC_CHANGE_BITS, 0, "a one in a change bit acknowledges it");
    assert_eq!(n & (1 << 16), 0, "LWS stays clear, so PLS is not written");
    assert_ne!(n & PORTSC_PP, 0, "power is kept");
    assert_eq!(n & (0x7 << 25), 0x7 << 25, "wake enables are kept");
}

#[test]
fn usbsts_acknowledges_only_what_was_asked() {
    let bar = controller(0);
    bar.present32(CAP_LEN + USBSTS as usize, USBSTS_PCD | USBSTS_EINT | USBSTS_HSE);
    usbsts_clear(op_base(&bar), USBSTS_HSE);
    assert_eq!(bar.wrote32(CAP_LEN + USBSTS as usize), USBSTS_HSE);
}

#[test]
fn a_usb3_port_decides_by_its_link_state() {
    let pls = |s: u32| s << PORTSC_PLS_SHIFT;
    assert_eq!(port_action(PORTSC_CCS | PORTSC_PED, true), PortAction::Reset, "trained link reset before addressing");
    assert_eq!(port_action(PORTSC_CCS | pls(6), true), PortAction::WarmReset, "Inactive");
    assert_eq!(port_action(PORTSC_CCS | pls(10), true), PortAction::WarmReset, "Compliance");
    assert_eq!(port_action(PORTSC_CCS | pls(7), true), PortAction::Reset, "Polling");
    assert_eq!(port_action(PORTSC_CCS | PORTSC_PED, false), PortAction::Reset, "USB 2 always");
}

/// A USB 3 device on port 1 whose link trained: connected, enabled, in U0.
/// Records every write the driver makes to the port.
fn trained_usb3(writes: Arc<AtomicU32>) -> impl Fn(&FakeBar) {
    let trained = PORTSC_PP | PORTSC_CCS | PORTSC_PED;
    // Acts only on a value it did not present itself: the driver's write.
    let shown = AtomicU32::new(trained);
    move |bar| {
        let written = bar.wrote32(PORT1);
        if written != shown.load(Ordering::Relaxed) {
            writes.fetch_or(written, Ordering::Relaxed);
            // A hot reset on a trained link ends enabled, with PRC to say so.
            let next = if written & PORTSC_PR != 0 { trained | PORTSC_PRC } else { trained };
            shown.store(next, Ordering::Relaxed);
            bar.present32(PORT1, next);
        }
    }
}

#[test]
fn an_enabled_usb3_port_is_reset_before_addressing_and_never_disabled() {
    // Another class driver may have addressed the device on a trained link and
    // let it go; real devices refuse a second Address Device without a reset.
    let bar = controller(0);
    bar.present32(PORT1, PORTSC_PP | PORTSC_CCS | PORTSC_PED);
    let writes = Arc::new(AtomicU32::new(0));
    let _dev = run(&bar, trained_usb3(writes.clone()));
    let got = reset_port(op_base(&bar), 1, true).expect("the port is enabled");
    assert_ne!(got & PORTSC_PED, 0);
    let w = writes.load(Ordering::Relaxed);
    assert_ne!(w & PORTSC_PR, 0, "a hot reset was written");
    assert_eq!(w & PORTSC_WPR, 0, "no warm reset on a trained link");
    assert_eq!(w & PORTSC_PED, 0, "PED was never written as one");
}

#[test]
fn a_usb3_link_in_inactive_gets_a_warm_reset() {
    let bar = controller(0);
    let inactive = PORTSC_PP | PORTSC_CCS | (6 << PORTSC_PLS_SHIFT);
    bar.present32(PORT1, inactive);
    let warm = Arc::new(AtomicU32::new(0));
    let seen = warm.clone();
    // Acts only on a value it did not present itself: the driver's write.
    let shown = AtomicU32::new(inactive);
    let _dev = run(&bar, move |bar: &FakeBar| {
        let written = bar.wrote32(PORT1);
        if written == shown.load(Ordering::Relaxed) {
            return;
        }
        let next = if written & PORTSC_WPR != 0 {
            seen.store(1, Ordering::Relaxed);
            PORTSC_PP | PORTSC_CCS | PORTSC_PED | PORTSC_PRC
        } else {
            PORTSC_PP | PORTSC_CCS | PORTSC_PED
        };
        shown.store(next, Ordering::Relaxed);
        bar.present32(PORT1, next);
    });
    reset_port(op_base(&bar), 1, true).expect("the warm reset enables the port");
    assert_eq!(warm.load(Ordering::Relaxed), 1);
}

#[test]
fn a_connect_that_drops_during_the_debounce_is_no_device() {
    let bar = controller(0);
    bar.present32(PORT1, PORTSC_PP | PORTSC_CCS);
    let started = Instant::now();
    let _dev = run(&bar, move |bar: &FakeBar| {
        // Gone 40 ms in, before 100 ms of stable connect.
        if started.elapsed() > Duration::from_millis(40) {
            bar.present32(PORT1, PORTSC_PP);
        }
    });
    assert_eq!(reset_port(op_base(&bar), 1, false), Err(XhciError::NoDeviceOnPort));
}

#[test]
fn reset_waits_after_hcrst_before_reading_the_controller() {
    let bar = controller(0);
    let _hc = run(&bar, host_controller);
    let before = nonos_libc::slept_ms();
    reset(op_base(&bar)).expect("reset completes");
    assert!(nonos_libc::slept_ms() >= before + 1, "no settle time after HCRST (Intel quirk)");
}

#[test]
fn the_page_size_register_is_read_as_its_lowest_bit() {
    assert_eq!(page_bytes(0x1), Some(4096));
    assert_eq!(page_bytes(0x2), Some(8192));
    assert_eq!(page_bytes(0x6), Some(8192), "the smallest of several");
    assert_eq!(page_bytes(0), None);
    assert_eq!(page_bytes(0xFFFF_0000), None, "bits 31:16 are reserved");
}

#[test]
fn scratchpads_are_aligned_to_the_controller_page_size() {
    let pool = DmaPool::new(1, 1);
    for page in [4096u64, 8192, 32768] {
        let pads = Scratchpads::allocate(&pool, 3, page).expect("scratchpads");
        assert_eq!(pads.page_count(), 3);
        let array = nonos_libc::dma_host(pads.array_phys()).expect("array mapped") as *const u64;
        for i in 0..3 {
            // SAFETY: the array holds three entries in a live grant.
            let entry = unsafe { array.add(i).read() };
            assert_eq!(entry % page, 0, "entry {i} not aligned to {page}");
            assert!(nonos_libc::dma_host(entry + page - 1).is_some(), "page {i} not whole");
        }
    }
    assert!(Scratchpads::allocate(&pool, 1, 6000).is_err(), "not a power of two");
    assert_eq!(aligned_page(0x1000, 0x2000), 0x2000);
    assert_eq!(aligned_page(0x4000, 0x2000), 0x4000);
}

#[test]
fn a_32_bit_controller_is_never_given_an_address_above_4_gib() {
    // The shim maps every grant above 4 GiB, as a kernel may.
    let pool = DmaPool::new(1, 1).with_ac64(false);
    assert_eq!(pool.alloc(4096).err(), Some(XhciError::ControllerUnsupported));
    DmaPool::new(1, 1).alloc(4096).expect("a 64-bit controller takes it");
    assert!(below_4g(0xFFFF_F000, 0x1000));
    assert!(!below_4g(0xFFFF_F000, 0x1001));
    assert!(!below_4g(u64::MAX, 1));
}

#[test]
fn a_controller_reporting_ac64_is_still_read_as_supported() {
    let bar = controller(0);
    bar.present32(HCCPARAMS1 as usize, 0);
    crate::controller::refuse_unsupported(bar.base()).expect("AC64 clear is served");
}

#[test]
fn ep0_starts_at_the_size_each_speed_fixes() {
    assert_eq!(max_packet_for_speed(1, false), 8, "full speed starts at 8");
    assert_eq!(max_packet_for_speed(2, false), 8, "low speed is 8");
    assert_eq!(max_packet_for_speed(3, false), 64, "high speed is 64");
    assert_eq!(max_packet_for_speed(4, false), 512);
    assert_eq!(max_packet_for_speed(5, false), 512);
    // A USB 3 port whose speed id is a custom PSIV (Gen 2x2 and the like).
    assert_eq!(max_packet_for_speed(7, true), 512);
    assert!(ep0_needs_descriptor(1, false));
    assert!(!ep0_needs_descriptor(2, false) && !ep0_needs_descriptor(3, false));
    assert!(!ep0_needs_descriptor(1, true), "nothing full speed is on a USB 3 port");
}

#[test]
fn bmaxpacketsize0_is_taken_only_where_the_speed_allows_it() {
    for b in [8u8, 16, 32, 64] {
        assert_eq!(ep0_max_packet(1, false, b), Some(b as u16));
    }
    for b in [0u8, 7, 9, 48, 128, 255] {
        assert_eq!(ep0_max_packet(1, false, b), None, "full speed {b}");
    }
    assert_eq!(ep0_max_packet(3, false, 64), Some(64));
    assert_eq!(ep0_max_packet(3, false, 8), None);
    assert_eq!(ep0_max_packet(2, false, 8), Some(8));
    assert_eq!(ep0_max_packet(4, true, 9), Some(512), "an exponent at SuperSpeed");
    assert_eq!(ep0_max_packet(4, true, 64), None);
}

#[test]
fn binterval_is_converted_by_speed() {
    // Full and low speed: frames, to the largest power of two of 125 us
    // not above, 1 ms to 128 ms.
    assert_eq!(interrupt_interval(1, false, 1), 3);
    assert_eq!(interrupt_interval(1, false, 8), 6);
    assert_eq!(interrupt_interval(1, false, 10), 6, "10 ms is served every 8 ms");
    assert_eq!(interrupt_interval(2, false, 10), 6);
    assert_eq!(interrupt_interval(1, false, 32), 8);
    assert_eq!(interrupt_interval(1, false, 255), 10, "never past 128 ms");
    assert_eq!(interrupt_interval(1, false, 0), 3, "a zero bInterval is read as 1");
    // High speed and above: an exponent plus one, 1 to 16.
    assert_eq!(interrupt_interval(3, false, 1), 0);
    assert_eq!(interrupt_interval(3, false, 4), 3);
    assert_eq!(interrupt_interval(3, false, 16), 15);
    assert_eq!(interrupt_interval(3, false, 0), 0);
    assert_eq!(interrupt_interval(3, false, 200), 15);
    assert_eq!(interrupt_interval(4, true, 4), 3);
    for b in 0..=255u8 {
        for (speed, usb3) in [(1, false), (2, false), (3, false), (4, true)] {
            assert!(interrupt_interval(speed, usb3, b) <= 15, "reserved Interval for {b}");
        }
    }
}

fn region(bytes: u64) -> crate::dma::DmaRegion {
    let r = DmaPool::new(1, 1).alloc(bytes).expect("region");
    r.zero();
    r
}

fn dw(r: &crate::dma::DmaRegion, byte: usize) -> u32 {
    // SAFETY: the region is a live grant and `byte` is inside it.
    unsafe { (r.as_mut_ptr::<u8>().add(byte) as *const u32).read() }
}

fn set_dw(r: &crate::dma::DmaRegion, byte: usize, v: u32) {
    // SAFETY: as for `dw`.
    unsafe { (r.as_mut_ptr::<u8>().add(byte) as *mut u32).write(v) }
}

#[test]
fn context_entries_only_ever_rise() {
    let dw0 = 0x0030_0000 | (5 << 27);
    assert_eq!(with_entries(dw0, 3) >> 27, 5, "a second interface keeps the first's endpoint");
    assert_eq!(with_entries(dw0, 9) >> 27, 9);
    assert_eq!(with_entries(dw0, 9) & 0x07FF_FFFF, 0x0030_0000, "the rest is unchanged");
}

#[test]
fn sixty_four_byte_contexts_are_laid_out_at_their_stride() {
    let csz = 64usize;
    let (input, output) = (region(33 * 64), region(32 * 64));
    // Output slot context: speed 3, root port 7, 1 entry; EP0 at 64 bytes.
    set_dw(&output, 0, (3 << 20) | (1 << 27));
    set_dw(&output, 4, 7 << 16);
    set_dw(&output, csz + 4, (3 << 1) | (4 << 3) | (8 << 16));
    copy_slot_context(&input, &output, csz as u8, 3);
    assert_eq!(dw(&input, csz), (3 << 20) | (3 << 27), "slot context at input index 1");
    assert_eq!(dw(&input, csz + 4), 7 << 16);

    write_evaluate_ep0_input(&input, &output, csz as u8, 64);
    assert_eq!(dw(&input, 4), 1 << 1, "only A1");
    assert_eq!(dw(&input, 2 * csz + 4) >> 16, 64, "EP0 at input index 2");
    assert_eq!(dw(&input, 2 * csz + 4) & 0xFFFF, (3 << 1) | (4 << 3), "type and CErr kept");

    let cfg = EndpointConfig {
        context_size: csz as u8,
        dci: 3,
        ring_phys: 0x1_2345_6780,
        max_packet: 8,
        interval: 6,
    };
    write_configure_endpoint_input(&input, &output, cfg);
    assert_eq!(dw(&input, 4), 1 | (1 << 3), "A0 and A3");
    let ep = 4 * csz;
    assert_eq!(dw(&input, ep) >> 16 & 0xFF, 6, "Interval");
    assert_eq!(dw(&input, ep + 4) >> 3 & 0x7, 7, "Interrupt IN");
    assert_eq!(dw(&input, ep + 8), 0x2345_6781, "dequeue low with DCS");
    assert_eq!(dw(&input, ep + 12), 0x1, "dequeue high: no 32-bit truncation");
    assert_eq!(dw(&input, csz) >> 27, 3, "Context Entries covers DCI 3");
}

#[test]
fn ports_of_several_controllers_are_numbered_one_after_another() {
    use crate::mux_ports::{local_port, port_ranges};
    // An Intel PCH with 16 ports beside a Thunderbolt controller with 4.
    let r = port_ranges(&[16, 4]);
    assert_eq!(r, vec![(0, 16), (16, 4)]);
    assert_eq!(local_port(&r, 1), Some((0, 1)));
    assert_eq!(local_port(&r, 16), Some((0, 16)));
    assert_eq!(local_port(&r, 17), Some((1, 1)));
    assert_eq!(local_port(&r, 20), Some((1, 4)));
    assert_eq!(local_port(&r, 21), None);
    assert_eq!(local_port(&r, 0), None, "port 0 is no port");
    // A single controller keeps its own numbers.
    let one = port_ranges(&[26]);
    assert!((1..=26).all(|p| local_port(&one, p) == Some((0, p))));
}

#[test]
fn global_port_numbers_stop_at_255() {
    use crate::mux_ports::{local_port, port_ranges};
    let r = port_ranges(&[200, 100, 10]);
    assert_eq!(r, vec![(0, 200), (200, 55), (255, 0)]);
    assert_eq!(local_port(&r, 255), Some((1, 55)));
    for p in 0..=255u8 {
        if let Some((c, l)) = local_port(&r, p) {
            assert!(l >= 1 && l <= r[c].1, "port {p} maps outside controller {c}");
        }
    }
}
