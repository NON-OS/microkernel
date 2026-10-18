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
| `OP_ACPI_HID` | none | a touchpad in reach, the next on each ask, the one setup bound first: I2C address, HID descriptor register, GPIO pin and HID_INFO bits; empty when nothing is in reach |
| `OP_GPIO_DOORBELL` | none | whether the touchpad's interrupt line is mapped, and asserted now; not mapped once the bound pad is out of reach |

Unknown operations reply `E_BAD_OP`. Non-empty bodies on fixed-width requests
reply `E_INVAL`, and so does a transfer or a probe to an address not in reach,
before anything reaches the bus (`src/transaction/types/reach.rs`). A transfer
that writes the HID descriptor register declared for its address and reads a
valid HID descriptor back narrows the reach to that address for the rest of
the session ([CONTRACT.md](CONTRACT.md#the-client-and-its-reach)).

`OP_ACPI_HID` names the addresses in reach in turn, one per ask, starting with
the one setup bound: multi-SKU firmware declares several pads and only one is
fitted, and until a client has read a pad's HID descriptor every pad declared on
the controller stays in reach, each named in its turn; once the reach has
narrowed, that pad alone is named (`src/transaction/types/named.rs`). The bound
address is the one the firmware declared or, for a declaration whose address the
firmware fills in at run time, the device that answered the sweep of 0x15, 0x2C
and 0x10 first, pad or not (`src/setup/sequence/sweep.rs`). Its register is the
one the probe found the descriptor at, the declared one when nothing answered
yet, since firmware that computes its `_DSM` at run time leaves the declared
register a guess; every other address carries its declared register. The GPIO
pin and HID_INFO bits are the bound declaration's, the line `OP_GPIO_DOORBELL`
senses, and zero for any other address (`src/server/handlers/acpi_hid.rs`).

Every request is wiped from the receive buffer once it is answered, and every
reply from the reply buffer once it is sent, with the engine's copy of the
bytes a transfer read: nothing a client sent or a device returned stays in the
capsule after it is delivered (`src/protocol/wipe.rs`).
