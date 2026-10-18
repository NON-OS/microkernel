// NONOS Operating System (AGPL-3.0-or-later)

//! The discrete AX210 and Meteor Lake: images and transport values.

use crate::gen3::select::{select, transport, Image, Refusal};

/* The discrete AX210 (TY, 0x2725) and Meteor Lake (MA, 0x7E40 and 0x2729)
share the gen3 boot. Linux names their firmware ty-a0-gf-a0 and ma-b0-gf-a0
(iwl_drv_get_fw_name: TY's step is always 'a'; MA's is the low nibble of
CSR_HW_REV, 1 for B0). */
#[test]
fn the_discrete_ax210_and_meteor_lake_get_their_images() {
    let ty = 0x0000_0420;
    let ma_a0 = 0x0000_0440;
    let ma_b0 = 0x0000_0441;
    let gf = 0x0010_D000;
    let hr1 = 0x0010_C000;
    assert_eq!(select(0x2725, ty, gf).map(|r| r.0), Ok(Image::TyGf));
    assert_eq!(
        select(0x2725, ty | 0x2, gf).map(|r| r.0),
        Ok(Image::TyGf),
        "TY is 'a' whatever the step"
    );
    assert_eq!(select(0x2725, ty, gf | 0x1000_0000), Err(Refusal::CdbNotBundled));
    assert_eq!(select(0x2725, ty, hr1), Err(Refusal::RfNotBundled(0x10C)));
    assert_eq!(select(0x7E40, ma_b0, gf).map(|r| r.0), Ok(Image::MaGf));
    assert_eq!(select(0x2729, ma_b0, gf).map(|r| r.0), Ok(Image::MaGf));
    assert_eq!(
        select(0x7E40, ma_a0, gf),
        Err(Refusal::MacStepNotBundled(0)),
        "ma-a0 is not published at API 86"
    );
    assert_eq!(select(0x7E40, ma_b0 + 1, gf), Err(Refusal::MacStepNotBundled(2)), "ma-c0");
    assert_eq!(
        select(0x7E40, ma_b0, hr1),
        Err(Refusal::RfNotBundled(0x10C)),
        "ma-b0-hr-b0 is not carried"
    );
    assert_eq!(select(0x7E40, ma_b0, gf | 0x1000_0000), Err(Refusal::CdbNotBundled));
}

#[test]
fn the_discrete_ax210_and_meteor_lake_get_linux_transport_values() {
    // cfg/ax210.c iwl_ty_mac_cfg: discrete, crystal latency 500, no LTR delay.
    let ty = transport(0x2725).unwrap();
    assert_eq!(
        (ty.integrated, ty.ltr_delay, ty.xtal_latency, ty.low_latency_xtal, ty.imr_enabled),
        (false, 0, 500, false, false)
    );
    // iwl_ma_mac_cfg: integrated, nothing else set.
    for id in [0x7E40, 0x2729] {
        let ma = transport(id).unwrap();
        assert_eq!(
            (ma.integrated, ma.ltr_delay, ma.xtal_latency, ma.low_latency_xtal, ma.imr_enabled),
            (true, 0, 0, false, false)
        );
    }
}
