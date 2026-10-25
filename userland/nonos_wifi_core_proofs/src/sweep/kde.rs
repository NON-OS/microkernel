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
//! Key Data built sound (802.11-2020, 12.7.2): elements and KDEs, each kind
//! with what the building says its first sound instance holds.

use super::kde_items::item;
use super::rng::Rng;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Rsne,
    Rsnxe,
    Gtk,
    Igtk,
    Bigtk,
    Disable,
    Other,
}

pub struct Item {
    pub kind: Kind,
    pub at: usize,
    pub bytes: Vec<u8>,
}

/// One to ten items, and the field they make.
pub fn build(rng: &mut Rng) -> (Vec<Item>, Vec<u8>) {
    let (mut items, mut data) = (Vec::new(), Vec::new());
    for _ in 0..1 + rng.below(10) {
        let (kind, bytes) = item(rng);
        items.push(Item { kind, at: data.len(), bytes: bytes.clone() });
        data.extend(bytes);
    }
    (items, data)
}

/// The first sound item of `kind` among `items`; a Transition Disable
/// KDE counts only with its data octet.
pub fn first(items: &[Item], kind: Kind) -> Option<&Item> {
    items.iter().find(|i| i.kind == kind && (kind != Kind::Disable || i.bytes.len() > 6))
}
