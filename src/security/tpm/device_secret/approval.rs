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

//! What the kernel needs from the release to use the secret: the policy key and
//! the release's signature over this kernel's approved policy. The key is
//! compiled in; the signature arrives beside the kernel, since it covers a PCR
//! the kernel image itself feeds and so cannot be inside it.

/// The ref every approval is signed under, so a signature made for another use
/// of the same key never authorizes this one.
pub const POLICY_REF: &[u8] = b"NONOS-DEVICE-SECRET-v1";

#[derive(Clone, Copy)]
pub struct Approval {
    /// The release policy key, P-256, affine coordinates big-endian.
    pub key_x: [u8; 32],
    pub key_y: [u8; 32],
    /// ECDSA, prehashed, over `a_hash(approved, POLICY_REF)`, `approved` being
    /// PolicyPCR over this kernel's PCR 9 from an empty session.
    pub sig_r: [u8; 32],
    pub sig_s: [u8; 32],
}

/// The release policy key this kernel was built to trust, as x || y. All zero
/// when the key bootstrap has not provisioned one.
const RELEASE_KEY: &[u8; 64] = include_bytes!(concat!(env!("OUT_DIR"), "/device_policy_p256.pub"));

/*
 * The approval the bootloader handed over, under this kernel's own key. The
 * file names a key too, and it must be this one: an approval under any other
 * key is refused here, before the TPM is asked. `None` when there is no key,
 * no approval, or one under another key; the device secret then stays sealed.
 */
pub fn from_boot() -> Option<Approval> {
    if RELEASE_KEY.iter().all(|&b| b == 0) {
        return None;
    }
    let h = crate::boot::handoff::get_handoff()?;
    let p = &h.policy;
    if p.approval_present != 1 || p.approval[..64] != RELEASE_KEY[..] {
        return None;
    }
    let part = |i: usize| {
        let mut out = [0u8; 32];
        out.copy_from_slice(&p.approval[32 * i..32 * (i + 1)]);
        out
    };
    Some(Approval { key_x: part(0), key_y: part(1), sig_r: part(2), sig_s: part(3) })
}
