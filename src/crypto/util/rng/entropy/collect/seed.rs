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

use super::super::hardware::read_cycle_counter;
use super::get::get_entropy64;

use super::pool::collect_seed_entropy_secure;

pub fn collect_seed_entropy() -> [u8; 32] {
    if let Ok(seed) = collect_seed_entropy_secure() {
        return seed;
    }
    let mut seed = [0u8; 32];
    for i in 0..4 {
        let entropy = get_entropy64();
        seed[i * 8..(i + 1) * 8].copy_from_slice(&entropy.to_le_bytes());
    }
    let stack_addr = crate::arch::stack_pointer();
    let sb = stack_addr.to_le_bytes();
    for i in 0..8 {
        seed[i] ^= sb[i];
    }
    seed
}

pub fn mix_entropy_into_seed(seed: &mut [u8; 32], additional: &[u8; 32]) {
    for i in 0..32 {
        seed[i] ^= additional[i];
    }
    let tb = read_cycle_counter().to_le_bytes();
    for i in 0..8 {
        seed[i] ^= tb[i];
        seed[i + 8] ^= tb[i];
        seed[i + 16] ^= tb[i];
        seed[i + 24] ^= tb[i];
    }
}
