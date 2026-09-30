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

#![no_std]

pub mod admin;
pub mod attest;
pub mod battery;
pub mod broker;
pub mod caps;
mod capsule_load;
pub mod capsule_verify;
mod consent;
pub mod crypto;
pub mod data;
pub mod debug;
pub mod foreign;
pub mod foreign_fork;
pub mod foreign_frame;
pub mod foreign_signal;
pub mod graphics;
#[cfg(feature = "heap")]
pub mod heap;
pub mod install_source;
pub mod ipc;
mod local_sign;
pub mod mem;
#[cfg(feature = "panic-handler")]
mod panic;
pub mod peer;
pub mod private_write;
pub mod proc_output;
pub mod process;
pub mod procstat;
pub mod procstat_header;
pub mod spawn_instance;
pub mod store;
pub mod surface_registry;
mod syscall;
pub mod time;
pub mod tool_run;
pub mod transport;
pub mod tty;
mod unistd;

pub use admin::{mk_admin_policy_push, mk_admin_reboot, mk_admin_shutdown};
pub use attest::{
    mk_attest_doc, mk_attest_entries, mk_attest_status, AttestStatus, ATTEST_DOC_REFUSED,
    ATTEST_ENTRY_LEN,
};
pub use battery::mk_battery_status;
pub use broker::*;
pub use caps::{mk_cap_check, mk_cap_grant, mk_cap_revoke};
pub use capsule_load::{mk_capsule_load, CapsuleLoadRequest};
pub use capsule_verify::{mk_capsule_verify, CapsuleVerifyRequest, CapsuleVerifySummary};
pub use consent::{
    mk_dev_root_confirm, mk_dev_root_local, mk_local_consent_grant, mk_local_consent_revoke,
    mk_local_restore,
};
pub use crypto::{
    crypto_decrypt, crypto_decrypt_aad, crypto_encrypt, crypto_encrypt_aad, crypto_hash,
    crypto_hkdf_sha256, crypto_hmac_sha256, crypto_keccak256, crypto_machine_key, crypto_random,
    crypto_x25519_public, crypto_x25519_shared, machine_key, MACHINE_KEY_LABEL_MAX,
    MACHINE_KEY_NO_TPM, MACHINE_KEY_WRONG_STATE,
};
pub use data::{
    mk_data_import, mk_data_read, mk_data_read_peer, mk_data_stat, mk_data_volume_passphrase,
};
pub use debug::mk_debug;
pub use foreign::{
    mk_foreign_exec, mk_foreign_reply, mk_foreign_resume, mk_foreign_spawn, mk_foreign_start,
    mk_foreign_thread, mk_foreign_wait,
};
pub use foreign_fork::{mk_foreign_fork, mk_foreign_fork_at};
pub use foreign_frame::{ForeignFrame, FOREIGN_NR_DIED};
pub use foreign_signal::{
    mk_foreign_context, mk_foreign_signal, ForeignRegs, SIGNAL_DELIVER, SIGNAL_RETURN,
};
pub use graphics::nonos_display_dimensions;
#[cfg(feature = "heap")]
pub use heap::{init as heap_init, init_sized as heap_init_sized, HeapError};
pub use install_source::{
    mk_install_source, mk_install_source_size, INSTALL_SOURCE_KERNEL_IMAGE,
    INSTALL_SOURCE_LOADER_IMAGE,
};
pub use ipc::{
    mk_ipc_call, mk_ipc_call_timeout, mk_ipc_recv, mk_ipc_recv_from, mk_ipc_reply, mk_ipc_send,
    mk_ipc_send_to_pid, mk_service_lookup, mk_service_register,
};
pub use local_sign::{
    mk_app_install, mk_app_install_status, mk_app_launch, mk_local_sign, mk_local_sign_len,
    mk_local_verify,
};
pub use mem::{mk_mmap, mk_munmap};
pub use private_write::mk_private_write;
pub use proc_output::{mk_proc_input, mk_proc_output, mk_stdin_read};
pub use process::{mk_args, mk_getpid, mk_kill, mk_pid_alive, mk_wait};
pub use procstat::{mk_proc_stat, ProcStatEntry, PROC_NAME_LEN};
pub use procstat_header::ProcStatHeader;
pub use spawn_instance::mk_spawn_instance;
pub use store::{mk_store_read, mk_store_write};
pub use surface_registry::{
    mk_display_vsync_wait, mk_input_event_drain, mk_input_event_post, mk_input_event_wait,
    mk_surface_attach, mk_surface_present, mk_surface_present_rect, mk_surface_register,
    mk_surface_release, mk_surface_share, InputEvent, SurfaceDescriptor, INPUT_KIND_BUTTON_DOWN,
    INPUT_KIND_BUTTON_UP, INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP, INPUT_KIND_POINTER_ABS,
    INPUT_KIND_POINTER_REL, INPUT_KIND_TOUCH, INPUT_KIND_WHEEL, SURFACE_FORMAT_ARGB8888,
};
pub use syscall::call_raw as mk_syscall_raw;
pub use time::{mk_time_adjust, mk_time_millis, mk_time_rtc, mk_uptime_ms, Deadline, RtcTime};
pub use tool_run::mk_tool_run;
pub use tty::{mk_tty_query, mk_tty_set, TTY_STDERR, TTY_STDIN, TTY_STDOUT};
pub use unistd::{mk_exit, mk_idle_ms, mk_yield};
