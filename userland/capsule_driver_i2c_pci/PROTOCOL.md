# capsule_driver_i2c_pci: operations

What each `driver.i2c_pci0` operation takes and answers. The contract is in
[README.md](README.md). Requests use the `NI2C` capsule header, version `1`,
and the shared 20-byte driver envelope; replies begin with a 4-byte signed
status word, and every multi-byte integer is little-endian.

| Operation | Input | Output |
|---|---|---|
| `OP_HEALTHCHECK` | none | status |
| `OP_CONTROLLER_INFO` | none | 52 bytes: device id (0..8), PCI id (8..10), clock Hz (10..14), claim epoch (14..22), MMIO grant (22..30), family name (30..52, zero padded) |
| `OP_REGISTER_SNAPSHOT` | none | DesignWare status/FIFO/config registers |
| `OP_TIMING_INFO` | none | standard/fast-mode SCL count registers |
| `OP_TRANSFER` | address in reach, write bytes, read length | status, abort source, read bytes |
| `OP_PROBE` | 7-bit address in reach | present / absent |
| `OP_ACPI_HID` | none | the touchpad setup bound: I2C address, HID descriptor register, GPIO pin and HID_INFO bits; with nothing bound, the first 7-bit address the firmware declared; empty when there is none |
| `OP_GPIO_DOORBELL` | none | whether the touchpad's interrupt line is mapped, and asserted now |

Unknown operations reply `E_BAD_OP`. Non-empty bodies on fixed-width requests
reply `E_INVAL`, and so does a transfer or a probe to an address not in reach,
before anything reaches the bus (`src/transaction/types/reach.rs`). A transfer
that writes the HID descriptor register declared for its address and reads a
valid HID descriptor back narrows the reach to that address for the rest of
the session ([CONTRACT.md](CONTRACT.md#the-client-and-its-reach)).

`OP_ACPI_HID` answers the address setup bound: the one the firmware declared,
or, for a declaration whose address the firmware fills in at run time, the
device that answered the sweep of 0x15, 0x2C and 0x10 first, pad or not
(`src/setup/sequence/sweep.rs`). The descriptor register is the one the probe
found the descriptor at, the declared one when nothing answered yet: multi-SKU
firmware declares several pads and only one is fitted, and firmware that
computes its `_DSM` at run time leaves the declared register a guess
(`src/server/handlers/acpi_hid.rs`).
