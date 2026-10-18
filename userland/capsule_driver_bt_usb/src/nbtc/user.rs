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

//! PASSKEY and ANSWER, Settings' alone: the user's part in pairing,
//! which only Settings can show and take.

use super::board::Board;
use super::ops::{E_INVAL, E_OK, PART_CONFIRM, PART_ENTER, PART_NONE};
use super::part::Part;
use super::wire::DATA_AT;
use crate::link::Answer;
use nonos_bt_proto::PASSKEY_MAX;

/// PASSKEY: the kind of part (none, show, enter, confirm), then the six
/// digits to show or confirm as a little-endian word, 0 otherwise.
pub fn passkey<K>(b: &Board<K>, tx: &mut [u8]) -> usize {
    tx[DATA_AT..DATA_AT + 5].copy_from_slice(&b.part.write());
    5
}

/// ANSWER: a kind and a little-endian word. ENTER with the digits typed,
/// CONFIRM with 1 when both sides show the same, or NONE to decline. It
/// must answer what is asked; a passkey shown needs no answer.
pub fn answer<K>(b: &mut Board<K>, body: &[u8]) -> i32 {
    let [kind, w @ ..] = body else { return E_INVAL };
    let Ok(w) = <[u8; 4]>::try_from(w) else { return E_INVAL };
    let v = u32::from_le_bytes(w);
    let a = match (*kind, b.part) {
        (PART_NONE, Part::Enter | Part::Confirm(_)) => Answer::Cancel,
        (PART_ENTER, Part::Enter) if v <= PASSKEY_MAX => Answer::Passkey(v),
        (PART_CONFIRM, Part::Confirm(_)) if v <= 1 => Answer::Same(v == 1),
        _ => return E_INVAL,
    };
    (b.part, b.answer) = (Part::None, Some(a));
    E_OK
}
