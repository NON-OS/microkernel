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
//! How long a keyring call may wait. Its own file, so wallet_proofs checks
//! the bound the wallet calls with.

// The keyring answers most ops in microseconds; HD generation and recovery
// run PBKDF2 with 2048 rounds plus derivation, still well under a second.
// The bound exists so a wedged keyring surfaces as an error on the UI
// thread instead of freezing the wallet.
pub const KEYRING_TIMEOUT_MS: u64 = 4000;
