# capsule_payment

## Role

`capsule_payment` is the userland payment authority. It runs as a CPL=3
capsule and issues signed NOX install receipts: a caller asks it to settle
an install, the capsule records the charge against a monotonic nonce, and
it has the keyring sign an EIP-712 receipt and returns the receipt's struct
hash. It owns the payment nonce and the pending outbox of signed receipts.
The install path it serves is described in
[docs/handbook/apps/market-and-store.md](../../docs/handbook/apps/market-and-store.md).

```text
installer / marketplace client
        |
        | OP_PAY / OP_DRAIN_RECEIPTS / OP_LIST_TOKENS
        v
capsule_payment -- MkIpcCall --> keyring (secp256k1 receipt signature)
        |
        `-- nonce + outbox state
```

## Microkernel contract

```text
CAPSULE_REQUIRED_CAPS = 0x18
```

IPC and Memory. Service `service:4114:payment`; replies go to the kernel
reply endpoint. The capsule resolves the keyring with `MkServiceLookup`,
requests receipt signatures with `MkIpcCall`, receives with `MkIpcRecv`,
answers with `MkIpcSend`, and reads the clock with `MkTimeMillis`. It requests
no hardware grants.

No kernel profile turns on `nonos-capsule-payment`: the capsule is built,
signed and enrolled, but no image carries it, so the installer's paid path
answers `EAGAIN` on every image.

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `OP_HEALTHCHECK` (1) | none | status 0 |
| `OP_PAY` (2) | owner pid, wallet id, capsule id, publisher address, amount, receipt type | the signed receipt's 32-byte struct hash |
| `OP_DRAIN_RECEIPTS` (3) | none | a count and up to 13 pending 297-byte receipt records, taken from the outbox |
| `OP_LIST_TOKENS` (4) | none | supported wallet/payment assets |

Unknown operations and malformed bodies reply `EINVAL`. `OP_PAY` answers
`EAGAIN` when no keyring runs or the outbox holds its 1024 records already.

## Authority

The capsule may talk to the keyring over IPC. It has no PCI, MMIO, IRQ,
DMA, PIO, network, display, or focus-routing authority. It never moves
funds itself.

## Limits

The keyring signs a receipt only when the owner pid in the request is the pid
the kernel stamped on the message (`resolve_caller`), and only with a key that
pid owns. `OP_PAY` forwards the wallet owner's pid while the kernel stamps the
payment capsule's own, so the keyring refuses the signature with `EACCES`
and `OP_PAY` returns that status.

## Privacy and persistence

The nonce and outbox live in capsule memory for the life of the boot.
A receipt record carries the capsule id, publisher address, amount, nonce,
epoch, expiry and receipt type, the paying account's address and the
keyring's signature.

## Token registry

`OP_LIST_TOKENS` returns the payment capsule's asset registry as a compact
binary payload:

```text
u32 count
repeat count:
  u8  symbol_len
  u8  decimals
  u16 settlement_kind
  u32 flags
  u64 chain_id
  u8  contract_address[20]
  u8  symbol[symbol_len]
```

Settlement kinds:

| Value | Meaning |
|---:|---|
| 1 | Native ETH |
| 2 | NOX receipt settlement |
| 3 | Primer x402 settlement |

Flags:

| Bit | Meaning |
|---:|---|
| `1 << 0` | enabled |
| `1 << 1` | native token |
| `1 << 2` | ERC-20 token |
| `1 << 3` | settled by local receipt |
| `1 << 4` | settled by x402/Primer rail |
| `1 << 5` | contract/configuration required before live settlement |

Current built-ins:

| Symbol | Chain | Contract | Status |
|---|---:|---|---|
| ETH | 1 | native | enabled |
| NOX | 1 | `0x0a26c80Be4E060e688d7C23aDdB92cBb5D2C9eCA` | enabled |
| PR | 8453 | pending | reserved for Primer x402; requires verified contract/config |
