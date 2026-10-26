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

//! A passphrase-keyed volume (passphrase_create.rs): its salt, nonce and
//! key are drawn before the passphrase is stretched, so with any refused,
//! or while the boot keeps nothing (key_header_io.rs), no key header is
//! written. Its real Argon2id and ChaCha20-Poly1305 seal the key.

use super::super::error::VolumeError;
use super::super::key_header::{parse_key_header, Keyed, KEY_LBA};
use super::super::key_seal::unseal_key;
use super::super::passphrase_create::create_keyed;
use super::super::passphrase_key::stretch;
use super::super::plan_types::{Plan, MAX_IMPORTS, PLAN_LBA};
use super::{open_key, turn};
use crate::hardware::block_device::BlockDeviceError::AccessDenied;
use crate::stand_in::{block_io::set_keeping, devices, kernel, rng, volume};

const PLAN: Plan =
    Plan { volume_base: 300_000, volume_sectors: 4096, imports: [(0, 0); MAX_IMPORTS], count: 0 };

#[test]
fn no_keyed_volume_is_made_when_any_draw_refuses() {
    let _turn = turn();
    for (from, only) in rng::cases(3) {
        rng::fail_at(Some(from), only);
        assert_eq!(create_keyed(b"pass", &PLAN), Err(VolumeError::NoRandomness));
        assert!(devices::plan_written().is_empty());
        assert!(volume::formats().is_empty());
        assert_eq!(open_key(), None);
    }
    assert_eq!((kernel::shootdowns(), rng::weak()), (0, 0));
}

#[test]
fn no_key_header_is_written_while_the_boot_keeps_nothing() {
    let _turn = turn();
    rng::fail_from(None);
    assert_eq!(create_keyed(b"pass", &PLAN), Err(VolumeError::Device(AccessDenied)));
    assert!(devices::plan_written().is_empty() && volume::formats().is_empty());
}

/// Finding: the format inside (mount_or_format.rs) draws the volume's uuid
/// by the weak fill, the one weak call here. The uuid is no secret.
#[test]
fn the_keyed_volume_seals_its_drawn_key_under_the_drawn_salt_and_nonce() {
    let _turn = turn();
    rng::fail_from(None);
    set_keeping(true);
    assert_eq!(create_keyed(b"pass", &PLAN), Ok(()));
    let given = rng::given();
    let header = devices::plan_written()[&(KEY_LBA - PLAN_LBA)];
    let Ok(Some(Keyed::Passphrase(sealed))) = parse_key_header(&header) else {
        panic!("no passphrase key header");
    };
    assert_eq!((sealed.salt.to_vec(), sealed.nonce.to_vec()), (given[0].clone(), given[1].clone()));
    let kek = stretch(b"pass", &sealed).expect("stretched");
    let key = unseal_key(&kek, &sealed).expect("opens");
    assert_eq!((key.to_vec(), open_key()), (given[2].clone(), Some(key)));
    assert_eq!(volume::formats()[0].0, key);
    assert_eq!(rng::weak(), 1);
}
