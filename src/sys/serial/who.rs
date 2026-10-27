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

//! A process named on the console. A Linux guest's name is the program it
//! runs ("foreign:" and the last part of argv[0], foreign/guest_name.rs), so
//! it says what a person does: which wallet, which messenger. A build
//! without capsule serial output (`capsule-serial-debug`, which no release
//! profile keeps) names every guest only "foreign".

/// What a guest's name begins with, and all a console without capsule
/// output shows of it.
pub(crate) const GUEST: &[u8] = b"foreign:";

/// The part of process name `name` a console shows; `console` is whether
/// the build has capsule serial output.
pub(crate) fn shown(name: &[u8], console: bool) -> &[u8] {
    if !console && name.starts_with(GUEST) {
        return &GUEST[..GUEST.len() - 1];
    }
    name
}
