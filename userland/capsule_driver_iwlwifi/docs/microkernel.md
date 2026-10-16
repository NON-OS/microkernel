## Microkernel contract

The manifest grants `IPC`, `Memory`, `Crypto`, `Driver`, `DeviceEnum`, `Mmio`,
`Irq`, and `Dma`:

```text
CAPSULE_REQUIRED_CAPS = 0xF8038
```

The driver reaches hardware only through `MkDeviceList`, `MkDeviceClaim`,
`MkMmioMap`, `MkIrqBind`, and `MkDmaMap`. The kernel validates the signed
manifest, routes IPC, brokers grants, and revokes every grant on capsule exit.

Only `net.core` and the Wi-Fi panels (`app.settings` in any of its three
windows, and `app.setup_wizard`) may send to `driver.iwlwifi0`: the kernel
holds the endpoint to them (`src/services/registry/held.rs`), by name and by
pid. Within that, the driver serves the link protocol (raw frames onto the
air) only to the process that owns `net.core`, as
the registry names it on each request; anyone else is refused (EPERM), so a
panel cannot put frames on the air around the route the person chose. The
panels keep the Wi-Fi control family: scan, join and status
(`src/server/link_serve.rs`, `nonos_wifi_core::netif::gate`).

Two limits of this grant shape the driver:

- The broker maps at most 64 pages (256 KiB) per DMA grant for a network-class
  device (`dma_page_limit_for_class`). The gen3 path spreads its memory over
  several grants of at most that size (control region, receive buffers, and the firmware sections planned
  first-fit by `gen3::plan`). The first join maps one more, the 160 KiB
  transmit region (`gen3::txq`: the management and data queues' rings, byte
  count tables and frame buffers), kept for the capsule's life once the
  firmware holds its addresses.
- `Crypto` is there for `CryptoRandom` alone: the station address is drawn
  fresh every boot (`server/radio/address.rs`, through `nonos_mac`), as are a
  join's handshake nonce and SAE secrets. If the kernel gives no randomness
  the interface takes the fixed address 02:00:00:00:00:01 and only the
  passive scan runs.
- The mask does not hold `Debug` (no console). Without it the `[iwlwifi]`
  console lines are refused by the kernel; the status reply carries the same
  facts.
