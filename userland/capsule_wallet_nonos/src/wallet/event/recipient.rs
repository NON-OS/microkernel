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

use crate::wallet::state::State;

pub fn recipient(state: &State) -> Option<[u8; 20]> {
    if state.send.to_len != 40 {
        return None;
    }
    let digit = |c: u8| super::hex_digit::hex_digit(u32::from(c));
    let mut out = [0u8; 20];
    for (byte, pair) in out.iter_mut().zip(state.send.to_hex.chunks_exact(2)) {
        let &[hi, lo] = pair else { return None };
        *byte = (digit(hi)? << 4) | digit(lo)?;
    }
    Some(out)
}
