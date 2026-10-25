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

// The quantum key vault (security/quantum/pqc/vault.rs), proved from
// inside its module: a rotation takes the old key under the lock and lets
// it go before making the new one, which takes the lock again. Each test
// runs on a thread of its own and is failed if it has not finished in ten
// seconds, so a rotation that kept the lock fails by hanging, not by
// hanging the run.

use super::QuantumKeyVault;
use crate::security::quantum::pqc::types::QuantumAlgorithm;
use std::sync::mpsc;
use std::time::Duration;

fn within_ten_seconds(proof: fn()) {
    let (done, finished) = mpsc::channel();
    std::thread::spawn(move || {
        proof();
        let _ = done.send(());
    });
    if finished.recv_timeout(Duration::from_secs(10)).is_err() {
        panic!("the proof hung or failed");
    }
}

#[test]
fn a_rotation_returns_a_new_key_and_logs_it() {
    within_ten_seconds(|| {
        let vault = QuantumKeyVault::new();
        let old = vault.generate_key(QuantumAlgorithm::Kyber768, 600).expect("key");
        let new = vault.rotate_key(&old.key_id, "scheduled").expect("rotated");
        assert_ne!(new.key_id, old.key_id);
        assert_eq!(new.expires_at - new.created_at, 600);
        assert_eq!(new.algo, QuantumAlgorithm::Kyber768);
        let log = vault.rotations.lock();
        assert_eq!(log.len(), 1);
        assert_eq!((log[0].old_key_id, log[0].new_key_id), (old.key_id, new.key_id));
        assert_eq!(log[0].reason, "scheduled");
        assert_eq!(vault.get_key(&old.key_id).map(|k| k.key_id), Some(old.key_id));
        assert_eq!(vault.get_key(&new.key_id).map(|k| k.public.clone()), Some(new.public.clone()));
    });
}

#[test]
fn an_unknown_key_is_not_rotated() {
    within_ten_seconds(|| {
        let vault = QuantumKeyVault::new();
        assert!(vault.rotate_key(&[9; 32], "scheduled").is_none());
        assert!(vault.rotations.lock().is_empty());
    });
}
