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

//! What a commit leaves in the store: load_by_name loads it, pkg_remove takes it out, a held name blocks it.

use crate::clients::vfs;
use crate::frame::{by_name, frame, reply};
use crate::packages::{elf, package, CERT, MANIFEST, ZK};
use crate::pkg_frame::{commit, digest, fresh, remove, stored, PATH, STORED};
use crate::protocol::OP_LOAD_BY_NAME;
use crate::syscall::{self, N_MK_CAPSULE_LOAD};
use crate::{services, verify::INSTALL_NAME};

const EEXIST: i32 = -17;
const ENOENT: i32 = -2;

fn committed() {
    fresh();
    let pkg = package(64);
    vfs::put(PATH, pkg.clone());
    assert_eq!(commit(&digest(&pkg), PATH), reply(0, &[]));
}

#[test]
fn a_committed_package_loads_by_name_from_its_verified_sections() {
    committed();
    syscall::reset(31);
    let p = [&0u64.to_le_bytes()[..], &[INSTALL_NAME.len() as u8], INSTALL_NAME].concat();
    assert_eq!(by_name(&frame(OP_LOAD_BY_NAME, &p), 77), reply(0, &31u32.to_le_bytes()));
    let loads = syscall::take();
    assert_eq!((loads.len(), loads[0].num), (1, N_MK_CAPSULE_LOAD));
    assert_eq!(loads[0].blobs[..4], [elf(64), CERT.to_vec(), MANIFEST.to_vec(), ZK.to_vec()]);
}

#[test]
fn pkg_remove_takes_out_the_four_artifacts_a_commit_installed() {
    committed();
    assert_eq!(remove(INSTALL_NAME), reply(0, &[]));
    assert_eq!(vfs::uninstalls(), STORED.map(stored));
    assert_eq!(remove(INSTALL_NAME), reply(ENOENT, &[]));
}

#[test]
fn a_name_a_running_service_holds_is_not_installed() {
    fresh();
    services::reset(&[b"app.demo_pkg"]);
    let pkg = package(64);
    vfs::put(PATH, pkg.clone());
    assert_eq!(commit(&digest(&pkg), PATH), reply(EEXIST, &[]));
    assert!(vfs::installs().is_empty(), "installed over a running service's name");
}
