# Lifecycle, failures and release

Part of [capsule_driver_rtl8852be](../README.md).

## Runtime lifecycle

Spawned by the kernel only when the boot scan found one of its ids. It
claims the card, powers off a MAC left on, turns Bus Master on, maps the
rings, probes (firmware, efuse, PHY capabilities), starts (MAC, PHY tables,
coexistence, calibration), draws its address, adds its station interface
and serves. It serves whatever stage it reached, so the panels can say
where it stopped. The steps and the Linux functions they follow are in
[stages.md](stages.md).

## Failure model

Each failure is a stage (`src/stage.rs`): 1 not claimed, 2 power, 4
firmware or mailbox, 5 no DMA, 6 efuse, 7 no randomness, 8 a one-antenna
board, 12 a PCIe, MAC, PHY or calibration step. The firmware download is
tried five times, as rtw89 tries it. After the claim, any stop powers the
MAC off, drops Bus Master and gives the claim back (`setup::give_back`).
With no card the capsule exits `EXIT_ABSENT` (2) before claiming anything.

## Release target

`microkernel-full-gui`, the Standard image, beside the other Wi-Fi drivers;
listed in `scripts/baselines/unsealed.txt` until the next seal.

## Release evidence

Host proofs only. No boot on a card.

## Release checklist

- [ ] A boot on an RTL8852BE laptop, by
      `docs/hardware/real-hardware-checklist.md` ("Realtek RTL8852BE Wi-Fi").
- [ ] WPA2 and WPA3 joins on 2.4 and 5 GHz, a DHCP lease and a download.
- [ ] The seal.

## Explicit non-goals today

TSSI, DPK, RSSI, HT/VHT/HE rates, 40 MHz and wider, 6 GHz, hardware crypto,
power save, the Bluetooth coexistence engine, single-antenna boards,
access point and monitor modes, the RTL8852AE and RTL8852CE.

## Verification

See [verification.md](verification.md).
