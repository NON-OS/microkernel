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

//! How long a slot is given. The waits after the link comes up and for the
//! attention button are the specifications' as recalled (unverified); the
//! debounce, the link's grace and the retry are this capsule's choices.

/// After the link is up, before the first configuration request.
pub const READY_MS: u64 = 100;
/// The attention button's abort window: a second press cancels.
pub const BUTTON_MS: u64 = 5_000;
/// A slot must look gone this long before what was in it departs.
pub const DEBOUNCE_MS: u64 = 100;
/// How long a card that is present has for its link to come up.
pub const LINK_UP_MS: u64 = 1_000;
/// An eject refused while a driver holds a function is asked again.
pub const RETRY_MS: u64 = 2_000;
/// How often the slots are read.
pub const POLL_MS: u64 = 50;
/// A slot control command completes within this.
pub const COMMAND_MS: u64 = 1_000;
