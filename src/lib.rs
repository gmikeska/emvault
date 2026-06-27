//! # asterism
//!
//! Umbrella **facade** for the Emerald multi-signature custody platform. This
//! crate owns no logic of its own; it re-exports [`asterism-core`](asterism_core)
//! and the feature-gated signer/network backends under stable namespaces so a
//! consuming application can depend on a single crate (`asterism`) as its public
//! API surface instead of reaching into the individual sub-crates.
//!
//! ## Feature gates
//!
//! Backends are **optional dependencies behind features** so that a build pulls
//! in exactly what it uses and nothing more (a Bitcoin-only app must not compile
//! the HSM/Elements dependency stack):
//!
//! - `xpub` — consumer hardware-wallet signer ([`xpub`]).
//! - `pkcs11` — HSM-backed signer ([`pkcs11`]).
//! - `elements` — Elements/Liquid support ([`elements`]); also forwards the
//!   `elements` feature into `pkcs11` when that backend is present.
//! - `dev-signer` — dev/CI HSM helper ([`dev_signer`]); implies `pkcs11`.
//! - `test-utils` — exposes test scaffolding (`MockSigner`, fixtures, the
//!   Elements `testkit`) to downstream test suites.
//!
//! [`core`] is always available. Example dependency lines:
//!
//! ```toml
//! # Bitcoin-only consumer hardware wallets:
//! asterism = { path = "../asterism", features = ["xpub"] }
//! # HSM federation with Elements + dev helpers:
//! asterism = { path = "../asterism", features = ["pkcs11", "elements", "dev-signer"] }
//! ```
//!
//! ## Why namespaced modules (not a flat glob)
//!
//! `asterism-core` and `asterism-elements` both expose `descriptor`, `error`,
//! `network`, and `federated_wallet` modules and overlapping item names
//! (`to_multipath_string`, `NetworkType` vs `ElementsNetwork`, `KeyMode` vs
//! `CtKeyMode`). A flat `pub use ::*` of every backend would collide, so each
//! crate keeps its own namespace here. The [`prelude`] gathers the
//! high-frequency, unambiguous types for the common case.

#![forbid(unsafe_code)]

/// Framework-agnostic configuration + hex helpers shared by consuming apps
/// ([`ConfigError`](config::ConfigError), [`require`](config::require),
/// [`optional`](config::optional), [`hex_decode`](config::hex_decode),
/// [`hex_encode`](config::hex_encode)). Always available — no backend feature.
pub mod config;

/// Backend-agnostic core: [`Federation`](asterism_core::Federation),
/// [`SigningCoordinator`](asterism_core::SigningCoordinator), descriptors,
/// chain-sync (`chain_sync`), the PSBT primitives (`psbt`), migration,
/// recovery, and snapshot. Always available.
pub mod core {
    pub use asterism_core::*;
}

/// XPUB signer backend for consumer hardware wallets (Trezor, Jade, Ledger, …).
#[cfg(feature = "xpub")]
pub mod xpub {
    pub use asterism_xpub::*;
}

/// PKCS#11 / HSM signer backend.
#[cfg(feature = "pkcs11")]
pub mod pkcs11 {
    pub use asterism_pkcs11::*;
}

/// Elements/Liquid support: confidential descriptors, PSET, wollet, LWK sync.
#[cfg(feature = "elements")]
pub mod elements {
    pub use asterism_elements::*;
}

/// Dev/CI HSM helper that pairs with `libasterism_dev_hsm`.
#[cfg(feature = "dev-signer")]
pub mod dev_signer {
    pub use asterism_dev_signer::*;
}

/// One import for the types most consumers touch. Backend-specific entries are
/// feature-gated to match the enabled namespaces.
pub mod prelude {
    pub use asterism_core::{
        BtcFederatedWallet, BuiltFederation, DescriptorBuilder, FederatedWallet, Federation,
        FinalizedPsbt, NetworkType, SigningAction, SigningCoordinator, SigningRequest,
        UnsignedPsbt, build_federation,
    };

    #[cfg(feature = "elements")]
    pub use asterism_elements::{ElementsSigner, ElementsWollet};
    #[cfg(feature = "pkcs11")]
    pub use asterism_pkcs11::{Pkcs11Config, Pkcs11Signer};
    #[cfg(feature = "xpub")]
    pub use asterism_xpub::ExternalSigner;
}
