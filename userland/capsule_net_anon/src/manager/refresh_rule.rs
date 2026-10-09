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

//! Serving from the consensus in hand while the next one is fetched.
//!
//! A consensus stops being fresh after an hour and stays valid for some hours
//! more. The refetch used to drop the client back to its cold bootstrap, so
//! from the hour mark until the new consensus and its microdescriptors were
//! in, every stream open was refused and no circuit was built, and the relay
//! set was rebuilt from the few microdescriptors fetched so far. A refresh
//! now serves from the relays in hand until the new set is whole, then swaps
//! it in at once. Once the old consensus is no longer valid nothing is built
//! from it, whatever the refresh has reached.
//!
//! Pure. Held in anon_ntor_proofs.

/// What a refresh does after one batch of microdescriptors.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RefreshJoin {
    /// More to come: keep serving from the relays in hand.
    Keep,
    /// Every batch is in and the new set draws a path: swap it in.
    Swap,
    /// Every batch is in and no path draws: fetch the consensus again,
    /// still serving from the relays in hand.
    Refetch,
}

/// The step after a batch during a refresh. `last` says every batch has
/// been asked for; `drawable` that the new set draws a path.
pub fn refresh_join(last: bool, drawable: bool) -> RefreshJoin {
    match (last, drawable) {
        (false, _) => RefreshJoin::Keep,
        (true, true) => RefreshJoin::Swap,
        (true, false) => RefreshJoin::Refetch,
    }
}

/// Whether the relays in hand may build circuits and carry streams: the
/// directory reached Ready, or is being refreshed after it did, there are
/// relays, and the consensus they came from is still valid.
pub fn usable(ready: bool, refreshing: bool, relays: usize, now: u64, valid_until: u64) -> bool {
    (ready || refreshing) && relays > 0 && now < valid_until
}
