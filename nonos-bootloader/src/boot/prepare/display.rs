// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::display::{log_hex, log_ok, show_handoff_message};
use crate::entropy::{rdrand64, rdseed64};
use crate::loader::KernelImage;

/*
 * The seed is the kernel's first secret: it seeds the CSPRNG that keys are
 * drawn from, so it is never shown or logged. The screen says only that
 * entropy was gathered and from what.
 */
pub fn show_handoff_status() {
    log_ok(b"Entropy collected");
    log_ok(entropy_source());
    log_ok(b"CryptoHandoff prepared");
    log_ok(b"FirmwareHandoff prepared");
}

fn entropy_source() -> &'static [u8] {
    if rdseed64().is_some() {
        b"Entropy source RDSEED, TSC jitter, RTC"
    } else if rdrand64().is_some() {
        b"Entropy source RDRAND, TSC jitter, RTC"
    } else {
        b"Entropy source TSC jitter, RTC"
    }
}

pub fn show_completion_status(kernel_image: &KernelImage) {
    log_ok(b"All boot stages COMPLETE");
    log_hex(b"jumping ", kernel_image.entry_point as u64);
    show_handoff_message();
}
