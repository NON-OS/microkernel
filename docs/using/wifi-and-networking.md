# Wi-Fi and networking

Put a NONOS machine on a network: join Wi-Fi in Settings, plug in a cable, and read what the status lines and join errors mean.

## Before you start

You need a network card NONOS has a driver for, an image that carries that driver, and a boot that runs the network.

- Wi-Fi: the Realtek RTL8821CE (PCI `10ec:c821`, `PCI_VENDOR_REALTEK` and `PCI_DEVICE_RTL8821CE` in `userland/capsule_driver_rtl8821ce/src/constants/mod.rs:25-26`) and Intel cards through the iwlwifi driver. These two are the only Wi-Fi drivers the panels know (`SERVICES` in `userland/nonos_wifi_client/src/driver/services.rs:32-35`). The Intel driver joins only on the parts whose firmware it boots; see [the Intel Wi-Fi page](../drivers/wifi/iwlwifi.md).
- Wired: `virtio_net` for QEMU, and the `e1000`, `rtl8139` and `rtl8169` driver capsules. Which cards each one takes, by PCI id, is on the [Ethernet](../drivers/ethernet/README.md) pages.
- The standard and hardened images carry all of these drivers: their kernel is built with the `microkernel-full-gui` feature list in `Cargo.toml` (`standard` and `hardened` in `tools/nix/config.nix:70-82`). The `qemu` image carries only `virtio_net`, so it has no Wi-Fi. An image built with the `airgapped` profile carries no network driver, stack or online program at all (`airgapped` in `tools/nix/config.nix:86-92`).
- The Air-Gapped, Safe Mode and Recovery boot modes start no network driver and no network service (`network` in `src/boot/handoff/api/profile.rs:44-46`). See [boot modes](../install/boot-modes.md).

Realtek RTL8821CE Wi-Fi, for scanning, joining, DHCP, DNS and browser traffic: Works on an x86_64 laptop (Intel Gemini Lake, 8 GB), maintainer hardware report, 6 October 2026; the image commit was not recorded.

Everything else on this page is stated from the code. The host tests of the Wi-Fi path pass on this commit: `rtl8821ce_proofs` (171 tests), `nonos_wifi_core_proofs` (70), `wifi_panel_proofs` (23), `iwlwifi_proofs` (205) and `net_core_proofs` (31).
