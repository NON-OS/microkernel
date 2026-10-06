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

//! Following a Launchpad launch the installer is still loading, one bounded
//! step per turn of the shell's serve loop: a registry lookup, now and then a
//! health probe of at most 20 ms, and, once the installer is done, one read of
//! the process table. None of it waits on the load itself
//! (`state/launch.rs` has the reasoning).

use nonos_app_skeleton::log_line::{say as log, Line};

use super::installed_launch::{already_running, boot, said, toast};
use super::launch_children::new_child;
use crate::installer_client::probe;
use crate::state::launch::{Launch, Step};
use crate::state::{Context, NotifyLevel, Uptime};

pub(crate) fn poll(ctx: &mut Context) {
    let now = crate::server::dock_clock::now();
    let Some(launch) = ctx.launch.as_mut() else {
        return;
    };
    if !launch.due(now) {
        return;
    }
    let mut step = launch.step(now, find(launch, now));
    if step == Step::Probe {
        launch.probed(now, probe());
        step = launch.step(now, find(launch, now));
    }
    match step {
        Step::Wait | Step::Probe => {}
        Step::Started(pid) => {
            if let Some(launch) = ctx.launch.take() {
                ctx.installed_pids.insert(launch.name, pid);
            }
            boot(pid);
        }
        Step::Failed => {
            if let Some(launch) = ctx.launch.take() {
                toast(ctx, &said(b"", &launch.name, b" did not start"), NotifyLevel::Error);
                let line = Line::new(b"LAUNCH").text(&launch.name).text(b" did not start: ");
                let _ =
                    log(&line.text(b"no process of it in 30 s, or the installer ended the load"));
            }
        }
    }
}

/// The app, by the service name packaged apps register, or, once the
/// installer has finished, as the child it made for the shell.
fn find(launch: &Launch, now: Uptime) -> Option<u32> {
    if let Some(pid) = already_running(&launch.name) {
        return Some(pid);
    }
    if launch.served() {
        return new_child(&launch.children, launch.elapsed_ms(now));
    }
    None
}
