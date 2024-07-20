mod bob_client;
mod ecdsa;
mod fhe_server;

use bob_client::BobClient;
use fhe_server::FheServer;
use std::error::Error;
use tfhe::{generate_keys, ConfigBuilder};

fn simulate_scholarship_application() -> Result<(), Box<dyn Error>> {
    let config = ConfigBuilder::default();
    let (client_key, server_key) = generate_keys(config.clone());

    let fhe_server = FheServer::new(server_key)?;
    let bob_client = BobClient::new(client_key)?;

    // Bobの残高（この例では50）
    let bob_balance = 50u8;

    // 残高の暗号化
    let encrypted_balance = bob_client.encrypt_balance(bob_balance)?;

    // FHE Serverでの審査
    let encrypted_result = fhe_server.review_application(encrypted_balance)?;

    // 結果の署名
    let signature = fhe_server.sign_result(&encrypted_result);

    // Bobによる結果の復号
    let is_eligible = bob_client.decrypt_result(encrypted_result)?;

    // 奨学金の受け取り
    let _result = bob_client.interact_with_smart_contract(signature);

    println!("Bob's eligibility: {}", is_eligible);

    Ok(())
}

fn main() {
    if let Err(e) = simulate_scholarship_application() {
        eprintln!("Error: {}", e);
    }
}
