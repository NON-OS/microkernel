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

//! The keyring's `server` modules the words check names, from capsule source.

/// One request answered, and both buffers wiped after the reply.
#[cfg(test)]
#[path = "../../../capsule_keyring/src/server/answer.rs"]
pub mod answer;
/// The keyring's EIP-1559 field builder, held to an encoding written apart.
#[cfg(test)]
pub mod eip1559;
/// The keyring's wire types, and the ops it retired.
pub mod protocol;
/// The keyring's RLP encoders, which its transactions are built from.
#[cfg(test)]
#[path = "../../../capsule_keyring/src/server/rlp/mod.rs"]
pub mod rlp;
#[path = "../../../capsule_keyring/src/server/secp.rs"]
pub mod secp;
/// The keyring's typed-data allowlist and the checks around it.
#[path = "../../../capsule_keyring/src/server/typed/mod.rs"]
pub mod typed;
#[cfg(test)]
#[path = "../../../capsule_keyring/src/server/wipe.rs"]
mod wipe;
#[path = "../../../capsule_keyring/src/server/words_own.rs"]
pub mod words_own;
