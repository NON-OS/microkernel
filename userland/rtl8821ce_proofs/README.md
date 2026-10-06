# rtl8821ce_proofs

Host proofs for the RTL8821CE Wi-Fi driver (`capsule_driver_rtl8821ce`). The
crate includes the shipping driver source with `#[path]` and drives it against
a modelled register file, and links `nonos_wifi_core` as the capsule does. Its
access point simulator is `nonos_wifi_core_proofs/src/ap_sim.rs`, included the
same way.

## What it proves

152 `#[test]` functions, among them:

- the power sequence, efuse read, firmware header, sections, staging and
  download over DDMA and H2C (`pwr_tests`, `efuse_tests`, `fw_tests`,
  `sections_tests`, `staging_tests`, `ddma_tests`, `download_tests`,
  `h2c_tests`, `prep_tests`);
- the MAC, PHY and RF init tables and the receive path (`mac_tests`,
  `mac_trx_tests`, `phy_tests`, `phy_rxpath_tests`, `tables_tests`);
- the receive ring and descriptor checks (`rx_tests`);
- the passive scan, and that the beacon hunt sends a probe only for a network
  marked hidden, naming only it (`scan_tests`, `connect_tests`);
- association, the link and the link port net_core drives (`assoc_tests`,
  `link_tests`, `linkport_tests`);
- the hardware security engine key install (`sec_tests`);
- the answer to a frame neither request family takes (`serve_refuse_tests`).

The crate includes `firmware/rtw8821c_fw.bin` with `include_bytes!`; without
that file it does not compile.

## What it does not prove

That a real RTL8821CE accepts this sequence, joins a network or carries data.
No hardware log for this driver is committed.

## Run

```sh
cd userland/rtl8821ce_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
