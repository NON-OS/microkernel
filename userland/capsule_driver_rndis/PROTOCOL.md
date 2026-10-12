# capsule_driver_rndis: NNET

NNET is the protocol net.core and net.l2 speak with every NIC driver. The shared core `nonos_usbnet` answers it for this capsule (`userland/nonos_usbnet/src/nnet`) on `driver.rndis0`, port 4254, reply port 4255. The rest of the contract is in [README.md](README.md).

## Operations

| Operation | Input | Output |
|---|---|---|
| `OP_HEALTHCHECK` (1) | none | status |
| `OP_LINK_STATUS` (2) | none | one byte: 1 up, 0 down or no device bound |
| `OP_MAC_ADDRESS` (3) | none | six bytes, the run's drawn station address, or `E_AGAIN` with no device |
| `OP_TX_PACKET` (4) | one Ethernet frame, 14 to 1514 bytes, no FCS | status: `E_MSGSIZE` when the length word and the body differ, `E_INVAL` for any other length, `E_AGAIN` with no device, or the device's errno |
| `OP_RX_PACKET` (5) | none | `len_le32` and the frame; `E_AGAIN` when none has come or no device is bound, or the device's errno |
| `OP_STATS` (0x0100) | none | twelve 32-bit counters |

The counters, in order: frames sent, send errors, frames received, receive errors, bytes sent, bytes received, devices bound, devices given up as gone, failures since a frame last moved, then three zero words.

`OP_STATS` sits off every number net.core sends. net.core's 6 is `OP_RX_BATCH`, which this capsule does not serve: it answers 6, like any operation it does not know, with `E_INVAL`, so net.core's first probe of a newly bound card finds no batches and takes frames one at a time with operation 5 (`userland/capsule_net_core/src/device/batch_served.rs`). A request that is not NNET is answered `E_INVAL`, and a delivery with no sender, the kernel's own, is not answered.

## Wire format

A request is the 20-byte header and its payload. A reply repeats the header with its own payload length, then a 4-byte signed status and the operation's data:

```text
magic_le32, version_le16, op_le16, flags_le16, 0_le16, request_id_le32, len_le32
```

The magic is `NNET` (0x4E4E4554) and the version 1.
