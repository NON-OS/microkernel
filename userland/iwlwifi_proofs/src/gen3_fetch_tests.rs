// NONOS Operating System (AGPL-3.0-or-later)
//! the plan: Bus Master goes on after the device is stopped and before the
//! first write that makes it fetch from memory. In the gen3 boot that write
//! is the context-info doorbell (`kick`: AUTO_FUNC_BOOT_ENA in
//! CSR_CTXT_INFO_BOOT_CTRL); the ring bases travel inside the context info,
//! in host memory. Bring-up and every restart call `master_on`, then `boot`.

use nonos_libc::{given_back, note, EPOCH};

use crate::command::command;
use crate::gen3::bringup::boot;
use crate::gen3::csr::{CSR_AUTO_FUNC_BOOT_ENA, CSR_CTXT_INFO_BOOT_CTRL};
use crate::gen3::dev::Dev;
use crate::gen3::dram_map::classify;
use crate::gen3::layout::{firmware_region_sizes, Board, Memory};
use crate::gen3::outcome::master_on;
use crate::gen3::plan::{control_len, RB_REGION};
use crate::gen3::ucode::Ucode;
use crate::gen3_model::{Mem, Model, Polls};
use crate::regs::Mmio;

static SO_GF: &[u8] =
    include_bytes!("../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");

/// The model, noting the moment the device would start fetching.
struct Fetches<'a>(&'a Model);

impl Mmio for Fetches<'_> {
    fn read32(&self, off: usize) -> u32 {
        self.0.read32(off)
    }
    fn write32(&self, off: usize, val: u32) {
        if off == CSR_CTXT_INFO_BOOT_CTRL && val & CSR_AUTO_FUNC_BOOT_ENA != 0 {
            note("fetch", 0);
        }
        self.0.write32(off, val);
    }
}

#[test]
fn bus_master_is_on_before_the_doorbell_that_makes_the_device_fetch() {
    let ucode = Ucode::parse(SO_GF).unwrap();
    let layout = classify(SO_GF);
    let ctrl = Mem::new(control_len(ucode.iml.len()).unwrap(), 0x1000_0000);
    let model = Model::new(ctrl, Mem::new(RB_REGION, 0x2000_0000));
    let sizes = firmware_region_sizes(&layout).unwrap();
    let fw: Vec<_> = sizes.iter().map(|&n| Mem::new(n, 0x4000_0000)).collect();
    let fw: Vec<&Mem> = fw.iter().map(|m| m.as_ref()).collect();
    let mem = Memory { ctrl: model.ctrl.as_ref(), rbs: model.rbs.as_ref(), fw: &fw };
    let board = Board { hw_rev: 0x370, imr_enabled: false, rf_id: 0x10D000, pnvm: None };
    let mut c = Polls { per_wait: 64, delays_us: 0 };
    let watch = Fetches(&model);
    let before = given_back().len();
    master_on(&watch, &mut c, || command(5, EPOCH, true)).expect("stopped, then Bus Master");
    let mut dev = Dev::new(&watch, model.ctrl.as_ref(), model.rbs.as_ref());
    boot(&mut dev, &mut c, &mem, &layout, &ucode, &board).expect("alive");
    let log: Vec<&str> = given_back()[before..].iter().map(|e| e.0).collect();
    let at = |what| log.iter().position(|&e| e == what).expect(what);
    assert!(at("pci_command") < at("fetch"), "Bus Master before the first fetch: {log:?}");
    assert_eq!(log.iter().filter(|&&e| e == "fetch").count(), 1, "one doorbell");
}
