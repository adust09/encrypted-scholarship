use ethers::prelude::*;
use eyre::Result;
use std::sync::Arc;

use keccak_hash::keccak;
use rand::rngs::OsRng;
use rand::RngCore;
use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint32};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default().build();

    // Key generation
    // Client key: used for encryption and decryption of data. This key must be kept secret.
    // Server key (or Evaluation key): used for performing operations on encrypted data. This key could be public.
    let (client_key, server_keys) = generate_keys(config);

    let clear_a = 1344u32;
    let clear_b = 5u32;

    // Encrypting the input data using the (private) client_key
    // FheUint32: Encrypted equivalent to u32
    let encrypted_a = FheUint32::try_encrypt(clear_a, &client_key)?;
    let encrypted_b = FheUint32::try_encrypt(clear_b, &client_key)?;

    // On the server side:
    set_server_key(server_keys);

    let encrypted_res = encrypted_a.lt(&encrypted_b);

    // Decrypting on the client side:
    let clear_res: bool = encrypted_res.decrypt(&client_key);
    assert_eq!(clear_res, clear_a < clear_b);

    let mut random = OsRng;
    let mut random_bytes = [0u8; 32];
    random.fill_bytes(&mut random_bytes);
    let random_string = hex::encode(random_bytes);

    let hash = keccak(random_string);
    println!("Keccak-256 hash: {}", hash.to_string());

    let address = "STUDENTS_ADDRESS".parse::<Address>().unwrap();
    permit_withdraw(address, hash.to_string());

    Ok(())
}

abigen!(
    HashLockWithdraw,
    r#"[
        function deposit(bytes32 _hash) public payable
        event DepositCreated(address indexed depositor, uint256 amount, bytes32 hash, uint256 depositId)
    ]"#,
);

async fn permit_withdraw(address: Address, hash: String) -> Result<()> {
    let provider = Provider::<Http>::try_from("http://localhost:8545")
        .expect("could not instantiate HTTP Provider");

    let wallet: LocalWallet = "0x..".parse().unwrap();
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    let contract_address = "CONTRACT_ADDRESS".parse::<Address>().unwrap();
    let contract = HashLockWithdraw::new(contract_address, client);

    let hash_bytes: [u8; 32] = hex::decode(hash.strip_prefix("0x").unwrap_or(&hash))?
        .try_into()
        .map_err(|_| eyre::eyre!("Invalid hash length"))?;
    let tx = contract.deposit(hash_bytes).from(address);
    let pending_tx = tx.send().await?;

    let receipt = pending_tx.await?;
    println!("Receipt: {:?}", receipt);

    if let Some(logs) = receipt.logs {
        for log in logs {
            if let Ok(decoded) =
                contract.decode_event::<DepositCreatedFilter>("DepositCreated", log)
            {
                println!("Deposit created: {:?}", decoded);
            }
        }
    }

    Ok(())
}
