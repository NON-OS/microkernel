# nsh v1: what is refused, and why

Each of these is refused by design. A later version may add a feature
only as a new, versioned interface.

## The protocol

| Refused | Why |
|---|---|
| Any version or suite but 1/1, and negotiation of any kind | A negotiation can be downgraded. With one suite there is nothing to downgrade |
| A classical-only key exchange | A recorder keeping traffic now could read it later |
| One signature algorithm alone | Both Ed25519 and ML-DSA-65 must verify, so breaking one is not enough |
| Trusting a host key the first time it is seen | A host is trusted only through enrolment and a fresh quote |
| Passwords, keyboard-interactive login, "host-based" login | The only credentials are keys and signed grants |
| Any credential before the host is checked | This is the point of nsh |
| Port, agent, X11 or socket forwarding; subsystems; file transfer; passing environment variables | Each is a channel the role cannot see. File transfer will be its own, recorded interface (a later version) |
| Compression | Compression leaks what it compresses |
| Rekeying, resumption, a session id chosen by the client | Each would be a second state machine. Sessions are bounded instead |
| Frames that are oversize, trailing, short, unknown or out of order, or that carry a nonzero reserved field | A parser that accepts these is a parser an attacker can steer |

## Logins and roles

| Refused | Why |
|---|---|
| A root shell, or a login with no role | Every login is a capsule holding only what its role names |
| `Admin`, `Debug`, `DeviceSecret`, `EnrolDevRoot`, `LocalSign`, `Keyring`, `CameraCapture`, `MicrophoneCapture` or `ScreenCapture` in any role | A login must never be able to mint authority, reach the machine's own secrets, or see or hear what is in front of the machine |
| Widening a running capsule | An escalation is a new capsule with an expiry, so the base capsule stays as it was |
| Escalating an escalation | `parent_id` must be the base grant |
| An escalation longer than an hour | Elevated power must end by itself |
| An approver who is also the subject or the issuer | The second person must be a different person |
| A revocation list older than the stated age | A host that cannot hear about revocations must not admit anyone |

## Host side

| Refused | Why |
|---|---|
| `nsh.attest` holding Network | The kernel refuses `MkAttestDoc` to a Network holder (`can_attest_doc`). nsh keeps that rule and splits the attester from the front |
| A session capsule holding Network unless its role names it | A login that cannot reach the network cannot send data off the host |
| More than 16 concurrent sessions (configurable) | Sessions use fixed per-pid TCP quotas, `MAX_CONN_PER_PID` = 32 |
