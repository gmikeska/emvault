# How HSMs Integrate with EmVault

EmVault's PKCS#11 backend (`emvault-pkcs11`) speaks the standard PKCS#11 API — it doesn't talk to any HSM directly. The bridge between EmVault and a specific HSM is always a **shared library** (`.so`, `.dylib`, or `.dll`) provided by the HSM manufacturer.

## How it works

- The HSM vendor ships a shared library that implements the PKCS#11 C API for their hardware (e.g., Thales provides `libCryptoki2.so`, Utimaco provides `libcs_pkcs11.so`, etc.)
- EmVault loads this library at runtime via the path you configure
- A thin **wrapper crate** adapts the vendor library into EmVault's `HsmBackend` trait — handling session management, key discovery, and signing

## `libemvault_dev_hsm` as a reference

`libemvault_dev_hsm` is a software-only shared library that mimics a real HSM's PKCS#11 interface using SoftHSM2 under the hood. It exists so you can develop and test HSM-backed federations without physical hardware. `emvault-dev-signer` is its companion wrapper crate.

This pair (`libemvault_dev_hsm` + `emvault-dev-signer`) serves as a working example of the pattern: shared library + wrapper crate.

## Building your own integration

We plan to release official wrapper crates for additional HSM manufacturers over time. But if one doesn't exist yet for your hardware, you can build your own — the pattern is straightforward:

1. Obtain the PKCS#11 shared library from your HSM vendor
2. Write a wrapper crate that implements `HsmBackend` using that library
3. Use `emvault-dev-signer` as your reference implementation

The shared library itself is always your responsibility to source and license from the vendor — EmVault never bundles proprietary HSM libraries.
