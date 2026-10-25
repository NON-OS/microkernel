# nonos_eth_rpc

Ethereum JSON-RPC as bytes for `no_std` capsules, with no JSON crate and no
syscalls. capsule_nox_settle (B4) links it.

- Requests: eth_chainId, eth_getBalance, eth_getTransactionCount,
  eth_gasPrice and eth_feeHistory, eth_estimateGas, eth_call,
  eth_sendRawTransaction, eth_getTransactionReceipt, eth_blockNumber, and
  eth_getLogs by one address and one topic0 over a block range
  (`request_logs`). The node never learns which id the reader filters for. Batches by `request_batch`, answers found by `object_for_id`.
- Answers: `http_answer` reads the HTTP framing strictly (a 200 and a whole
  body, chunks joined); the `parse_*` readers take results; `parse_logs`
  reads a page of logs, drops reorged ones, and refuses the page if any log
  does not read whole.
- `http_post` frames a request for a host.

Taken from the wallet's own rpc module (`capsule_wallet_nonos/src/wallet/rpc`,
B owns both); the wallet moves onto this crate in B8. Logs, quantities and
the JSON scanning are new.

```sh
cargo test
cargo build --release -Zbuild-std=core,alloc --target ../x86_64-nonos-user.json
```
