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

use alloc::vec::Vec;

use super::bits::Bools;
use super::tables::{default_prob, update_prob};

/// Coefficient token probabilities, each default replaced when its update
/// flag is set (RFC 6386 13.4).
pub(super) fn token_probs(br: &mut Bools) -> Vec<u8> {
    (0..1056)
        .map(|i| if br.bit(update_prob(i)) { br.literal(8) as u8 } else { default_prob(i) })
        .collect()
}
