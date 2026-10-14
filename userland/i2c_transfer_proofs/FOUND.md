# i2c_transfer_proofs: what was found

The defects these proofs caught in the shipping engine. What they prove is in
[README.md](README.md).

## What was found

Three defects in the shipping engine, each caught by a test here before the
fix and passing after it.

A NACKed transfer reported success. An abort empties the TX FIFO and idles
the bus exactly as a completed transfer does; only `TX_ABRT` tells them apart,
and the engine checked it at the top of its loop but declared completion at
the bottom. On the model the window is deterministic; on silicon it is a race
the driver loses a fraction of the time, and every probe of an absent address
could come back as a device. The engine now checks the abort again before
trusting completion.

The FIFO depths were assumed. The engine pushed against a constant 64 while
the core reports its depths in `IC_COMP_PARAM_1`, and silicon ships with 8,
16 and 32. Against an eight-entry core the driver lost 56 of 64 commands. The
depths are now read at bring-up and carried in the driver.

Reads in flight were not counted. `IC_RXFLR` counts bytes that have landed,
not reads issued, so a burst of read commands could exceed the room left in
the receive FIFO. The engine now counts issued reads against that room.

## See also

- [README.md](README.md)
