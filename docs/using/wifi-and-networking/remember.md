## Remember networks

Turn on `Remember networks I join` with `R`. A network you then join is saved, with its passphrase, and joined again at the next boot when it is in range.

- At most four networks are kept. Saving a fifth drops the oldest one not pinned to WPA3, or the oldest of all when all four are, so a pin is lost only when nothing else can go. The panel names the one dropped, on screen only: Settings says `Remembered; forgot <name> to make room`, and setup's review says `Forgot <name> to make room for this network` before it goes on.
- The list is saved only on a boot that keeps data: a boot of a disk NONOS was installed to, or a stick boot whose setup chose Install on its Mode step. Settings shows it as the read-only `Keep data across reboots` row under `Privacy`. On any other boot the row reads `Off: this boot keeps nothing`.
- It is saved only with a TPM. The list is sealed with ChaCha20-Poly1305 under a [machine key](../../overview/glossary.md#machine-key) the TPM derives for the label `wifi/saved-networks`. That key is bound to PCRs 0, 4, 7 and 9, so after a firmware or kernel change the saved list no longer opens and you join again by hand.
- The sealed record is the file `/nonos/wifi/saved` in the NONOS store.

At boot, `net.core` scans and joins the first saved network in range. A network whose join fails is tried again in later rounds, at most eight, each after a longer wait (from 4 seconds up to a minute), so someone sending forged refusals costs time, not the network; a wrong passphrase is sent at most that many times per boot. After eight scans with none in range it stops and logs `no saved Wi-Fi network in range; join one from Settings`.

## What the status shows

The `Status` row on the Wi-Fi page reads:

| Line | Meaning |
|---|---|
| `Connected to <name>` | The card is associated with that network. |
| `Not connected` | The driver is ready and no network is joined. |
| `<adapter>: <stage>` | The driver is still bringing the card up, or stopped at a step, for example `The card's firmware did not load`. |
| `Firmware stopped: <step> (0x....)` | The RTL8821CE firmware load stopped at the named step, with the control register's bits. |
| `No Wi-Fi hardware found` | No wireless chip is on the bus. |
| `Wi-Fi driver did not start for vvvv:dddd` | This build has a driver for the chip, and the driver is not running. |
| `Wi-Fi chip vvvv:dddd has no NONOS driver; use Ethernet or USB Wi-Fi` | No driver for this chip. NONOS has no USB Wi-Fi driver in this release, so a cable is the only other way, and on real hardware a cable gets no address in this release (see [Ethernet](wired.md#ethernet)). |

On an Intel card the driver takes but cannot use, the stage reads `card not supported yet; use Ethernet or USB Wi-Fi`. The same note about USB Wi-Fi applies. [Not supported](../../drivers/wifi/not-supported.md) lists the cards with no driver.

Settings' `Network` page shows where the machine stands with DHCP. Its `Network status` card has `Connection` (`Connected`, `No address`, `Not responding` or `Offline`), `IP address`, `Gateway` and `DNS`. Its `Interfaces` card shows the `Wireless adapter`, or `None detected`.

In the Terminal, `ifconfig` (or `ip`) asks the DHCP client and prints one line:

```
ifconfig
```

Not tested in this release.

With a lease the line has the form `net0: inet <address>/<prefix> gw <gateway> dns <server>`; without one it is `net0: down`.
