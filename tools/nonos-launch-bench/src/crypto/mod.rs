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

//! The kernel's `crate::crypto` as the admission code reaches it. `alg_id`
//! and the BLAKE3 hasher are the kernel's own files; the two signature
//! verifiers are the crates whose code the kernel links.

pub mod asymmetric;

pub mod pqc {
    pub mod ml_dsa_65 {
        pub fn ml_dsa_65_verify_bytes(pk: &[u8], msg: &[u8], sig: &[u8]) -> Result<bool, ()> {
            let pk: &[u8; nonos_mldsa65::PUBLIC_LEN] = pk.try_into().map_err(|_| ())?;
            let sig: &[u8; nonos_mldsa65::SIG_LEN] = sig.try_into().map_err(|_| ())?;
            Ok(nonos_mldsa65::verify(pk, msg, sig))
        }
    }
}

pub mod constant_time {
    pub fn compiler_fence() {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }

    pub fn secure_zero(buf: &mut [u8]) {
        for b in buf.iter_mut() {
            // SAFETY: `b` is a valid, aligned, exclusive reference.
            unsafe { core::ptr::write_volatile(b, 0) };
        }
        compiler_fence();
    }
}

pub mod hash;

pub use hash::blake3::{blake3_keyed_hash, Hasher as Blake3Hasher};
