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

//! What Settings changed, kept on a machine that keeps state.
//!
//! Setup's answers (setup_record) are what the person chose at first boot.
//! Everything changed afterwards in Settings lives in the policy store, which
//! is memory: on an installed machine a wallpaper chosen, a volume turned
//! down or a name corrected would come back as setup left it at every boot.
//! The policy service writes this record whenever a value changes on a
//! machine that keeps state, and puts it back after the answers at boot, so
//! a change made in Settings stays made.
//!
//! The record is `NSV1`, then one entry per kept field: its id (u16 LE), its
//! kind, the value's length, the value. A record that does not frame whole is
//! refused whole; an entry for a field this build does not keep is skipped,
//! so a record from a later build still restores what this one knows.

use alloc::vec::Vec;

use crate::field::Field;
use crate::field_decode::decode;
use crate::field_kind::kind_of;
use crate::kind::{KIND_BOOL, KIND_I8, KIND_STR, KIND_U64, KIND_U8};
use crate::limits::STR_MAX;

pub const SETTINGS_DIR: &[u8] = b"/nonos/settings";
pub const VALUES_PATH: &[u8] = b"/nonos/settings/values";

const MAGIC: [u8; 4] = *b"NSV1";

/// The fields a person sets in Settings or setup and expects to stay set.
/// Persistent is not among them: it is what decides whether any are kept.
pub const KEPT: &[Field] = &[
    Field::Username,
    Field::Hostname,
    Field::QwenTier,
    Field::Timezone,
    Field::ClockFormat24,
    Field::NotificationsEnabled,
    Field::WifiRadio,
    Field::Wallpaper,
    Field::WallpapersKept,
    Field::MouseSensitivity,
    Field::SoundEnabled,
    Field::Volume,
    Field::AlertSounds,
    Field::KernelPreempt,
    Field::NetworkRoute,
    Field::KeyboardLayout,
    Field::AppsOff,
];

/// The most a record can be, every kept field at its widest.
pub const VALUES_MAX: usize = 4 + KEPT.len() * (4 + STR_MAX);

/// One kept value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Value<'a> {
    Bool(bool),
    U8(u8),
    I8(i8),
    U64(u64),
    Str(&'a [u8]),
}

impl Value<'_> {
    fn kind(&self) -> u8 {
        match self {
            Value::Bool(_) => KIND_BOOL,
            Value::U8(_) => KIND_U8,
            Value::I8(_) => KIND_I8,
            Value::U64(_) => KIND_U64,
            Value::Str(_) => KIND_STR,
        }
    }
}

/// The record for `values`. A value whose kind is not its field's, or a
/// field not kept, is left out rather than written wrong.
pub fn encode<'a>(values: impl IntoIterator<Item = (Field, Value<'a>)>) -> Vec<u8> {
    let mut out = Vec::from(MAGIC);
    for (field, value) in values {
        if !KEPT.contains(&field) || kind_of(field) != value.kind() {
            continue;
        }
        let (small, body): ([u8; 8], &[u8]);
        let len = match value {
            Value::Bool(b) => {
                small = [b as u8, 0, 0, 0, 0, 0, 0, 0];
                1
            }
            Value::U8(v) => {
                small = [v, 0, 0, 0, 0, 0, 0, 0];
                1
            }
            Value::I8(v) => {
                small = [v as u8, 0, 0, 0, 0, 0, 0, 0];
                1
            }
            Value::U64(v) => {
                small = v.to_le_bytes();
                8
            }
            Value::Str(s) if s.len() <= STR_MAX => {
                small = [0; 8];
                s.len()
            }
            Value::Str(_) => continue,
        };
        body = match value {
            Value::Str(s) => s,
            _ => &small[..len],
        };
        out.extend_from_slice(&(field as u32 as u16).to_le_bytes());
        out.push(value.kind());
        out.push(len as u8);
        out.extend_from_slice(body);
    }
    out
}

/// Every kept value in `raw`, or `None` when the record does not frame whole.
pub fn decode_values(raw: &[u8]) -> Option<Vec<(Field, Value<'_>)>> {
    let mut rest = raw.strip_prefix(&MAGIC)?;
    let mut out = Vec::new();
    while !rest.is_empty() {
        let head = rest.get(..4)?;
        let id = u16::from_le_bytes([head[0], head[1]]) as u32;
        let (kind, len) = (head[2], head[3] as usize);
        let body = rest.get(4..4 + len)?;
        rest = &rest[4 + len..];
        let Some(field) = decode(id).filter(|f| KEPT.contains(f) && kind_of(*f) == kind) else {
            continue;
        };
        let value = match (kind, body) {
            (KIND_BOOL, [b]) => Value::Bool(*b != 0),
            (KIND_U8, [v]) => Value::U8(*v),
            (KIND_I8, [v]) => Value::I8(*v as i8),
            (KIND_U64, b) if b.len() == 8 => Value::U64(u64::from_le_bytes(b.try_into().ok()?)),
            (KIND_STR, s) if s.len() <= STR_MAX => Value::Str(s),
            _ => return None,
        };
        out.push((field, value));
    }
    Some(out)
}
