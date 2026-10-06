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

//! Starting the write from the prepared plan: the one made when the disk
//! was chosen and shown on the confirm screen, so what is written is what
//! the person read. Nothing touches the disk before this.

use alloc::string::{String, ToString};

use nonos_blk_client::DeviceSink;
use nonos_disk::{Layout, Session, WriteError};
use nonos_libc::mk_time_millis;

use super::work::Job;
use crate::install::state::State;

pub fn start(state: &mut State) -> Result<(), String> {
    let prepared = match state.prepared.take() {
        Some(Ok(p)) => p,
        Some(Err(why)) => return Err(why),
        None => return Err(String::from("no plan was made for this disk")),
    };
    let now = mk_time_millis().max(0) as u64;
    let sink = DeviceSink { device: prepared.device };
    state.job = Some(Job::writing(sink, Session::new(prepared.plan), now));
    Ok(())
}

/// `e` in the writer's words, and for a sector that read back wrong, what
/// that sector holds.
pub fn describe(e: WriteError, layout: Option<&Layout>) -> String {
    match (e, layout) {
        (WriteError::Mismatch { lba }, Some(l)) => alloc::format!("{e}, in {}", l.what_is_at(lba)),
        _ => e.to_string(),
    }
}
