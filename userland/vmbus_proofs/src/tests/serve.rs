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

//! The bus kept: offers added and withdrawn, the bound on one look, the
//! table's bound, the console, and the host's messages read only as far as
//! they reach.

use nonos_libc::logged;

use super::util::{chan, host, up, usual, HEARTBEAT, IDE, KBD, NET, ROOT, SCSI};
use crate::give::give;
use crate::guid::{name, Guid};
use crate::model::{offer_bytes, rescind_bytes};
use crate::msg;
use crate::say;
use crate::serve::step;
use crate::setup;

#[test]
fn the_console_names_the_version_and_each_channel_by_its_interface() {
    let (d, _) = up(usual());
    say::up(d.bus, &d.offers);
    let log = logged();
    assert_eq!(
        log[0],
        "driver.vmbus0: VMBus 5.3 on connection 4, 5 channels offered, 3 given to drivers\n"
    );
    assert_eq!(
        log[1..],
        [
            "driver.vmbus0: channel 1: network, given to its driver\n",
            "driver.vmbus0: channel 2: scsi, given to its driver\n",
            "driver.vmbus0: channel 3: keyboard\n",
            "driver.vmbus0: channel 4: heartbeat\n",
            "driver.vmbus0: channel 5: scsi, sub-channel 1, given to its driver\n",
        ]
    );
    assert!(log.iter().all(|l| !l.contains("a5") && !l.contains("A5")), "no instance GUID");
}

#[test]
fn an_offer_added_later_is_kept_and_named() {
    let (mut d, m) = up(vec![chan(1, NET, 0)]);
    m.send(&offer_bytes(&chan(9, KBD, 0)));
    assert_eq!(step(&mut d), 1);
    assert!(d.offers.iter().any(|o| o.relid == 9));
    assert_eq!(logged().last().unwrap(), "driver.vmbus0: channel 9: keyboard\n");
    assert_eq!(step(&mut d), 0, "nothing more");
}

#[test]
fn a_withdrawn_offer_is_forgotten_and_released() {
    let (mut d, m) = up(usual());
    m.send(&rescind_bytes(2));
    step(&mut d);
    assert!(!d.offers.iter().any(|o| o.relid == 2));
    assert_eq!(logged().last().unwrap(), "driver.vmbus0: channel 2 (scsi) withdrawn\n");
    assert_eq!(m.0.borrow().released, [2]);
    m.send(&rescind_bytes(77));
    step(&mut d);
    assert_eq!(m.0.borrow().released, [2, 77], "an unknown relid is released too");
}

#[test]
fn a_withdrawal_among_the_offers_is_answered() {
    let m = host(usual());
    m.0.borrow_mut().rescind_among = Some(3);
    let d = setup::run(ROOT).expect("up");
    assert_eq!(m.0.borrow().released, [3]);
    assert_eq!(d.offers.count(), 4);
    assert!(!d.offers.iter().any(|o| o.relid == 3));
}

#[test]
fn one_look_handles_a_bounded_number_of_messages() {
    let (mut d, m) = up(vec![]);
    for i in 0..40 {
        m.send(&offer_bytes(&chan(100 + i, HEARTBEAT, 0)));
    }
    assert_eq!(step(&mut d), crate::constants::MESSAGES_PER_LOOK);
    assert_eq!(d.offers.count(), crate::constants::MESSAGES_PER_LOOK);
}

#[test]
fn the_table_is_bounded() {
    let (mut d, m) = up(vec![]);
    for i in 0..70 {
        m.send(&offer_bytes(&chan(200 + i, SCSI, 0)));
    }
    while step(&mut d) > 0 {}
    assert_eq!(d.offers.count(), crate::constants::MAX_CHANNELS);
    assert_eq!(d.offers.dropped, 6);
    assert!(logged().last().unwrap().ends_with("not kept: the table is full\n"));
}

#[test]
fn an_offer_sent_again_replaces_the_first() {
    let (mut d, m) = up(vec![chan(1, NET, 0)]);
    m.send(&offer_bytes(&chan(1, KBD, 0)));
    step(&mut d);
    assert_eq!(d.offers.count(), 1);
    assert_eq!(name(d.offers.iter().next().unwrap().interface), "keyboard");
}

#[test]
fn host_messages_are_read_only_as_far_as_they_reach() {
    let full = offer_bytes(&chan(4, NET, 2));
    let o = msg::offer(&full).unwrap();
    assert_eq!((o.relid, o.sub_channel, o.connection), (4, 2, 0x1004));
    assert_eq!(o.interface, NET);
    assert_eq!(o.monitor, None, "no monitor allocated");
    assert_eq!(msg::offer(&full[..195]), None);
    assert_eq!(msg::offer(&full[..188]), None, "short of the monitor bytes");
    let mut monitored = full.clone();
    monitored[188] = 7;
    assert_eq!(msg::offer(&monitored).unwrap().monitor, None, "an id without the bit");
    monitored[189] = 1;
    assert_eq!(msg::offer(&monitored).unwrap().monitor, Some(7));
    assert_eq!(msg::rescind(&rescind_bytes(5)[..11]), None);
    assert_eq!(msg::version_response(&[15, 0, 0, 0, 0, 0, 0, 0]), None);
    assert_eq!(msg::version_response(&[15, 0, 0, 0, 0, 0, 0, 0, 1, 0]), Some((true, 0)));
    assert_eq!(name(Guid::default()), "unknown");
}

#[test]
fn an_offer_cut_short_during_bring_up_fails_it() {
    let m = host(usual());
    m.0.borrow_mut().short_offers = true;
    assert_eq!(setup::run(ROOT).err(), Some("offer cut short"));
    assert!(m.0.borrow().unloaded, "the contact undone");
}

#[test]
fn an_offer_cut_short_later_is_dropped() {
    let (mut d, m) = up(vec![]);
    m.send(&offer_bytes(&chan(9, KBD, 0))[..150]);
    assert_eq!(step(&mut d), 1);
    assert_eq!(d.offers.count(), 0);
}

#[test]
fn a_slot_message_short_of_its_header_or_of_another_type_is_dropped() {
    let (mut d, m) = up(vec![]);
    m.send_raw(vec![1, 0, 0, 0, 0, 0, 0, 0]);
    let mut other = crate::model::synic(&offer_bytes(&chan(9, KBD, 0)));
    other[0] = 5;
    m.send_raw(other);
    assert_eq!(step(&mut d), 0, "the short one is no VMBus message");
    assert_eq!(step(&mut d), 0, "nor is one of another type");
    assert_eq!(d.offers.count(), 0);
    m.send(&offer_bytes(&chan(9, KBD, 0)));
    assert_eq!(step(&mut d), 1, "the slot behind them is read");
}

#[test]
fn storage_and_network_channels_are_given_records_for_their_drivers() {
    let (d, _) = up(vec![
        chan(1, NET, 0),
        chan(2, SCSI, 0),
        chan(3, KBD, 0),
        chan(4, HEARTBEAT, 0),
        chan(5, SCSI, 1),
        chan(6, IDE, 0),
    ]);
    let given: Vec<(u32, u16)> = nonos_libc::given().iter().map(|&(_, r, dev)| (r, dev)).collect();
    assert_eq!(given, [(1, 0x4E45), (2, 0x5343), (5, 0x5343), (6, 0x4944)]);
    let records: Vec<(u32, bool)> =
        d.offers.iter().map(|o| (o.relid, o.record.is_some())).collect();
    assert_eq!(records, [(1, true), (2, true), (3, false), (4, false), (5, true), (6, true)]);
    let first = nonos_libc::given()[0].0;
    assert_eq!(d.offers.iter().next().unwrap().record, Some(first));
}

#[test]
fn a_channel_offered_later_is_given_and_a_withdrawn_one_given_again_afresh() {
    let (mut d, m) = up(vec![chan(1, NET, 0)]);
    m.send(&offer_bytes(&chan(9, SCSI, 0)));
    assert_eq!(step(&mut d), 1);
    assert_eq!(logged().last().unwrap(), "driver.vmbus0: channel 9: scsi, given to its driver\n");
    let first = nonos_libc::given().last().unwrap().0;
    m.send(&rescind_bytes(9));
    step(&mut d);
    assert!(!d.offers.iter().any(|o| o.relid == 9));
    m.send(&offer_bytes(&chan(9, SCSI, 0)));
    step(&mut d);
    let again = d.offers.iter().find(|o| o.relid == 9).unwrap().record.unwrap();
    assert_ne!(again, first, "a withdrawn channel's record is never handed out again");
}

#[test]
fn a_channel_the_kernel_will_not_give_is_said_and_left_without_a_record() {
    let (d, _) = up(vec![chan(1, NET, 0)]);
    let mut again = *d.offers.iter().next().unwrap();
    again.record = None;
    give(&d.hv, &mut again);
    assert_eq!(again.record, None);
    assert_eq!(
        logged().last().unwrap(),
        "driver.vmbus0: channel 1 (network) not given to a driver: already given\n"
    );
    let mut unknown = again;
    unknown.relid = 77;
    give(&d.hv, &mut unknown);
    assert_eq!(
        logged().last().unwrap(),
        "driver.vmbus0: channel 77 (network) not given to a driver: not known to the kernel\n"
    );
    assert_eq!(nonos_libc::given().len(), 1, "no second record");
}

#[test]
fn a_given_channel_s_answers_never_reach_the_core() {
    let (mut d, m) = up(vec![chan(2, SCSI, 0)]);
    let mut opened = vec![0u8; 20];
    opened[0] = 6;
    opened[8..12].copy_from_slice(&2u32.to_le_bytes());
    m.send(&opened);
    assert_eq!(step(&mut d), 0, "the driver's answer is the driver's");
    // Another channel's answer, given to no driver, still reaches the core.
    let (mut d, m) = up(vec![chan(3, KBD, 0)]);
    opened[8..12].copy_from_slice(&3u32.to_le_bytes());
    m.send(&opened);
    assert_eq!(step(&mut d), 1);
}
