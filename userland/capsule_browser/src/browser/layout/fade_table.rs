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

use alloc::string::String;
use alloc::vec::Vec;

use spin::Mutex;

/* Gradient mask-image values, each kept once. Fx is Copy, so a box holds
 * only its value's index here plus one. The table is shared by every page
 * and never cleared, so an id keeps its meaning; past MAX_FADES distinct
 * values a new one gets 0 and its box paints unmasked. */
const MAX_FADES: usize = 256;

static FADES: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// The id of mask-image value `v` when each of its layers is a gradient;
/// 0 for none, for a url layer, or once the table is full.
pub fn fade_id(v: &str) -> u16 {
    let Some(layers) = super::fade_layers::gradient_layers(v) else { return 0 };
    let mut t = FADES.lock();
    if let Some(i) = t.iter().position(|s| *s == layers) {
        return i as u16 + 1;
    }
    if t.len() >= MAX_FADES {
        return 0;
    }
    t.push(layers);
    t.len() as u16
}

/// The comma-separated gradient layers behind `id`.
pub fn fade_value(id: u16) -> Option<String> {
    FADES.lock().get((id as usize).checked_sub(1)?).cloned()
}
