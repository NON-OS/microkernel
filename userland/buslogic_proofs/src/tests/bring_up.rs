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

//! From the claim to the targets scanned: the broker calls, the adapter's
//! commands in order, every refusal and what it gives back, and discovery.

use nonos_libc::{
    calls, logged, script_dma, Bar, DeviceRecord, Script, BAR_KIND_MMIO, BAR_KIND_PIO, BUS_KIND_PCI,
};
use nonos_scsi::Kind;

use crate::discover::is_match;
use crate::model::sim::Unit;

use super::util::{up, up_with, DEVICE};

fn released() -> Vec<(&'static str, u64)> {
    calls().into_iter().filter(|c| c.0.ends_with("release") || c.0 == "dma_unmap").collect()
}

#[test]
fn claim_ports_a_dma32_grant_then_reset_and_mailboxes() {
    let (d, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    let c = calls();
    assert_eq!(
        c[..3],
        [("claim", DEVICE), ("pci_command", (1 << 2) | (1 << 10)), ("pio_grant", 0)]
    );
    assert_eq!(c[3], ("dma_map", (1 << 1) | (1 << 2)), "below 4 GiB, coherent");
    let s = m.borrow();
    let cmds: Vec<(u8, Vec<u8>)> = s.log.iter().take_while(|l| l.0 != 0x02).cloned().collect();
    assert_eq!(
        cmds,
        vec![(0xFF, vec![]), (0x25, vec![0]), (0x0B, vec![]), (0x81, vec![1, 0, 0, 0x10, 0])]
    );
    assert_eq!(s.mbox, Some((0x0010_0000, 1)), "one mailbox pair at the grant's start");
    assert_eq!(d.units.hba.host_id, 7);
    assert_eq!(s.interrupt & 0x04, 0, "every command's completion cleared");
}

#[test]
fn every_target_but_the_adapters_own_is_scanned() {
    let mut t2 = Unit::disk(64, 512);
    t2.luns = vec![0, 3];
    let (d, m) = up(vec![((2, 3), Unit::cd(10)), ((2, 0), t2), ((15, 0), Unit::disk(32, 512))]);
    let at: Vec<(u16, u16, Kind)> =
        d.units.disks.iter().flatten().map(|u| (u.at.target, u.at.lun, u.kind)).collect();
    assert_eq!(at, vec![(2, 0, Kind::Disk), (2, 3, Kind::Cd), (15, 0, Kind::Disk)]);
    let s = m.borrow();
    let targets: std::collections::BTreeSet<u8> = s.ccbs.iter().map(|c| c.target).collect();
    assert_eq!(targets.len(), 15);
    assert!(!targets.contains(&7), "the adapter's own ID is never selected");
}

#[test]
fn an_adapter_without_the_interrupt_command_still_comes_up() {
    let (d, m) = up_with(vec![((0, 0), Unit::disk(8, 512))], Script::default(), |m| {
        m.borrow_mut().no_disable_command = true
    });
    assert_eq!(d.expect("bring-up").units.count(), 1);
    assert!(m.borrow().log.iter().any(|l| l.0 == 0x81));
}

#[test]
fn refusals_give_back_everything_taken() {
    let (d, _) = up_with(vec![], Script { claim: -1, ..Script::default() }, |_| {});
    assert_eq!(d.err(), Some("claim failed"));
    let (d, _) = up_with(vec![], Script { ports: [0; 5], ..Script::default() }, |_| {});
    assert_eq!(d.err(), Some("port grant failed"));
    assert_eq!(released(), vec![("device_release", 3)]);
    let (d, _) = up_with(vec![], Script { ports: [2, 0, 0, 0, 0], ..Script::default() }, |_| {});
    assert_eq!(d.err(), Some("port window short"));
}

#[test]
fn a_dma_grant_above_4_gib_is_given_back() {
    script_dma(Ok((0xFFFF_8000, 0)));
    let (d, m) = up_with(vec![], Script::default(), |_| {});
    assert_eq!(d.err(), Some("dma grant unusable"));
    assert_eq!(released(), vec![("dma_unmap", 70), ("pio_release", 50), ("device_release", 3)]);
    assert_eq!(m.borrow().resets, 1, "the adapter reset on the way out");
}

#[test]
fn failed_diagnostics_and_refused_mailboxes_are_refusals() {
    let (d, _) = up_with(vec![], Script::default(), |m| m.borrow_mut().diag_fail = true);
    assert_eq!(d.err(), Some("adapter diagnostics failed"));
    let r = released();
    assert_eq!(r[..2], [("dma_unmap", 70), ("pio_release", 50)]);
    let (d, _) = up_with(vec![], Script::default(), |m| m.borrow_mut().refuse_mailboxes = true);
    assert_eq!(d.err(), Some("command invalid"));
}

#[test]
fn mailboxes_the_adapter_did_not_take_are_a_refusal() {
    let (d, _) = up_with(vec![], Script::default(), |m| m.borrow_mut().ignore_mailboxes = true);
    assert_eq!(d.err(), Some("mailboxes not taken"));
}

#[test]
fn each_command_s_completion_is_cleared_before_the_next() {
    let m = crate::model::install(vec![]);
    let p = crate::port::Port::new(50);
    let va = std::hint::black_box(vec![0u8; 0x200]).leak().as_mut_ptr() as u64;
    crate::adapter::reset(&p).unwrap();
    let mut cfg = [0u8; 3];
    crate::adapter::command(&p, 0x0B, &[], &mut cfg).unwrap();
    assert_eq!(m.borrow().interrupt, 0);
    let _ = va;
}

#[test]
fn the_adapters_own_id_is_read_from_its_configuration() {
    let units = vec![((7, 0), Unit::disk(8, 512)), ((3, 0), Unit::disk(8, 512))];
    let (d, m) = up_with(units, Script::default(), |m| m.borrow_mut().host_id = 3);
    let d = d.expect("bring-up");
    assert_eq!(d.units.hba.host_id, 3);
    assert_eq!(d.units.disk(0).unwrap().at.target, 7);
    assert!(!m.borrow().ccbs.iter().any(|c| c.target == 3));
}

#[test]
fn an_underrun_status_on_a_short_answer_is_not_an_error() {
    let (d, _) = up_with(vec![((0, 0), Unit::disk(8, 512))], Script::default(), |m| {
        m.borrow_mut().underrun_status = true
    });
    let d = d.expect("bring-up");
    let u = d.units.disk(0).unwrap();
    assert_eq!(
        &u.serial[..u.serial_len],
        b"drive-scsi0-0-0-0",
        "the serial page is shorter than asked for"
    );
}

#[test]
fn a_disk_past_2_tib_needs_a_16_byte_cdb_the_ccb_cannot_carry() {
    let (d, _) = up(vec![((0, 0), Unit::disk(1 << 33, 512))]);
    assert_eq!(d.units.count(), 0);
    assert!(logged().iter().any(|l| l.contains("target 0 lun 0: not served, not ready")));
}

fn rec(device: u16, bar0: (u8, u64)) -> DeviceRecord {
    let mut r = DeviceRecord {
        device_id: 3,
        bus_kind: BUS_KIND_PCI,
        vendor: 0x104B,
        device,
        bar_count: 2,
        ..DeviceRecord::default()
    };
    r.bars[0] = Bar { kind: bar0.0, size: bar0.1 };
    r
}

#[test]
fn discovery() {
    assert!(is_match(&rec(0x1040, (BAR_KIND_PIO, 4))));
    assert!(!is_match(&rec(0x8130, (BAR_KIND_PIO, 4))), "a FlashPoint");
    assert!(!is_match(&rec(0x1040, (BAR_KIND_MMIO, 4))));
    assert!(!is_match(&rec(0x1040, (BAR_KIND_PIO, 2))));
}
