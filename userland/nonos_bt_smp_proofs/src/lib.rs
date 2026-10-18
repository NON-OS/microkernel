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

//! Proofs for the Security Manager. Every module is test-only.

#[cfg(test)]
mod auth_level_tests;
#[cfg(test)]
mod cmac_tests;
#[cfg(test)]
mod ecdh_tests;
#[cfg(test)]
mod funcs_tests;
#[cfg(test)]
mod hex;
#[cfg(test)]
mod identity_level_tests;
#[cfg(test)]
mod method_tests;
#[cfg(test)]
mod pairing_fail_tests;
#[cfg(test)]
mod pairing_identity_tests;
#[cfg(test)]
mod pairing_passkey_tests;
#[cfg(test)]
mod pairing_refuse_tests;
#[cfg(test)]
mod pairing_tests;
#[cfg(test)]
mod pairing_timer_tests;
#[cfg(test)]
mod pairing_user_tests;
#[cfg(test)]
mod pdu_tests;
#[cfg(test)]
mod privacy_draw_tests;
#[cfg(test)]
mod privacy_tests;
#[cfg(test)]
mod sim;
#[cfg(test)]
mod sweep_key_tests;
#[cfg(test)]
mod sweep_tests;
