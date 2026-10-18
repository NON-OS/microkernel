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

//! After a transfer failed or never finished, the endpoint it ran on is
//! brought back (`recover_endpoint`) before the reply goes out, so the class
//! driver's next request on it is served. Before this a STALLed control
//! request, which many HID devices answer to SET_IDLE, left EP0 halted and
//! every later request on the device timed out.

use super::recover_ring::recover_ring;
use crate::dma::DmaRegion;
use crate::error::XhciError;
use crate::server::context::Context;

/// The DCI of the default control endpoint.
pub const DCI_EP0: u8 = 1;

/// Recover endpoint `dci` of `slot` if `err` left it halted or running on
/// TRBs that will never complete, then let go of `buf`, the request's own
/// DMA buffer, once nothing can run them; until then it stays with the ring
/// (`recover_ring`), as a TRB the controller ran would DMA into a frame given
/// to someone else. A failed controller is sent no command. Returns whether
/// the endpoint is usable, so a STALL is not answered as a clear pipe.
pub fn recover_after(
    ctx: &mut Context,
    slot: u8,
    dci: u8,
    err: XhciError,
    buf: Option<DmaRegion>,
) -> bool {
    let left_behind = matches!(
        err,
        XhciError::TransferCompletionFailed(_) | XhciError::TransferCompletionTimeout
    );
    if left_behind && !ctx.driver.failed() {
        return recover_ring(ctx, slot, dci, buf);
    }
    let _ = keep_if_failed(ctx, slot, err, buf);
    !left_behind
}

/// Let go of `regions`, DMA memory a request or a Configure Endpoint that
/// ended in `err` gave the controller. A Host Controller Error does not
/// halt it (xHCI 1.2 section 5.4.2), so a failed controller may still reach
/// them: they stay with the slot until a revival has reset the controller,
/// teardown has released it, or Disable Slot drops the slot. `failed` also
/// covers an `err` of `ControllerDead`: each wait that answers it read the
/// failure from USBSTS or found the command ring wedged, and both hold until
/// the reset. Returns `err`.
pub fn keep_if_failed(
    ctx: &mut Context,
    slot: u8,
    err: XhciError,
    regions: impl IntoIterator<Item = DmaRegion>,
) -> XhciError {
    if ctx.driver.failed() {
        for region in regions {
            ctx.driver.slots.quarantine(slot, region);
        }
    }
    err
}
