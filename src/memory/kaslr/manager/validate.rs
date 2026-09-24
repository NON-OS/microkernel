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

//! The state KASLR is in, checked rather than assumed.

use super::super::constants::{SAFE_SLIDE_MAX, SAFE_SLIDE_MIN};
use super::super::error::{KaslrError, KaslrResult};
use super::init::{boot_nonce, get_slide};
use crate::memory::layout;

/// Accept the two states this kernel supports and reject the rest.
///
/// The nonce must be set. It is the per-boot secret behind the stack canary
/// and the allocator seeds, and a zero there is the whole difference between
/// a secret and a compile-time constant.
///
/// A zero slide is accepted, and means the layout was not moved. That is
/// where this kernel stands while its consumers still read the layout
/// constants directly rather than the layout record; `nonce.rs` carries the
/// argument. A slide that is set is held to the alignment and the range the
/// derivation promises.
///
/// The previous form read the nonce with `?` and then tested it for zero,
/// which the `?` had already ruled out, and applied the slide bounds to a
/// zero slide, so an unslid kernel could never validate.
pub fn validate() -> KaslrResult<()> {
    boot_nonce()?;
    let slide = get_slide();
    if slide == 0 {
        return Ok(());
    }
    if !slide.is_multiple_of(layout::PAGE_SIZE as u64) {
        return Err(KaslrError::SlideNotAligned);
    }
    if !(SAFE_SLIDE_MIN..=SAFE_SLIDE_MAX).contains(&slide) {
        return Err(KaslrError::SlideOutOfRange);
    }
    Ok(())
}
