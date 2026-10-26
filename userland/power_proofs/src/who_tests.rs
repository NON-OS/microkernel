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

//! The check that keeps a power-off the shell's: the kernel lets the shell
//! and both installers into capsule_power's inbox (HELD), so the capsule
//! tells them apart by the sender the kernel stamped. An installer may
//! restart; its POFF frame and the `power` port's shutdown are refused.

use crate::rule::{op_allowed, verdict, Caller, Verdict, EPERM, INSTALLERS};
use crate::who::{classify, SHELL};

const RUNNER: &str = include_str!("../../capsule_power/src/server/runner.rs");
const HELD: &str = include_str!("../../../src/services/registry/held_table.rs");

fn flat(s: &str) -> String {
    s.lines()
        .map(|l| l.split("//").next().unwrap())
        .collect::<String>()
        .split_whitespace()
        .collect()
}

#[test]
fn an_installer_may_restart_and_may_not_power_off() {
    for name in INSTALLERS {
        let from = classify(|s| s == name);
        assert_eq!(from, Caller::Installer, "{name:?}");
        assert_eq!(verdict(true, from == Caller::Shell), Verdict::Refuse(EPERM), "POFF");
        assert!(!op_allowed(3, from) && op_allowed(2, from), "{name:?}");
    }
    assert_eq!(classify(|s| s == SHELL || s == INSTALLERS[0]), Caller::Shell, "shell first");
    for name in [&b"app.settings"[..], b"power", b"install", b""] {
        assert_eq!(classify(|s| s == name), Caller::Other, "{name:?}");
    }
    assert_eq!(classify(|_| false), Caller::Other, "a process that owns no service");
}

#[test]
fn the_runner_classifies_the_kernel_stamped_sender() {
    let r = flat(RUNNER);
    let stamped = r.find("RECV_TIMEOUT_MS,&mutsender_pid,);").expect("the kernel's sender");
    let who = r.find("letfrom=classify(|s|super::caller::runs(sender_pid,s));").expect("classify");
    assert!(stamped < who && who < r.find("letfrom_shell=from==Caller::Shell;").unwrap());
    let sets = r.matches("sender_pid=").count() - r.matches("sender_pid==").count();
    assert_eq!(sets, 1, "nothing else sets the sender");
    assert!(!r.contains("INSTALLERS") && !r.contains("desktop_shell"));
}

#[test]
fn the_kernels_list_names_the_shell_and_both_installers_only() {
    let line = HELD.lines().find(|l| l.contains("(\"power.off\",")).expect("HELD line");
    let names: Vec<&str> = line.split('"').skip(3).step_by(2).collect();
    let mut want = vec![core::str::from_utf8(SHELL).unwrap()];
    want.extend(INSTALLERS.iter().map(|s| core::str::from_utf8(s).unwrap()));
    assert_eq!(names, want);
}
