# Realtek RTL8852BE

The RTL8852BE driver [capsule](../../overview/glossary-boot-root-record.md#capsule) (`driver.rtl8852be0`) runs the Realtek RTL8852BE PCIe Wi-Fi 6 card as a 20 MHz station on 2.4 and 5 GHz, with 802.11a/b/g rates. It is a port of Linux v6.12's rtw89 for this one chip; the station above the frames (scan, open and SAE authentication, association, the WPA2 and WPA3 handshakes) is the shared `nonos_wifi_core` soft-MAC station; CCMP-128 runs in the chip, on keys the station installs as rtw89 installs them.

## What works

Nothing has been run on the card. The driver is proven on the host only, in `userland/rtl8852_proofs`: its own bring-up runs against a modelled chip (registers, efuse, the firmware CPU's download and mailbox, the PCI DMA engine, RF channel registers) to Ready, then scans, joins WPA2 and WPA3 access points on 2.4 and 5 GHz and carries traffic both ways through the rings, sealed and opened by the chip under the keys the join and each rekey installed. The model's chip drops a protected frame no installed key opens, the behaviour that would break a station keeping its keys to itself. One host mutant per rule and bound is in `verification/host_mutants/rtl8852be.json`, all killed. [The real-hardware checklist](../../hardware/real-hardware-checklist.md#realtek-rtl8852be-wi-fi) says how to give it its first run.

## Identity

The driver takes one PCI function: vendor 0x10EC, device 0xB852 or 0xB85B, the ids Linux v6.12 rtw89 binds the chip by (`userland/capsule_driver_rtl8852be/src/ids.rs`, `RTL8852BE_DEVICE_IDS`), with a memory BAR of at least 512 KiB; the kernel's inventory and the broker's scope row name the same two, and `kernel_proofs` holds both to the driver's list. The kernel starts the capsule only when the boot PCI scan found one of them (`src/userspace/init/spawn_plan/drivers_wifi/rtl8852be.rs`). Its service is `driver.rtl8852be0` on port 4268, reply port 4269. The RTL8852CE (10ec:c852) and RTL8852AE are listed by the inventory as Realtek Wi-Fi parts with no driver.

## Authority

The same mask as the RTL8821CE's, 0xB8038 (IPC, Memory, Crypto, Driver, DeviceEnum, Mmio, Dma), Debug optional; no Irq (the rings are polled), no FileSystem, no Network. The station address is drawn at random each boot; the efuse address is never read out.

## Keys

What Linux v6.12 rtw89 does, read from `cam.c`, `core.c` and `mac80211.c`:

- `rtw89_ops_set_key` takes CCMP-128 (and CCMP-256, GCMP, WEP, the AES-CMAC IGTK) into the chip by `rtw89_cam_sec_key_add`; mac80211 runs software crypto only for a cipher it refuses (`-EOPNOTSUPP`, e.g. TKIP, BIP-GMAC). A key goes into a security CAM entry (`rtw89_cam_send_sec_key_cmd`: `H2C_CL_MAC_SEC_CAM`, `H2C_FUNC_MAC_SEC_UPD`, 24 octets), then the address CAM entry is pointed at it (`rtw89_cam_attach_sec_cam`: normal mode, slots 0-1 pairwise, 2-4 group, 5-6 BIP; `rtw89_fw_h2c_cam`). Deleting a key detaches it from the address CAM; the security CAM entry is left (`rtw89_cam_sec_key_del`).
- On the 8852B (`hw_sec_hdr` and `hw_mgmt_tx_encrypt` false) rtw89 sets `IEEE80211_KEY_FLAG_GENERATE_IV` and `IEEE80211_KEY_FLAG_SW_MGMT_TX`: mac80211 writes each data frame's CCMP header and packet number, the chip encrypts and adds the MIC (WD info word 2: `SEC_TYPE`, `SEC_HW_ENC`, `SEC_CAM_IDX`, `rtw89_core_tx_update_sec_key`), and management frames are sealed in software and sent as they are.
- Receive (`rtw89_core_update_rx_status`): `HW_DEC` without `SW_DEC` or `ICV_ERR` is `RX_FLAG_DECRYPTED`, the CCMP header and MIC left in the frame (no `IV_STRIPPED`); `ICV_ERR` or `CRC32_ERR` is `RX_FLAG_FAILED_FCS_CRC`; a frame with `SW_DEC` goes up undecrypted for mac80211 to open.

Here: the station installs its keys through `Radio::set_key`; each key's security CAM entry is fixed by its index (pairwise 0, a group or integrity key its index plus one, the same as its address CAM slot) where rtw89 takes the first free; a key counts as in only once the DMA engine has taken its commands. The command's body is wiped at once and its ring slot once the engine has read past the CAM update after it and 20 ms more have passed (rtw89 keeps an H2C buffer until eight more are taken). A protected frame the chip passes undecrypted is opened by the station in software, as mac80211 opens one with `SW_DEC`. If the engine never takes the join's pairwise key the station runs CCMP itself; at a rekey, the link ends and the join runs again. Leaving points the address CAM at no key.

## Stages

The bring-up steps, each with the rtw89 function it follows and the stage it reports, are in `userland/capsule_driver_rtl8852be/docs/stages.md`. Besides the shared codes, the driver reports `ChipInit` (12) for a PCIe, MAC, PHY or calibration step the chip did not finish.

## What it does not do yet

TSSI and DPK (TX power is open loop, from rtw89's world-wide tables), RSSI (frames report none), HT/VHT/HE rates, 40 and 80 MHz, 6 GHz, power save (the chip stays awake), the Bluetooth coexistence engine (the antenna is granted to Wi-Fi alone), and boards with one antenna (`NoAirPath`).

## Licence

The register tables and calibration lists come from rtw89 (Linux v6.12), taken under its BSD-3-Clause option, with Realtek's notice beside them (`userland/capsule_driver_rtl8852be/tables/LICENSE`). The firmware, `rtw8852b_fw-1.bin` from linux-firmware, is Realtek's redistributable binary, pinned by sha256 (`userland/capsule_driver_rtl8852be/docs/licence.md`).
