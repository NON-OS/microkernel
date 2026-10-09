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

use super::super::super::spec::SpawnError;
use super::params::InstallParams;
use crate::capabilities::Capability;
use crate::ipc::nonos_inbox;
use crate::kernel_core::process_spawn::{
    allocate_kernel_stack, allocate_user_stack, setup_initial_user_context,
};
use crate::process::core::inbox_name::InboxName;
use crate::process::core::{create_process_with_parent, ProcessState};
use crate::services::registry::{adopt_endpoint, register_endpoint, required_caps};

pub(crate) fn run(params: &InstallParams<'_>) -> Result<u32, SpawnError> {
    super::trace::trace(params.name, b"install enter");
    if params.elf.is_empty() {
        return Err(SpawnError::FeatureDisabled);
    }
    /*
     * The name is checked before anything is registered under it. Failing
     * after the inbox and the endpoint exist would leave an unowned endpoint
     * and a process nobody will schedule, with no path that takes them back.
     */
    if InboxName::new(params.reply_inbox).is_none() {
        return Err(SpawnError::InboxName);
    }
    nonos_inbox::register_or_get_bootstrap_inbox(params.reply_inbox);
    register_endpoint(params.reply_inbox, params.reply_port, 0, 0)
        .map_err(|_| super::trace::collided(params.name))?;
    let pid = match create_process_with_parent(
        params.name,
        ProcessState::Ready,
        super::priority::for_capsule(params.name),
        0,
        params.on_behalf_of,
    ) {
        Ok(pid) => pid,
        Err(_) => {
            forget_reply(params.reply_inbox);
            return Err(SpawnError::ProcessCreation);
        }
    };
    let named = crate::process::with_process(pid, |pcb| pcb.set_reply_inbox(params.reply_inbox));
    if named != Some(true) {
        forget_reply(params.reply_inbox);
        crate::process::exit::teardown(pid, SPAWN_FAILED, false);
        return Err(if named.is_none() { SpawnError::ProcessCreation } else { SpawnError::InboxName });
    }
    /*
     * From here the process exists and holds the reply inbox. A step that
     * fails tears it down as an exit does, which gives back its address
     * space, its endpoints and the reply slot; left standing, the next spawn
     * under this name collides on the slot (a Settings window that could not
     * get memory once could then never open again).
     */
    match finish(params, pid) {
        Ok(()) => Ok(pid),
        Err(e) => {
            crate::process::exit::teardown(pid, SPAWN_FAILED, false);
            Err(e)
        }
    }
}

/// The exit status a spawn that failed half way is torn down with.
const SPAWN_FAILED: i32 = -1;

/* The reply endpoint and inbox registered before a pid existed. */
fn forget_reply(reply_inbox: &str) {
    let _ = crate::services::registry::unregister_endpoint_by_name(reply_inbox);
    let _ = nonos_inbox::unregister_inbox(reply_inbox);
}

fn finish(params: &InstallParams<'_>, pid: u32) -> Result<(), SpawnError> {
    /*
     * The reply inbox was registered unowned above, because its name is needed
     * before a pid exists. Claim it now. An unowned inbox with no entry
     * requirement is one any capsule may write into, which is how a forged
     * reply gets into somebody else's request and response flow.
     */
    adopt_endpoint(params.reply_inbox, pid, Capability::IPC.bit())
        .map_err(|_| SpawnError::EndpointCollision)?;
    super::own_inboxes::register(pid)?;
    let entry = super::load_elf_into_pid::load_elf_into_pid(params.elf, pid, params.debug_tag)?;
    let caps = params.caps_bits;
    super::install_caps::install_caps(pid, caps)?;
    let _kernel_stack = allocate_kernel_stack(pid).map_err(|_| SpawnError::AddressSpace)?;
    let user_rsp = allocate_user_stack(pid).map_err(|_| SpawnError::AddressSpace)?;
    setup_initial_user_context(pid, entry, user_rsp).map_err(|_| SpawnError::AddressSpace)?;
    let service_caps = required_caps(params.name, Capability::IPC.bit());
    register_endpoint(params.name, params.service_port, pid, service_caps)
        .map_err(|_| super::trace::collided(params.name))?;
    super::spawn_log::emit(params.name, pid, caps, entry);
    /*
     * Nothing below can fail. A terminal's run of the Linux personality gets
     * its request and is marked private here, before it can first run.
     */
    crate::userspace::capsule_linux::admit_terminal_run(params.name, pid);
    crate::sched::add_to_run_queue(pid);
    crate::sys::bench::mark_named(b"capsule_runqueue_ok", params.name.as_bytes());
    super::trace::trace(params.name, b"runqueue ok");
    Ok(())
}
