// NONOS Operating System (AGPL-3.0-or-later)
//! A read that stalls on any byte, the address's own included, leaves no
//! efuse content in the reply: only the named flags, the stalled address and
//! the bank and LDO bits.

use super::card::{carries_factory, Efuse};
use crate::efuse::{diag, read, EfuseDiag};
use crate::link::LinkStats;
use crate::serve_status::{status_body, STATUS_BODY_LEN};

#[test]
fn a_stall_on_any_byte_reports_no_efuse_content() {
    for at in 0..512 {
        assert_eq!(read(&Efuse::new(Some(at))), None, "{at}");
        let d = diag();
        assert_eq!(d.flags, EfuseDiag::ADDR_LATCHED, "{at}: the window answers");
        assert_eq!(d.stalled_at, at as u32 + 1);
        assert_eq!(d.bank_ldo, (1 << 31) | (1 << 9) | (1 << 8), "{at}: bank and LDO only");
        let mut out = [0u8; STATUS_BODY_LEN];
        assert_eq!(status_body(2, &LinkStats::default(), &d, &mut out), Some(STATUS_BODY_LEN));
        assert!(!carries_factory(&out), "{at}: {out:?}");
    }
}
