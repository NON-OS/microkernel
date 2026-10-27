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

//! A selection a guest sets is offered to every device it holds: a new
//! offer at a server id, each mime type its source offered, then the
//! selection; and a receive on that offer goes to that source as its send.

use super::clip_wire::{events, guest, Ev, DEVICES, SOURCES};
use crate::ops::data::*;
use crate::wayland_clip::offer::SERVER_START;
use crate::wayland_clip::set::{forward, set, target};
use crate::wayland_clip::which::Which;
use crate::wayland_object::Object;

#[test]
fn a_selection_is_offered_to_every_device_with_its_mime_types() {
    let (mut objects, mut clip) = guest();
    let mut out = Vec::new();
    let held = set(&mut objects, &mut clip, (Which::Data, SOURCES[0]), None, &mut out);
    assert_eq!(held, Some(SOURCES[0]));
    let offers = [SERVER_START, SERVER_START + 1];
    let mut want = Vec::new();
    for (device, offer) in DEVICES.into_iter().zip(offers) {
        want.push(Ev::Word(device, DEVICE_DATA_OFFER, offer));
        want.push(Ev::Text(offer, OFFER_OFFER, b"text/plain;charset=utf-8".to_vec()));
        want.push(Ev::Text(offer, OFFER_OFFER, b"image/png".to_vec()));
        want.push(Ev::Word(device, DEVICE_SELECTION, offer));
    }
    assert_eq!(events(&out, |o, _| o >= SERVER_START), want);
    for offer in offers {
        assert!(objects.get(offer) == Some(Object::DataOffer));
        assert!(objects.at_least(offer, 3), "an offer has its device's version");
        assert_eq!(target(&objects, &clip, Which::Data, offer), Some(SOURCES[0]));
    }
}

#[test]
fn a_receive_is_forwarded_to_the_source_as_its_send() {
    let (mut objects, mut clip) = guest();
    let mut out = Vec::new();
    set(&mut objects, &mut clip, (Which::Data, SOURCES[0]), None, &mut out);
    let source = target(&objects, &clip, Which::Data, SERVER_START).expect("a source");
    let mut sent = Vec::new();
    forward(Which::Data, source, b"text/plain;charset=utf-8", &mut sent);
    let want = vec![Ev::Text(SOURCES[0], SOURCE_SEND, b"text/plain;charset=utf-8".to_vec())];
    assert_eq!(events(&sent, |_, _| true), want);
    assert_eq!(target(&objects, &clip, Which::Primary, SERVER_START), None, "not its kind");
    assert_eq!(target(&objects, &clip, Which::Data, 99), None, "an offer never made");
    // A source the client destroyed is sent nothing more.
    objects.drop_id(SOURCES[0]);
    assert_eq!(target(&objects, &clip, Which::Data, SERVER_START), None);
}
