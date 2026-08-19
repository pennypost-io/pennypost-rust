pennypost — official Rust SDK for [PennyPost](https://pennypost.io).

```rust
let pp = pennypost::PennyPost::new("pp_live_<secret>");
pp.send_email(&req, None)?;
```
