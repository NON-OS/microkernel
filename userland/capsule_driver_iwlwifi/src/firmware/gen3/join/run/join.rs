// NONOS Operating System (AGPL-3.0-or-later)

//! A whole join, from the network's beacon to an open port.

use nonos_wifi_core::mlme::{JoinRequest, Mlme, MlmeFailure, MlmeState};

use super::super::super::region::{Clock, Region};
use super::super::bss::Bss;
use super::super::exchange::{exchange, state_code, ExchangeEnd, Progress};
use super::super::fw::Fw;
use super::super::target::target;
use super::ends::{JoinEnd, Joined};
use super::open::open;
use crate::regs::Mmio;

/// Join the network `req` names, from `beacon` heard on `heard_on`.
/// `progress` counts the exchange either way.
pub fn join<M: Mmio, R: Region + ?Sized, C: Clock>(
    fw: &mut Fw<'_, '_, M, R, C>,
    req: &JoinRequest<'_>,
    beacon: &[u8],
    heard_on: u8,
    tx_ant: u8,
    rx_ant: u8,
    progress: &mut Progress,
) -> Result<Joined, JoinEnd> {
    let mut mlme = Mlme::join(req);
    let first = mlme.on_mgmt(beacon).tx;
    progress.state = state_code(mlme.state());
    match (mlme.state(), first) {
        (MlmeState::Failed, _) => Err(JoinEnd::Refused(mlme.failure().unwrap_or(MlmeFailure::MalformedRsne))),
        (MlmeState::Authenticating, Some(first)) => {
            let t = target(beacon, heard_on).ok_or(JoinEnd::NotTheNetwork)?;
            let mut bss = Bss::new(t, req.our_mac, tx_ant, rx_ant);
            if let Err(e) = bss.setup(fw) {
                bss.teardown(fw);
                return Err(JoinEnd::Setup(e));
            }
            if let Err(e) = exchange(fw, &mut bss, &mut mlme, first, progress) {
                bss.teardown(fw);
                return Err(match e {
                    ExchangeEnd::Refused(f) => JoinEnd::Refused(f),
                    ExchangeEnd::TimedOut => JoinEnd::TimedOut,
                    ExchangeEnd::Firmware(c) => JoinEnd::Firmware(c),
                });
            }
            open(fw, bss, &mlme)
        }
        _ => Err(JoinEnd::NotTheNetwork),
    }
}
