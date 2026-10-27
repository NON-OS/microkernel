// NONOS Operating System (AGPL-3.0-or-later)

//! The 22000 family's images and transports, as cfg/22000.c and pcie/drv.c
//! name them.

use super::rig::{G2, HR2, QUZ_B, QU_B0, QU_C0};
use crate::firmware::blob::gen3_blob;
use crate::firmware::carried::runs;
use crate::gen3::select::{select, transport, Image, Refusal};

const QU_IDS: [u16; 7] = [0x02F0, 0x06F0, 0x34F0, 0x3DF0, 0x4DF0, 0x43F0, 0xA0F0];
const HR1: u32 = 0x0010_C000;

#[test]
fn a_qu_or_quz_cnvi_with_an_hr_radio_gets_its_mac_and_steps_image() {
    for id in QU_IDS {
        let img = |rev: u32, rf: u32| select(id, rev, rf).map(|r| r.0);
        assert_eq!(img(QU_B0, HR2), Ok(Image::QuB0Hr), "{id:#06x}");
        assert_eq!(img(QU_B0, HR1), Ok(Image::QuB0Hr));
        assert_eq!(img(QU_C0, HR2), Ok(Image::QuC0Hr));
        assert_eq!(img(QUZ_B, HR2), Ok(Image::QuZHr));
        assert_eq!(img(0x350, HR1), Ok(Image::QuZHr), "QuZ with HR1, any step");
        assert_eq!(img(0x350, HR2), Ok(Image::QuZHr), "QuZ A with HR2, as a quz_hr row has it");
        assert_eq!(img(0x333, HR2), Err(Refusal::MacStepNotBundled(3)), "a Qu D step");
        assert_eq!(img(QU_B0, 0x0010_5000), Err(Refusal::RfNotBundled(0x105)), "JF: a 9560");
        assert_eq!(img(QU_B0, 0), Err(Refusal::RfNotBundled(0)), "a blank OTP");
        assert_eq!(img(QU_B0, HR2 | 0x1000_0000), Err(Refusal::CdbNotBundled));
        assert_eq!(img(0x370, HR2), Err(Refusal::MacNotBundled(0x37)), "an SO MAC");
        assert!(runs(id));
    }
}

#[test]
fn the_ax200_runs_cc_a0_whatever_its_ids_say() {
    for (rev, rf) in [(0x340, HR2), (0x341, HR1), (0x370, 0), (0, 0x0010_5000)] {
        assert_eq!(select(0x2723, rev, rf).map(|r| r.0), Ok(Image::CcHr), "{rev:#x} {rf:#x}");
    }
    assert!(runs(0x2723));
    let names = [
        (Image::QuB0Hr, "iwlwifi-Qu-b0-hr-b0-77.ucode"),
        (Image::QuC0Hr, "iwlwifi-Qu-c0-hr-b0-77.ucode"),
        (Image::QuZHr, "iwlwifi-QuZ-a0-hr-b0-77.ucode"),
        (Image::CcHr, "iwlwifi-cc-a0-77.ucode"),
    ];
    for (image, name) in names {
        assert_eq!(gen3_blob(image).map(|b| b.name), Some(name));
    }
}

#[test]
fn each_platform_gets_cfg_22000s_transport_values() {
    let v = |id| {
        let t = transport(id).unwrap();
        assert_eq!((t.csr, t.imr_enabled), (G2, false), "{id:#06x}");
        (t.integrated, t.ltr_delay, t.xtal_latency, t.low_latency_xtal)
    };
    for id in [0x02F0, 0x06F0] {
        assert_eq!(v(id), (true, 1, 500, false), "iwl_qu_trans_cfg");
    }
    for id in [0x34F0, 0x3DF0, 0x4DF0] {
        assert_eq!(v(id), (true, 3, 1820, false), "iwl_qu_medium_latency_trans_cfg");
    }
    for id in [0x43F0, 0xA0F0] {
        assert_eq!(v(id), (true, 2, 12000, true), "iwl_qu_long_latency_trans_cfg");
    }
    assert_eq!(v(0x2723), (false, 0, 0, false), "iwl_ax200_trans_cfg");
}
