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

extern crate alloc;
use super::state::TRUSTED_BOOT_KEYS;
use super::types::{SecureBootError, SecureBootResult, TrustedKey};
use crate::crypto::constant_time::ct_eq_32;
use alloc::string::String;
use alloc::vec::Vec;

/// The keys `verify_code_signature` will accept a signature from.
pub fn load_embedded_keys() -> SecureBootResult<()> {
    let mut keys = TRUSTED_BOOT_KEYS.write();

    let primary_key = TrustedKey {
        name: String::from("NONOS-PRIMARY-2026"),
        public_key: [
            0x1a, 0x74, 0x55, 0xad, 0xcf, 0x2e, 0xfb, 0xfb, 0x73, 0x44, 0x2f, 0xc6, 0x7d, 0x5f,
            0x2c, 0xe7, 0x67, 0xdd, 0x0e, 0xf1, 0x52, 0x2f, 0xa0, 0x7c, 0x25, 0x23, 0x5c, 0x15,
            0xb5, 0x74, 0x40, 0x6d,
        ],
        fingerprint: [0u8; 32],
        created_at: 1789776000,
        expires_at: 0,
        is_production: true,
    };

    let mut key_with_fp = primary_key.clone();
    key_with_fp.fingerprint = crate::crypto::blake3::blake3_hash(&key_with_fp.public_key);
    keys.production_keys.push(key_with_fp);

    let backup_key = TrustedKey {
        name: String::from("NONOS-BACKUP-2026"),
        public_key: [
            0x23, 0xc8, 0xd8, 0xcc, 0x1f, 0xb5, 0x87, 0xc8, 0xff, 0xcc, 0xbd, 0xb7, 0x5f, 0x77,
            0xe5, 0x54, 0xa8, 0xaf, 0xee, 0xae, 0x43, 0xab, 0xc9, 0xcf, 0xa4, 0x6a, 0xdd, 0x05,
            0x26, 0x01, 0xbc, 0xac,
        ],
        fingerprint: [0u8; 32],
        created_at: 1789776000,
        expires_at: 0,
        is_production: true,
    };

    let mut backup_with_fp = backup_key.clone();
    backup_with_fp.fingerprint = crate::crypto::blake3::blake3_hash(&backup_with_fp.public_key);
    keys.production_keys.push(backup_with_fp);

    let recovery_key = TrustedKey {
        name: String::from("NONOS-RECOVERY-2026"),
        public_key: [
            0x88, 0x46, 0xc1, 0xa0, 0x84, 0xcd, 0xad, 0x50, 0xf5, 0xf2, 0x25, 0x38, 0x80, 0xf7,
            0xcc, 0x77, 0xd1, 0xd5, 0xe4, 0x83, 0xa4, 0x77, 0xd6, 0xe2, 0xb7, 0x59, 0x79, 0xeb,
            0x76, 0x69, 0x0c, 0x24,
        ],
        fingerprint: [0u8; 32],
        created_at: 1789776000,
        expires_at: 0,
        is_production: true,
    };

    let mut recovery_with_fp = recovery_key.clone();
    recovery_with_fp.fingerprint = crate::crypto::blake3::blake3_hash(&recovery_with_fp.public_key);
    keys.production_keys.push(recovery_with_fp);

    crate::log::info!("[SECURE_BOOT] Loaded {} production keys", keys.production_keys.len());

    Ok(())
}

pub fn add_trusted_key(key: TrustedKey) -> SecureBootResult<()> {
    if let Some(proc) = crate::process::current_process() {
        let token = proc.capability_token();
        if !token.grants(crate::capabilities::Capability::Admin) {
            return Err(SecureBootError::PolicyViolation);
        }
    }

    let mut keys = TRUSTED_BOOT_KEYS.write();

    let mut key_with_fp = key;
    key_with_fp.fingerprint = crate::crypto::blake3::blake3_hash(&key_with_fp.public_key);

    for existing in &keys.production_keys {
        if ct_eq_32(&existing.fingerprint, &key_with_fp.fingerprint) {
            return Ok(());
        }
    }

    if key_with_fp.is_production {
        keys.production_keys.push(key_with_fp.clone());
    } else {
        keys.development_keys.push(key_with_fp.clone());
    }

    keys.rotation_count += 1;

    crate::log::info!("[SECURE_BOOT] Added trusted key: {}", key_with_fp.name);

    Ok(())
}

pub fn revoke_key(fingerprint: [u8; 32]) -> SecureBootResult<()> {
    if let Some(proc) = crate::process::current_process() {
        let token = proc.capability_token();
        if !token.grants(crate::capabilities::Capability::Admin) {
            return Err(SecureBootError::PolicyViolation);
        }
    }

    let mut keys = TRUSTED_BOOT_KEYS.write();

    for revoked in &keys.revoked_fingerprints {
        if ct_eq_32(revoked, &fingerprint) {
            return Ok(());
        }
    }

    keys.revoked_fingerprints.push(fingerprint);
    keys.rotation_count += 1;

    crate::log::log_warning!("[SECURE_BOOT] Key REVOKED: {:02x?}...", &fingerprint[..8]);

    Ok(())
}

pub fn list_trusted_keys() -> Vec<([u8; 32], String, bool)> {
    let keys = TRUSTED_BOOT_KEYS.read();
    let mut result = Vec::new();

    for key in &keys.production_keys {
        result.push((key.fingerprint, key.name.clone(), true));
    }

    for key in &keys.development_keys {
        result.push((key.fingerprint, key.name.clone(), false));
    }

    result
}
