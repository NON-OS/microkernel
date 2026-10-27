## Real hardware

The driver has never run on an RTL8822CE. Its first boot on one is its
test; the steps and what to send back are in
[the real-hardware checklist](../../../docs/hardware/real-hardware-checklist.md#realtek-rtl8822ce-wi-fi).

### What the log says, in order

On a development image (Debug granted) `log rtl8822ce` shows:

1. `[rtl8822ce] chip powered and answering; bus master on`
2. `[rtl8822ce] firmware v9.9`, then `firmware loaded and ready`
3. `trx engines enabled`, `mac initialised`
4. `phy tables loaded, two rf paths`
5. `antenna granted to wi-fi`, `phy configured`
6. `serving net_core`

A line in place of one of these names the stage that stopped, and the
Settings Wi-Fi panel shows the same stage. The lines that only this driver
prints: `pcie link config: dbi read timed out` (or `write`), `pcie clock
delay: dbi write timed out`, `efuse unreadable on a freshly powered mac`,
`rf front-end type has no table in this driver, radio not configured` (an RFE type above 6),
`dac calibration did not read back; radio left as calibrated` (the radio
still comes up), `coex window did not answer; antenna not granted`. No line
carries the efuse MAC address, which is never read out.

### What a first run decides

- Whether the uncalibrated receive path hears beacons at usable strength
  (the scan list's signal column against a phone's): if not, IQK is next.
- Whether the TX power cap (18 dBm conducted) lets a join complete at a
  few metres.
- Whether any card answers on 10ec:c82f, which Linux binds but no report
  has shown yet.
