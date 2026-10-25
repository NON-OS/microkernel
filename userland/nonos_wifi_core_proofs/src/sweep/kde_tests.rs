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
//! The Key Data walk: what the building says, padded or not; a lying
//! length, or a first GTK, IGTK or BIGTK too short to hold its key, fails
//! the whole field, so nothing in it is half-read.

use super::kde::{build, first, Kind};
use super::kde_check::agree;
use super::rng::each;
use nonos_wifi_core::eapol::kde::parse_key_data;

#[test]
fn sound_key_data_reads_as_built_padded_or_not() {
    each(|rng| {
        let (items, mut data) = build(rng);
        agree(&parse_key_data(&data).expect("sound"), &items, &data);
        data.push(0xdd);
        data.extend(vec![0; rng.below(8)]);
        agree(&parse_key_data(&data).expect("padded"), &items, &data);
    });
}

#[test]
fn a_lying_length_or_a_short_first_key_fails_the_field() {
    each(|rng| {
        let (items, mut data) = build(rng);
        let k = rng.below(items.len());
        let at = items[k].at;
        let room = data.len() - at - 2;
        if room < 255 && !(data[at] == 0xdd && data[at + 2..].iter().all(|&b| b == 0)) {
            let mut lie = data.clone();
            lie[at + 1] = (room + 1 + rng.below(255 - room)) as u8;
            assert!(parse_key_data(&lie).is_none(), "a length past the field");
        }
        let Some(key) =
            [Kind::Gtk, Kind::Igtk, Kind::Bigtk].into_iter().find_map(|k| first(&items, k))
        else {
            return;
        };
        let short = if key.kind == Kind::Gtk { 3 } else { 9 };
        let cut = rng.below(short);
        let tail = data.split_off(key.at + 6 + cut);
        let rest = &tail[key.bytes.len() - 6 - cut..];
        data[key.at + 1] = (4 + cut) as u8;
        data.extend_from_slice(rest);
        assert!(parse_key_data(&data).is_none(), "{:?} with {cut} data octets", key.kind);
    });
}
