# the hardware runs, booted by the developers on main 6f0bbe716

The image is standard-dev with throwaway keys, the same image as
gate-6f0bbe716. The steps and expected lines are hw's. Each run used
`--fresh` and its lane (`nic=X`, `usb-audio`, `emmc`, `ufs`).

| run | result |
|---|---|
| nic-e1000, e1000e, igb, rtl8139, pcnet, vmxnet3, tulip | each driver spawned and came up; `[DHCP] lease` never seen, stopped at that step (stuck.txt) |
| audio | driver.usb_audio0 spawned; no `usb-audio:` line, and audio.server chose driver.hda0; stopped there |
| emmc | driver.sdhci0 spawned, then `[EXIT] status 6: device present, bring-up failed and was given up`; eMMC "no disk" |
| ufs | driver.ufs0 spawned; the run completed (see 06-disks.png) |

Every run: 0 `unanswered` (rtl8139: 1), 0 ABORT, 0 PANIC or FATAL. Every run,
the gate included, logs `[MUNMAP] release_va_failed` once or more.
