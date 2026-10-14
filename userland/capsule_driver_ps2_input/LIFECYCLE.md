# capsule_driver_ps2_input: startup and failures

The order the capsule comes up in, and what each failure leads to. The summary
is in [README.md](README.md).

## Runtime lifecycle

Startup is deliberately ordered so partial ownership does not survive a
failed phase:

1. Discover the PS/2 keyboard platform record.
2. Claim the keyboard record.
3. Grant the i8042 PIO window.
4. Bind IRQ1.
5. Discover and claim the AUX platform record.
6. Bind IRQ12.
7. Drain stale controller output.
8. Enable keyboard scanning.
9. Enable the AUX port, set the controller config byte, reset mouse defaults,
   and enable mouse reporting.
10. Start the IPC service loop, whose first pass sets the lock LEDs to the
    driver's state (Num Lock on, Caps and Scroll Lock off) whatever firmware
    left lit.

PIO or IRQ setup failure rolls back broker grants that were already acquired.
If the mouse enable sequence is not acknowledged, the keyboard service still
runs and status replies report `mouse_enabled = 0`.

## Failure model

| Failure | Behavior |
|---|---|
| keyboard platform record missing | one log line, exit `EXIT_ABSENT` (2), nothing claimed |
| no i8042 answers (the ports float, the output buffer never empties) | the probe gives its claim back, one log line, exit `EXIT_ABSENT` (2) |
| keyboard enable fails | both claims are released (with the port grant and both lines); the attempt is repeated on the shared bounded schedule, and running out exits `EXIT_GAVE_UP` (6) |
| PIO grant denied | startup fails and device claim is released |
| IRQ1 bind denied | PIO grant and device claim are released |
| AUX record or IRQ12 bind denied | keyboard grants are released and startup fails |
| mouse command not acknowledged | keyboard path stays live, mouse status reports disabled |
| kernel input ring refuses a key event | counted in `OP_GET_STATE` |
| mouse queue full | new mouse events are dropped and counted |
| packet sync loss, or a part packet older than 500 ms | packet is discarded and counted |
| mouse resets (0xAA 0x00) | mouse brought up again, keys typed meanwhile still posted; one that does not come back, or resets again after three bring-ups within a minute, is turned off and said so once |
| keyboard does not acknowledge the LED command | LEDs left as they are until the next lock key |
| a bring-up wait on a slow controller or device | after eight fast polls, sleeps 1 ms between polls until its bound |
