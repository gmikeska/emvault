![EmVault](https://raw.githubusercontent.com/gmikeska/emvault/refs/heads/master/emvault.jpg?raw=true)
# EmVault

Wrapper crate for the EmVault multi-signature custody architecture.

It provides a unified, backend-agnostic foundation for building secure m-of-n federations that can mix consumer hardware wallets (Trezor, Ledger, etc.. any signer that can export a BIP-48 XPUB and sign a P2WSH sortedmulti PSBT) with HSMs (via PKCS#11) in the same federation. EmVault handles descriptor construction, heterogeneous signing coordination, federation mutation, efficient multi-account migrations, recovery templates, and policy enforcement — while staying a pure library with no runtime or network ownership.

It also carries the handful of framework-agnostic utilities that every app needs but that don't belong in `emvault-core` (env-var + hex helpers).

> [!WARNING]
> ## 🚧 Under Active Development — Not Ready for Use 🚧
>
> **This crate is a work in progress and is NOT yet ready for production or general use.**
> APIs, features, and behavior may change without notice, and breakage is expected.
>
>
> ⛔️ **Feel free to play around with the crate but if you need to store real money please come back soon.**


## 📦 Demo Implementations

Want to see `emvault` wired into a real application? Two reference web apps
exercise the library end-to-end and are the fastest way to learn the API:

- **[test-app-xpub](https://github.com/gmikeska/test-app-xpub)** — self-custody,
  multi-party signing. Users bring their own hardware wallet, onboard an XPUB,
  and co-sign m-of-n P2WSH transactions through a proposal lifecycle.

- **[test-app-pkcs11](https://github.com/gmikeska/test-app-pkcs11)** — custodial
  HSM-backed wallets. A federation of HSMs signs every spend server-side, across
  both Bitcoin and Elements/Liquid.

Each repo ships a `FEATURES.md` that maps every capability to the source symbol
that implements it — start there.


## The library family

| Crate | Role | Repository |
| ----- | ---- | ---------- |
| `emvault-core` | Backend-agnostic core: `Signer` trait, `Federation`, descriptor builder, PSBT pipeline, migration, recovery, snapshots, roster, the shared wallet view types. | <https://github.com/gmikeska/emvault-core> |
| `emvault-xpub` | XPUB signer backend for consumer hardware wallets (Trezor, Jade, Ledger, Coldcard, Passport). | <https://github.com/gmikeska/emvault-xpub> |
| `emvault-pkcs11` | PKCS#11 / HSM signer backend. | <https://github.com/gmikeska/emvault-pkcs11> |
| `emvault-elements` | Elements/Liquid support: confidential descriptors, PSET pipeline, client-side wollet, daemon RPC. | <https://github.com/gmikeska/emvault-elements> |
| `emvault-dev-signer` | Dev/CI `HsmBackend` that pairs with `libemvault_dev_hsm`. | <https://github.com/gmikeska/emvault-dev-signer> |

## Why namespaced modules (not a flat glob)

`emvault-core` and `emvault-elements` both expose `descriptor`, `error`,
`network`, and `federated_wallet` modules and overlapping item names
(`to_multipath_string`, `NetworkType` vs `ElementsNetwork`, `KeyMode` vs
`CtKeyMode`). A flat `pub use ::*` of every backend would collide, so each crate
keeps its own namespace here, and the [`prelude`](#prelude) gathers the
high-frequency, unambiguous types for the common case.

| Namespace | Always on? | Backed by |
| --------- | ---------- | --------- |
| `emvault::core` | yes | [`emvault-core`](https://github.com/gmikeska/emvault-core) |
| `emvault::config` | yes | this crate (env-var + hex helpers) |
| `emvault::xpub` | `xpub` feature | [`emvault-xpub`](https://github.com/gmikeska/emvault-xpub) |
| `emvault::pkcs11` | `pkcs11` feature | [`emvault-pkcs11`](https://github.com/gmikeska/emvault-pkcs11) |
| `emvault::elements` | `elements` feature | [`emvault-elements`](https://github.com/gmikeska/emvault-elements) |
| `emvault::dev_signer` | `dev-signer` feature | [`emvault-dev-signer`](https://github.com/gmikeska/emvault-dev-signer) |
| `emvault::prelude` | yes | curated re-exports (feature-gated entries) |

## Feature gates

Backends are **optional dependencies behind features** so a build pulls in
exactly what it uses and nothing more (a Bitcoin-only app must not compile the
HSM/Elements dependency stack):

| Feature | Effect |
| ------- | ------ |
| `xpub` | Consumer hardware-wallet signer (`emvault::xpub`). |
| `pkcs11` | HSM-backed signer (`emvault::pkcs11`). |
| `elements` | Elements/Liquid support (`emvault::elements`); turns on `emvault-core/elements` and forwards the Elements feature into `pkcs11` **only if** that backend is also enabled (weak `?/` dep). |
| `dev-signer` | Dev/CI HSM helper (`emvault::dev_signer`); implies `pkcs11`. |
| `test-utils` | Exposes test scaffolding (`MockSigner`, fixtures, the Elements `testkit`) to downstream test suites. |

`emvault::core` and `emvault::config` are always available.

## Usage

```toml
# Bitcoin-only consumer hardware wallets:
emvault = { git = "https://github.com/gmikeska/emvault", features = ["xpub"] }

# HSM federation with Elements + dev helpers:
emvault = { git = "https://github.com/gmikeska/emvault", features = ["pkcs11", "elements", "dev-signer"] }
```

### Prelude

One import for the types most consumers touch (backend-specific entries are
feature-gated to match the enabled namespaces):

```rust,ignore
use emvault::prelude::*;
// Federation, DescriptorBuilder, build_federation, BuiltFederation,
// SigningCoordinator, UnsignedPsbt, FinalizedPsbt, NetworkType, …
// + ExternalSigner (xpub), Pkcs11Signer (pkcs11), ElementsWollet (elements)
```

### `emvault::config`

Framework-agnostic helpers shared by every consuming app — always available, no
backend feature needed:

```rust,ignore
use emvault::config::{require, optional, hex_decode, hex_encode, ConfigError};

let secret = hex_decode(&require("APP_SESSION_SECRET")?)?;
let manifest = optional("TREZOR_MANIFEST_EMAIL");
# Ok::<(), emvault::config::ConfigError>(())
```

## Build and test

```sh
cargo build                       # core + config only
cargo build --all-features        # every backend
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings -W clippy::pedantic
cargo doc --no-deps --all-features
```

## License

MIT OR Apache-2.0.
