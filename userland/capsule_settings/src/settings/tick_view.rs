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
//! A page tick that re-reads the running system asks for a frame only when
//! what the page shows came back different. The Wi-Fi and Network pages
//! re-read the driver and the lease every second, and each tick drew the
//! whole window again (409810 pixels a second on a development guest with
//! the page left open) though nothing on it had changed. Pure, so the host
//! proofs hold it (capsule_settings_proofs tick_view_tests).

/// Run `read` on `state`; whether `view`, what the page shows of it, is not
/// what it was before.
pub fn reread<S, V: PartialEq>(
    state: &mut S,
    view: impl Fn(&S) -> V,
    read: impl FnOnce(&mut S),
) -> bool {
    let before = view(state);
    read(state);
    view(state) != before
}
