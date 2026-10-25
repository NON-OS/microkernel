# nsh-enrol

The fleet operator's tool for nsh/1 enrolment certificates (`nsh-enrol/1`,
`docs/design/nsh-v1-attest.md`). Run it on the operator's own machine. No
image or capsule builds it.

```
nsh-enrol keygen --out DIR
nsh-enrol challenge --host-keys FILE
nsh-enrol body --doc DOC --nonce HEX --ak HEX --host-keys FILE --host NAME \
    --fleet HEX --loader HEX [--loader HEX ...] [--boot-app HEX ...] \
    --days N --authority FILE --out FILE
nsh-enrol sign --body FILE --expect HEX --key FILE --out FILE
nsh-enrol show --cert FILE --authority FILE
nsh-enrol inspect --doc FILE | --serial FILE [--challenge HEX]
```

- **`keygen`** writes a new fleet authority key to `authority.secret`
  (mode 0600) and `authority.public`, and prints its key id. Keep the
  secret offline.
- **`challenge`** draws a fresh nonce and prints it with the challenge for
  those host keys: SHA-256 of a label, the nonce and the host keys' digest.
  Ask the machine for a document over that challenge.
- **`body`** takes the machine's `NONOSATT` version 5 document and runs the
  client's own checks on it:
  - the document parses;
  - the quote's binding and signature are correct;
  - the event log replays to PCRs 0, 1, 2, 4 and 7.

  The document must answer the challenge for `--nonce` and these host
  keys, so it is fresh and names them. It must also be quoted by the AK
  given (`--ak`, x then y). Exactly one `--loader` digest must be in PCR 4.
  Every other application there must be one the operator expects
  (`--boot-app`), and becomes a boot application, each once, in the order
  it ran; there can be at most eight.

  The output is every field of the certificate before the signature. It is
  shown, with its SHA-256.
- **`sign`** signs only the body whose SHA-256 is `--expect`, and only
  while it has not run out. It uses both halves of the authority key, then
  checks the certificate as the client will: it must parse, name this
  authority, and verify. It shows the certificate, then writes it.
- **`show`** checks a certificate against an authority's public key and
  prints its fields.
- **`inspect`** checks a document as far as it can be without an
  enrolment, and describes it. `--serial` reads the one a development
  image prints after its boot.

A public key, an authority's or a host's, is the Ed25519 key then the
ML-DSA-65 key: 1984 bytes. A file is written whole or not at all, and
never over an existing one.

## What it does not do

It does not show that the AK lives in the machine's TPM. That is the
registrar's job: MakeCredential to the machine's EK, answered by
`src/security/tpm/enroll` (`activate_credential`). Pass this tool only an
AK the registrar activated.

The tests are in `userland/nsh_attest_proofs` (`tests/tool`), against
documents built there, and in `tests/` for the command line and the
serial reader.
