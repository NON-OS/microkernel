// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The real bring-up, run against the model host and card: the order of
//! commands, the voltage, clock, width and timing it ends in, and each
//! fallback it takes when the hardware says no.

use crate::emmc::error::EmmcError;
use crate::model::{CardCfg, HostCfg, Rig, St, GLK_CAPS};

fn dedup13(idx: &[u8]) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();
    for &i in idx {
        if i == 13 && v.last() == Some(&13) {
            continue;
        }
        v.push(i);
    }
    v
}

#[test]
fn a_gemini_lake_host_brings_its_emmc_to_eight_bit_high_speed() {
    let rig = Rig::new(HostCfg::default(), CardCfg::default());
    let disk = rig.disk(true).expect("bring-up");
    rig.assert_clean();
    assert_eq!(disk.capacity_sectors(), 61_071_360);
    assert_eq!((disk.card.width, disk.card.hs, disk.card.hz), (8, true, 50_000_000));
    assert!(disk.card.sector_mode && disk.card.cmd23);
    {
        let sim = rig.sim.borrow();
        assert_eq!(sim.card.state, St::Tran);
        assert_eq!((sim.card.width, sim.card.hs), (8, true));
        assert_eq!((sim.host_width(), sim.sd_hz()), (8, 50_000_000));
        // A 1.8 V-only host: powered once, at 1.8 V.
        assert_eq!(sim.power_ons, vec![5]);
        assert_eq!(sim.card.cmd1_args, vec![0, 0x4000_0080, 0x4000_0080, 0x4000_0080]);
        // Identification ran at 400 kHz.
        assert!(sim.cmds.iter().filter(|c| c.index <= 3).all(|c| c.hz == 400_000));
    }
    assert_eq!(
        dedup13(&rig.indices()),
        vec![0, 1, 0, 1, 1, 1, 2, 3, 9, 7, 8, 6, 13, 6, 13, 8, 13, 17],
        "log: {:#?}",
        rig.logs()
    );
    assert!(rig.logged("card DF4032 mid 0x45"));
    assert!(rig.logged("ready: 61071360 sectors (29820 MiB), 8-bit, high speed 50000 kHz"));
    assert!(rig.logged("base clock 200 MHz"));
}

#[test]
fn a_dual_voltage_host_powers_at_3v3_then_settles_on_1v8() {
    let host = HostCfg { caps: GLK_CAPS | 1 << 24, ..HostCfg::default() };
    let rig = Rig::new(host, CardCfg::default());
    rig.disk(true).expect("bring-up");
    rig.assert_clean();
    let sim = rig.sim.borrow();
    assert_eq!(sim.power_ons, vec![7, 5]);
    assert_eq!(sim.card.cmd1_args[0], 0);
    assert!(sim.card.cmd1_args[1..].iter().all(|&a| a == 0x4000_0080));
}

#[test]
fn a_3v3_only_host_stays_at_3v3_with_the_matching_window() {
    let host = HostCfg { caps: (GLK_CAPS & !(1 << 26)) | 1 << 24, ..HostCfg::default() };
    let rig = Rig::new(host, CardCfg::default());
    rig.disk(true).expect("bring-up");
    rig.assert_clean();
    let sim = rig.sim.borrow();
    assert_eq!(sim.power_ons, vec![7]);
    assert!(sim.card.cmd1_args[1..].iter().all(|&a| a == 0x4030_0000));
}

#[test]
fn a_3v0_and_3v3_host_settles_on_3v0() {
    let host = HostCfg { caps: (GLK_CAPS & !(1 << 26)) | 1 << 24 | 1 << 25, ..HostCfg::default() };
    let rig = Rig::new(host, CardCfg::default());
    rig.disk(true).expect("bring-up");
    rig.assert_clean();
    let sim = rig.sim.borrow();
    assert_eq!(sim.power_ons, vec![7, 6]);
    assert!(sim.card.cmd1_args[1..].iter().all(|&a| a == 0x4006_0000));
}

#[test]
fn a_generic_host_without_8_bit_runs_four_lines() {
    let rig = Rig::new(HostCfg::default(), CardCfg::default());
    let disk = rig.disk(false).expect("bring-up");
    rig.assert_clean();
    assert_eq!(disk.card.width, 4);
    assert_eq!(rig.sim.borrow().card.width, 4);
}

#[test]
fn a_generic_host_that_offers_8_bit_runs_eight_lines() {
    let host = HostCfg { caps: GLK_CAPS | 1 << 18, ..HostCfg::default() };
    let rig = Rig::new(host, CardCfg::default());
    let disk = rig.disk(false).expect("bring-up");
    rig.assert_clean();
    assert_eq!(disk.card.width, 8);
}

#[test]
fn eight_lines_that_corrupt_data_fall_back_to_four() {
    let rig = Rig::new(HostCfg::default(), CardCfg { broken8: true, ..CardCfg::default() });
    let disk = rig.disk(true).expect("bring-up");
    rig.assert_clean();
    assert_eq!(disk.card.width, 4);
    assert_eq!(rig.sim.borrow().card.width, 4);
    assert!(rig.logged("CMD6 BUS_WIDTH 8: EXT_CSD read back differs"));
    assert!(rig.logged("bus width 4 bits"));
}

#[test]
fn a_card_that_refuses_high_speed_runs_legacy_at_20_mhz() {
    let rig = Rig::new(HostCfg::default(), CardCfg { refuse_hs: true, ..CardCfg::default() });
    let disk = rig.disk(true).expect("bring-up");
    rig.assert_clean();
    assert!(!disk.card.hs);
    assert_eq!(disk.card.hz, 20_000_000);
    let sim = rig.sim.borrow();
    assert_eq!(sim.sd_hz(), 20_000_000);
    assert!(!sim.card.hs);
    assert!(!rig.cmds().last().unwrap().hs);
    drop(sim);
    assert!(rig.logged("HS_TIMING status failed: emmc: card refused a mode switch"));
    assert!(rig.logged("staying at legacy timing"));
}

#[test]
fn a_card_with_only_26_mhz_high_speed_runs_at_25_mhz() {
    let rig = Rig::new(HostCfg::default(), CardCfg { device_type: 0x01, ..CardCfg::default() });
    let disk = rig.disk(true).expect("bring-up");
    rig.assert_clean();
    assert_eq!((disk.card.hs, disk.card.hz), (true, 25_000_000));
}

#[test]
fn a_card_with_no_high_speed_runs_legacy() {
    let rig = Rig::new(HostCfg::default(), CardCfg { device_type: 0, ..CardCfg::default() });
    let disk = rig.disk(true).expect("bring-up");
    rig.assert_clean();
    assert_eq!((disk.card.hs, disk.card.hz), (false, 20_000_000));
    assert!(rig.cmds().iter().all(|c| c.index != 6 || (c.arg >> 16) & 0xff != 185));
}

#[test]
fn a_boot_partition_left_selected_is_switched_to_the_user_area() {
    let rig =
        Rig::new(HostCfg::default(), CardCfg { partition_config: 0x49, ..CardCfg::default() });
    let disk = rig.disk(true).expect("bring-up");
    rig.assert_clean();
    assert_eq!(rig.sim.borrow().card.ext[179], 0x48);
    assert_eq!(disk.card.ext.unwrap().partition_access(), 0);
    let six: Vec<u32> = rig.cmds().iter().filter(|c| c.index == 6).map(|c| c.arg).collect();
    assert_eq!(six[0], 0x03b3_4800);
}

#[test]
fn a_byte_mode_card_without_ext_csd_comes_up_on_one_line() {
    let card = CardCfg { ocr: 0x00ff_8080, spec_vers: 3, c_size: 0x7ff, ..CardCfg::default() };
    let rig = Rig::new(HostCfg::default(), card);
    let disk = rig.disk(true).expect("bring-up");
    rig.assert_clean();
    assert!(!disk.card.sector_mode);
    assert_eq!(disk.capacity_sectors(), 1_048_576);
    assert_eq!((disk.card.width, disk.card.hs, disk.card.hz), (1, false, 20_000_000));
    let idx = rig.indices();
    assert!(idx.contains(&16) && !idx.contains(&8) && !idx.contains(&6));
    assert_eq!(rig.cmds().iter().find(|c| c.index == 16).unwrap().arg, 512);
    // The byte-mode probe read is at byte address 0.
    assert_eq!(rig.cmds().last().unwrap().index, 17);
}

#[test]
fn an_empty_slot_is_no_card_and_the_host_is_left_off() {
    let rig = Rig::new(HostCfg::default(), CardCfg { absent: true, ..CardCfg::default() });
    let e = rig.disk(true).err().expect("no card");
    assert_eq!(e, EmmcError::NoCard);
    rig.assert_clean();
    let mut sim = rig.sim.borrow_mut();
    assert_eq!(sim.read(0x29, 1), 0, "bus power left on");
    assert_eq!(sim.read(0x2c, 2), 0, "clock left on");
    drop(sim);
    assert!(rig.logged("CMD1 query failed: emmc: no card answered CMD1"));
}

#[test]
fn a_card_that_never_finishes_power_up_is_card_busy() {
    let rig = Rig::new(HostCfg::default(), CardCfg { ready_after: u32::MAX, ..CardCfg::default() });
    assert_eq!(rig.disk(true).err(), Some(EmmcError::CardBusy));
    // A second's polling, at 10 ms apart, then the error.
    assert!(rig.ms() < 3_000, "took {} ms", rig.ms());
}

#[test]
fn bus_power_that_sticks_on_the_second_write_is_accepted() {
    let rig = Rig::new(HostCfg { power_ignores: 2, ..HostCfg::default() }, CardCfg::default());
    rig.disk(true).expect("bring-up");
    rig.assert_clean();
    assert_eq!(rig.sim.borrow().power_ons, vec![5]);
}

#[test]
fn bus_power_that_never_sticks_fails_bring_up() {
    let rig = Rig::new(HostCfg { power_ignores: 10, ..HostCfg::default() }, CardCfg::default());
    assert_eq!(rig.disk(true).err(), Some(EmmcError::NoPower));
    assert!(rig.logged("bus power did not stay on"));
}

#[test]
fn a_reset_that_never_finishes_fails_bring_up() {
    let rig = Rig::new(HostCfg { reset_reads: 100_000, ..HostCfg::default() }, CardCfg::default());
    assert_eq!(rig.disk(true).err(), Some(EmmcError::HostReset));
}

#[test]
fn a_host_without_adma2_is_refused() {
    let rig =
        Rig::new(HostCfg { caps: GLK_CAPS & !(1 << 19), ..HostCfg::default() }, CardCfg::default());
    assert_eq!(rig.disk(true).err(), Some(EmmcError::NoAdma));
    assert!(rig.indices().is_empty());
}

#[test]
fn a_slot_whose_card_detect_reads_empty_is_forced_present() {
    let rig = Rig::new(HostCfg { cd_inserted: false, ..HostCfg::default() }, CardCfg::default());
    rig.disk(true).expect("bring-up");
    rig.assert_clean();
    assert!(rig.logged("forced present"));
}

#[test]
fn dma_regions_above_4_gib_use_64_bit_descriptors() {
    let rig = Rig::at(HostCfg::default(), CardCfg::default(), 0x1_0000_0000, 0x1_2000_0000);
    rig.disk(true).expect("bring-up");
    rig.assert_clean();
    let sim = rig.sim.borrow();
    let last = sim.tables.last().unwrap();
    assert_eq!(last.len(), 1);
    assert_eq!(last[0].addr, 0x1_2000_0000);
}

#[test]
fn dma_regions_above_4_gib_on_a_32_bit_host_are_refused() {
    let host = HostCfg { caps: GLK_CAPS & !(1 << 28), ..HostCfg::default() };
    let rig = Rig::at(host, CardCfg::default(), 0x1_0000_0000, 0x1_2000_0000);
    assert_eq!(rig.disk(true).err(), Some(EmmcError::DmaAddress));
}

#[test]
fn a_spec_2_host_divides_by_powers_of_two() {
    let host = HostCfg {
        caps: (GLK_CAPS & !(0xff << 8)) | 48 << 8,
        version: 0x0001,
        ..HostCfg::default()
    };
    let rig = Rig::new(host, CardCfg::default());
    let disk = rig.disk(true).expect("bring-up");
    rig.assert_clean();
    // 48 MHz base: 48 / 128 = 375 kHz for identification, 48 MHz for HS.
    assert!(rig.cmds().iter().filter(|c| c.index <= 3).all(|c| c.hz == 375_000));
    assert_eq!(disk.card.hz, 48_000_000);
}
