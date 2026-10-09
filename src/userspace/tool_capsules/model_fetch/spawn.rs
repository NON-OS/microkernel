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

/* Running the model fetcher for the caller, as a tool capsule runs: through
 * the verified path, parented to the caller so it drains the fetcher's
 * output and can stop it, with `argv` (NUL-separated words) as its argv.
 * Its caps are its Capsule.mk's; no FileSystem, so it reads nothing on disk. */

use alloc::string::String;
use alloc::vec::Vec;

use super::embed::{ATTESTATION, CERT, ELF, MANIFEST};
use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified};
use crate::security::nonos_trust_anchor::{decode, BAKED_TRUST_ANCHOR_POLICY};
use crate::syscall::microkernel::errnos::{ERRNO_NETDOWN, ERRNO_NOENT, ERRNO_PERM};

/* The name `MkToolRun` takes for it. */
pub(crate) const TOOL: &[u8] = b"tool.model-fetch";

const CAPS: u64 = Capability::CoreExec.bit()
    | Capability::Network.bit()
    | Capability::IPC.bit()
    | Capability::Memory.bit()
    | Capability::Crypto.bit()
    | Capability::StreamImport.bit();

/* The fetcher's pid; ENOENT if not built in, ENETDOWN on a no-network boot,
 * EPERM when its signed artifacts do not verify, EBUSY while one runs. */
pub(crate) fn run_for_caller(argv: &[u8]) -> Result<u32, i64> {
    if ELF.is_empty() {
        return Err(ERRNO_NOENT);
    }
    /* The boot profile runs no network; say that, not that the caller lacks it. */
    if !crate::boot::handoff::boot_profile().network() {
        return Err(ERRNO_NETDOWN);
    }
    /* Only a caller that holds the network and the files itself, as the Terminal does. */
    let caller = crate::syscall::caps::current_caps_or_default();
    if !(caller.can_network() && caller.can_open_files()) {
        return Err(ERRNO_PERM);
    }
    let anchor = decode(BAKED_TRUST_ANCHOR_POLICY).map_err(|_| ERRNO_PERM)?;
    let spec = CapsuleSpecVerified {
        name: "tool.model-fetch",
        service_port: 4960,
        reply_inbox: "endpoint.tool.model-fetch.reply",
        reply_port: 4961,
        elf: ELF,
        nonos_id_cert_bytes: CERT,
        manifest_bytes: MANIFEST,
        attestation_trailer: ATTESTATION,
        target_triple: env!("NONOS_USER_TARGET"),
        requested_caps: CAPS,
        debug_tag: b"",
    };
    let pid = capsule_spawn::spawn_verified(&spec, &anchor, None).map_err(super::refusal::errno)?;
    let words = argv.split(|&b| b == 0).filter(|w| !w.is_empty());
    let words: Vec<String> = words.map(|w| String::from_utf8_lossy(w).into_owned()).collect();
    crate::process::with_process(pid, |pcb| *pcb.argv.lock() = words);
    Ok(pid)
}
