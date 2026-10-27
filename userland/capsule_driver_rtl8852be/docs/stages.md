# The bring-up, stage by stage

Each step follows the Linux v6.12 rtw89 function named, file and line in
`drivers/net/wireless/realtek/rtw89/`. The stage is the byte the status
op carries when that step stops; a stop after the claim powers the MAC
off, drops Bus Master and gives the claim back (`setup::give_back`).
Every one of these stops is proven on the modelled chip
(`userland/rtl8852_proofs`, `stage_tests`, `give_back_tests`).

| Step | Linux v6.12 | Driver | Stage if it stops |
|---|---|---|---|
| Find, claim, map BAR 2 (at least 512 KiB) | `rtw89_pci_probe`, `rtw89_pci_setup_mapping` (pci.c) | `discover.rs`, `setup/run.rs` | 1 NotClaimed |
| A MAC a warm reboot left on, off before Bus Master | `rtw89_mac_power_switch` (mac.c:1444) | `start.rs` | (none: best effort) |
| Rings in 32-bit DMA | `rtw89_pci_alloc_trx_rings` (pci.c) | `ring/map.rs` | 5 NoDma |
| Cut, firmware suit (normal CE first) | `rtw89_chip_info_setup` (core.c:4652), `rtw89_fw_recognize` (fw.c) | `bring/probe.rs`, `fw/mfw.rs` | 4 FirmwareFailed |
| Power on | `rtw89_mac_power_switch` (mac.c:1444), `rtw8852b_pwr_on_func` (rtw8852b.c) | `pwr/on.rs` | 2 PowerFailed |
| HCI DMA, DMAC pre-init, PCIe pre-init, rings programmed | `rtw89_mac_partial_init` (mac.c:3916), `rtw89_pci_ops_mac_pre_init_ax` (pci.c:2813) | `bring/partial.rs`, `pcie/` | 10 ChipInit |
| Firmware download, five tries | `rtw89_fw_download` (fw.c:1496) | `fw/download.rs` | 4 FirmwareFailed |
| Efuse and phycap maps (the MAC address is never read) | `rtw89_chip_efuse_info_setup` (core.c:4551), `rtw89_parse_efuse_map_ax` (efuse.c:239) | `efuse/` | 6 EfuseFailed |
| PHY capabilities over the register mailbox | `rtw89_mac_setup_phycap` (mac.c:2920), `rtw89_fw_msg_reg` (fw.c:5805) | `bring/probe.rs`, `h2c/reg.rs` | 4 FirmwareFailed |
| One-antenna board refused | `hal.antenna_rx` RF_B | `bring/up.rs` | 8 NoAirPath |
| MAC init: partial init again, BB/RF on, sys, DLE and HFC, TRX, PCIe post-init | `rtw89_mac_init` (mac.c:3953), `trx_init_ax` (mac.c:3668), `rtw89_pci_ops_mac_post_init_ax` (pci.c:2971) | `bring/mac_init.rs`, `mac/`, `dle/` | 10 ChipInit |
| BB and RF tables; RF direct words to the firmware in pages | `rtw89_phy_init_bb_reg` (phy.c:1612), `rtw89_phy_init_rf_reg` (phy.c:1640) | `bring/phy_init.rs`, `phy/` | 10 ChipInit |
| Coexistence, Wi-Fi only | `rtw89_btc_ntfy_init` (coex.c:7063), chip part | `coex/` | 10 ChipInit |
| DM init: DIG, CFO, EDCCA, NCTL, RCK, DACK, RX DCK, TX power reference and trims, both RX paths | `rtw89_phy_dm_init` (phy.c:5979), `rtw8852b_rfk_init` (rtw8852b.c:554) | `bring/dm_init.rs`, `rfk/` | 10 ChipInit |
| Station address drawn at random | (mac80211 takes the efuse's; this never does) | `station.rs` | 7 NoStationAddress |
| Station interface: port, MAC ID tables, unpause, role, join info, CAM, CMAC table | `rtw89_mac_vif_init` (mac.c:4522) | `sta/vif.rs` | 10 ChipInit |
| Ready | | | 0 Ready |

After Ready, the station drives the chip through `nonos_wifi_core`'s
`Radio` (`radio/`):

| Operation | Linux v6.12 | Driver |
|---|---|---|
| Channel set, 20 MHz, scheduler paused over the mailbox | `rtw8852b_set_channel` (rtw8852b.c:485), `rtw8852b_set_channel_help` | `sta/tune.rs`, `mac/sch.rs` |
| Scan start and end: EDCCA at maximum, filter open | `rtw89_core_scan_start` (core.c:4447), `rtw89_core_scan_complete` (core.c:4467) | `sta/scan.rs` |
| Join: BSSID in the CAM, RX DCK and IQK on the channel | `rtw89_core_sta_add` (core.c:3455), `rtw8852b_rfk_channel` (rtw8852b.c:565), `_doiqk` (rtw8852b_rfk.c:1611) | `sta/join.rs`, `rfk/iqk/` |
| Associated: CMAC table, join info, CAM with AID, rate adaptation, port | `rtw89_core_sta_assoc` (core.c:3594) | `sta/join.rs` |
| Leave | `rtw89_core_sta_disconnect` (core.c:3548) | `sta/join.rs` |
| Frames out: management on CH8 at a fixed rate, data on ACH0 with firmware rate adaptation | `rtw89_core_tx_write` (core.c) | `sta/send.rs`, `tx/` |
| Release reports free the slots | `rtw89_pci_release_rpp` (pci.c:565) | `sta/pump.rs` |
| Frames in: whole WIFI packets only, FCS stripped | `rtw89_core_rx` (core.c) | `rx/` |

Code 10 (ChipInit) is this driver's: a MAC, PHY or calibration step the
chip did not finish. `nonos_wifi_client` reads it with the others.
