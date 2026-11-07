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

//! Any bytes as a run of packets from a controller, each a length octet and
//! that many bytes: every one is read as an HCI event and as an ACL packet.
//! No reader panics. An event's parameters and an ACL packet's data are the
//! length their header states, a handle is never past HANDLE_MAX, and every
//! report list holds the count it names with advertising data of at most 31
//! octets. ACL data is joined by the reassembler, which only ever hands back
//! a whole basic frame within its bound; each frame is read on its channel
//! as LE signaling (a reply always parses back with the request's
//! identifier, and a timing update is only taken when acceptable) or as an
//! ATT response.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_bt_wire::att::response::{error_rsp, found_ranges, handle_pairs, handle_value};
use nonos_bt_wire::hci::data_length::MaxDataLength;
use nonos_bt_wire::hci::directed::directed_reports;
use nonos_bt_wire::hci::event_code::*;
use nonos_bt_wire::hci::le_meta::{split, LeConnectionComplete, LeLtkRequest};
use nonos_bt_wire::hci::reports::{adv_reports, completed_packets};
use nonos_bt_wire::hci::{
    parse_acl, parse_command, parse_event, CommandComplete, CommandStatus,
    DisconnectionComplete, EncryptionChange, HANDLE_MAX,
};
use nonos_bt_wire::l2cap::{answer_central, cid, parse_basic, parse_signal, Reassembler};

const MAX_PDU: usize = 512;

fn event(packet: &[u8]) {
    let _ = parse_command(packet);
    let Ok(e) = parse_event(packet) else { return };
    assert_eq!(e.params.len() + 2, packet.len());
    let p = e.params;
    match e.code {
        COMMAND_COMPLETE => {
            if let Ok(c) = CommandComplete::parse(p) {
                let _ = c.ret.split_first().map(|(_, ret)| MaxDataLength::parse(ret));
            }
        }
        COMMAND_STATUS => drop(CommandStatus::parse(p)),
        DISCONNECTION_COMPLETE => drop(DisconnectionComplete::parse(p)),
        ENCRYPTION_CHANGE => drop(EncryptionChange::parse(p)),
        NUMBER_OF_COMPLETED_PACKETS => {
            if let Ok(list) = completed_packets(p) {
                assert_eq!(list.len(), usize::from(p[0]));
            }
        }
        LE_META => le_meta(p),
        _ => {}
    }
}

fn le_meta(p: &[u8]) {
    let Ok((code, sub)) = split(p) else { return };
    match code {
        LE_CONNECTION_COMPLETE => drop(LeConnectionComplete::parse(sub)),
        LE_LTK_REQUEST => drop(LeLtkRequest::parse(sub)),
        LE_ADVERTISING_REPORT => {
            if let Ok(list) = adv_reports(sub) {
                assert_eq!(list.len(), usize::from(sub[0]));
                assert!(list.iter().all(|r| r.data.len() <= 31));
            }
        }
        LE_DIRECTED_ADVERTISING_REPORT => {
            if let Ok(list) = directed_reports(sub) {
                assert_eq!(list.len(), usize::from(sub[0]));
            }
        }
        _ => {}
    }
}

fn frame(pdu: &[u8]) {
    assert!(pdu.len() <= MAX_PDU, "a {}-octet PDU past the bound", pdu.len());
    let Ok((channel, payload)) = parse_basic(pdu) else { panic!("a joined PDU that is not a frame") };
    match channel {
        cid::LE_SIGNALING => signaling(payload),
        cid::ATT => att(payload),
        _ => {}
    }
}

fn signaling(payload: &[u8]) {
    if let Ok(s) = parse_signal(payload) {
        assert!(s.id != 0 && s.data.len() + 4 == payload.len());
    }
    let Ok(a) = answer_central(payload) else { return };
    if let Some(reply) = a.reply {
        let back = parse_signal(&reply).map(|r| r.id);
        assert_eq!(back.ok(), parse_signal(payload).ok().map(|s| s.id));
    }
    assert!(a.update.is_none_or(|u| u.acceptable()));
}

fn att(payload: &[u8]) {
    let Some((_, p)) = payload.split_first() else { return };
    let _ = error_rsp(p);
    let _ = found_ranges(p);
    let _ = handle_value(p);
    if let Some((&each, list)) = p.split_first() {
        if let Ok(pairs) = handle_pairs(list, usize::from(each)) {
            assert!(pairs.iter().all(|(_, v)| v.len() + 2 == usize::from(each)));
        }
    }
}

fuzz_target!(|data: &[u8]| {
    let mut join = Reassembler::new(MAX_PDU);
    let mut rest = data;
    while let Some((&n, tail)) = rest.split_first() {
        let (packet, next) = tail.split_at(usize::from(n).min(tail.len()));
        rest = next;
        event(packet);
        let Ok((head, acl)) = parse_acl(packet) else { continue };
        assert!(head.handle <= HANDLE_MAX && acl.len() == usize::from(head.len));
        if let Ok(Some(pdu)) = join.push(head.boundary, acl) {
            frame(&pdu);
        }
    }
});
