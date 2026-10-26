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

//! Every opener of a volume scope holds a guard and passes a refusal on with
//! `?`: on any path out, success or error, the guard's drop lets the slot go.

const OPENERS: [(&str, &str, usize); 6] = [
    ("cryptoblock/device.rs", include_str!("../../../../../src/fs/cryptoblock/device.rs"), 3),
    (
        "blockfs_volume/plan_read.rs",
        include_str!("../../../../../src/fs/blockfs_volume/plan_read.rs"),
        1,
    ),
    (
        "blockfs_volume/key_header_io.rs",
        include_str!("../../../../../src/fs/blockfs_volume/key_header_io.rs"),
        2,
    ),
    (
        "blockfs_volume/ring_blank.rs",
        include_str!("../../../../../src/fs/blockfs_volume/ring_blank.rs"),
        1,
    ),
    (
        "blockfs_volume/import_stream.rs",
        include_str!("../../../../../src/fs/blockfs_volume/import_stream.rs"),
        1,
    ),
    (
        "blockfs/validate_geometry.rs",
        include_str!("../../../../../src/fs/blockfs/validate_geometry.rs"),
        1,
    ),
];

fn flat(s: &str) -> String {
    s.lines()
        .map(|l| l.split("//").next().unwrap())
        .collect::<String>()
        .split_whitespace()
        .collect()
}

#[test]
fn each_opener_holds_a_guard_and_passes_a_refusal_on() {
    for (name, src, n) in OPENERS {
        let f = flat(src);
        let opened = f.matches("volume_scope()").count();
        let guarded =
            f.matches("volume_scope()?;").count() + f.matches("volume_scope().map_err(").count();
        assert_eq!(
            (opened, guarded),
            (n, n),
            "{name}: every scope a guard, every refusal passed on"
        );
        assert!(!f.contains("INSIDE"), "{name}: no slot let go by hand");
    }
}
