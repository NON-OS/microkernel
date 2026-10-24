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

//! A kernel signed and attested by the seal's own code: sign-kernel builds the
//! signature block and its footer, and embed-trailer loads that the way it
//! loads sign-kernel's output and assembles the attested image. The proofs
//! read exactly the layout the seal writes, not a copy of it.

use super::tree_fixture::Tree;
use ed25519_dalek::Signer as _;
use nonos_capsule_sign::algs::{mldsa65::MlDsa65, Signer};
use nonos_sign_kernel::constants::SIG_ALG_ED25519_MLDSA65;
use nonos_sign_kernel::{footer::create_image_footer, release_signature_blob, signed_message};

pub const KERNEL: &[u8] = b"\x7fELF the kernel's bytes, as the build gave them";
pub const ROLLBACK: u32 = 1;

pub fn attested(tree: &Tree, kernel: &[u8], rollback: u32) -> Vec<u8> {
    let msg = signed_message(kernel, rollback);
    let ed_sig = tree.ed.sign(&msg).to_bytes();
    let pq_sig = <MlDsa65 as Signer>::sign(&tree.pq_secret, &msg).unwrap();
    let ed_pub = tree.ed.verifying_key().to_bytes();
    let blob = release_signature_blob(&ed_pub, &ed_sig, &tree.pq_pub, &pq_sig);
    let (k, s) = (kernel.len() as u32, blob.len() as u32);
    let total = (kernel.len() + blob.len() + 64) as u64;
    let footer = create_image_footer(k, s, SIG_ALG_ED25519_MLDSA65, total, rollback);
    let signed = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(signed.path(), [kernel, &blob, &footer].concat()).unwrap();
    let loaded = embed_trailer::load_signed_kernel(signed.path()).unwrap();
    embed_trailer::assemble_attested_image(&loaded, b"the attestation trailer".to_vec()).data
}
