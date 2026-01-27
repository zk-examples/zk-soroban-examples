# zk-soroban-examples

JSON files (`verification_key.json`, `proof.json`, etc.) in `data/` are taken from [zk-ton-examples](https://github.com/zk-examples/zk-ton-examples).

## Usage

```sh
stellar contract init soroban-hello-world
cd soroban-hello-world

cargo install soroban-verifier-gen
soroban-verifier-gen --vk data/circom/verification_key.json --out contracts/circom_verifier
soroban-verifier-gen --vk data/gnark/verification_key.json --out contracts/gnark_verifier
soroban-verifier-gen --vk data/arkworks/verification_key.json --out contracts/arkworks_verifier --crate-name arkworks_verifier
```
