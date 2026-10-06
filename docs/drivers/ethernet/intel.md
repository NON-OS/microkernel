# Intel Ethernet

NONOS has three Intel Ethernet driver [capsules](../../overview/glossary.md#capsule): e1000 for the 8254x family is in the image, while e1000e (82574, 82583, I217, I218, I219) and igc (I225, I226) are written and host-tested but built into no image in 0.9.2.

## Which driver takes which chip

| Driver | Chips | In the 0.9.2 image | State |
|---|---|---|---|
| `driver.e1000_0` | 8254x family, 28 device ids | yes | Partial: host tests pass, [receive fault](README.md#the-receive-fault), no hardware report, no QEMU run target |
| `driver.e1000e_0` | 82574, 82583, and the I217, I218, I219 PHYs on Intel PCH chipsets, 64 device ids | no | Not supported: not built, not started, not bound by `net.core` |
| `driver.igc_0` | I225, I226 and their variants, 16 device ids | no | Not supported: not built, not started, not bound by `net.core` |

An Intel I217, I218, I219, I225 or I226 port has no driver in the 0.9.2 image. The e1000 driver that is in the image carries [the receive fault](README.md#the-receive-fault); read [what to use instead](../wifi/not-supported.md#what-to-use-instead) before you plan on a wired link.
