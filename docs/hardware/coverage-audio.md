# Coverage: Audio

Written by `tools/hardware/coverage.py` from `coverage-parts.tsv`; edit that file,
not this page. [How to read it](coverage.md#how-to-read-it).

|---|---|---|---|---|---|---|---|---|
| P1 | Realtek ALC codecs (ALC2xx, ALC897, ALC1220) | 2017-2026 | most machines | `10ec:0256` `10ec:0257` `10ec:0269` `10ec:0295` `10ec:0897` `10ec:1220` | 6 of 6 | yes | B | EAPD table covers them; per-board pin fixups not checked on real boards |
| P2 | Intel SST/SOF DSP laptops with digital microphones | 2019-2026 | most machines | `8086:02c8` `8086:a0c8` `8086:51c8` `8086:7e28` | 4 of 4 | yes | B | codec output through HDA; the DSP's DMIC capture has no driver |
| P2 | Conexant, Cirrus Logic and other HDA codecs | 2017-2026 | common | `14f1:*` `1013:*` `111d:*` | no driver | yes | B | Cirrus and IDT family setup and Apple GPIOs, Conexant EAPD (host-tested); per-board pin fixups not taken |
| P3 | AMD ACP (laptop microphones and speakers on some boards) | 2019-2026 | common | `1022:15e2` | no driver | no feature | B | detected only |
| - | Intel HD Audio controller (PCH) | 2017-2026 | most machines | `class:04/03` | by class | yes | B | works on the bench laptop (6 Oct report) |
