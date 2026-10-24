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

//! The kernel the seal signed and attested (sealed as `kernel`), read as
//! untrusted. Its layout (embed-trailer, sign-kernel): the kernel's bytes,
//! the release signature block (`NKRSIG2\0`, the Ed25519 key id and
//! signature, the ML-DSA-65 key id and signature), the attestation trailer,
//! then a 64-byte `NONOSIMG` footer giving every size. Each size is checked
//! against the others and against the file before any part is used.

pub const FOOTER: usize = 64;
const MAGIC: &[u8; 8] = b"NONOSIMG";
const SIG_MAGIC: &[u8; 8] = b"NKRSIG2\0";
/// The magic, the Ed25519 key id and signature, the ML-DSA-65 key id, and
/// the ML-DSA-65 signature (sign-kernel's RELEASE_SIG_SIZE).
pub const SIG_BLOCK: usize = 8 + 32 + 64 + 32 + 3309;
const HASH_BLAKE3: u8 = 1;
const SIG_ED25519_MLDSA65: u8 = 2;

pub struct Attested<'a> {
    pub kernel: &'a [u8],
    pub ed_key_id: &'a [u8],
    pub ed_sig: &'a [u8],
    pub pq_key_id: &'a [u8],
    pub pq_sig: &'a [u8],
    pub rollback: u32,
}

pub fn parse(b: &[u8]) -> Result<Attested<'_>, String> {
    let at = b.len().checked_sub(FOOTER).ok_or("it is shorter than its own footer")?;
    let f = &b[at..];
    let u32_at = |o: usize| u32::from_le_bytes([f[o], f[o + 1], f[o + 2], f[o + 3]]) as usize;
    if &f[..8] != MAGIC || u16::from_le_bytes([f[8], f[9]]) != 1 {
        return Err("it has no NONOSIMG footer of version 1".into());
    }
    if f[12] != HASH_BLAKE3 || f[13] != SIG_ED25519_MLDSA65 {
        return Err(
            "its footer names other algorithms than BLAKE3 with Ed25519 and ML-DSA-65".into()
        );
    }
    // As embed-trailer writes it (footer/create.rs): flags 1, reserved 0, 1 at 48.
    let zero =
        [14..16, 24..28, 52..56, 60..64].iter().all(|r| f[r.clone()].iter().all(|&b| b == 0));
    if u16::from_le_bytes([f[10], f[11]]) != 1 || !zero || u32_at(48) != 1 {
        return Err("its footer's reserved bytes are not as embed-trailer writes them".into());
    }
    let total = u64::from_le_bytes([f[16], f[17], f[18], f[19], f[20], f[21], f[22], f[23]]);
    let (kernel, sig, proof_at, proof) = (u32_at(28), u32_at(36), u32_at(40), u32_at(44));
    let fills = kernel.checked_add(sig) == Some(proof_at)
        && proof_at.checked_add(proof) == Some(at)
        && proof > 0;
    if total != b.len() as u64 || u32_at(32) != kernel || !fills || kernel == 0 {
        return Err("the sizes its footer gives do not add up to the file".into());
    }
    if sig != SIG_BLOCK || &b[kernel..kernel + 8] != SIG_MAGIC {
        return Err("it carries no NKRSIG2 release signature block".into());
    }
    let s = &b[kernel..kernel + sig];
    let rollback = u32_at(56) as u32;
    let (ed_key_id, ed_sig, pq_key_id, pq_sig) = (&s[8..40], &s[40..104], &s[104..136], &s[136..]);
    Ok(Attested { kernel: &b[..kernel], ed_key_id, ed_sig, pq_key_id, pq_sig, rollback })
}
