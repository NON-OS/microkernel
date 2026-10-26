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

//! The Mk* calls the microkernel dispatch handles, part 4 of 4
//! (microkernel_ops.rs).

use crate::syscall::numbers::SyscallNumber::{self, *};

pub(super) fn matches(nr: SyscallNumber) -> bool {
    matches!(
        nr,
        MkDataPassphrase
            | MkDataFeedBegin
            | MkDataFeed
            | MkDataRemove
            | MkSpawnInstance
            | MkForeignSpawn
            | MkForeignStart
            | MkForeignWait
            | MkForeignReply
            | MkForeignContext
            | MkForeignSignal
            | MkForeignInterrupt
            | MkPeerMap
            | MkPeerCopy
            | MkPeerProtect
            | MkForeignThread
            | MkPeerTls
            | MkForeignFork
            | MkPeerUnmap
            | MkForeignExec
            | MkLinuxRunNet
            | MkSealRead
            | MkSealNote
            | MkProveJob
            | MkLocalSign
            | MkLocalVerify
            | MkAppInstall
            | MkDevRootLocal
            | MkLocalConsent
            | MkLocalRestore
            | MkAppLaunch
            | MkAppInstallStatus
            | MkAppUninstall
            | MkToolRun
            | MkCapsOf
            | MkTtySet
            | MkTtyQuery
    )
}
