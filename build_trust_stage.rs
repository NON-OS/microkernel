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

//! What the kernel build stages from the trust set into OUT_DIR.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

// The image capability ceiling is baked into the kernel, but most images do not
// set one: the default is unrestricted and lives nowhere on disk. include_bytes!
// cannot express an optional file, so the source path is staged through OUT_DIR.
// A ceiling file present under the trust policy is copied verbatim; when it is
// absent the eight zero bytes written here decode, per image_ceiling::ceiling,
// to the unrestricted default.
pub fn stage_image_capability_ceiling(trust: &Path) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let src = trust.join("policy/image_capability_ceiling.bin");
    println!("cargo:rerun-if-changed={}", src.display());
    let dst = PathBuf::from(&out_dir).join("image_capability_ceiling.bin");
    let bytes = fs::read(&src).unwrap_or_else(|_| vec![0u8; 8]);
    fs::write(&dst, bytes).expect("stage image capability ceiling");
}

// The release's P-256 policy key, the one that approves which kernels may use
// the device secret and signs the boot-root record: x || y, 64 bytes, copied
// verbatim; absent, 64 zero bytes, and the kernel treats the key as not
// provisioned. NONOS_DEVICE_POLICY_PUBKEY names a test key in place of the
// trust set's, made outside the tree.
pub fn stage_device_policy_key(trust: &Path) {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rerun-if-env-changed=NONOS_DEVICE_POLICY_PUBKEY");
    let named = env::var_os("NONOS_DEVICE_POLICY_PUBKEY").map(PathBuf::from);
    let src = named.clone().unwrap_or_else(|| trust.join("policy/device_policy_p256.pub"));
    println!("cargo:rerun-if-changed={}", src.display());
    let dst = PathBuf::from(&out_dir).join("device_policy_p256.pub");
    let bytes = match fs::read(&src) {
        Ok(b) if b.len() == 64 => b,
        Ok(b) => panic!("{} must be 64 bytes (x || y), got {}", src.display(), b.len()),
        Err(_) if named.is_some() => panic!("{} cannot be read", src.display()),
        Err(_) => vec![0u8; 64],
    };
    fs::write(&dst, bytes).expect("stage device policy key");
}
