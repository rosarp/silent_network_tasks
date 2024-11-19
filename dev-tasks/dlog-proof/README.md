# DLOG Proof Implementation

A Rust implementation of the Non-interactive Schnorr Zero-Knowledge Discrete Logarithm (DLOG) Proof scheme with Fiat-Shamir transformation.

## Features

- Generates secure random scalar values using `k256` cryptographic primitives
- Provides both proof generation and verification

## Usage

```bash
cargo run
```

Example output shows:
- Random scalar value (x)
- Proof computation time
- Proof coordinates (x, y)
- Verification result and computation time

### Note
The provided python implementation was not compiling due to missing dependency 'htss_ecdsa'.
The serialization constructs used from this library were replaced with suitable alternatives for the purpose of testing the python implementation.
The output of the python implementation is the same as the output of the rust implementation.