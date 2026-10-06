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

//! Signed programs under `/linux/` and `/capsules/` travel whole, guests
//! first; nothing enrolled here alone, incomplete, unreadable or from
//! another tree; what vfs cannot load is left out whole and counted.

#[path = "common/fake_vfs.rs"]
mod fake_vfs;
#[path = "common/vfs/mod.rs"]
mod vfs;

/*
 * The vfs files name `alloc`, as a no_std capsule's do.
 */
extern crate alloc;

use fake_vfs::FakeVfs;
use nonos_disk::gather;
use nonos_disk_map::MAX_TOTAL_BYTES;

const MIB: usize = 1 << 20;
const STORE: usize = MAX_TOTAL_BYTES as usize;

const PROOFS: [&str; 3] = [".nonos_id_cert.bin", ".manifest.bin", ".zk_trailer.bin"];

fn signed(v: &mut FakeVfs, program: &str, base: &str, size: usize) -> Vec<String> {
    v.put(program, &vec![0xE1; size]);
    PROOFS.iter().for_each(|proof| v.put(&format!("{base}{proof}"), proof.as_bytes()));
    [program.to_string()].into_iter().chain(PROOFS.map(|p| format!("{base}{p}"))).collect()
}

fn names(v: &mut FakeVfs) -> (nonos_disk::Carried, Vec<String>) {
    let c = gather(v);
    let names = vfs::load(c.store.bytes()).into_iter().map(|(n, _)| n).collect();
    (c, names)
}

#[test]
fn signed_programs_travel_whole_and_guests_first() {
    let mut v = FakeVfs::default();
    let capsule = signed(&mut v, "/capsules/b.elf", "/capsules/b", 100);
    let guest = signed(&mut v, "/linux/bin/qwenchat", "/linux/bin/qwenchat", 300);
    signed(&mut v, "/linux-deb/usr/bin/jq", "/linux-deb/usr/bin/jq", 50);
    v.put("/linux/bin/local", b"x");
    v.put("/linux/bin/local.zk_trailer.bin", b"enrolled here alone");
    signed(&mut v, "/capsules/c.elf", "/capsules/c", 10);
    v.files.remove("/capsules/c.manifest.bin");
    signed(&mut v, "/capsules/d.elf", "/capsules/d", 10);
    v.unreadable.insert("/capsules/d.elf".to_string());
    let (c, names) = names(&mut v);
    assert_eq!(names, [guest, capsule].concat());
    assert_eq!((c.programs, c.left_out, c.skipped, c.answers), (2, 0, 1, None));
}

#[test]
fn programs_past_the_store_are_left_out_whole() {
    let mut v = FakeVfs::default();
    /* Together a MiB past the store; either alone fits. */
    signed(&mut v, "/linux/bin/big", "/linux/bin/big", STORE / 2 + 2 * MIB);
    signed(&mut v, "/capsules/also_big.elf", "/capsules/also_big", STORE / 2 - MIB);
    let small = signed(&mut v, "/capsules/small.elf", "/capsules/small", 1000);
    let (c, names) = names(&mut v);
    let proofs: usize = PROOFS.iter().map(|p| p.len()).sum();
    assert_eq!((c.programs, c.left_out, c.left_out_bytes), (2, 1, (STORE / 2 - MIB + proofs) as u64));
    assert_eq!(names[4..], small);
}

#[test]
fn the_files_linux_programs_read_travel_after_them() {
    let mut v = FakeVfs::default();
    let python = signed(&mut v, "/linux/usr/bin/python3", "/linux/usr/bin/python3", 200);
    v.put("/linux/usr/lib/python312.zip", b"stdlib");
    v.put("/linux/etc/nonos-links", b"/usr/bin/python /usr/bin/python3\n");
    v.put("/linux/bin/local", b"x");
    v.put("/linux/bin/local.zk_trailer.bin", b"enrolled here alone");
    v.put("/linux/bin/stray.manifest.bin", b"a proof without its program");
    let (c, names) = names(&mut v);
    let data = ["/linux/etc/nonos-links", "/linux/usr/lib/python312.zip"].map(String::from);
    assert_eq!(names, [python, data.to_vec()].concat(), "programs first, then data, in path order");
    assert_eq!((c.programs, c.data, c.data_left_out), (1, 2, 0));
}

#[test]
fn data_past_the_store_is_left_out_and_counted() {
    let mut v = FakeVfs::default();
    signed(&mut v, "/linux/usr/bin/python3", "/linux/usr/bin/python3", STORE - 5 * MIB);
    v.put("/linux/usr/lib/python312.zip", &vec![0; 10 * MIB]);
    let (c, _) = names(&mut v);
    assert_eq!((c.programs, c.data, c.data_left_out), (1, 0, 1));
}
