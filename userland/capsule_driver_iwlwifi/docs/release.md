## Wire format

Control family requests: `[magic u32][op u16][request id u32][body]`, replies
echo the header. Status reply after the header: `[stage u8][step u8]` then
seven little-endian u32 words: detail, CSR_HW_REV, CSR_HW_RF_ID, sweeps
completed, sweeps stalled, frames received, beacons parsed. Scan reply after
the header: sweeps, frames, beacons (u32 each), then `[count]` and per network
`[signal 0..100][flags][ssid_len][ssid]` (flags: bit 0 secured, bit 1 WPA2,
bit 2 WPA3). The largest reply is 583 bytes. The connect body and the
connect and link replies are laid out in the Interface contract above and in
`server/join_wire.rs`; the client reads them with `nonos_wifi_client`'s own
parsers (proven in `serve_tests`).

net_core link protocol requests use its 20-byte envelope (`NNET`); a frame is
at most 1514 bytes.

Any other request gets no reply. All multi-byte integers are little-endian.

## State ownership

`driver.iwlwifi0` owns only hardware-facing Wi-Fi state: PCI identity, broker
grant ids, BAR mapping, IRQ binding, DMA grants, hardware and RF ids, the
firmware's queues and contexts, the current scan list, and while joined the
link's keys, counters and the network's name. The client and panels own scan
presentation, saved networks and join policy.

## Operating rules

- Do not place DHCP, DNS, IP, or socket policy in this capsule.
- Do not import kernel driver or memory internals.
- Do not use inline PIO or architecture assembly.
- Do not persist Wi-Fi profiles or scan history.
- Every DMA access goes through `Region` with its bounds checked; every wait is
  bounded.
- Firmware upload must go through bounded DMA and explicit completion.

## Release target

The hardware chain is:

```text
driver.iwlwifi0 -> nonos_wifi_core -> net.core -> net.ip -> apps
```

## Release evidence

- `iwlwifi_proofs`: 197 tests run by `cargo test` (`src/link_tests.rs` is not
  compiled), host only. The gen3 boot runs end to end
  against `gen3_model` with the bundled so-a0-gf-a0-86 image, and so do WPA2
  and WPA3 joins with data through the transmit and receive rings.
- `nonos_wifi_core_proofs`: 70 tests (the shared scan list among them).
- The iwlwifi layout check: every join command's size
  and field offsets against Linux v6.12's headers, compiled with gcc, `bad=0`.
- No hardware log for this driver is committed.

## Release checklist

- Capsule builds with zero warnings for `x86_64-nonos-user`.
- Static gates confirm brokered MMIO/IRQ/DMA authority and endpoint ownership.
- Kernel profile `microkernel-driver-iwlwifi` resolves signed artifacts.
- Firmware selection picks the expected `.ucode` for the detected MAC and RF.
- Firmware upload and ALIVE pass on supported hardware.
- Passive scan returns bounded results without persisting SSID history.
- A WPA2 and a WPA3 join reach a DHCP lease on supported hardware.
