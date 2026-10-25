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

//! MkSpawnInstance, the other right SpawnWindow gives the Terminal, starts
//! an app instance with exactly the app's own caps: the call carries a name
//! and nothing else, init spawns the instance in its own context, and the
//! caps are the app's fixed spawn set, never anything read from the caller.

use super::source::code_of;

macro_rules! app {
    ($a:literal) => {
        include_str!(concat!("../../../../src/userspace/capsule_", $a, "/spawn.rs"))
    };
}

#[rustfmt::skip]
const APPS: [&str; 14] = [
    app!("about"), app!("audio_player"), app!("browser"),
    app!("calculator"), app!("clock"), app!("file_manager"),
    app!("process_manager"), app!("prove"), app!("settings"),
    app!("snake"), app!("terminal"), app!("text_editor"),
    app!("video_player"), app!("wallet_nonos"),
];

const CALLER: [&str; 4] = ["current_caps", "current_pid", "caps::bits", "caps::has"];

fn flat(src: &str) -> String {
    code_of(src).split_whitespace().collect()
}

fn reads_no_caller(src: &str, what: &str) {
    for c in CALLER {
        assert!(!flat(src).contains(c), "{what} reads `{c}`");
    }
}

#[test]
fn the_call_carries_a_name_and_only_queues_it() {
    let sys = include_str!("../../../../src/syscall/microkernel/spawn_instance.rs");
    let code = flat(sys);
    assert!(code.contains("pubfnsys_spawn_instance(name_ptr:u64,name_len:u64)->i64{"));
    assert!(code.contains("letresult=queue_by_name(name);"));
    reads_no_caller(sys, "the syscall");
}

#[test]
fn an_instance_installs_its_app_s_own_caps() {
    let inst =
        include_str!("../../../../src/kernel_core/process_spawn/capsule_spawn/instance/mod.rs");
    let code = flat(inst);
    assert!(code.contains("requested_caps:app.requested_caps,"));
    assert!(code.contains("letpid=spawn_verified(&spec,&trust,None)?;"));
    reads_no_caller(inst, "spawn_next");
    for (i, src) in APPS.iter().enumerate() {
        assert!(flat(src).contains("spawn_next_instance(&InstanceSpawn{"), "app {i}");
        reads_no_caller(src, "an app's spawn");
    }
}
