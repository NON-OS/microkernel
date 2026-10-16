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
//! What a Key Data reading must hold, given what the building says.

use super::kde::{first, Item, Kind};
use super::rng::within;
use nonos_wifi_core::eapol::kde::KeyData;

pub fn agree(kd: &KeyData, items: &[Item], data: &[u8]) {
    let f = |k| first(items, k);
    assert_eq!(kd.rsne, f(Kind::Rsne).map(|i| &i.bytes[..]));
    assert_eq!(kd.rsnxe, f(Kind::Rsnxe).map(|i| &i.bytes[..]));
    let gtk = kd.gtk.map(|g| (g.key_id, g.key));
    assert_eq!(gtk, f(Kind::Gtk).map(|i| (i.bytes[6] & 3, &i.bytes[8..])));
    for (got, kind) in [(kd.igtk, Kind::Igtk), (kd.bigtk, Kind::Bigtk)] {
        let want = f(kind).map(|i| {
            (u16::from_le_bytes([i.bytes[6], i.bytes[7]]), i.bytes[8..14].to_vec(), &i.bytes[14..])
        });
        assert_eq!(got.map(|g| (g.key_id, g.ipn.to_vec(), g.key)), want, "{kind:?}");
        assert!(got.is_none_or(|g| within(data, g.key)));
    }
    assert_eq!(kd.transition_disable, f(Kind::Disable).map(|i| i.bytes[6]));
}
