// NONOS Operating System (AGPL-3.0-or-later)
//! A rekeyed group key is not kept once the firmware was asked to take it,
//! whether it went in or not.

use nonos_wifi_core::key::HeldKey;
use nonos_wifi_core::rsn::JoinPolicy;

use super::drive::run_join;
use crate::join_rig::*;

#[test]
fn a_group_key_the_firmware_refused_is_not_kept() {
    let r = rig(Security::Wpa2, Switches::default());
    let mut seen = None;
    run_join(&r, JoinPolicy::ANY, |fw, out, _| {
        let mut j = out.unwrap_or_else(|e| panic!("joined, not {e:?}"));
        assert!(j.rekey(fw).is_none(), "nothing waits");
        // Key index 7 is no group key index: the firmware is not asked.
        j.link.new_group_key = Some((7, HeldKey([0x99; 16])));
        seen = Some((j.rekey(fw).map(|r| r.is_err()), j.link.new_group_key.is_none()));
    });
    assert_eq!(seen, Some((Some(true), true)), "refused, and gone");
}
