// NONOS Operating System (AGPL-3.0-or-later)

//! BZ-family selection: the PCI ids, their transport values (cfg/bz.c), the
//! MAC and RF types and steps that name gl-c0-fm-c0 and bz-b0-fm-c0, and a
//! named refusal for everything else.

use super::rig::{BZ_REV, FM_C, GL_REV};
use crate::gen3::csr::CsrLayout;
use crate::gen3::select::{select, transport, Image, Refusal};

#[test]
fn the_be200_and_be201_ids_take_their_cfg_bz_transports() {
    let t = |id: u16| transport(id).unwrap();
    let v = |id: u16| {
        let x = t(id);
        (x.integrated, x.ltr_delay, x.xtal_latency, x.low_latency_xtal, x.imr_enabled, x.csr)
    };
    // iwl_gl_trans_cfg: discrete, no LTR delay; iwl_bz_trans_cfg: 2500 us.
    assert_eq!(v(0x272B), (false, 0, 12000, true, false, CsrLayout::V2));
    for id in [0xA840u16, 0x7740, 0x4D40] {
        assert_eq!(v(id), (true, 2, 12000, true, false, CsrLayout::V2), "{id:#06x}");
    }
    for id in [0x51F0u16, 0x2725, 0x7E40, 0x7AF0] {
        assert_eq!(t(id).csr, CsrLayout::V1, "{id:#06x} keeps V1");
    }
}

#[test]
fn gl_c0_and_bz_b0_with_an_fm_c0_radio_select_their_images() {
    assert_eq!(select(0x272B, GL_REV, FM_C).map(|s| s.0), Ok(Image::GlFm));
    for id in [0xA840u16, 0x7740, 0x4D40] {
        assert_eq!(select(id, BZ_REV, FM_C).map(|s| s.0), Ok(Image::BzFm), "{id:#06x}");
    }
    assert_eq!(select(0xA840, GL_REV, FM_C).map(|s| s.0), Ok(Image::GlFm), "the MAC decides");
}

#[test]
fn every_other_bz_family_part_is_refused_by_name() {
    let gl = |rev: u32, rf: u32| select(0x272B, rev, rf).map(|s| s.0);
    assert_eq!(gl(0x471, FM_C), Err(Refusal::MacStepNotBundled(1)), "gl-b0");
    assert_eq!(gl(GL_REV, 0x0011_2100), Err(Refusal::RfStepNotBundled(1)), "fm-b0");
    assert_eq!(gl(GL_REV, FM_C | 1 << 28), Err(Refusal::CdbNotBundled), "fm4");
    assert_eq!(gl(GL_REV, 0x0010_D000), Err(Refusal::RfNotBundled(0x10D)), "gf");
    assert_eq!(gl(GL_REV, 0), Err(Refusal::RfNotBundled(0)), "blank OTP");
    assert_eq!(select(0xA840, 0x460, FM_C).map(|s| s.0), Err(Refusal::MacStepNotBundled(0)));
    // A family's MAC on the other family's PCI id.
    assert_eq!(gl(0x370, 0x0010_D000), Err(Refusal::MacNotBundled(0x37)));
    assert_eq!(select(0x51F0, GL_REV, FM_C).map(|s| s.0), Err(Refusal::MacNotBundled(0x47)));
    assert_eq!(select(0xA841, GL_REV, FM_C).map(|s| s.0), Err(Refusal::NotSoDevice(0xA841)));
    assert_eq!(Refusal::RfStepNotBundled(1).code(), 0x7_0001);
    assert_eq!(Image::GlFm as u8, 4);
    assert_eq!(Image::BzFm as u8, 5);
}
