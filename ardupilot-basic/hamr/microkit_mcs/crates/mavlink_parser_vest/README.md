# MAVLink parser using upstream Vest

Both this crate and the MAVLink firewall use the official `vest_lib` crate from
https://github.com/secure-foundations/vest, pinned to commit
`e0671fbbec7c7a4daeb5581fd12d21ec89ec005c` (Vest 0.2.0).
No library source is vendored or patched locally.

Regenerate from this directory using the matching official compiler:

```sh
cargo +1.97.1 install --git https://github.com/secure-foundations/vest \
  --rev e0671fbbec7c7a4daeb5581fd12d21ec89ec005c --locked vest
vest src/mavlink.vest -o src/mavlink.rs
```

`src/mavlink.rs` is compiler output. `src/lib.rs` supplies the application's
`parse_mavlink_msg` entry point over upstream's `MavlinkMsgFmt` parser. The grammar
uses the named `Signed` enum variant for the MAVLink 2 signature branch.
The firewall uses upstream's typed message-ID enums and `DeepView` specifications.
Existing acceptance semantics are retained; this grammar does not validate CRCs
or authenticate signatures.

The library uses `default-features = false` and `alloc` for the seL4 target.
The shared crates use `vstd` and `verus_builtin_macros` version
`0.0.0-2026-08-09-0044`; that vstd release uses `verus_builtin` version
`0.0.0-2026-08-09-0044`. Verification requires Verus `0.2026.08.09.92f466f`,
with Rust 1.97.1. Preserve these aligned dependency pins after HAMR regeneration.
