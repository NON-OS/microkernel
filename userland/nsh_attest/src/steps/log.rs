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

//! Step 5, the firmware's log: replayed, it must give exactly the quoted
//! values of the PCRs firmware extends. In PCR 4 every application must be
//! the release's loader or a boot application this machine's enrolment
//! lists, and the loader must be among them. Which one ran last is not
//! enough: an earlier application could load the loader and never start
//! it. The boot applications are per machine and signed by the fleet's
//! authority with the rest of the enrolment, so one machine's boot
//! manager admits nothing on another.

use crate::consts::REPLAYED_PCRS;
use crate::log::{replay, Replayed};
use crate::{Doc, Refusal, Release};
use nsh_wire::Enrol;

pub fn log(doc: &Doc<'_>, release: &Release, enrol: &Enrol<'_>) -> Result<Replayed, Refusal> {
    let r = replay(doc.log).map_err(Refusal::Log)?;
    for i in REPLAYED_PCRS {
        if doc.pcr(i) != r.pcrs.get(i as usize) {
            return Err(Refusal::Replay(i));
        }
    }
    let listed = |a: &[u8; 32]| *a == release.loader || enrol.boot_application(a);
    if !r.applications.iter().all(listed) {
        return Err(Refusal::BootApplication);
    }
    match r.applications.contains(&release.loader) {
        true => Ok(r),
        false => Err(Refusal::Loader),
    }
}
