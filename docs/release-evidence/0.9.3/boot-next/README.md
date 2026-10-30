# Boot of s4/next (battery, AC and lid; Bluetooth) on QEMU (TCG)

Image: `tools/nonos-dev-image --profile standard` (throwaway keys) built
from `s4/next` at e0bcc78650: main 8815d039ec with capsule_acpi in its
`release-power-only` build and driver.bt_usb0 (Realtek, Intel and MediaTek
setup). The serial log is beside this file; it carries no MAC address.

What it shows on q35 (OVMF, no battery, no Bluetooth controller):

- `acpi` spawns, takes the DSDT (9361 bytes, 0 SSDTs), builds a namespace
  of 300 objects, finds `no embedded controller`, arms 2 GPEs and finishes
  its first publish pass in 204 ms; no fault.
- `[ACPI] power battery=no charge=unknown ac=unknown`: the bar says "No
  battery".
- `driver.bt_usb0` spawns with caps 0x10208138 and says `no controller yet;
  looking each second`.
- The setup wizard starts and `[NET-CORE] lease 10.0.2.15/24 gw 10.0.2.2`.

Found by this boot and fixed after it (e69752f654): capsule_acpi's lines
had no newline of their own, so they ran together on the serial line; each
now ends in one.

Not shown: a battery, an EC or a lid (QEMU has none), a Bluetooth
controller. Those wait for the owner's HP (`docs/hardware/real-hardware-checklist.md`).
