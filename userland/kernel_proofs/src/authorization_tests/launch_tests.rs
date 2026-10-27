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
//! The desktop shell may launch an installed app from a tile, and install,
//! uninstall and ask after an install no more than before. Its name adds
//! MkAppLaunch to the table and nothing else, and only to a valid token.

use super::token::{registered_syscalls, token};
use crate::capabilities::Capability;
use crate::syscall::contract::{is_allowed, is_allowed_as_shell};
use crate::syscall::numbers::SyscallNumber;

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

/// The caps the kernel grants the shell at spawn (capsule_desktop_shell).
fn shell_caps() -> Vec<Capability> {
    use Capability::*;
    vec![CoreExec, IPC, Network, Memory, FileSystem, GraphicsSurfaceCreate, SpawnWindow, Debug]
}

#[test]
fn the_shell_may_launch_and_still_not_install_or_uninstall() {
    let shell = token(shell_caps());
    assert!(is_allowed_as_shell(&shell, SyscallNumber::MkAppLaunch));
    assert!(!is_allowed(&shell, SyscallNumber::MkAppLaunch), "its caps alone do not launch");
    for n in [
        SyscallNumber::MkAppInstall,
        SyscallNumber::MkAppUninstall,
        SyscallNumber::MkAppInstallStatus,
    ] {
        assert!(!is_allowed_as_shell(&shell, n), "{n:?}");
    }
}

#[test]
fn the_shell_s_name_adds_only_the_launch() {
    for caps in [shell_caps(), vec![Capability::IPC], vec![Capability::AppInstall]] {
        let t = token(caps);
        let added: Vec<SyscallNumber> = registered_syscalls()
            .into_iter()
            .filter(|&n| is_allowed_as_shell(&t, n) != is_allowed(&t, n))
            .collect();
        let held = is_allowed(&t, SyscallNumber::MkAppLaunch);
        let want = if held { vec![] } else { vec![SyscallNumber::MkAppLaunch] };
        assert_eq!(added, want);
    }
    assert!(!is_allowed_as_shell(&token(vec![]), SyscallNumber::MkAppLaunch));
}

#[test]
fn an_app_install_holder_keeps_all_four() {
    let store = token(vec![Capability::IPC, Capability::AppInstall]);
    for n in [
        SyscallNumber::MkAppInstall,
        SyscallNumber::MkAppLaunch,
        SyscallNumber::MkAppInstallStatus,
        SyscallNumber::MkAppUninstall,
    ] {
        assert!(is_allowed(&store, n), "{n:?}");
    }
}
