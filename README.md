# zk-soroban-examples

JSON files (`verification_key.json`, `proof.json`, etc.) in `data/` are taken from [zk-ton-examples](https://github.com/zk-examples/zk-ton-examples).

Soroban smart contract verifier for **Groth16** zero-knowledge proofs.  
It supports the **snarkjs-compatible JSON** format (e.g. `verification_key.json`) produced by:

- [Circom](https://docs.circom.io/)
- [arkworks](https://github.com/arkworks-rs) via [ark-snarkjs](https://github.com/mysteryon88/ark-snarkjs)
- [gnark](https://github.com/Consensys/gnark) via [gnark-to-snarkjs](https://github.com/mysteryon88/gnark-to-snarkjs)
- [Noname](https://github.com/zksecurity/noname) - [Article about integration with SnarkJS](https://blog.zksecurity.xyz/posts/noname-r1cs/)

Provide `verification_key.json` (and optionally `proof.json` / `public.json`), then generate and deploy the Soroban verifier contract.

## Usage

```sh
stellar contract init soroban-hello-world
cd soroban-hello-world

cargo install soroban-verifier-gen
soroban-verifier-gen --vk data/circom/verification_key.json --out contracts/circom_verifier
soroban-verifier-gen --vk data/gnark/verification_key.json --out contracts/gnark_verifier
soroban-verifier-gen --vk data/arkworks/verification_key.json --out contracts/ark_verifier --crate-name ark_verifier
```
