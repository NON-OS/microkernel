# Design decisions

Some behaviour in the tree depends on a numbered decision. This page says what each one settles and what
the tree does today, as the code and the docs that cite it state it. A decision marked open is not yet
decided, and the tree holds the cautious default until it is.

| No. | What it settles | State | In the tree |
|---|---|---|---|
| 4 | Devices below a switch port that has no ACS can reach one another. Such a group is held by at most one capsule, and an unclaimed device in it is reachable from the others. | Decided | The boot says so on its `[IOMMU]` line (`src/arch/x86_64/iommu/acs/close.rs`). Hardened and Air-Gapped refuse the claim ([security limits](../release/0.9.3-security-limits.md)). |
| 6 | Whether NØNOS suspends at all. | Decided: no suspend | `SUSPEND = false` in `capsule_acpi`; the kernel's resume path is not wired (`userland/capsule_acpi/src/suspend/switch.rs`, [ACPI and platform matrix](../hardware/platform-rulings.md)). |
| 9 | Which capsule may hold `CameraCapture`, `MicrophoneCapture` and `ScreenCapture`, and who may hold `driver.fido0`. | Open | No capsule holds the capture capabilities, so the camera, microphone and screen-capture paths stay off ([capabilities](../abi/capabilities-what-a-capability.md)). |
| 10 | Whether a driver whose register programming was read from Linux's GPL-only code may ship. | Open | `driver-rtl8153` and `driver-ax88179` are built by their own profiles and are not in Standard ([feature inventory](../build/feature-inventory.md)). |
| 12 | Whether NØNOS balloons guest memory at all, and the default policy for authorizing a USB configuration. | Open | Nothing claims `virtio-balloon` (`src/hardware/broker/claim/hold_rule.rs`). The xHCI authorization policy defaults to Allow, so nothing changes on any machine (`userland/capsule_driver_xhci/src/slots/authorize.rs`). |
| 14 | Whether a Wi-Fi network may keep the machine's address across reboots. | Open | The station's secret is drawn fresh at each start, so no network sees this boot's address again after a reboot (`userland/nonos_wifi_core/src/address/keys.rs`). |
| 15 | Whether the display accepts hints from a host. | Open | The registry holds none and the Bochs display driver sends none (`src/services/registry/display_hints.rs`, `userland/capsule_driver_bga`). |
| 19(c) | Which Intel display firmware (DMC) blobs may load. | Decided | A blob loads only when its SHA-256 is sealed in `data/dmc_sealed.txt`. The file ships empty, so no DMC loads and the display's low-power states stay off. The cost is idle power with the panel lit, never a picture (`userland/capsule_driver_intel_display/src/dmc.rs`). |
| 21 | Which Bluetooth transport the host speaks. | Decided: LE only, HID over GATT | `nonos_bt_hci`, `nonos_bt_hogp`, `nonos_bt_smp` and `nonos_bt_bond`. |
| 27 | How long Bluetooth bonds last. | Decided: this boot only | Bonds are kept in memory and gone at power off (`userland/capsule_driver_bt_usb/README.md`). |
