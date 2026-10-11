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

use super::frames;
use super::icc;
use super::redistributor;
use super::state::redist_base;

/// Bring up the calling core's half of a GICv3: find the redistributor frame
/// that names it, configure its SGIs and PPIs, and turn on the CPU interface.
///
/// Without its redistributor this CPU can neither wake for SGIs nor enable
/// its private interrupts, so the timer tick would never arrive. That is an
/// error for the caller to report, not something to boot on past.
pub(super) fn init_gic_cpu() -> Result<(), &'static str> {
    let redist = redistributor::for_this_cpu(redist_base())
        .ok_or("no GICv3 redistributor frame claims this CPU")?;
    redist.init()?;
    frames::record(&redist);
    icc::init();
    Ok(())
}
