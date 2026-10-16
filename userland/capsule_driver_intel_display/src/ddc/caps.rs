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

//! What a capabilities string says this driver may use: the codes its vcp
//! list names, and for the input source and the power mode the values
//! listed after each. The string is a parenthesised list of keys, each a
//! word and its parenthesised value (`vcp(10 12 60(0F 11) D6(01 04))`);
//! only the vcp key at the top level is read, codes and values as pairs
//! of hex digits with any spacing. A string with no vcp list gives nothing.

use super::vcp::{INPUT_SOURCE, POWER_MODE};

/// The most values kept for one code.
pub const VALUES_MAX: usize = 16;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Caps {
    codes: [u64; 4],
    input: [u8; VALUES_MAX],
    inputs: u8,
    power: [u8; VALUES_MAX],
    powers: u8,
}

impl Caps {
    /// Whether the vcp list names `code`.
    pub fn has(&self, code: u8) -> bool {
        self.codes[code as usize / 64] & 1 << (code % 64) != 0
    }

    /// Whether `code` lists `value` among its values (input source and
    /// power mode only; anything else lists none).
    pub fn lists(&self, code: u8, value: u16) -> bool {
        let (list, n) = match code {
            INPUT_SOURCE => (&self.input, self.inputs),
            POWER_MODE => (&self.power, self.powers),
            _ => return false,
        };
        u8::try_from(value).is_ok_and(|v| list[..n as usize].contains(&v))
    }

    /// The vcp list of `s`, if it has one that closes.
    pub fn parse(s: &[u8]) -> Option<Caps> {
        let list = super::caps_list::vcp_list(s)?;
        let mut c = Caps::default();
        super::caps_walk::each(list, |code, value| match value {
            None => c.codes[code as usize / 64] |= 1 << (code % 64),
            Some(v) if code == INPUT_SOURCE => push(&mut c.input, &mut c.inputs, v),
            Some(v) if code == POWER_MODE => push(&mut c.power, &mut c.powers, v),
            Some(_) => {}
        })?;
        Some(c)
    }
}

fn push(list: &mut [u8; VALUES_MAX], n: &mut u8, v: u8) {
    if let Some(slot) = list.get_mut(*n as usize) {
        (*slot, *n) = (v, *n + 1);
    }
}
