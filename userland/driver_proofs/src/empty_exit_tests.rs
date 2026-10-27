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

//! A storage controller that came up with no disk behind it leaves with
//! EXIT_EMPTY, which the kernel reads as "controller present, nothing behind
//! it to serve"; EXIT_ABSENT would say the controller itself was not there.
//! The exit sits in each capsule's no_std entry, so the proof reads the
//! source it ships with.

use crate::bringup_policy::{EXIT_ABSENT, EXIT_EMPTY};

const MAINS: [(&str, &str); 6] = [
    ("virtio_scsi", include_str!("../../capsule_driver_virtio_scsi/src/main.rs")),
    ("pvscsi", include_str!("../../capsule_driver_pvscsi/src/main.rs")),
    ("buslogic", include_str!("../../capsule_driver_buslogic/src/main.rs")),
    ("mpt", include_str!("../../capsule_driver_mpt/src/main.rs")),
    ("piix_ide", include_str!("../../capsule_driver_piix_ide/src/main.rs")),
    ("storvsc", include_str!("../../capsule_driver_storvsc/src/main.rs")),
];

/// The status the code after the driver's "no disk" line exits with.
fn no_disk_exit(src: &str) -> &str {
    let at = src.find("say::no_disk(").expect("a no-disk branch");
    let rest = &src[at..];
    let call = rest.find("mk_exit(").expect("an exit after it") + "mk_exit(".len();
    rest[call..].split(')').next().unwrap()
}

#[test]
fn a_controller_with_no_disk_leaves_with_exit_empty() {
    assert_eq!((EXIT_ABSENT, EXIT_EMPTY), (2, 3));
    for (name, src) in MAINS {
        assert_eq!(no_disk_exit(src), "EXIT_EMPTY", "{name}");
        assert!(!src.contains("mk_exit(EXIT_ABSENT)"), "{name} still leaves absent");
    }
}
