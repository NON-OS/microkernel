// NONOS Operating System (AGPL-3.0-or-later)

//! Before the hunt: a rejoin the person ended is refused and the address key
//! drawn. A hidden network's probes go out under an address drawn for this
//! hunt alone: its security, which keys its own address, is read only from
//! the answer.

use nonos_wifi_core::address::one_off;
use nonos_wifi_core::link_history::CODE_LEFT;
use nonos_wifi_core::vault::wipe;

use super::request::ConnectRequest;
use super::result::{ConnectResult, CODE_BAD_REQUEST, CODE_NO_ENTROPY};
use crate::fw::dma::Grant;
use crate::link::{NoAddress, RtlLink};
use crate::regs::Regs;
use crate::station::draw_key;

/// The address a hidden network's probes go out under (none for a network
/// that is heard, not asked for), or the answer that ends the join.
pub(super) fn ready(
    link: &mut RtlLink<Regs, Grant, Grant>,
    req: &ConnectRequest<'_>,
) -> Result<Option<[u8; 6]>, ConnectResult> {
    if req.rejoin && link.history().refuses_rejoin() {
        return Err(ConnectResult::code(CODE_LEFT));
    }
    link.address_ready(draw_key).map_err(refused)?;
    if !req.hidden {
        return Ok(None);
    }
    let mut draw = draw_key().ok_or(refused(NoAddress::NoKey))?;
    let mut six = [0u8; 6];
    six.copy_from_slice(&draw[..6]);
    wipe(&mut draw);
    let mac = one_off(&six);
    link.take_address(mac).map_err(refused)?;
    Ok(Some(mac))
}

/// The answer for a join with no address.
pub(super) fn refused(why: NoAddress) -> ConnectResult {
    match why {
        NoAddress::NoKey => ConnectResult::code(CODE_NO_ENTROPY),
        NoAddress::Associated => ConnectResult::code(CODE_BAD_REQUEST),
    }
}
