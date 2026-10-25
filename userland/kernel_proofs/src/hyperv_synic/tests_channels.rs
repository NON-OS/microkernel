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

//! The kernel's table of VMBus channels: learnt from the host's offers
//! alone, forgotten when withdrawn, given only to drivers of the interfaces
//! a driver exists for; a GPADL tied to the channel its header named until
//! the host says it is torn down; a driver's let-go told to the core once.

use super::channels::{BindError, Channels, Interface, Retire};
use super::tests_vmbus::{gpadl_header, offer, SCSI};

/// A GUID in Linux's text form, as VMBus carries it.
fn wire(text: &str) -> [u8; 16] {
    let h: String = text.chars().filter(|c| *c != '-').collect();
    let b: Vec<u8> = (0..16).map(|i| u8::from_str_radix(&h[2 * i..2 * i + 2], 16).unwrap()).collect();
    let mut w = [0u8; 16];
    w[..4].copy_from_slice(&[b[3], b[2], b[1], b[0]]);
    w[4..6].copy_from_slice(&[b[5], b[4]]);
    w[6..8].copy_from_slice(&[b[7], b[6]]);
    w[8..].copy_from_slice(&b[8..]);
    w
}

fn rescind(relid: u32) -> Vec<u8> {
    let mut m = vec![0u8; 12];
    m[0] = 2;
    m[8..12].copy_from_slice(&relid.to_le_bytes());
    m
}

fn torndown(id: u32) -> Vec<u8> {
    let mut m = vec![0u8; 12];
    m[0] = 12;
    m[8..12].copy_from_slice(&id.to_le_bytes());
    m
}

#[test]
fn the_interfaces_are_linux_s_guids_and_the_rest_are_refused() {
    // include/linux/hyperv.h: HV_NIC_GUID, HV_SCSI_GUID, HV_IDE_GUID,
    // HV_KBD_GUID, HV_MOUSE_GUID, HV_SYNTHVID_GUID.
    let given = [
        ("f8615163-df3e-46c5-913f-f2d2f965ed0e", Interface::Network, 0x4E45),
        ("ba6163d9-04a1-4d29-b605-72e2ffb1dc7f", Interface::Scsi, 0x5343),
        ("32412632-86cb-44a2-9b5c-50d1417354f5", Interface::Ide, 0x4944),
        ("f912ad6d-2b17-48ea-bd65-f927a61c7684", Interface::Keyboard, 0x4B42),
        ("cfa8b69e-5b4a-4cc0-b98b-8ba1a1f3f95a", Interface::Mouse, 0x4D53),
        ("da0a7802-e377-4aac-8e77-0558eb1073f8", Interface::Video, 0x5644),
    ];
    for (text, i, device) in given {
        assert_eq!(Interface::of(&wire(text)), Some(i), "{text}");
        assert_eq!(i.device(), device);
    }
    assert_eq!(wire("ba6163d9-04a1-4d29-b605-72e2ffb1dc7f"), SCSI);
    // Heartbeat, shutdown, time sync, KVP, VSS, dynamic memory, file copy, PCI.
    let services = [
        "57164f39-9115-4e78-ab55-382f3bd5422d",
        "0e0b6031-5213-4934-818b-38d90ced39db",
        "9527e630-d0ae-497b-adce-e80ab0175caf",
        "a9a0f4e7-5a45-4d96-b827-8a841e8c03e6",
        "35fa2e29-ea23-4236-96ae-3a6ebacba440",
        "525074dc-8985-46e2-8057-a307dc18a502",
        "34d14be3-dee4-41c8-9ae7-6b174977c192",
        "44c4f61d-4444-4400-9d52-802e27ede19f",
    ];
    let mut c = Channels::new();
    for (n, text) in services.iter().enumerate() {
        let relid = 20 + n as u32;
        assert_eq!(Interface::of(&wire(text)), None, "{text}");
        c.from_host(&offer(relid, wire(text), 0x2000 + relid));
        assert_eq!(c.bind(relid, 0x99), Err(BindError::Interface), "{text}");
    }
}

#[test]
fn an_offer_is_learnt_from_the_host_alone() {
    let mut c = Channels::new();
    let mut m = offer(5, SCSI, 0x2005);
    m[180..182].copy_from_slice(&3u16.to_le_bytes());
    assert_eq!(c.from_host(&m), Retire::None);
    let ch = c.get(5).copied().unwrap();
    assert_eq!((ch.relid, ch.connection, ch.interface, ch.sub_index), (5, 0x2005, SCSI, 3));
    assert_eq!(ch.record, None);
    // Too short, past the event flags, or not an offer: nothing learnt.
    assert_eq!(c.from_host(&offer(6, SCSI, 1)[..195]), Retire::None);
    assert!(c.get(6).is_none());
    assert_eq!(c.from_host(&offer(2048, SCSI, 1)), Retire::None);
    assert!(c.get(2048).is_none());
    c.from_host(&offer(2047, SCSI, 1));
    assert!(c.get(2047).is_some());
    // What the core posts teaches the table nothing about offers.
    c.from_guest(&offer(9, SCSI, 1));
    assert!(c.get(9).is_none());
}

#[test]
fn the_table_holds_sixty_four_channels() {
    let mut c = Channels::new();
    for r in 0..64 {
        c.from_host(&offer(r, SCSI, r));
    }
    c.from_host(&offer(64, SCSI, 64));
    assert!(c.get(63).is_some());
    assert!(c.get(64).is_none(), "the 65th is not kept");
    c.from_host(&rescind(0));
    c.from_host(&offer(64, SCSI, 64));
    assert!(c.get(64).is_some(), "room again after a withdrawal");
}

#[test]
fn a_channel_is_given_once_to_one_record() {
    let mut c = Channels::new();
    assert_eq!(c.bind(5, 0xA), Err(BindError::Unknown));
    c.from_host(&offer(5, SCSI, 0x2005));
    assert!(c.bind(5, 0xA).is_ok());
    assert_eq!(c.bind(5, 0xB), Err(BindError::Bound));
    assert_eq!(c.by_record(0xA).map(|x| x.relid), Some(5));
    assert!(c.by_record(0xB).is_none());
}

#[test]
fn a_withdrawal_or_a_new_offer_retires_the_record() {
    let mut c = Channels::new();
    c.from_host(&offer(5, SCSI, 0x2005));
    c.bind(5, 0xA).unwrap();
    // A fresh offer of the same relid replaces it, unbound.
    assert_eq!(c.from_host(&offer(5, SCSI, 0x3005)), Retire::Record(0xA));
    assert_eq!(c.get(5).map(|x| (x.connection, x.record)), Some((0x3005, None)));
    c.bind(5, 0xB).unwrap();
    c.from_guest(&gpadl_header(5, 7, 1, &[1]));
    assert_eq!(c.from_host(&rescind(5)), Retire::Record(0xB));
    assert!(c.get(5).is_none());
    assert_eq!(c.gpadl_relid(7), None, "its GPADLs go with it");
    assert_eq!(c.from_host(&rescind(5)), Retire::None);
    // An unbound channel withdrawn retires nothing.
    c.from_host(&offer(6, SCSI, 1));
    assert_eq!(c.from_host(&rescind(6)), Retire::None);
}

#[test]
fn an_unload_forgets_everything() {
    let mut c = Channels::new();
    c.from_host(&offer(5, SCSI, 1));
    c.bind(5, 0xA).unwrap();
    c.from_guest(&gpadl_header(5, 7, 1, &[1]));
    assert_eq!(c.from_host(&[17, 0, 0, 0, 0, 0, 0, 0]), Retire::All);
    assert!(c.get(5).is_none() && c.gpadl_relid(7).is_none());
}

#[test]
fn bound_bits_stay_the_driver_s() {
    let mut c = Channels::new();
    for r in [1, 63, 64, 130] {
        c.from_host(&offer(r, SCSI, r));
    }
    c.bind(1, 0xA).unwrap();
    c.bind(64, 0xB).unwrap();
    c.bind(130, 0xC).unwrap();
    assert_eq!(c.bound_mask(0), 1 << 1, "63 is offered, not given");
    assert_eq!(c.bound_mask(1), 1);
    assert_eq!(c.bound_mask(2), 1 << 2);
    assert_eq!(c.bound_mask(3), 0);
}

#[test]
fn a_driver_s_let_go_keeps_its_pages_until_the_host_tears_down() {
    let mut c = Channels::new();
    c.from_host(&offer(5, SCSI, 1));
    c.bind(5, 0xA).unwrap();
    assert_eq!(c.released(0xA), (false, None), "no GPADL: nothing for the host to reach");
    assert_eq!(c.released(0xB), (false, None), "no such record");
    c.from_guest(&gpadl_header(5, 7, 1, &[1]));
    c.from_guest(&gpadl_header(5, 8, 1, &[2]));
    assert_eq!(c.released(0xA), (true, Some(5)), "kept, and the core told");
    assert_eq!(c.released(0xA), (true, None), "told once");
    c.unnote(5);
    assert_eq!(c.released(0xA), (true, Some(5)), "told again when the note did not go");
    assert_eq!(c.from_host(&torndown(7)), Retire::None, "one GPADL still up");
    assert_eq!(c.released(0xA), (true, None));
    assert_eq!(c.from_host(&torndown(99)), Retire::None, "an unknown GPADL");
    assert_eq!(c.from_host(&torndown(8)), Retire::Settle(0xA), "the last torn down");
    assert_eq!(c.released(0xA), (false, None));
    assert_eq!(c.from_host(&torndown(8)), Retire::None, "said twice");
}

#[test]
fn a_gpadl_of_an_unbound_channel_settles_nothing() {
    let mut c = Channels::new();
    c.from_host(&offer(5, SCSI, 1));
    c.from_guest(&gpadl_header(5, 7, 1, &[1]));
    assert_eq!(c.gpadl_relid(7), Some(5));
    assert_eq!(c.from_host(&torndown(7)), Retire::None);
}
