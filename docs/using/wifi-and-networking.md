# Wi-Fi and networking

Put a NONOS machine on a network: join Wi-Fi in Settings, plug in a cable, and read what the status lines and join errors mean.

## Before you start

You need a network card NONOS has a driver for, an image that carries that driver, and a boot that runs the network.

- Wi-Fi: the Realtek RTL8821CE (PCI `10ec:c821`) and Intel cards through the iwlwifi driver. These two are the only Wi-Fi drivers the panels know. The Intel driver joins only on the parts whose firmware it boots; see [the Intel Wi-Fi page](../drivers/wifi/iwlwifi.md).
- Wired: `virtio_net` for QEMU, and the `e1000`, `rtl8139` and `rtl8169` driver capsules. Read from the code, only `virtio_net` gets a DHCP lease in this release: `net.core` drops every frame the other three receive (see [Ethernet](wifi-and-networking/wired.md#ethernet)). Which cards each one takes, by PCI id, is on the [Ethernet](../drivers/ethernet/README.md) pages.
- The standard and hardened images carry all of these drivers. The `qemu` image carries only `virtio_net`, so it has no Wi-Fi. An image built with the `airgapped` profile carries no network driver, stack or online program at all.
- The Air-Gapped, Safe Mode and Recovery boot modes start no network driver and no network service. See [boot modes](../install/boot-modes.md).

Realtek RTL8821CE Wi-Fi, for scanning, joining, DHCP, DNS and browser traffic: Works on an x86_64 laptop (Intel Gemini Lake, 8 GB), developer hardware report, 6 October 2026; the image commit was not recorded.

Everything else on this page is stated from the code, and the host tests of the Wi-Fi path pass on this commit.

## Which networks can be joined

The drivers share one join engine. It runs WPA2-Personal (PSK and PSK-SHA256) and WPA3-Personal (SAE), with CCMP-128 only. It refuses:

- open networks, with no RSN element;
- TKIP, 802.1X Enterprise, FT and OWE;
- access points that admit only 802.11n stations.

A network that offers WPA3 and WPA2 side by side is joined with WPA3 first. If that does not finish, for any reason but a wrong password, the RTL8821CE driver joins it again with WPA2. A network saved after a WPA3 join is saved as WPA3 and is never joined with WPA2 again.

A passphrase is 8 to 63 characters, or 64 hex digits.

Two notes in the interface are older than this code. The `Connection` card in Settings reads `WPA2-Personal or open. WPA3 (SAE) cannot be joined.`, and setup says WPA3 and enterprise networks cannot be joined. The drivers' code is what runs: WPA3-Personal joins, open networks do not.

A hidden network cannot be joined in this release. Settings joins only a network its scan heard, and there is no field to type a network name. The drivers can probe by name for a network saved as hidden, but nothing in this release saves a network that way.

## Limits in this release

- No open, TKIP, Enterprise or hidden networks.
- No wired network on real hardware: the e1000, RTL8139 and RTL8169 drivers get no DHCP lease.
- No USB Wi-Fi, no USB tethering, no USB Ethernet.
- No Wi-Fi in the `qemu` image.
- No IPv6.
- No Wi-Fi command in the Terminal.
- Saved networks need a TPM and a boot that keeps data.

## Parts

- [Join a Wi-Fi network](wifi-and-networking/join.md)
- [Remember networks, and what the status shows](wifi-and-networking/remember.md)
- [Join errors](wifi-and-networking/errors.md)
- [Ethernet, USB tethering, DHCP and DNS](wifi-and-networking/wired.md)
- [Where this comes from](wifi-and-networking/sources.md)

## See also

- [Wi-Fi drivers](../drivers/wifi/README.md)
- [RTL8821CE](../drivers/wifi/rtl8821ce.md)
- [Ethernet drivers](../drivers/ethernet/README.md)
- [Privacy networks](privacy-network.md)
- [Settings](settings.md)
- [Terminal](terminal.md)
- [Hardware support matrix](../hardware/MATRIX.md)
