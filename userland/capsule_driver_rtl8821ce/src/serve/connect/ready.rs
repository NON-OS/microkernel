// NONOS Operating System (AGPL-3.0-or-later)

//! Before the hunt: a rejoin the person ended is refused, and the join takes
//! this network's own address, so no two networks see the same one.

use nonos_wifi_core::link_history::CODE_LEFT;

use super::request::ConnectRequest;
use super::result::{ConnectResult, CODE_BAD_REQUEST, CODE_NO_ENTROPY};
use crate::fw::dma::Grant;
use crate::link::{NoAddress, RtlLink};
use crate::regs::Regs;

/// The address the join runs under, or the answer that ends it.
pub(super) fn ready(
    link: &mut RtlLink<Regs, Grant, Grant>,
    req: &ConnectRequest<'_>,
) -> Result<[u8; 6], ConnectResult> {
    if req.rejoin && link.history().refuses_rejoin() {
        return Err(ConnectResult::code(CODE_LEFT));
    }
    match link.readdress(req.ssid, crate::station::draw_key) {
        Ok(mac) => Ok(mac),
        Err(NoAddress::NoKey) => Err(ConnectResult::code(CODE_NO_ENTROPY)),
        Err(NoAddress::Associated) => Err(ConnectResult::code(CODE_BAD_REQUEST)),
    }
}
