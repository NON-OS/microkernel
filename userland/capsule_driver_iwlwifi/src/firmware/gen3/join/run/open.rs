// NONOS Operating System (AGPL-3.0-or-later)

//! The keys put in, the station authorized and the port opened.

use nonos_wifi_core::mlme::{Mlme, MlmeFailure};

use super::super::super::region::{Clock, Region};
use super::super::super::station::rates::rate_n_flags;
use super::super::bss::Bss;
use super::super::fw::Fw;
use super::super::keys::{Installed, KeyError};
use super::super::link::Link;
use super::ends::{JoinEnd, Joined};
use crate::regs::Mmio;

// Key the firmware from the connected MLME, authorize, open the port.
pub(super) fn open<M: Mmio, R: Region + ?Sized, C: Clock>(
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
