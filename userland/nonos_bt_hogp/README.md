# nonos_bt_hogp

HID over GATT for an LE-only host (Decision 21). It follows HOGP 1.0 and
HIDS 1.0, over the ATT protocol and GATT of the Bluetooth Core
Specification 5.4 (Vol 3, Parts F and G).

The host is a **Boot Host**:
- It puts each HID service in Boot Protocol Mode.
- It reads the boot keyboard and boot mouse input reports, whose layout
  the USB HID specification fixes (HID 1.11, Appendix B).
- It parses no report map, so a peripheral cannot steer it with a crafted
  one.

| Module | What it holds |
|---|---|
| `host` | `Host`, the GATT client as a state machine. It is fed the peripheral's ATT PDUs and the time. Each call returns the PDUs to send on CID 0x0004, the input events, and how discovery ended. |
| `report` | `Keyboard` and `Mouse` turn boot reports into `Input` events: key and button down and up, and relative motion. Keys are HID Keyboard page usages, so a capsule maps them with the same keymap as the USB keyboard driver. |
| `uuids` | The UUIDs and characteristic properties it looks for. |

Discovery and configuration run in this order:
1. Find the HID services (UUID 0x1812) with Find By Type Value.
2. Read each service's characteristic declarations with Read By Type.
3. Keep the boot keyboard input (0x2A22) and boot mouse input (0x2A33)
   reports that can notify, and Protocol Mode (0x2A4E).
4. Find each report's CCCD with Find Information, within its own
   characteristic.
5. Write 0 (boot) to every Protocol Mode with a Write Command.
6. Write 0x0001 to each CCCD with a Write Request, one at a time.

After that, notifications on the report handles become events.

The ATT bearer must already be encrypted. A peripheral that answers with
an authentication, encryption or key-size error ends discovery with
`HogpError::Security`, so the caller can pair first.

The host stops with an error, and never loops, on any of these:
- a response to no request, or an Error Response naming another request;
- handles that do not move forward;
- a descriptor outside its characteristic;
- more than 4 HID services, 64 declarations in one service, or 512
  requests;
- no response within the 30 s ATT transaction timeout.

After a timeout nothing more is sent on the bearer.

A request from the peripheral is answered with Request Not Supported,
because this host keeps no attribute database. Commands are ignored.
Indications are confirmed.

When the link drops, `disconnected` releases every key and button still
held.

The proofs live in `nonos_bt_hogp_proofs`.
