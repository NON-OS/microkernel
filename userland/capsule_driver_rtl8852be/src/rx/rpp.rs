// NONOS Operating System (AGPL-3.0-or-later)

//! One RPQ buffer's release reports, `rtw89_pci_release_tx_skbs` with
//! `rtw89_pci_release_rpp` (Linux v6.12 rtw89 `pci.c`): after the info
//! word and the descriptor, one 4-byte report per frame the MAC is done
//! with, naming its WD's sequence and the queue it left. A buffer that is
//! not one whole segment is skipped, as rtw89 skips it.

use super::desc::parse;
use crate::ring::bd::{rx_info, RXBD_INFO_SIZE};
use crate::ring::wd::{rpp, Rpp};

/// The reports in `buf`, handed to `each`; how many.
pub fn reports(buf: &[u8], mut each: impl FnMut(Rpp)) -> usize {
    let Some(head) = buf.get(..RXBD_INFO_SIZE) else { return 0 };
    let info = rx_info(u32::from_le_bytes([head[0], head[1], head[2], head[3]]));
    if !(info.fs && info.ls) {
        return 0;
    }
    let len = (info.len as usize).min(buf.len());
    let Some(d) = parse(&buf[..len]) else { return 0 };
    let mut n = 0;
    let mut at = d.data;
    while at + 4 <= len {
        each(rpp(u32::from_le_bytes([buf[at], buf[at + 1], buf[at + 2], buf[at + 3]])));
        at += 4;
        n += 1;
    }
    n
}
