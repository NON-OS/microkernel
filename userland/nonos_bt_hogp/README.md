# nonos_bt_hogp

HID over GATT for an LE-only host (Decision 21). It follows HOGP 1.0 and
HIDS 1.0, over the ATT protocol and GATT of the Bluetooth Core
Specification 5.4 (Vol 3, Parts F and G).

The host starts as a **Boot Host** and upgrades keyboards and mice to
report mode where it can:
- It reads the boot keyboard and boot mouse input reports, whose layout
  the USB HID specification fixes (HID 1.11, Appendix B).
- A keyboard service with a Report Map is taken to Report Protocol Mode
  when its map parses within fixed bounds and lays out a report of
  Keyboard page keys. That brings the keys a boot report cannot carry:
  more than six at once, and Mute, Volume Up and Volume Down.
- A service with a boot mouse is upgraded the same way when its map lays
  out a relative pointer. That brings the wheel. Each axis is clamped to
  what a boot mouse report carries, one signed octet.
- Every other service stays in Boot Protocol Mode, and so does a keyboard
  whose map is malformed, too long, unreadable or of no use. Boot mode
  needs no map, so a crafted map can at worst cost a peripheral its
  volume keys and its wheel.

| Module | What it holds |
|---|---|
| `host` | `Host`, the GATT client as a state machine. It is fed the peripheral's ATT PDUs and the time. Each call returns the PDUs to send on CID 0x0004, the input events, and how discovery ended. |
| `report::map` | The Report Map parser (HID 1.11, 6.2.2) into a `Layout`, the decoding of an input report into the keys held, and the LED output report. |
| `report` | `Keyboard` and `Mouse` turn boot reports into `Input` events: key and button down and up, and relative motion. `Locks` keeps the Num, Caps and Scroll Lock state. Keys are HID Keyboard page usages, so a capsule maps them with the same keymap as the USB keyboard driver. |
| `server` | The host's own attribute database, and the answer to each request the peripheral sends (Vol 3, Part F, 3.4). |
| `uuids` | The UUIDs and characteristic properties it looks for and serves. |

Discovery and configuration run in this order:
0. Ask for a larger ATT_MTU (below).
1. Find the HID services (UUID 0x1812) with Find By Type Value.
2. Read each service's characteristic declarations with Read By Type.
3. Keep the boot keyboard input (0x2A22) and boot mouse input (0x2A33)
   reports that can notify, Protocol Mode (0x2A4E), and the boot keyboard
   output reports (0x2A32) that take Write Without Response.
   For report mode it also keeps the Report Map (0x2A4B) and up to 16
   Reports (0x2A4D).
4. Look at report mode for each service worth it (below).
5. Find each boot report's CCCD with Find Information, within its own
   characteristic.
6. Write each Protocol Mode with a Write Command: 1 (report) for an
   upgraded service, 0 (boot) for the rest.
7. Write 0x0001 to each CCCD with a Write Request, one at a time.

After that, notifications on the report handles become events.

## The ATT_MTU

Before discovery the host sends an Exchange MTU Request (Vol 3, Part F,
3.4.2), offering 247: a 251-octet LE data PDU less the 4-octet L2CAP
header, well within the 512 octets the capsule reassembles. It asks once.

A server's answer from 23 up to 247 becomes the bearer's ATT_MTU. Every
other answer is ignored, never trusted, and the default 23 holds:
- under 23;
- over the offer;
- malformed;
- an error.

The standard would take the smaller of the two for an answer over the
offer. Ignoring it instead costs nothing: every read the host makes goes
on while answers come full, so a server that sends longer answers than
the MTU the host keeps is still read whole. Discovery then goes on as
before. An exchange with no answer ends the bearer at the ATT timeout,
like any other request.

The server's own Exchange MTU Response gives the same 247, so a peer's
exchange and the host's agree. The server's responses stay within 23,
which any ATT_MTU allows.

## Report mode

A service is worth upgrading only if it has a Protocol Mode, a readable
Report Map, a boot keyboard or boot mouse input report, and a Report
that can notify. Protocol Mode is one per service, so report mode
silences every boot report in it. The service is upgraded only if its
map replaces each one: a boot keyboard by a Report with Keyboard page
keys, a boot mouse by a Report with relative X and Y.

For such a service the host:
1. Reads the Report Map with a Read, then Read Blobs from the length so
   far, while each answer fills a response at the bearer's ATT_MTU. An empty
   answer, Attribute Not Long or Invalid Offset ends the map. A map past
   512 octets, the most HIDS 1.0 allows, is not read further.
2. Finds each Report's Report Reference (0x2908) and CCCD with Find
   Information, within its own characteristic, and reads the Reference:
   one octet of Report ID, one of type.
3. Parses the map. The parser holds every count to a fixed bound: 512
   octets of map, 8 open collections, 4 pushes, 16 usage runs per item,
   64 fields, 16 reports, 512 bits per report, 32 bits per element. It
   refuses long items, reserved items, unpaired collections, pushes and
   usage ranges, Report ID 0, and numbered and unnumbered reports mixed.
4. Upgrades the service if its input Reports that notify and have a
   CCCD replace each of its boot reports, as above. Those Reports, and
   any that hold Consumer keys, replace the boot input reports. The
   output Reports whose fields carry LED usages replace the boot output
   report.

An input report then becomes the keys it holds: modifier and key bits,
key arrays counted from their Logical Minimum, and Mute, Volume Increment
and Volume Decrement as the Keyboard page's Mute, Volume Up and Volume
Down. A report shorter than its layout, a report with ErrorRollOver,
POSTFail or ErrorUndefined, or one holding more than 32 keys changes
nothing, as a phantom boot report does not.

A pointer report becomes buttons 1 to 3 from the Button page, then
relative X, Y and Wheel from the Generic Desktop page. Absolute axes, as
a tablet has, are not read, so a tablet keeps boot mode. A value outside
its field's logical extent counts as none. Each axis is clamped to −128
to 127, the boot report's range, so a report-mode pointer cannot move
further in one report than a boot one. Motion comes first, then the
wheel as `Input::Wheel` with HID's sign (positive away from the user),
then the buttons that changed.

Anything the peripheral gets wrong while report mode is looked at leaves
the service in boot mode, not failed: a refused read, a descriptor out of
place, a Reference of the wrong length, a map that does not parse. Two
things still end discovery as anywhere: a response to no request, and a
security error, which asks for pairing.

## Battery

A HID device carries the Battery Service (HOGP 1.0, 3). Once the reports
are on, the same host runs a Battery Service client (BAS 1.0). It runs
after the reports, so typing never waits on it, and it keeps to one
request outstanding:
1. Find the Battery Services (0x180F), at most 4. Instances past that are
   left unread, not refused.
2. In each, keep the first Battery Level (0x2A19) that can be read. The
   service holds one (BAS 1.0, 3).
3. Read each level. If it can notify, find its CCCD within its own
   characteristic and write 0x0001.

Each level read or notified comes out as `Step::battery`, a `Battery`
naming the instance, in database order, and the percentage. Only one
octet from 0 to 100 is a level (BAS 1.0, 3.1); anything else is passed
over.

The battery client ends without touching HID when it hits any of these:
- a malformed answer or one out of place;
- an ATT error, apart from Not Found. A refused read or CCCD write passes
  on to the next level instead, since a level may need more security than
  the reports;
- the bearer's request budget. The battery yields it rather than end HID.

The ATT timeout is the exception. It ends the bearer for every client on
it (Vol 3, Part F, 3.3.3), so a battery request with no answer for 30 s
ends HID too.

The host owns the lock state, as with a USB keyboard. A Caps Lock, Num
Lock or Scroll Lock key going down toggles its lock. The new octet (HID
1.11, B.1) then goes to every boot keyboard output report in a Write
Command, so the LEDs never wait on a response. In report mode the LEDs
go in the output report the map lays out, each LED usage on its own bit. `set_locks` takes the
state from the session instead, for a lock changed on another keyboard.
A lock already on when discovery ends is written then.

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

A request from the peripheral is answered by `server`, from a small fixed
database that GAP and GATT ask of every LE device:
- the GAP service (0x1800, Vol 3, Part C, 12) with Device Name, "NONOS"
  (12.1), and Appearance, Generic Computer (12.2);
- the GATT service (0x1801, Vol 3, Part G, 7), with no characteristic,
  because the database never changes (7.1).

Every attribute can be read and none written. The server answers
Exchange MTU (with 247), Find Information, Find By Type Value, Read By
Type, Read By Group Type and Read. A Write Request gets Write Not
Permitted. Any other request gets Request Not Supported. Every response
fits the default ATT_MTU of 23. Commands are ignored. Indications are
confirmed.

When the link drops, `disconnected` releases every key and button still
held.

The proofs live in `nonos_bt_hogp_proofs`.
