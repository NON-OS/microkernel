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

//! Narrowing the table.

use super::keys::{Act, Binding};
use super::keys_group::Group;

pub const FILTERS: &[Binding] = &[
    Binding {
        key: b"r",
        codes: &[0x52, 0x72],
        label: b"sample the kernel again now",
        group: Group::Act,
        act: Act::Refresh,
    },
    Binding {
        key: b"k",
        codes: &[0x4B, 0x6B],
        label: b"ask the selected process to stop, twice to confirm",
        group: Group::Act,
        act: Act::Terminate,
    },
    Binding {
        key: b"f",
        codes: &[0x46, 0x66],
        label: b"force it to stop, twice to confirm",
        group: Group::Act,
        act: Act::ForceKill,
    },
];
