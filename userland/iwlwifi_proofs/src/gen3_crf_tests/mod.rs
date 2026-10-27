// NONOS Operating System (AGPL-3.0-or-later)

//! What the card is, read as Linux v6.12's probe reads it (`get_crf_id`,
//! `map_crf_id`): the peripheral registers given to the MAC through the
//! family's UMAC window, the CRF id from SD_REG_VER (22000) or
//! SD_REG_VER_GEN2 (AX210 on), CSR_HW_RF_ID kept when it names an RF and
//! built from the CRF id on a blank OTP, a Bz MAC's step ORed in.

mod card;

use crate::gen3::csr::CsrLayout;
use crate::gen3::select::crf::AUX_IF_MAC_OWNER;
use crate::gen3::select::probe::probe;
use crate::gen3_model::Polls;
use crate::regs::Mmio;
use card::{card, run};

#[test]
fn a_programmed_rf_id_is_kept_and_a_blank_one_is_built_from_the_crf_id() {
    let so = CsrLayout::V1;
    assert_eq!(run(&card(so, 0x10D000, 0x0100_0910, 0, 0), so, 0x370), Some((0x370, 0x10D000)));
    let blank = |crf, cnv, wfpm| run(&card(so, 0, crf, cnv, wfpm), so, 0x370).unwrap().1;
    assert_eq!(blank(0x0100_0410, 0, 0), 0x10D100, "GF, step B");
    assert_eq!(blank(0x0000_0410, 0, 1 << 4), 0x1010_D000, "CDB from the WFPM OTP word");
    assert_eq!(blank(0x0008_0410, 0, 0), 0x1010_D000, "CDB from the CRF slave bit");
    assert_eq!(blank(0x0000_0910, 1 << 19, 0), 0x2011_2000, "FM, jacket from the CNVi");
    assert_eq!(blank(0x0000_0503, 0, 1 << 5), 0x2010_A000, "HR CDB is HR2; WFPM jacket");
    assert_eq!((blank(0x501, 0, 0), blank(0x201, 0, 0)), (0x10C000, 0x108000), "HR1, JF1");
    assert_eq!(blank(0x0000_0532, 0, 0), 0, "a CRF type Linux does not map stays blank");
}

#[test]
fn each_family_reads_its_own_crf_register_and_gives_the_mac_its_registers() {
    for (csr, owner) in [(CsrLayout::Gen2, 0x0_3030), (CsrLayout::V1, 0xD0_3030)] {
        let c = card(csr, 0, 0x0200_0504, 0, 0);
        assert_eq!(run(&c, csr, 0x330).map(|r| r.1), Some(0x10A200), "{csr:?}: HR2 step C");
        assert_eq!(c.1.borrow()[&owner] & AUX_IF_MAC_OWNER, AUX_IF_MAC_OWNER, "{csr:?}");
    }
    let bz = |hw_rev| run(&card(CsrLayout::V2, 0x112200, 0, 0x0100_0900, 0), CsrLayout::V2, hw_rev);
    assert_eq!(bz(0x460), Some((0x461, 0x112200)), "a Bz MAC takes the CNVi's step");
    assert_eq!(bz(0x472), Some((0x472, 0x112200)), "Gl keeps its own");
    struct Asleep;
    impl Mmio for Asleep {
        fn read32(&self, _: usize) -> u32 {
            0
        }
        fn write32(&self, _: usize, _: u32) {}
    }
    assert_eq!(probe(&Asleep, &mut Polls { per_wait: 4, delays_us: 0 }, CsrLayout::V1, 0), None);
}
