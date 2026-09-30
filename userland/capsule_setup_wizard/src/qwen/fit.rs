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

/*
 * Whether a tier fits this machine. Running a model takes more than its
 * file: the KV cache and the runtime need a working margin, estimated here
 * as file size * 1.2 + 512 MiB. The system keeps 1 GiB of the memory for
 * itself. A tier fits when its file plus that margin is at most what is
 * left. This is setup's estimate, not a measurement of the model running.
 */

const MIB: u64 = 1 << 20;
const RUNTIME: u64 = 512 * MIB;
const SYSTEM: u64 = 1024 * MIB;

pub fn fits(file: u64, memory: u64) -> bool {
    file + file / 5 + RUNTIME <= memory.saturating_sub(SYSTEM)
}
