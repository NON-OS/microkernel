# nonos_http

`nonos_http` is the HTTP/1.1 client framing the terminal (`git` over HTTPS and its `nox http` commands) and
the model fetcher use. Requests are built into bytes and responses parsed from
bytes, so the same code runs over a TLS session in a capsule and over a buffer
in a test. It opens no socket and has no dependencies. It is `no_std`.

## What it offers

- `parse_url` takes `https://host/path` only, at most 2048 bytes. Plain HTTP is
  refused, since anyone on the path could serve the response.
- `RequestBuilder::get` and `post` build a request; `user_agent` and `accept`
  set those fields.
- `fetch` sends a request over any `Stream` (a socket, a TLS session or a test
  buffer) and reads the response until the stream closes, refusing before the
  body is read a response that states more than the caller's `limit`.
- `parse_response` parses a whole response from bytes into a `Response` with
  its status, fields and body.

## What a response must be

- The head, status line and fields together, is at most 64 KiB; a head with no
  blank line within that is too large, not unfinished. At most 128 fields.
- The status line is `HTTP/1.x`, a space, three digits from 100 to 599, then a
  space or the line end (RFC 9112 4).
- A field name is a token with no whitespace before the colon (RFC 9110 5.1,
  RFC 9112 5.1). Names are lowercased once.
- The body is framed as RFC 9112 6.3 says (`src/response/framing.rs`): when the
  last Transfer-Encoding coding is chunked, the chunks frame it; when codings
  are listed and chunked is not last, it runs to the close. Content-Length is
  digits only, and every value it states must agree, since two lengths are two
  framings of one stream.
- Interim (1xx) responses before the final one are skipped, at most eight of them;
  more is taken as a loop, not a reply.
- Chunked bodies: each chunk is a hex size, optional whitespace and
  extensions, CRLF, the bytes and a CRLF that is checked, not skipped. A zero
  size ends the body and trailers are ignored.

Errors are `HttpError`: a truncated head, a bad status line, a bad field, a
body length that cannot be determined or did not arrive, a bad chunk size, a
body over the limit, or a stream failure.

## What it does not do

- No HTTP/2, no keep-alive or pipelining: one request per stream, read to the
  close.
- No content decoding: a gzip body is returned as sent.
- No redirects or cookies; callers follow redirects themselves.
- No TLS: the caller hands it a stream, usually from `nonos_tls`.

## Tests

`userland/http_proofs` includes the crate's sources on the host and runs the
framing, status line, field, chunk and head-size rules.

See [the network stack](../../docs/handbook/network/stack.md).
