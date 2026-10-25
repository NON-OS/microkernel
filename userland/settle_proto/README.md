# nonos_settle_proto

The wire of `capsule_nox_settle` (service `nox_settle`, the plan and the plan),
shared by settle and its callers, the meter first. `no_std`, alloc only.

- Header: market_proto's 20 bytes under the magic "NSTL". A reply's payload
  starts with a status, and `reply_body` believes it only for its own call.
- Ops:
  - 1 HEALTHCHECK;
  - 2 BROADCAST: chain_id | raw signed tx -> its hash;
  - 3 is retired: READ's view 14 (openNonce) replaces READ_NONCE; EINVAL;
  - 4 CHANNEL_STATE (`ChannelState`), settled read from two hosts;
  - 5 GAS (`Gas`): balance, next nonce, fee suggestions;
  - 6 OPENED (`Opened`): a transaction's ChannelOpened event, EAGAIN while
    unmined, ENOENT without one. `Opened::from_log` reads the event, its
    topic pinned against its signature;
  - 7 READ (`Read`, `read_reply`): a view by number from `VIEWS`. The caller
    never names `to` or calldata; `view_calldata` holds each argument word to
    its form (an address, a uint8, a uint64, a word);
  - 8 INTENT (`Intent`, `Quote`): `kind_of` checks the calldata's selector
    belongs to one of `KINDS` (1 Approve to 17 WithdrawCredit), and
    `approve_spender` reads an approve's spender. `padded` is the estimate
    times 6/5;
  - 9 FETCH (`Fetch`, `Page`): 0 RELEASE (an https url, `https_target`, and
    the releaseHash), 2 PAGE (the releaseHash and an offset), 1 ESCROW (B5).
    A page is at most 60 KiB, a release at most 1 MiB.
- Statuses: the Linux errnos settle answers, EIO for two hosts that disagree
  and ENETDOWN for a route that is down (settle never goes Direct).
- Every view's and every kind's selector is pinned by a test against its
  signature. A signature's `T` is OrderTerms' tuple and its `V` is Voucher's,
  which a test pins against nonos_nox_config's Voucher type.

```sh
cargo test
```
