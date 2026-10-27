// NONOS Operating System (AGPL-3.0-or-later)

//! The 8852B's data-link-engine memory plans: `rtw8852b_dle_mem_pcie`
//! (Linux v6.12 rtw89 `rtw8852b.c`) over `rtw89_mac_size` (`mac.c`): the
//! firmware-download plan (DLFW: wde_size9, ple_size8, wde_qt4, ple_qt13)
//! and the single-channel plan (SCC: wde_size7, ple_size6, wde_qt7,
//! ple_qt18 min, ple_qt58 max). Each plan must use exactly the FIFO the
//! mode leaves (`dle_used_size` against `dle_expected_used_size`).

/// Page size, linked pages, unlinked pages (`struct rtw89_dle_size`).
pub type Size = (u32, u32, u32);
/// hif, wcpu, pkt_in, cpu_io (`struct rtw89_wde_quota`).
pub type Wde = [u16; 4];
/// cma0_tx, cma1_tx, c2h, h2c, wcpu, mpdu_proc, cma0_dma, cma1_dma, bb_rpt,
/// wd_rel, cpu_io (`struct rtw89_ple_quota`, the 8852B's eleven).
pub type Ple = [u16; 11];

pub struct Plan {
    pub wde: Size,
    pub ple: Size,
    pub wde_min: Wde,
    pub wde_max: Wde,
    pub ple_min: Ple,
    pub ple_max: Ple,
}

pub const DLFW: Plan = Plan {
    wde: (64, 0, 1024),
    ple: (128, 64, 960),
    wde_min: [0, 0, 0, 0],
    wde_max: [0, 0, 0, 0],
    ple_min: [0, 0, 16, 48, 0, 0, 0, 0, 0, 0, 0],
    ple_max: [0, 0, 16, 48, 0, 0, 0, 0, 0, 0, 0],
};

pub const SCC: Plan = Plan {
    wde: (64, 510, 2),
    ple: (128, 496, 16),
    wde_min: [446, 48, 0, 16],
    wde_max: [446, 48, 0, 16],
    ple_min: [147, 0, 16, 20, 17, 13, 89, 0, 32, 14, 8],
    ple_max: [147, 0, 16, 20, 157, 13, 229, 0, 172, 14, 24],
};

/// `chip->fifo_size` and `chip->dle_scc_rsvd_size` for the 8852B.
pub const FIFO_SIZE: u32 = 196_608;
pub const SCC_RSVD: u32 = 98_304;

/// `dle_used_size`: the bytes a plan's pages take.
pub const fn used(p: &Plan) -> u32 {
    p.wde.0 * (p.wde.1 + p.wde.2) + p.ple.0 * (p.ple.1 + p.ple.2)
}

/// `dle_expected_used_size`: the FIFO, less the SCC reservation in SCC.
pub const fn expected(scc: bool) -> u32 {
    if scc {
        FIFO_SIZE - SCC_RSVD
    } else {
        FIFO_SIZE
    }
}

/// `DLE_BOUND_UNIT`: the PLE starts after the WDE's pages, in 8 KiB units.
pub const fn ple_bound(p: &Plan) -> u32 {
    (p.wde.1 + p.wde.2) * p.wde.0 / 0x2000
}

const _: () = assert!(used(&DLFW) == expected(false) && used(&SCC) == expected(true));
