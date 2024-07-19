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

    Ok(())
}
