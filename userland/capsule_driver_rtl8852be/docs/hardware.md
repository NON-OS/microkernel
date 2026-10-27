# Real hardware

The driver has never run on an RTL8852BE. The host proofs run its real
source against a model of the chip written from rtw89, so they prove the
driver does what rtw89 does in the order rtw89 does it, and that it stops
and says so where the model misbehaves. They cannot prove the model is
the chip. What only a card can confirm:

- That the status bits rtw89 polls come up as the model raises them, and
  in time (power, DLE, SS init, NCTL, DACK, ADDCK, DRCK, IQK reports).
- That the firmware 0.29.29.15 takes the download, the mailbox and the
  commands as rtw89 sends them, including the RF register pages.
- That RX and TX work on the air at the open-loop TX power the world-wide
  tables give: no TSSI and no DPK run.
- That the card's BAR is at least 512 KiB (1 MiB is expected).
- The release report and receive ring behaviour under load.

The steps for a first boot are in
`docs/hardware/real-hardware-checklist.md` ("Realtek RTL8852BE Wi-Fi").

## Not goals today

HT, VHT and HE rates; 40, 80 and 160 MHz; 6 GHz; hardware crypto; power
save; the Bluetooth coexistence engine; RSSI; single-antenna boards; the
RTL8852AE, RTL8852CE and RTL8851BE, which the inventory lists with no
driver.
