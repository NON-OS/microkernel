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

//! Every policy is enforced exactly as its file is written: for each path,
//! capsule_linux's own write rule (boxed/folders.rs), on the box the file
//! compiles to, lets it be written iff the file says so, read here from the
//! file's words alone. Exec is always held to the box, and no network.

use agentproof_policy::boxed::allowed::allowed;
use agentproof_policy::boxed::folders::may_write;
use agentproof_policy::{compile, parse, Base};
use nonos_linux_policy::mask::{grants, EXEC, FILES_HOME, NET_ANON, TTY};

use super::corpus::{CORPUS, PATHS};
use super::file;
use super::random::Regs;

/// What the file says may be written, from its own words.
fn says(lines: &[&str], path: &str) -> bool {
    let own = ["/tmp", "/home", "/root", "/run", "/var/tmp", "/dev/shm"];
    let folders: Vec<&str> = match lines.iter().find_map(|l| l.strip_prefix("base ")) {
        Some("strict") => vec!["/tmp"],
        Some("offline") => own.to_vec(),
        _ => lines.iter().filter_map(|l| l.strip_prefix("write ")).collect(),
    };
    folders.iter().any(|f| path == *f || path.strip_prefix(f).is_some_and(|r| r.starts_with('/')))
}

#[test]
fn each_write_is_let_through_iff_the_file_says_so() {
    let mut r = Regs::new(0xa6e7_9f00);
    for lines in CORPUS {
        let c = compile(&parse(&file(lines)).unwrap()).unwrap();
        for _ in 0..20_000 {
            let path: String = (0..1 + r.below(4)).map(|_| PATHS[r.below(PATHS.len())]).collect();
            assert_eq!(may_write(&c.policy.folders, path.as_bytes()), says(lines, &path), "{path}");
        }
    }
}

#[test]
fn every_box_holds_exec_and_no_network() {
    for lines in CORPUS {
        let c = compile(&parse(&file(lines)).unwrap()).unwrap();
        assert!(grants(c.policy.mask, TTY) && grants(c.policy.mask, EXEC));
        assert!(!grants(c.policy.mask, NET_ANON) && c.policy.dests.is_empty());
        let shown = allowed(&c.policy, None);
        assert_eq!(shown[2], "network none");
        assert_eq!(shown[3], "execve, each program held to this box");
    }
}

#[test]
fn a_home_folder_is_listed_only_when_the_program_has_a_home() {
    let lines = CORPUS[3];
    let c = compile(&parse(&file(lines)).unwrap()).unwrap();
    assert_eq!(c.policy.mask & FILES_HOME, FILES_HOME);
    assert_eq!(allowed(&c.policy, None)[0], "write /home/work /tmp/out");
    assert_eq!(allowed(&c.policy, Some(TTY | EXEC))[0], "write /tmp/out");
    assert_eq!(parse(&file(CORPUS[0])).unwrap().base, Base::Strict);
}
