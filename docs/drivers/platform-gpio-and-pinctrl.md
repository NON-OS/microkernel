# Platform: GPIO and pinctrl

Part of [Platform](platform.md).

## GPIO and pinctrl

An I2C-HID touchpad holds its interrupt line asserted while a report waits, so
reading the line's level tells the touchpad driver when to read, without routing
the interrupt, as the `doorbell` module explains
(`userland/capsule_driver_i2c_pci/src/setup/gpio/mod.rs:17-38`). The I2C
controller driver reads GPIO levels and writes no GPIO register; the same module
comment says so.

The kernel registers each GPIO controller ACPI declares with
`register_acpi_gpio`, as a `GPIO_CTRL` record with one BAR per community window
(`src/hardware/broker/acpi_gpio/register.rs:28-49`,
`src/hardware/broker/class.rs:39-42`). When no fixed window can be read from
`_CRS`, as on Sunrise Point and later where firmware patches the windows in at
run time, `register_acpi_gpio` computes them with `community_window` from
SBREG_BAR and each community's port id
(`src/hardware/broker/acpi_gpio/register.rs:33-44`,
`src/arch/x86_64/acpi/aml/gpio_enumerate/sideband.rs:51-55`). `sbreg_bar` reads
it from the P2SB bridge at 00:1f.1, unhiding the bridge and hiding it again,
before any capsule can reach configuration space
(`src/hardware/broker/acpi_gpio/p2sb.rs:17-66`).

The controllers whose pins are placed:

| Controllers | ACPI ids | Source |
|---|---|---|
| Intel Broxton, Apollo Lake, Gemini Lake | INT3452, INT3453, one device per community | `userland/capsule_driver_i2c_pci/src/setup/gpio/mod.rs` |
| AMD FCH GPIO bank | AMD0030, AMDI0030, AMDI0031, AMDI0033 | `userland/nonos_pinctrl/src/controller.rs` |

The Intel pad layouts from Sunrise Point on were dropped in the originality
audit (F 46): every table had been transcribed from another operating system's
source rather than from a published datasheet. `LAYOUTS` is empty until a
layout is derived from Intel's PCH datasheet and cited by section
(`userland/nonos_pinctrl/src/layouts.rs`), so on those chipsets a firmware pin
is refused, never guessed, and the touchpad driver reads on a timer. `AMD_IDS`
lists the AMD bank (`userland/nonos_pinctrl/src/controller.rs`). The register
facts the crate keeps are recalled from Intel's and AMD's published documents
and marked unverified. `nonos_pinctrl` itself touches no register: it turns a
firmware pin number into the place its level is read.

`driver.i2c_pci0` answers the touchpad driver's doorbell request in `handle`
with two words, whether a line is mapped and whether it is asserted now
(`userland/capsule_driver_i2c_pci/src/server/handlers/doorbell.rs:17-52`).
Without a mapped layout, I2C-HID reads the pad on a timer. At this commit
`pinctrl_proofs` passes 8 tests over the controller ids, pin resolution on a
synthetic layout and the register arithmetic, `acpi_aml_proofs` 82 and
`i2c_pci_proofs` 35. [I2C-HID
touchpads](input/i2c-hid.md) covers the touchpad.

## What is not supported

- An AML interpreter, and with it battery level, lid events, a control-method
  power button, thermal zones and backlight control through ACPI.
- Shutting down from the desktop.
- Routing a GPIO interrupt; only levels are read.

## See also

- [Drivers](README.md)
- [Broker API](broker-api.md)
- [Input drivers](input/README.md)
- [Audio](audio.md)
- [IOMMU](../kernel/iommu.md)
- [Scheduler and SMP](../kernel/scheduler-and-smp.md)
- [PCI and ACPI](../kernel/pci-and-acpi.md)
- [Measured boot and the TPM](../security/measured-boot-and-tpm.md)
- [Support matrix](../hardware/MATRIX.md)
