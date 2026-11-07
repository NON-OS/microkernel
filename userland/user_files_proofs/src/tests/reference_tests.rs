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

//! Thousands of calls in random order, each made of the broker and of
//! Linux on a host directory, every answer compared: the errno, the bytes
//! read, the sizes, the listings. Now and then everything is closed and the
//! broker rebooted from its volume, and the comparison goes on.

use std::fs::File;

use nonos_user_files::limits::DEFAULT_QUOTA;
use nonos_user_files::wire::flags::{APPEND, CREATE, DIRECTORY, EXCL, READ, TRUNC, WRITE};
use nonos_user_files::{Broker, Kind};

use super::live::no_strays;
use super::reference::{Got, Reference};
use super::{settle, APP};
use crate::doubles::{client, Desk, Volume};
use crate::rng::Rng;

const NAMES: [&str; 11] = ["a", "b", "c", "d", "e", "d/a", "d/b", "d/e", "d/e/a", "a/x", "e/d"];
const OFFSETS: [u64; 6] = [0, 3, 4096, 1_048_573, 1_048_576, 2_097_159];
const LENGTHS: [usize; 6] = [0, 1, 100, 5000, 70_000, 200_000];

struct Run {
    b: Broker<Volume>,
    desk: Desk,
    linux: Reference,
    open: Vec<(u64, File)>,
    rng: Rng,
}

fn path(rel: &str) -> Vec<u8> {
    format!("Documents/{rel}").into_bytes()
}

fn pick<T: Copy>(rng: &mut Rng, from: &[T]) -> T {
    from[rng.below(from.len() as u64) as usize]
}

fn same<T: PartialEq + std::fmt::Debug>(what: &str, ours: Got<T>, linux: Got<T>) -> Option<T> {
    match (ours, linux) {
        (Ok(a), Ok(b)) => {
            assert_eq!(a, b, "{what}: answers differ");
            Some(a)
        }
        (Err(a), Err(b)) => {
            assert_eq!(a, b, "{what}: errno differs");
            None
        }
        (a, b) => panic!("{what}: broker {a:?}, linux {b:?}"),
    }
}

impl Run {
    fn step(&mut self, i: usize) {
        let name = pick(&mut self.rng, &NAMES);
        let p = path(name);
        let mut c = client(&mut self.b, &self.desk, APP);
        match self.rng.below(15) {
            0 | 1 => {
                let mut flags = if self.rng.below(2) == 0 {
                    READ
                } else {
                    WRITE | pick(&mut self.rng, &[0, READ])
                };
                for f in [CREATE, EXCL, TRUNC, APPEND, DIRECTORY] {
                    if self.rng.below(4) == 0 {
                        flags |= f;
                    }
                }
                if flags & WRITE == 0 {
                    flags &= !(TRUNC | APPEND);
                }
                if flags & CREATE != 0 {
                    flags &= !DIRECTORY;
                }
                let ours = c.open(&p, flags, 0o644).map(|(h, _)| h);
                let linux = self.linux.open(name, flags);
                match (ours, linux) {
                    (Ok(h), Ok(f)) => self.open.push((h, f)),
                    (Err(a), Err(b)) => assert_eq!(a, b, "{i}: open {name} {flags:#x}"),
                    (a, b) => {
                        panic!("{i}: open {name} {flags:#x}: broker {a:?}, linux {:?}", b.map(drop))
                    }
                }
            }
            2 | 3 if !self.open.is_empty() => {
                let k = self.rng.below(self.open.len() as u64) as usize;
                let (off, len) = (pick(&mut self.rng, &OFFSETS), pick(&mut self.rng, &LENGTHS));
                let data = self.rng.bytes(len);
                let ours = c.write(self.open[k].0, off, &data).map(|w| w.0);
                same(
                    &format!("{i}: write {off}+{len}"),
                    ours,
                    Reference::write(&self.open[k].1, off, &data),
                );
            }
            4 | 5 if !self.open.is_empty() => {
                let k = self.rng.below(self.open.len() as u64) as usize;
                let (off, len) = (pick(&mut self.rng, &OFFSETS), pick(&mut self.rng, &LENGTHS));
                let mut buf = vec![0u8; len];
                let ours = c.read(self.open[k].0, off, &mut buf).map(|n| buf[..n].to_vec());
                same(
                    &format!("{i}: read {off}+{len}"),
                    ours,
                    Reference::read(&self.open[k].1, off, len),
                );
            }
            6 if !self.open.is_empty() => {
                let k = self.rng.below(self.open.len() as u64) as usize;
                let (h, _) = self.open.swap_remove(k);
                c.close(h).unwrap();
            }
            7 if !self.open.is_empty() => {
                let k = self.rng.below(self.open.len() as u64) as usize;
                let size = pick(&mut self.rng, &OFFSETS);
                let ours = c.ftruncate(self.open[k].0, size);
                same(
                    &format!("{i}: ftruncate {size}"),
                    ours,
                    Reference::ftruncate(&self.open[k].1, size),
                );
                let ours = c.fsync(self.open[k].0);
                same(&format!("{i}: fsync"), ours, Reference::fsync(&self.open[k].1));
            }
            8 => {
                let ours = c.stat(&p).map(|a| (a.kind == Kind::Dir, a.size));
                same(&format!("{i}: stat {name}"), ours, self.linux.stat(name));
            }
            9 => same(
                &format!("{i}: mkdir {name}"),
                c.mkdir(&p, 0o755).map(drop),
                self.linux.mkdir(name),
            )
            .unwrap_or(()),
            10 => same(&format!("{i}: rmdir {name}"), c.rmdir(&p), self.linux.rmdir(name))
                .unwrap_or(()),
            11 => same(&format!("{i}: unlink {name}"), c.unlink(&p), self.linux.unlink(name))
                .unwrap_or(()),
            12 => {
                let to = NAMES[self.rng.below(NAMES.len() as u64) as usize];
                let ours = c.rename(&p, &path(to), 0);
                same(&format!("{i}: rename {name} {to}"), ours, self.linux.rename(name, to))
                    .unwrap_or(())
            }
            13 => {
                let size = pick(&mut self.rng, &OFFSETS);
                let ours = c.truncate(&p, size);
                same(&format!("{i}: truncate {name} {size}"), ours, self.linux.truncate(name, size))
                    .unwrap_or(())
            }
            14 => {
                let dir = pick(&mut self.rng, &["", "d", "d/e", "a", "zz"]);
                let p = if dir.is_empty() { b"Documents".to_vec() } else { path(dir) };
                let ours = c
                    .list(&p, 0, 512)
                    .map(|l| l.entries.into_iter().map(|e| e.2).collect::<Vec<_>>());
                same(&format!("{i}: list {dir}"), ours, self.linux.list(dir)).unwrap_or_default();
            }
            _ if i % 400 == 399 => self.reboot(),
            _ => {}
        }
    }

    fn reboot(&mut self) {
        for (h, _) in self.open.drain(..) {
            client(&mut self.b, &self.desk, APP).close(h).unwrap();
        }
        settle(&mut self.b, &self.desk);
        no_strays(self.b.objects());
        self.b = Broker::new(self.b.objects().reboot(), DEFAULT_QUOTA);
    }
}

fn run(seed: u64, steps: usize) {
    let mut r = Run {
        b: Broker::new(Volume::default(), DEFAULT_QUOTA),
        desk: Desk::new(&[APP]),
        linux: Reference::new(seed),
        open: Vec::new(),
        rng: Rng(seed),
    };
    for i in 0..steps {
        r.step(i);
    }
    r.reboot();
    let dirs = ["", "d", "d/e", "e"];
    for dir in dirs {
        let p = if dir.is_empty() { b"Documents".to_vec() } else { path(dir) };
        let ours = client(&mut r.b, &r.desk, APP)
            .list(&p, 0, 512)
            .map(|l| l.entries.into_iter().map(|e| e.2).collect());
        same(&format!("after reboot: list {dir}"), ours, r.linux.list(dir));
    }
}

#[test]
fn every_call_answers_as_linux_does_seed_1() {
    run(0x1111_2222_3333_4444, 4000);
}

#[test]
fn every_call_answers_as_linux_does_seed_2() {
    run(0x9e37_79b9_7f4a_7c15, 4000);
}

#[test]
fn every_call_answers_as_linux_does_seed_3() {
    run(0x0123_4567_89ab_cdef, 4000);
}
