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

//! A whole join, from the network's beacon to an open port, and leaving it.
//!
//! The MLME reads the beacon first: a network it will not join (open, TKIP,
//! Enterprise, HT-only, saved as WPA3 and now offering only WPA2, a
//! passphrase that cannot be one) is refused there, before any firmware
//! context exists or any frame is sent. Then the contexts go up (`Bss`), the
//! frames are exchanged to a connected MLME (`exchange`), the pairwise and
//! group keys go into the firmware (`Installed`), the station entry is
//! authorized, and only then is the port opened (`Link`). Any step that ends
//! the join takes down what was put up, keys first.

use nonos_wifi_core::mlme::{JoinRequest, Mlme, MlmeFailure, MlmeState};
use nonos_wifi_core::wpa::akm::Akm;

use super::super::region::{Clock, Region};
use super::super::station::rates::rate_n_flags;
use super::bss::{Bss, SetupError};
use super::exchange::{exchange, state_code, ExchangeEnd, Progress};
use super::fw::{CommandFailed, Fw, Queue};
use super::keys::{Installed, KeyError};
use super::link::Link;
use super::target::target;
use crate::regs::Mmio;

/// How long a leaving station waits for its deauthentication to go out
/// before the queues are flushed.
pub const LEAVE_MS: u32 = 100;

/// A join that reached an open port.
pub struct Joined {
    pub bss: Bss,
    pub keys: Installed,
    pub link: Link,
    pub akm: Akm,
}

/// How a join ended without a port.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum JoinEnd {
    /// The beacon is not the network's, or names no channel.
    NotTheNetwork,
    /// The MLME refused the network or the access point refused the station.
    Refused(MlmeFailure),
    /// The access point stopped answering.
    TimedOut,
    /// The firmware's contexts could not be put up.
    Setup(SetupError),
    /// A firmware command failed after the contexts were up, or the firmware
    /// raised its error cause.
    Firmware(Option<CommandFailed>),
    /// A key did not go in.
    Keys(KeyError),
}

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

// Key the firmware from the connected MLME, authorize, open the port.
fn open<M: Mmio, R: Region + ?Sized, C: Clock>(
    fw: &mut Fw<'_, '_, M, R, C>,
    mut bss: Bss,
    mlme: &Mlme,
) -> Result<Joined, JoinEnd> {
    let (Some(tk), Some(gtk), Some(gtk_id), Some(akm), Some(sup)) =
        (mlme.tk(), mlme.gtk(), mlme.gtk_id(), mlme.akm(), mlme.supplicant())
    else {
        bss.teardown(fw);
        return Err(JoinEnd::Refused(MlmeFailure::AssocRejected(0)));
    };
    let (Ok(tk), Ok(gtk)) = (<[u8; 16]>::try_from(tk), <[u8; 16]>::try_from(gtk)) else {
        bss.teardown(fw);
        return Err(JoinEnd::Keys(KeyError::Group(None)));
    };
    let mut keys = Installed::default();
    let keyed = keys.install(fw, &tk, sup.pmf(), &gtk, gtk_id).map_err(JoinEnd::Keys);
    let authorized = keyed.and_then(|()| bss.authorized(fw, sup.pmf()).map_err(|c| JoinEnd::Firmware(Some(c))));
    if let Err(e) = authorized {
        keys.remove(fw);
        bss.teardown(fw);
        return Err(e);
    }
    let rates = bss.target.rates;
    let link = Link::new(
        bss.addr,
        bss.target.bssid,
        tk,
        sup,
        rate_n_flags(rates.data, bss.tx_ant),
        rate_n_flags(rates.mgmt, bss.tx_ant),
    );
    Ok(Joined { bss, keys, link, akm })
}

/// Leave: tell the access point, close the port, remove the keys and take the
/// contexts down.
pub fn leave<M: Mmio, R: Region + ?Sized, C: Clock>(fw: &mut Fw<'_, '_, M, R, C>, j: &mut Joined) {
    if let Some(deauth) = j.link.deauth() {
        if fw.send(Queue::Mgmt, &deauth, j.link.mgmt_rate) {
            fw.settle(LEAVE_MS);
        }
    }
    forget(fw, j);
}

/// Close the port, remove the keys and take the contexts down, telling the
/// access point nothing (it already ended the association).
pub fn forget<M: Mmio, R: Region + ?Sized, C: Clock>(fw: &mut Fw<'_, '_, M, R, C>, j: &mut Joined) {
    j.link.station.deassociate();
    j.keys.remove(fw);
    j.bss.teardown(fw);
}
