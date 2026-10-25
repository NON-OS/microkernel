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
//! One drawing of the splash at one canvas size: its surface, the logo
//! placed for that size and the layout around it, painted and handed to
//! the compositor. A later drawing is submitted at the same depth, so the
//! compositor puts it in place of the earlier one; the earlier surface is
//! given back only after that, and the screen never shows neither.

use nonos_libc::{mk_munmap, mk_surface_release, mk_uptime_ms, AttestStatus};

use crate::attest_lines::Chain;
use crate::follow::Size;
use crate::look::Look;
use crate::{layout_fit, logo, paint, say, scene, surface};

/// A splash surface the compositor holds.
pub(crate) struct Up {
    pub base: u64,
    pub handle: u64,
    pub size: Size,
}

/// Draws the splash at `size` and submits it; the error is run's exit code.
pub(crate) fn raise(
    comp: u32,
    size: Size,
    att: AttestStatus,
    chain: Option<Chain>,
) -> Result<(Up, Look), i32> {
    let (w, h, stride) = size;
    let (base, handle) = surface::setup(w, h, stride).ok_or(5)?;
    let up = Up { base, handle, size };
    let logo = logo::fetch::fetch(w, h);
    let fit = layout_fit::layout(logo.as_ref().map(|l| &l.spot), w, h);
    let look = Look { att, chain, logo, fit, dma_unconfined: crate::dma::unconfined() };
    paint::splash(base, w, h, stride, &look);
    if scene::submit(comp, 3, handle, w, h).is_err() {
        give_back(&up);
        return Err(6);
    }
    let _ = scene::damage(comp, 4, (0, 0, w, h));
    say::line(&alloc::format!("up {}x{} at {} ms", w, h, mk_uptime_ms()));
    Ok((up, look))
}

/// Gives `up`'s surface back, then its pages; the pages stay mapped if the
/// kernel did not take the release, so nothing can read freed memory.
pub(crate) fn give_back(up: &Up) {
    if mk_surface_release(up.handle) < 0 {
        return;
    }
    let len = up.size.2 as usize * up.size.1 as usize;
    let _ = mk_munmap(up.base as *mut u8, len);
}
