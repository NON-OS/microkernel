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

//! The file a learn run prints is a policy this reader takes, and the box it
//! compiles to lets every write and connection the run made, and nothing it
//! did not need: each path's own folder, not the run's whole folder.

use agentproof_policy::boxed::folders::may_write;
use agentproof_policy::{compile, parse};
use nonos_linux_policy::dests::{allows, Target};

use super::learn_mount::learned::Learned;
use super::random::Regs;

pub fn learned(wrote: &[&str], tried: &[&str], refused: &str) -> String {
    let mut l = Learned::default();
    wrote.iter().for_each(|p| l.wrote(p.as_bytes()));
    tried.iter().for_each(|d| l.tried.note(d.as_bytes()));
    l.file(refused)
}

pub fn lines(text: &str) -> Vec<&str> {
    text.lines().filter(|l| !l.starts_with('#')).collect()
}

#[test]
fn every_learned_file_is_taken_and_lets_what_the_run_did() {
    const PARTS: [&str; 8] = ["out", "a", "b.txt", "work", "x", "deep", "y", "\u{1}z"];
    const ROOTS: [&str; 6] = ["/tmp", "/home", "/root", "/run", "/var/tmp", "/dev/shm"];
    let mut r = Regs::new(0x1ea2_5eed);
    for _ in 0..3000 {
        let wrote: Vec<String> = (0..r.below(40))
            .map(|_| {
                let mut p = ROOTS[r.below(6)].to_string();
                (0..1 + r.below(4)).for_each(|_| p += &format!("/{}", PARTS[r.below(8)]));
                p
            })
            .collect();
        let dest = |r: &mut Regs| format!("h{}.example:{}", r.below(9), 1 + r.below(9));
        let tried: Vec<String> = (0..r.below(5)).map(|_| dest(&mut r)).collect();
        let w: Vec<&str> = wrote.iter().map(String::as_str).collect();
        let d: Vec<&str> = tried.iter().map(String::as_str).collect();
        let text = learned(&w, &d, "write 2, call 1");
        let c = compile(&parse(text.as_bytes()).unwrap_or_else(|e| panic!("{e:?}\n{text}")));
        let c = c.unwrap_or_else(|e| panic!("{e}\n{text}"));
        for p in w.iter().filter(|p| p.bytes().all(|b| b.is_ascii_graphic())) {
            assert!(may_write(&c.policy.folders, p.as_bytes()), "{p} not let\n{text}");
        }
        for dest in &d {
            let (host, port) = dest.rsplit_once(':').unwrap();
            let port = port.parse().unwrap();
            assert!(allows(&c.policy.dests, Target::Name(host.as_bytes()), port), "{dest}");
        }
        assert!(c.policy.folders.len() <= 16);
    }
}
