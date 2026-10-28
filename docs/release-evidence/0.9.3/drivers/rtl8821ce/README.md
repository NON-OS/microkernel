# RTL8821CE: Wi-Fi on the owner's HP laptop (real hardware)

- **What:** the owner's report, relayed by the maintainers on 28 October 2026:
  "wifi works on real HW".
- **Machine:** the owner's HP laptop, the Gemini Lake bench machine (Realtek
  RTL8821CE, 10ec:c821, beside a Samsung MZVLQ256HBJD NVMe disk).
- **Image:** a sealed image built from main at 0bd7c2263, the main after the
  isolation-group fix (82ee1a3110: two vendor drivers may share an
  isolation group on a standard boot). That fix is what let the Wi-Fi card
  be claimed beside driver.nvme0, which had held their shared group since
  K11 (`HANDOFF/letters/2026-10-27-s4-to-s1-hp-claim.md`).
- **What it proves:** `capsule_driver_rtl8821ce` brings the card up, scans and
  joins on that machine with the 0.9.3 kernel. It does not prove the S4
  changes landed after that seal (ad2a58a01): the claim step and errno on the
  panel, 10ec:b821, the idle waits.
- **What it does not hold:** no serial log or screenshot was sent; the
  owner's words are the record. The next report from that machine should be
  `log rtl8821ce net-core > wifi.txt` (no address or SSID is in those lines).
