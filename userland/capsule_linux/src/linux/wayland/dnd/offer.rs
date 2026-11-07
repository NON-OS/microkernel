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

//! What the client under a drag says of its offer: the type it would take
//! (accept), told to the source as its target; and the actions it takes
//! and prefers (set_actions), from which the action is chosen and told to
//! both, when it changes. The source's own actions are set_actions on it.

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::super::ops::data::*;
use super::super::out::Event;
use super::action::{chosen, one_or_none, ALL};

/// wl_data_source.set_actions, before its drag starts.
pub fn source_actions(guest: &mut Guest, source: u32, args: &mut Args<'_>) {
    let actions = args.u32().unwrap_or(0) & ALL;
    let kept = &mut guest.scene.dnd.actions;
    kept.retain(|a| a.0 != source);
    kept.push((source, actions));
}

/// wl_data_offer.accept(serial, mime or null).
pub fn accept(guest: &mut Guest, offer: u32, args: &mut Args<'_>) {
    let _serial = args.u32();
    let mime = args.string().filter(|m| !m.is_empty());
    if !guest.scene.dnd.offer_of(offer) {
        return;
    }
    let Some(d) = guest.scene.dnd.drag.as_mut().filter(|d| !d.dropped) else { return };
    d.accepted = mime.is_some();
    let Some(source) = d.source else { return };
    let ev = Event::new(source, SOURCE_TARGET);
    match mime {
        Some(m) => ev.string(m).send(&mut guest.display.to_client),
        None => ev.u32(0).send(&mut guest.display.to_client),
    }
}

/// wl_data_offer.set_actions(actions, preferred).
pub fn set_actions(guest: &mut Guest, offer: u32, args: &mut Args<'_>) {
    let (Some(dest), Some(preferred)) = (args.u32(), args.u32()) else { return };
    if !guest.scene.dnd.offer_of(offer) || dest & !ALL != 0 || !one_or_none(preferred) {
        return;
    }
    let offered = guest.scene.dnd.drag.as_ref().map(|d| d.source);
    let offered = guest.scene.dnd.offered(offered.flatten());
    let Some(d) = guest.scene.dnd.drag.as_mut() else { return };
    d.dest = (dest, preferred);
    let action = chosen(offered, dest, preferred);
    if action == d.action {
        return;
    }
    d.action = action;
    let source = d.source;
    let to = &mut guest.display.to_client;
    Event::new(offer, OFFER_ACTION).u32(action).send(to);
    if let Some(s) = source.filter(|&s| guest.objects.at_least(s, 3)) {
        Event::new(s, SOURCE_ACTION).u32(action).send(to);
    }
}
