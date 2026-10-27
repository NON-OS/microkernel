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

//! The largest program this personality installs, binds and runs: 128 MiB.
//! Every reader of a whole program holds to this one number, so raising it
//! is one edit (s3's 2026-10-26 large-programs letter, step 1). The kernel
//! hashes a program up to its own MAX_ELF (local_image.rs), the vfs keeps
//! a file up to MAX_FILE_BYTES, and a run's heap must hold it whole
//! (heap.rs): all three are held to at least this.

pub const PROGRAM_MAX: u32 = 128 << 20;
