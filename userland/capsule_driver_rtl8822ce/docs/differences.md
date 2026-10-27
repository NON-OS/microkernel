# What differs from the RTL8821CE

Read from Linux v6.12 rtw88, `rtw8821c.c` against `rtw8822c.c` (and the
tables, `reg.h`, `mac.c`, `phy.c`, `coex.c`, `pci.c`, `efuse.c`). Each row
names where this driver does it; everything not listed is the 8821CE's
code, mounted unchanged.

| Area | RTL8821CE (8821C) | RTL8822CE (8822C) | Here |
|---|---|---|---|
| PCI ids | 10ec:b821, c821 | 10ec:c822, c82f | `src/constants/pci.rs` |
| Power on / off | 8821C tables | `card_enable_flow_8822c`, `card_disable_flow_8822c` (other registers, a poll on 0x0006 bit 1, 0x1018, 0x1064) | `src/pwr/` |
| Efuse | 512 physical, 96 protected, 512 logical | 512 physical, 124 protected (388 packed), 768 logical; LDO25 at 0x29 bit 0 | `src/efuse/layout.rs`, `deshuffle.rs` |
| Efuse facts | 8821C map | RFE at 0xCA (whole byte), crystal at 0xB9, TX power at 0x10 every 42 bytes, PPG trims in the protected area | `src/efuse/facts.rs` |
| Cut and paths | from REG_SYS_CFG1 | cut bits 15:12, RF_TYPE_ID (bit 27) gives two paths | `src/efuse/info.rs` |
| Firmware | `rtw8821c_fw.bin`, 0x8821 | `rtw8822c_fw.bin` v9.9, signature 0x8822, H2C format 15, DMEM/IMEM/EMEM | `src/fw/header.rs`, `src/fwload/image.rs` |
| FIFO | 64 KiB TX | 256 KiB TX (2048 pages), 24 KiB RX, 16 driver pages, 50 CSI pages, 64 a queue: reserved boundary 1938, public queue 1681, H2C queue at 1986, RX boundary 24319 | `src/mac/trx/pages.rs` |
| MAC init | 8821C table | 8822C's: 81 operations, read-modify-writes as clear-then-set pairs | `src/mac/tables/` |
| PCIe | ASPM and clock request | also CLKDLY_CTRL (0x725) to 0 through DBI; RFE 5 sets BIT_CF_L_V2 at 0x1010; cut D and later set HCI_MIX_CFG bit 26 | `src/pcie/cfg.rs` |
| RF access | one path, through SIPI | two paths: direct windows 0x3C00 and 0x4C00, register 0 through SIPI 0x1808 / 0x4108 | `src/phy/rf.rs` |
| PHY tables | 8821C BB/AGC/RF | 8822C BB, AGC, `rtw8822c_cal_init` (with its 0x1E24 / 0x1CD0 prelude), RF B then RF A | `src/phy/apply/load.rs`, `src/phy/tables/` |
| PHY set-up | 1T1R | BB/RF power and DPD off, header init before and after the tables, crystal at 0x1040 (mask 0xFFFC00), RX on A and B, TX 1SS on A | `src/phy/setup.rs`, `power.rs`, `trx.rs` |
| Calibration | none at bring-up | DAC calibration (ADC 10 rounds, DAC 10 rounds a path, 10000 reads / 100 search rounds a measurement), every register put back after | `src/phy/dack/` |
| Factory trims | none | crystal doubler check, thermal, TX gain table, PA bias | `src/phy/trim/` |
| Channel | 8821C BB/RF | BB (2.4 GHz, 20 MHz, SCOTRK, channel 13 and 14 filters), MAC, RF 0x18 on both paths inside the 3-wire reset, IGI toggle | `src/phy/channel/` |
| TX power | 0.5 dB a step | 0.25 dB a step (txgi factor 2): references 0x18A0 / 0x41A0 (CCK) and 0x18E8 / 0x41E8 (OFDM), differences from 0x3A00; capped at the world domain's 20 dBm EIRP less 2 dBi | `src/phy/txpower/` |
| PHY status | page layout of 8821C | page 0 CCK with the AGC's gain bounds, page 1 OFDM | `src/rx/signal.rs` |
| Coexistence | 8821C wifi-only | LTE coex off, grants to Wi-Fi, scoreboard 0x8003, RF_MODOPT on path B, no antenna switch | `src/coex/` |

## Not ported, with the reason

| rtw88 step | Why not yet |
|---|---|
| IQK, DPK, TXGAPK | IQK and DPK run through H2C packets and the reserved-page queue, which the shared code has no sender for; the 8821CE runs without them too. |
| General / PHY-DM info H2C | Same H2C packet path. |
| DIG, power tracking | Need a hardware run to tune against. |
| Power-by-rate and limit tables | The cap above keeps the radio inside the world domain; per-rate tables refine it. |
| 5 GHz | The shared scanner and the 8821CE path are 2.4 GHz only. |
| Beamforming | Not needed to join. |
