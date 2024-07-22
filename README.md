
# Welcome to the encrypted-scholarship wiki

encrypted-scholarship is a project aimed at executing all processes related to scholarship applications in an encrypted manner. This allows for the protection of privacy by enabling scholarship applications without providing personal information to trusted third parties, as is the case with traditional scholarship applications. Additionally, the scholarship application process, which usually takes 1-2 months, can be completed in less than a day.

# Directory Structure

## /doc

This directory contains documents explaining the concept and architecture of encrypted-scholarship. Please start by reading here.

## /hardhat

This directory contains the smart contracts used internally by encrypted-scholarship. You can test them using Hardhat.

```shell
$cd hardhat
$npx hardhat compile
$npx hardhat test
$npx hardhat node
$npx hardhat ignition deploy ./ignition/modules/Lock.ts --network localhost
```

## /src

Most of encrypted-scholarship is implemented in Rust.

- Interaction with smart contracts is done using [ethers-rs](https://github.com/gakonst/ethers-rs).
- FHE processing is implemented based on TFHEzama's [tfhe-rs](https://github.com/zama-ai/tfhe-rs).
- Future implementations of TLS Notary processing will also be done in Rust.

```shell
    $cd src
    $cargo test
    $cargo run
```

>This repository implements an MVP
>
> - Only the minimum required bank balance is reviewed.
> - Retrieval of bank balance via TLS Notary is not implemented.
> - Smart contracts only operate on the local network.
> - Some type conversions are not implemented and are hardcoded for convenience.
