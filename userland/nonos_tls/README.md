# nonos_tls

`nonos_tls` is the TLS 1.3 client every NONOS capsule that speaks HTTPS uses:
the browser, the wallet, the terminal, the model fetcher, the Linux personality
and the two privacy transports (`net.nym` for its directory fetch, `net.anon`
for its relay links). It was lifted out of the browser so there is one
certificate check, not one per capsule. It is `no_std`.

## What runs where

- In the caller: the transcript hash (SHA-256 and SHA-384), HKDF and the key
  schedule, and the record ciphers, ChaCha20-Poly1305 and AES-128-GCM. These
  touch the traffic secrets, which already live in the caller's memory.
- In `crypto_pool`: the X25519 agreement and every signature check (RSA-PSS,
  RSA PKCS#1 v1.5 and ECDSA). A P-256 ECDHE share, for servers that refuse
  X25519, is computed in the caller with `p256`.

## The handshake

- Cipher suites `TLS_CHACHA20_POLY1305_SHA256` and `TLS_AES_128_GCM_SHA256`;
  groups X25519 and secp256r1; SNI; a HelloRetryRequest is answered.
- Signature schemes advertised are only those the verifier handles:
  `rsa_pss_rsae_sha256` and `_sha384`, `ecdsa_secp256r1_sha256`,
  `ecdsa_secp384r1_sha384` and `rsa_pkcs1_sha256`.
- `verify_chain` (`src/chain_walk.rs`) checks that the leaf names the host and
  is inside its validity window, that each certificate is signed by the next,
  and that the top one is a pinned root or is signed by a root found by its
  issuer. It walks at most `MAX_CHAIN`, ten, certificates and fails closed.
  The roots are the Mozilla set from `nonos-data/cacert.pem`, embedded in
  `src/roots/`.
- Host names are matched against `subjectAltName` read as the extension it
  is, not as a byte pattern (`cert_dns_match`, exported as
  `cert_names_host` so the browser's own walk uses the same rule).
- One Certificate message per flight, and none after CertificateVerify.

## Records

Protected records are refused past the RFC 8446 limits: a body over
2^14 + 256 bytes, or a plaintext over 2^14 + 1. One such record ends the
session. Alerts are named by their standard description (`alert_name`).

## The API

`exchange` runs a whole request and response over a caller's `Io`;
`HandshakeState` drives the handshake one step at a time for callers that own
their own loop; `AppReader`, `application_write`, `application_request` and
`application_plaintext` carry application data once the keys are in place.
`rtc_now` gives the time certificates are checked against.

## What it does not do

- TLS 1.3 only: no TLS 1.2 or earlier, no session resumption, no 0-RTT, no
  client certificates.
- No `rsa_pss_sha512`: the RSA verify path wires SHA-256 and SHA-384 only.
- No revocation checks (OCSP or CRL).
- No KeyUpdate: one from the server ends the session. A session ticket is
  read past and not used.
- `server_complete_unauthenticated` and `stream::connect_unauthenticated`
  complete a handshake without walking the chain, for a caller that checks the
  peer another way: `net.anon` uses it for relay links, which it binds to the
  relay's Ed25519 identity through the CERTS cell.

## Tests

`userland/tls_proofs` includes the crate's sources on the host, with a stand-in
libc that answers crypto calls with the crypto capsule's own handlers. It runs
the RFC 8448 handshake trace, bytes captured from a live Anyone relay, the chain
a re-signing gateway served, a played server, and the record, chain, name and
flight-order rules. `.github/workflows/fuzz.yml` fuzzes it.

See [the network stack](../../docs/handbook/network/stack.md) and
[the browser](../../docs/handbook/apps/browser.md).
