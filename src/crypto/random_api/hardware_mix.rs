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

use crate::crypto::util::rng;

pub(super) fn mix_hardware_entropy(buffer: &mut [u8]) {
    if buffer.is_empty() {
        return;
    }
    mix_cpu_rng(buffer);
}

fn mix_cpu_rng(buffer: &mut [u8]) {
    if !(rng::has_cpu_random() || rng::has_cpu_entropy()) {
        return;
    }

    for chunk in buffer.chunks_mut(8) {
        let Some(v) = next_cpu_random() else {
            continue;
        };
        let bytes = v.to_le_bytes();
        for (idx, byte) in chunk.iter_mut().enumerate() {
            *byte ^= bytes[idx];
        }
    }
}

fn next_cpu_random() -> Option<u64> {
    rng::cpu_entropy64().or_else(rng::cpu_random64)
}
