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

/// secp256k1 public key recovery to an Ethereum address, injected for the
/// same reason the hashes are. `sig` is r, s, v with v 27 or 28; anything
/// that does not recover is `None`.
pub trait Recover {
    fn signer(&self, digest: &[u8; 32], sig: &[u8; 65]) -> Option<[u8; 20]>;
}
