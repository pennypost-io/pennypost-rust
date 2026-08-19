pennypost — official Rust SDK for [PennyPost](https://pennypost.io).

```rust
let pp = pennypost::PennyPost::new("pp_live_<secret>");
pp.send_email(&req, None)?;
```

Also included: batch send (up to 100 messages in one request), cc/bcc, log filters by recipient, domain, and API key, suppressions, the full Marketing Emails surface (audiences, contacts with custom properties, broadcasts with markdown drafts, send-time filters, and test sends), idempotency keys, and typed errors.
