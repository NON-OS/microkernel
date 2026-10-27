# rtl8852_proofs

Host proofs for the RTL8852BE Wi-Fi driver (`capsule_driver_rtl8852be`).
The crate includes the shipping driver source with `#[path]` (all of it
but `main.rs`) and runs it, through the driver's own bring-up
(`bring::up`), against a modelled chip (`src/model`). It links
`nonos_wifi_core` as the capsule does, and its access points are the Wi-Fi
core proofs' simulated air (`nonos_wifi_core_proofs/src/soft`, mounted by
path) for WPA2 and `src/sae_air` for WPA3.

## The modelled chip

- Registers: a word file, every access checked for alignment and for
  the BAR the driver demands; the status bits rtw89 polls on are raised
  the way the hardware raises them (`model/on_read.rs`, `on_write.rs`).
- The XTAL SI bus, the main and DAV efuse (a packed physical map with a
  programmed MAC address the driver must never take), the PCIe config
  space through DBI, the LTE coexistence registers.
- RF registers behind SWSI; the channel the air hears is the one RF path
  A was programmed to.
- The WiFi CPU: started only for a download, the header then every
  section byte counted against the header's own lengths, chunks of at
  most 2020 bytes; then H2C commands logged with their sequence and acks,
  C2H acks queued back as non-frame packets; the register mailbox
  (GET_FEATURE answers PHY_CAP, SCH_TX_EN applies to `R_AX_CTN_TXEN`).
- The PCI DMA engine over host memory: TX channels run only when HCI TX is
  on, PCIe I/O and the channel are not stopped (and WP DMA for the WD-page
  queues); each WD page is checked (channel, queue, sizes, address info)
  and its frame goes on the air and is reported released on the RPQ; RX
  buffers are written at the card's pointer, never overrunning the host's.
- Faults, one at a time (`model/faults.rs`): stuck bits, a firmware
  status, a mute mailbox, stalled queues, no release reports, a dead
  efuse, a card off the bus.

Anything the driver does that the chip would not take is a violation;
every proof expects none unless it is the point.

## What it proves

`cargo test --release`: 55 tests. Bring-up to Ready with no violation, and
every stage on its fault (`stage_tests`, `give_back_tests`); the firmware
download byte for byte, five tries, two per boot (`fw_tests`); the efuse
fields and the PHY capabilities (`efuse_tests`); H2C numbering, acks and
rtw89's command order for the interface and for association
(`h2c_tests`); every 2.4 and 5 GHz channel on the RF and the scheduler
given back (`tune_tests`); scan (`scan_tests`); WPA2 association on
channels 6 and 36, disconnect and rejoin, wrong passphrase
(`assoc_tests`); WPA3-SAE association and traffic (`wpa3_tests`); data
both ways under CCMP (`data_tests`); ring reuse, full rings and hostile
receive buffers (`ring_tests`, `rx_tests`); size bounds at their edges
(`limit_tests`); every wait's timeout (`bound_tests`); and that no
console line carries an address or network name, and the efuse MAC
reaches nothing (`privacy_tests`); and that each stage reads as the
Wi-Fi client's (`client_stage_tests`).

The host mutants these kill, one per rule or bound, are
`verification/host_mutants/rtl8852be.json` (`tools/nonos-host-mutants`).
