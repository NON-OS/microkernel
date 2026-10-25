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

/// A salt for `password`, drawn from the seeded RNG. An empty password is
/// refused: no account is kept without a credential. Either refusal comes
/// before anything is written to `salt`.
pub(super) fn fresh_salt(password: &str, salt: &mut [u8]) -> Result<(), &'static str> {
    if password.is_empty() {
        return Err("an account needs a password");
    }
    crate::crypto::rng::fill_random_bytes_secure(salt).map_err(|_| "no randomness")
}
