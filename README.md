# erc-nft-metadata

Type definitions for ERC-based non-fungible token (NFT) metadata.

While [EIP-721](https://eips.ethereum.org/EIPS/eip-721#specification) defines a "ERC-721 Metadata JSON Schema", in practice
it is rarely used. Instead, this crate implements the more popular [OpenSea metadata standard](https://docs.opensea.io/docs/metadata-standards).

This crate does not attempt to perform validation more than what is strictly necessary. Since every secondary
market will use the fields in the metadata in a different way, it is up to the crate consumer to make sure the fields are appropriately populated.

License: MIT OR Apache-2.0
