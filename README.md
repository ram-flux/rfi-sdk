# Ram Flux: Secure and Private Instant Messaging with Rust

Ram Flux is an open-source, anonymous instant messaging project built using the Rust programming language. The project aims to provide users with a secure and private communication experience. Ram Flux is licensed under the GPLv3, encouraging community participation and contributions.

## Client SDK Implementation

In the client SDK implementation, Ram Flux utilizes the BIP39 mnemonic phrase algorithm to generate easy-to-remember mnemonic phrases, enabling users to backup and restore their keys conveniently. By leveraging the hierarchical deterministic (HD) derivation paths defined in BIP44, the mnemonic phrase is used to derive a seed. This seed is then employed to generate a secp256k1 curve private key, from which a public key is derived to serve as the account identifier. This approach ensures security while maintaining usability and recoverability.

## Current Development Status

The Ram Flux project is currently in its early development stage, with several key features such as client-server authentication and message encryption still being designed and implemented. We are committed to maintaining transparency and openness by sharing progress updates and technical details with the community. We welcome feedback and suggestions from all parties as we strive to create a decentralized, privacy-focused instant messaging platform.

## Minimum Supported Rust Version (MSRV) Policy

Ram Flux adheres to a Minimum Supported Rust Version (MSRV) policy. Any changes to the MSRV are considered breaking changes and will only be performed with a minor version bump. This policy ensures compatibility and stability for users and contributors.

## Get Involved

As an open-source project, Ram Flux warmly invites developers to contribute their expertise and efforts to drive the project's growth and advancement. Together, let's explore the new paradigm of privacy communication in the Web3 era and deliver a more secure, private, and user-friendly experience to our users.
