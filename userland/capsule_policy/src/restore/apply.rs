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

//! Putting kept answers back into the store.

use nonos_libc::mk_debug;
use nonos_policy_proto::setup_record::Record;
use nonos_policy_proto::Field;

use crate::push;
use crate::store::{set_bool, set_i8, set_str, set_u64, set_u8};

/*
 * Persistent goes back on first: the answers were only kept because setup
 * chose a mode that keeps state, and vfs asks this field before every write.
 */
pub(super) fn apply(record: Record) {
    let answers = record.answers;
    let _ = set_bool::set(Field::Persistent, true);
    /* Zero, every app on, in a record kept before setup asked. */
    let _ = set_u8::set(Field::AppsOff, record.apps_off);
    /* The Nym mixnet in a record kept before setup asked which network. */
    let _ = set_u8::set(Field::NetworkRoute, record.route);
    let _ = set_u64::set(Field::WallpapersKept, record.wallpapers_kept);
    let _ = set_u8::set(Field::KeyboardLayout, answers.keyboard_layout);
    let _ = set_u8::set(Field::Wallpaper, answers.wallpaper);
    /* Empty in a version 1 record, which leaves each unset as setup would. */
    let _ = set_str::set(Field::Username, answers.username.as_bytes());
    let _ = set_str::set(Field::QwenTier, answers.qwen_tier.as_bytes());
    if set_i8::set(Field::Timezone, answers.timezone) {
        push::on_i8_set(Field::Timezone, answers.timezone);
    }
    /* Empty before version 4 and when none was typed: the system's name stands. */
    let host = record.hostname.as_bytes();
    if !host.is_empty() && set_str::set(Field::Hostname, host) {
        push::on_string_set(Field::Hostname, host);
    }
    say(b"[POLICY] restored the answers setup kept on an earlier boot\n");
}

pub(super) fn refused(part: &str) {
    say(b"[POLICY] kept setup answers refused, nothing restored: bad ");
    say(part.as_bytes());
    say(b"\n");
}

pub(super) fn say(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
