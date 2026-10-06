# keelrs

The Rust SDK for [keel](https://github.com/mvaleed/keel).

Keel is a durable execution engine. One `keeld` process owns each
invocation's journal and calls your worker over one WebSocket. This
crate writes that protocol for you, so a Rust worker speaks to the
engine and a Rust client submits invocations without raw HTTP code.

## Status

Early work. The crate has no public API yet. The first release adds:

- A client that submits, reads, lists, and cancels invocations.
- A worker that registers itself, answers the invoke connection, and
  replays a journal on resume.

## The engine

Keel needs one static binary and one S3-compatible bucket. See the
[engine repository](https://github.com/mvaleed/keel) for the quick
start and the [HTTP API](https://github.com/mvaleed/keel/blob/main/docs/http-api.md)
that this crate wraps.

## License

Apache 2.0. See [LICENSE](LICENSE).
