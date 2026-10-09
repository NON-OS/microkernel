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

pub(crate) const SHIELD_VECTORS_ELF: &[u8] = include_bytes!(concat!(
    "../../../userland/capsule_shield_vectors/target/",
    env!("NONOS_USER_TARGET"),
    "/release/shield_vectors"
));

pub(crate) const SHIELD_VECTORS_NONOS_ID_CERT_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/shield_vectors.nonos_id_cert.bin");

pub(crate) const SHIELD_VECTORS_MANIFEST_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/shield_vectors.manifest.bin");

pub(crate) const SHIELD_VECTORS_ATTESTATION_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/shield_vectors.zk_trailer.bin");
