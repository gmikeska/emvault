![Asterism](https://github.com/gmikeska/asterism/blob/master/asterism.jpg?raw=true)
# Asterism

Wrapper crate for the Asterism multi-signature custody architecture.

It provides a unified, backend-agnostic foundation for building secure m-of-n federations that can mix consumer hardware wallets (Trezor, Ledger, etc.. any signer that can export a BIP-48 XPUB and sign a P2WSH sortedmulti PSBT) with HSMs (via PKCS#11) in the same federation. Asterism handles descriptor construction, heterogeneous signing coordination, federation mutation, efficient multi-account migrations, recovery templates, and policy enforcement — while staying a pure library with no runtime or network ownership.

It also carries the handful of framework-agnostic utilities that every app needs but that don't belong in `asterism-core` (env-var + hex helpers).

## The library family

| Crate | Role | Repository |
| ----- | ---- | ---------- |
| `asterism-core` | Backend-agnostic core: `Signer` trait, `Federation`, descriptor builder, PSBT pipeline, migration, recovery, snapshots, roster, the shared wallet view types. | <https://github.com/gmikeska/asterism-core> |
| `asterism-xpub` | XPUB signer backend for consumer hardware wallets (Trezor, Jade, Ledger, Coldcard, Passport). | <https://github.com/gmikeska/asterism-xpub> |
| `asterism-pkcs11` | PKCS#11 / HSM signer backend. | <https://github.com/gmikeska/asterism-pkcs11> |
| `asterism-elements` | Elements/Liquid support: confidential descriptors, PSET pipeline, client-side wollet, daemon RPC. | <https://github.com/gmikeska/asterism-elements> |
| `asterism-dev-signer` | Dev/CI `HsmBackend` that pairs with `libasterism_dev_hsm`. | <https://github.com/gmikeska/asterism-dev-signer> |

## Why namespaced modules (not a flat glob)

`asterism-core` and `asterism-elements` both expose `descriptor`, `error`,
`network`, and `federated_wallet` modules and overlapping item names
(`to_multipath_string`, `NetworkType` vs `ElementsNetwork`, `KeyMode` vs
`CtKeyMode`). A flat `pub use ::*` of every backend would collide, so each crate
keeps its own namespace here, and the [`prelude`](#prelude) gathers the
high-frequency, unambiguous types for the common case.

| Namespace | Always on? | Backed by |
| --------- | ---------- | --------- |
| `asterism::core` | yes | [`asterism-core`](https://github.com/gmikeska/asterism-core) |
| `asterism::config` | yes | this crate (env-var + hex helpers) |
| `asterism::xpub` | `xpub` feature | [`asterism-xpub`](https://github.com/gmikeska/asterism-xpub) |
| `asterism::pkcs11` | `pkcs11` feature | [`asterism-pkcs11`](https://github.com/gmikeska/asterism-pkcs11) |
| `asterism::elements` | `elements` feature | [`asterism-elements`](https://github.com/gmikeska/asterism-elements) |
| `asterism::dev_signer` | `dev-signer` feature | [`asterism-dev-signer`](https://github.com/gmikeska/asterism-dev-signer) |
| `asterism::prelude` | yes | curated re-exports (feature-gated entries) |

## Feature gates

Backends are **optional dependencies behind features** so a build pulls in
exactly what it uses and nothing more (a Bitcoin-only app must not compile the
HSM/Elements dependency stack):

| Feature | Effect |
| ------- | ------ |
| `xpub` | Consumer hardware-wallet signer (`asterism::xpub`). |
| `pkcs11` | HSM-backed signer (`asterism::pkcs11`). |
| `elements` | Elements/Liquid support (`asterism::elements`); turns on `asterism-core/elements` and forwards the Elements feature into `pkcs11` **only if** that backend is also enabled (weak `?/` dep). |
| `dev-signer` | Dev/CI HSM helper (`asterism::dev_signer`); implies `pkcs11`. |
| `test-utils` | Exposes test scaffolding (`MockSigner`, fixtures, the Elements `testkit`) to downstream test suites. |

`asterism::core` and `asterism::config` are always available.

## Usage

```toml
# Bitcoin-only consumer hardware wallets:
asterism = { git = "https://github.com/gmikeska/asterism", features = ["xpub"] }

# HSM federation with Elements + dev helpers:
asterism = { git = "https://github.com/gmikeska/asterism", features = ["pkcs11", "elements", "dev-signer"] }
```

### Prelude

One import for the types most consumers touch (backend-specific entries are
feature-gated to match the enabled namespaces):

```rust,ignore
use asterism::prelude::*;
// Federation, DescriptorBuilder, build_federation, BuiltFederation,
// SigningCoordinator, UnsignedPsbt, FinalizedPsbt, NetworkType, …
// + ExternalSigner (xpub), Pkcs11Signer (pkcs11), ElementsWollet (elements)
```

### `asterism::config`

Framework-agnostic helpers shared by every consuming app — always available, no
backend feature needed:

```rust,ignore
use asterism::config::{require, optional, hex_decode, hex_encode, ConfigError};

let secret = hex_decode(&require("APP_SESSION_SECRET")?)?;
let manifest = optional("TREZOR_MANIFEST_EMAIL");
# Ok::<(), asterism::config::ConfigError>(())
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
