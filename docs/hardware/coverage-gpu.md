# Coverage: Graphics

Written by `tools/hardware/coverage.py` from `coverage-parts.tsv`; edit that file,
not this page. [How to read it](coverage.md#how-to-read-it).

|---|---|---|---|---|---|---|---|---|
| P1 | Intel display, Haswell to Raptor Lake | 2017-2024 | most machines | `8086:3e92` `8086:9bc8` `8086:9a49` `8086:46a6` `8086:a780` | 5 of 5 | no | G | wired; out of Standard until sealed; firmware framebuffer meanwhile |
| P2 | Intel Meteor Lake, Arrow Lake, Lunar Lake, Arc | 2023-2026 | most machines | `8086:7d55` `8086:7dd5` `8086:64a0` `8086:56a0` | no driver | no feature | G | firmware framebuffer only |
| P2 | AMD Radeon integrated and discrete | 2017-2026 | most machines | `1002:*` | no driver | no feature | G | firmware framebuffer only |
| P3 | NVIDIA GeForce | 2017-2026 | most machines | `10de:*` | no driver | no feature | G | firmware framebuffer only |
