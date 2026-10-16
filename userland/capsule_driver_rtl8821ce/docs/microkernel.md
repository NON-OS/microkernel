## Microkernel contract

The manifest grants `IPC`, `Memory`, `Crypto`, `Driver`, `DeviceEnum`, `Mmio`,
`Dma`, and, optionally, `Debug`:

```text
CAPSULE_REQUIRED_CAPS = 0xB8038
CAPSULE_OPTIONAL_CAPS = 0x100
```

`Debug` (0x100) is optional so a serial-debug kernel can report radio
bring-up progress; a hardened build grants no `Debug` and the driver
still spawns. The driver reaches hardware only through `MkDeviceList`,
`MkDeviceClaim`, `MkMmioMap`, and `MkDmaMap`; it polls and binds no interrupt,
so the manifest holds no `Irq`. `Crypto` admits `CryptoRandom`, which draws
the station address and a join's nonces and SAE secrets. The kernel validates
the signed manifest, routes IPC, brokers grants, and revokes every grant on
capsule exit.

Only `net.core` and the Wi-Fi panels (`app.settings` in any of its three
windows, and `app.setup_wizard`) may send to `driver.rtl8821ce0`: the kernel
holds the endpoint to them (`src/services/registry/held.rs`), by name and by
pid.
