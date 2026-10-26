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
 * A Linux guest's peer calls hold that guest's lock, not one lock over all
 * guests: the kernel's rule for which lock, and how a call comes to hold the
 * right one while the guest may be torn down under it.
 */

#[cfg(test)]
mod race_tests;
#[path = "../../../../src/process/foreign/peer_lock/stripe.rs"]
pub mod stripe;
#[cfg(test)]
mod tests;
