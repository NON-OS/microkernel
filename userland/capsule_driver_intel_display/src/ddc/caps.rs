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
//! list names, and the values listed after the input source and the power
//! mode. Only the top-level vcp key (`vcp(10 12 60(0F 11) D6(01 04))`) is
//! read, codes and values as hex pairs with any spacing; no list, nothing.

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

    /// The values listed after `code`: the input source's and the power
    /// mode's; any other code lists none.
    pub fn listed(&self, code: u8) -> &[u8] {
        match code {
            INPUT_SOURCE => &self.input[..self.inputs as usize],
            POWER_MODE => &self.power[..self.powers as usize],
            _ => &[],
        }
    }

    /// Whether `code` lists `value` among its values.
    pub fn lists(&self, code: u8, value: u16) -> bool {
        u8::try_from(value).is_ok_and(|v| self.listed(code).contains(&v))
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
