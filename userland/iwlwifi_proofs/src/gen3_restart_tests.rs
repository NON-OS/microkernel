// NONOS Operating System (AGPL-3.0-or-later)
//! W4: a firmware that raised its error cause boots again in the regions the
//! first bring-up mapped, each time stopped and seen stopped before it.

use crate::gen3::bringup::boot;
use crate::gen3::dev::Dev;
use crate::gen3::dram_map::classify;
use crate::gen3::layout::{firmware_region_sizes, Board, Memory};
use crate::gen3::plan::{control_len, RB_REGION};
use crate::gen3::regs::{CSR_INT, INT_BIT_SW_ERR};
use crate::gen3::start::stop_device;
use crate::gen3::ucode::Ucode;
use crate::gen3_model::{Mem, Model, Polls};

static SO_GF: &[u8] =
    include_bytes!("../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");

#[test]
fn a_failed_firmware_boots_again_in_the_same_regions() {
    let ucode = Ucode::parse(SO_GF).unwrap();
    let layout = classify(SO_GF);
    let model = Model::new(
        Mem::new(control_len(ucode.iml.len()).unwrap(), 0x1000_0000),
        Mem::new(RB_REGION, 0x2000_0000),
    );
    let sizes = firmware_region_sizes(&layout).unwrap();
    let fw: Vec<std::rc::Rc<Mem>> = sizes
        .iter()
        .enumerate()
        .map(|(i, &n)| Mem::new(n, 0x4000_0000 + (i as u64) * 0x10_0000))
        .collect();
    let fw: Vec<&Mem> = fw.iter().map(|m| m.as_ref()).collect();
    let mem = Memory { ctrl: model.ctrl.as_ref(), rbs: model.rbs.as_ref(), fw: &fw };
    let board = Board { hw_rev: 0x370, imr_enabled: false, rf_id: 0x10D000, pnvm: None };
    let mut c = Polls { per_wait: 64, delays_us: 0 };
    for boot_no in 0..3 {
        let mut dev = Dev::new(&model, model.ctrl.as_ref(), model.rbs.as_ref());
        let b = boot(&mut dev, &mut c, &mem, &layout, &ucode, &board);
        assert!(b.is_ok(), "boot {boot_no}: {:?}", b.err());
        // The firmware asserts: the cause latches until the next boot clears it.
        *model.s.borrow_mut().regs.entry(CSR_INT).or_insert(0) |= INT_BIT_SW_ERR;
        assert!(dev.error_cause(), "the firmware raised its error cause");
        assert!(stop_device(&model, &mut c), "stopped and seen stopped before the next boot");
    }
}
