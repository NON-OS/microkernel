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

use super::job::worker::{send, Sent};
use super::say;
use crate::state::Context;

/// Start the job the plan says is due, if any. Returns at once: the worker
/// makes the catalog calls.
pub fn start(ctx: &mut Context) {
    let Some((index, kept)) = ctx.plan.begin() else {
        return;
    };
    let Some(catalog_port) = ctx.catalog_port else {
        say::failed(index, "finding the catalog");
        ctx.plan.not_started(index, kept, true);
        return;
    };
    match send(catalog_port, index, kept) {
        Sent::Out => {}
        Sent::Busy(kept) => ctx.plan.not_started(index, kept, false),
        Sent::Refused(kept) => {
            say::failed(index, "starting its worker");
            ctx.plan.not_started(index, kept, true);
        }
    }
}
