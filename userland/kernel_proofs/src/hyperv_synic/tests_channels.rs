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

//! The kernel's table of VMBus channels. It learns channels from the host's
//! offers alone and forgets them when withdrawn. It gives a channel only to
//! a driver of an interface a driver exists for, and routes the host's
//! answers for a given channel to that channel's queue. A GPADL stays tied
//! to its channel until the host says it is gone. When a driver lets go
//! with GPADLs up, the kernel owes the host the close and the teardowns,
//! and settles once the last is gone.

use super::channels::{BindError, Channels, Interface, Retire, Route, MAX_GPADLS};
use super::tests_vmbus::{
    close, gpadl_header, msg, offer, open, post, put32, world, RECORD, RELID, SCSI,
};

/// A GUID in Linux's text form, as VMBus carries it.
fn wire(text: &str) -> [u8; 16] {
    let h: String = text.chars().filter(|c| *c != '-').collect();
    let b: Vec<u8> =
        (0..16).map(|i| u8::from_str_radix(&h[2 * i..2 * i + 2], 16).unwrap()).collect();
    let mut w = [0u8; 16];
    w[..4].copy_from_slice(&[b[3], b[2], b[1], b[0]]);
    w[4..6].copy_from_slice(&[b[5], b[4]]);
    w[6..8].copy_from_slice(&[b[7], b[6]]);
    w[8..].copy_from_slice(&b[8..]);
    w
}

fn rescind(relid: u32) -> Vec<u8> {
    let mut m = msg(2, 12);
    put32(&mut m, 8, relid);
    m
}

fn torndown(id: u32) -> Vec<u8> {
    let mut m = msg(12, 12);
    put32(&mut m, 8, id);
    m
}

fn created(relid: u32, id: u32, status: u32) -> Vec<u8> {
    let mut m = msg(10, 20);
    put32(&mut m, 8, relid);
    put32(&mut m, 12, id);
    put32(&mut m, 16, status);
    m
}

fn opened(relid: u32, status: u32) -> Vec<u8> {
    let mut m = msg(6, 20);
    put32(&mut m, 8, relid);
    put32(&mut m, 16, status);
    m
}

fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
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
    assert_eq!(c.from_host(&m), (Route::Core, Retire::None));
    let ch = c.get(5).copied().unwrap();
    assert_eq!((ch.relid, ch.connection, ch.interface, ch.sub_index), (5, 0x2005, SCSI, 3));
    assert_eq!((ch.record, ch.open, ch.releasing), (None, false, false));
    // Too short, past the event flags: nothing learnt.
    c.from_host(&offer(6, SCSI, 1)[..195]);
    assert!(c.get(6).is_none());
    c.from_host(&offer(2048, SCSI, 1));
    assert!(c.get(2048).is_none());
    c.from_host(&offer(2047, SCSI, 1));
    assert!(c.get(2047).is_some());
    // What a guest posts teaches the table nothing about offers.
    c.from_driver(&offer(9, SCSI, 1));
    c.from_core(1, &offer(9, SCSI, 1));
    assert!(c.get(9).is_none());
}

#[test]
fn the_bus_s_connection_is_the_core_s_request_for_offers() {
    let mut c = Channels::new();
    assert_eq!(c.bus(), None);
    c.from_core(1, &msg(14, 40));
    assert_eq!(c.bus(), None, "the contact names none");
    c.from_core(4, &msg(3, 8));
    assert_eq!(c.bus(), Some(4));
}

#[test]
fn the_table_holds_sixty_four_channels() {
    let mut c = Channels::new();
    for r in 0..65 {
        c.from_host(&offer(r, SCSI, r));
    }
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
    let mut c = world();
    assert_eq!(c.from_host(&offer(RELID, SCSI, 0x3005)).1, Retire::Record(RECORD));
    assert_eq!(c.get(RELID).map(|x| (x.connection, x.record)), Some((0x3005, None)));
    c.bind(RELID, 0xB).unwrap();
    c.from_driver(&gpadl_header(RELID, 7, 1, &[1]));
    assert_eq!(c.from_host(&rescind(RELID)).1, Retire::Record(0xB));
    assert!(c.get(RELID).is_none());
    assert_eq!(c.gpadl_relid(7), None, "its GPADLs go with it");
    assert_eq!(c.from_host(&rescind(RELID)).1, Retire::None);
    c.from_host(&offer(6, SCSI, 1));
    assert_eq!(c.from_host(&rescind(6)).1, Retire::None, "an unbound channel retires nothing");
}

#[test]
fn an_unload_forgets_everything() {
    let mut c = world();
    c.from_driver(&gpadl_header(RELID, 7, 1, &[1]));
    assert_eq!(c.from_host(&msg(17, 8)).1, Retire::All);
    assert!(c.get(RELID).is_none() && c.gpadl_relid(7).is_none() && c.bus().is_none());
}

#[test]
fn a_bound_channel_s_answers_go_to_its_queue() {
    let mut c = world();
    c.from_driver(&gpadl_header(RELID, 7, 1, &[1]));
    assert_eq!(c.from_host(&created(RELID, 7, 0)), (Route::Channel(RELID), Retire::None));
    assert_eq!(c.from_host(&opened(RELID, 0)), (Route::Channel(RELID), Retire::None));
    let (n, b) = c.take(RECORD).unwrap();
    assert_eq!((n, u32_at(&b, 0), u32_at(&b, 12)), (20, 10, 7));
    let (n, b) = c.take(RECORD).unwrap();
    assert_eq!((n, u32_at(&b, 0)), (20, 6));
    assert!(c.take(RECORD).is_none());
    assert!(c.take(RECORD + 1).is_none(), "another record has no queue");
    // An unbound or unknown channel's answers stay the core's.
    c.from_host(&offer(6, SCSI, 1));
    assert_eq!(c.from_host(&opened(6, 0)).0, Route::Core);
    assert_eq!(c.from_host(&opened(9, 0)).0, Route::Core);
    assert_eq!(c.from_host(&torndown(99)).0, Route::Core);
}

#[test]
fn a_full_queue_drops_the_answer() {
    let mut c = world();
    for _ in 0..8 {
        assert_eq!(c.from_host(&opened(RELID, 0)).0, Route::Channel(RELID));
    }
    assert_eq!(c.from_host(&opened(RELID, 0)).0, Route::Dropped(RELID));
    c.take(RECORD).unwrap();
    assert_eq!(c.from_host(&opened(RELID, 0)).0, Route::Channel(RELID));
}

#[test]
fn a_gpadl_the_host_failed_to_make_is_untied() {
    let mut c = world();
    c.from_driver(&gpadl_header(RELID, 7, 1, &[1]));
    c.from_driver(&gpadl_header(RELID, 8, 1, &[2]));
    assert_eq!(c.from_host(&created(RELID, 7, 0xC000_0001)), (Route::Channel(RELID), Retire::None));
    assert_eq!(c.gpadl_relid(7), None);
    assert_eq!(c.gpadl_relid(8), Some(RELID));
    // A failure naming another channel's GPADL unties nothing.
    let mut other = world();
    other.from_driver(&gpadl_header(RELID, 7, 1, &[1]));
    other.from_host(&created(6, 7, 1));
    assert_eq!(other.gpadl_relid(7), Some(RELID));
    // The last one failed: nothing left for the host to reach.
    assert_eq!(c.from_host(&created(RELID, 8, 1)).1, Retire::Settle(RECORD));
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
fn opening_and_closing_are_noted() {
    let mut c = world();
    post(&mut c, &gpadl_header(RELID, 7, 1, &[0x800])).unwrap();
    post(&mut c, &open(RELID, 7, 0)).unwrap();
    assert!(c.get(RELID).unwrap().open);
    post(&mut c, &close(RELID)).unwrap();
    assert!(!c.get(RELID).unwrap().open);
}

#[test]
fn a_driver_s_let_go_is_cleaned_up_by_the_kernel() {
    let mut c = world();
    let mut owed = [([0u8; 16], 0usize); MAX_GPADLS + 1];
    assert!(!c.release(RECORD), "no GPADL: nothing for the host to reach");
    assert!(!c.release(0xBAD), "no such record");
    assert_eq!(c.owed(&mut owed), None);
    post(&mut c, &gpadl_header(RELID, 7, 1, &[0x800])).unwrap();
    post(&mut c, &gpadl_header(RELID, 8, 1, &[0x801])).unwrap();
    post(&mut c, &open(RELID, 7, 0)).unwrap();
    c.from_host(&opened(RELID, 0));
    assert!(c.release(RECORD));
    assert!(c.take(RECORD).is_none(), "the gone driver's queue is emptied");
    // Its close, then a teardown of each GPADL, on its relid.
    let (relid, n) = c.owed(&mut owed).unwrap();
    assert_eq!((relid, n), (RELID, 3));
    assert_eq!((u32_at(&owed[0].0, 0), u32_at(&owed[0].0, 8), owed[0].1), (7, RELID, 12));
    let ids: Vec<(u32, u32, usize)> =
        owed[1..3].iter().map(|(m, l)| (u32_at(m, 0), u32_at(m, 12), *l)).collect();
    assert_eq!(ids, [(11, 7, 16), (11, 8, 16)]);
    // Not sent: owed again. Sent: owed no more.
    c.cleaned(RELID, false);
    assert_eq!(c.owed(&mut owed).map(|x| x.1), Some(3));
    c.cleaned(RELID, true);
    assert_eq!(c.owed(&mut owed), None);
    assert!(!c.get(RELID).unwrap().open);
    // The host's word: the kernel's own teardowns are no driver's answers.
    assert_eq!(c.from_host(&torndown(7)), (Route::Core, Retire::None), "one still up");
    assert_eq!(c.from_host(&torndown(8)), (Route::Core, Retire::Settle(RECORD)));
    assert!(!c.get(RELID).unwrap().releasing);
    assert!(!c.release(RECORD), "nothing left to keep");
}

#[test]
fn a_closed_channel_is_only_torn_down() {
    let mut c = world();
    let mut owed = [([0u8; 16], 0usize); MAX_GPADLS + 1];
    post(&mut c, &gpadl_header(RELID, 7, 1, &[0x800])).unwrap();
    assert!(c.release(RECORD));
    let (_, n) = c.owed(&mut owed).unwrap();
    assert_eq!((n, u32_at(&owed[0].0, 0)), (1, 11));
}

#[test]
fn a_driver_s_own_teardown_answer_is_its_own() {
    let mut c = world();
    post(&mut c, &gpadl_header(RELID, 7, 1, &[0x800])).unwrap();
    assert_eq!(c.from_host(&torndown(7)), (Route::Channel(RELID), Retire::Settle(RECORD)));
    assert_eq!(c.take(RECORD).map(|(n, b)| (n, u32_at(&b, 0), u32_at(&b, 8))), Some((12, 12, 7)));
}
