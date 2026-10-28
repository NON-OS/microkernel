// NONOS Operating System (AGPL-3.0-or-later)
//! The LPSS resets: the function is released, the integrated DMA kept in
//! reset, as driver.i2c_pci0 keeps it (its LPSS_PRIV_RESETS_FUNC).

use crate::facts::{PRIV_RESETS, PRIV_RESETS_DEASSERT};

const I2C: &str = include_str!("../../capsule_driver_i2c_pci/src/constants/lpss.rs");
const PROVE: &str = include_str!("../../capsule_driver_spi/src/setup/prove.rs");
/// The integrated DMA's reset, bit 2 (Linux intel-lpss.c LPSS_PRIV_RESETS_IDMA).
const RESETS_IDMA: u32 = 1 << 2;

#[test]
fn the_dma_engine_stays_in_reset() {
    assert_eq!(PRIV_RESETS_DEASSERT, 0x3, "function and APB released");
    assert_eq!(PRIV_RESETS_DEASSERT & RESETS_IDMA, 0, "the DMA released");
    assert!(I2C.contains("pub const LPSS_PRIV_RESETS_FUNC: u32 = 0x3;"), "as i2c_pci");
    let writes = PROVE.matches("write32(PRIV_RESETS,").count();
    assert_eq!(writes, 2, "the resets written only to assert, then release");
    assert_eq!(PRIV_RESETS, 0x204);
}
