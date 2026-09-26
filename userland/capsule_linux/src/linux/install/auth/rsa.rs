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

//! Asking the crypto service whether a signature verifies. The capsule
//! holds no RSA of its own, so there is one verifier on the machine.

pub fn verify(spki: &[u8], sig: &[u8], hashid: u8, digest: &[u8]) -> bool {
    nonos_tls::verify_rsa(0, hashid, spki, sig, digest)
}
