# Boots of main ad2a58a01 on QEMU (TCG)

Image: `tools/nonos-dev-image --profile standard` (Standard's drivers,
throwaway keys) built from main at ad2a58a01, the driver landing.
Booted with `tools/nonos-drive`; the serial logs are beside this file. No
line in them carries a MAC address.

| Boot | Lane | Result |
|---|---|---|
| default | virtio-net, xHCI stick, NVMe target, ICH9 AHCI with a QEMU disk | setup wizard up, `[NET-CORE] lease 10.0.2.15/24 gw 10.0.2.2`, `[HW] 11 driven, 0 known and held, 2 need none, 0 unknown`, no fault, no driver exit error |
| e1000e | `nic=e1000e` | `[NET-CORE] lease 10.0.2.15/24 gw 10.0.2.2` |
| igb | `nic=igb` | lease, as above |
| e1000 | `nic=e1000` | lease, as above |
| rtl8139 | `nic=rtl8139` | lease, as above |

What the default boot shows for the new Wi-Fi drivers on a machine without
their cards: `[DRIVER-IWLWIFI]`, `[DRIVER-RTL8821CE]`, `[DRIVER-RTL8822CE]`,
`[DRIVER-MT7921]` and `[DRIVER-RTL8852BE]` each `no controller present, not
spawned`; the loader prints nothing for the absent MediaTek package.

Not shown: any Wi-Fi card (QEMU has none), traffic past the gateway (no
outbound network here: Nym and Anyone retry their directories).
