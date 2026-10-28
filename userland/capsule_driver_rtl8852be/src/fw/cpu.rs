// NONOS Operating System (AGPL-3.0-or-later)

//! The WiFi CPU stopped and restarted for a download:
//! `rtw89_mac_disable_cpu_ax` (with the 885xb's watchdog-off,
//! `rtw89_disable_fw_watchdog`) and `rtw89_mac_enable_cpu_ax` (Linux v6.12
//! rtw89 `mac.c`), boot reason 0, download enabled.

use crate::regs::{field, Mmio, Rw};

pub const R_AX_SYS_CLK_CTRL: u32 = 0x8;
pub const B_AX_CPU_CLK_EN: u32 = 0x4000;
pub const R_AX_PLATFORM_ENABLE: u32 = 0x88;
pub const B_AX_PLATFORM_EN: u32 = 0x1;
pub const B_AX_WCPU_EN: u32 = 0x2;
pub const B_AX_APB_WRAP_EN: u32 = 0x4;
pub const R_AX_HALT_H2C_CTRL: u32 = 0x160;
pub const R_AX_HALT_C2H_CTRL: u32 = 0x164;
pub const R_AX_HALT_H2C: u32 = 0x168;
pub const R_AX_HALT_C2H: u32 = 0x16c;
pub const R_AX_WCPU_FW_CTRL: u32 = 0x1e0;
pub const B_AX_WCPU_FWDL_EN: u32 = 0x1;
pub const B_AX_H2C_PATH_RDY: u32 = 0x2;
pub const B_AX_FWDL_PATH_RDY: u32 = 0x4;
pub const B_AX_WCPU_FWDL_STS_MASK: u32 = 0xe0;
pub const R_AX_BOOT_REASON: u32 = 0x1e6;
pub const B_AX_BOOT_REASON_MASK: u16 = 0x7;
pub const R_AX_UDM1: u32 = 0x1f4;
pub const R_AX_UDM2: u32 = 0x1f8;
pub const R_AX_SEC_CTRL: u32 = 0xc00;
pub const B_AX_SEC_IDMEM_SIZE_CONFIG_MASK: u32 = 0x3_0000;

pub fn disable(r: &impl Mmio) {
    r.clr32(R_AX_PLATFORM_ENABLE, B_AX_WCPU_EN);
    r.clr32(R_AX_WCPU_FW_CTRL, B_AX_WCPU_FWDL_EN | B_AX_H2C_PATH_RDY | B_AX_FWDL_PATH_RDY);
    r.clr32(R_AX_SYS_CLK_CTRL, B_AX_CPU_CLK_EN);
    r.clr32(R_AX_PLATFORM_ENABLE, B_AX_APB_WRAP_EN);
    r.set32(R_AX_PLATFORM_ENABLE, B_AX_APB_WRAP_EN);
    r.clr32(R_AX_PLATFORM_ENABLE, B_AX_PLATFORM_EN);
    r.set32(R_AX_PLATFORM_ENABLE, B_AX_PLATFORM_EN);
}

/// Start the CPU in download mode. False when it was still running, as
/// rtw89 refuses (-EFAULT).
pub fn enable_for_download(r: &impl Mmio) -> bool {
    if r.read32(R_AX_PLATFORM_ENABLE) & B_AX_WCPU_EN != 0 {
        return false;
    }
    for reg in [R_AX_UDM1, R_AX_UDM2, R_AX_HALT_H2C_CTRL, R_AX_HALT_C2H_CTRL] {
        r.write32(reg, 0);
    }
    r.write32(R_AX_HALT_H2C, 0);
    r.write32(R_AX_HALT_C2H, 0);
    r.set32(R_AX_SYS_CLK_CTRL, B_AX_CPU_CLK_EN);
    let clear = B_AX_WCPU_FWDL_EN | B_AX_H2C_PATH_RDY | B_AX_FWDL_PATH_RDY;
    let v = r.read32(R_AX_WCPU_FW_CTRL) & !clear & !B_AX_WCPU_FWDL_STS_MASK;
    r.write32(R_AX_WCPU_FW_CTRL, v | field(B_AX_WCPU_FWDL_STS_MASK, 0) | B_AX_WCPU_FWDL_EN);
    r.mask32(R_AX_SEC_CTRL, B_AX_SEC_IDMEM_SIZE_CONFIG_MASK, 0x2);
    r.mask16(R_AX_BOOT_REASON, B_AX_BOOT_REASON_MASK, 0);
    r.set32(R_AX_PLATFORM_ENABLE, B_AX_WCPU_EN);
    true
}
