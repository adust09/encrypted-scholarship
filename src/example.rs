use std::error::Error;

use crate::client::BobClient;
use crate::server::FheServer;
use tfhe::integer::{
    keycache::IntegerKeyCache,
    {IntegerKeyKind, PublicKey},
};
use tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2;

fn simulate_scholarship_application() -> Result<(), Box<dyn Error>> {
    let (client_key, server_key) =
        IntegerKeyCache.get_from_params(PARAM_MESSAGE_2_CARRY_2, IntegerKeyKind::Radix);
    let public_key = PublicKey::new(&client_key);

    let fhe_server = FheServer::new(server_key, public_key)?;
    let bob_client = BobClient::new(client_key)?;

    // Bobの残高（この例では50）
    let bob_balance = 50u8;

    // 残高の暗号化
    let encrypted_balance = bob_client.encrypt_balance(bob_balance)?;

    // FHE Serverでの審査
    let encrypted_result = fhe_server.review_application(encrypted_balance)?;

    let signature = fhe_server.sign_result(&encrypted_result);

    // Bobによる結果の復号
    let is_eligible = bob_client.decrypt_result(encrypted_result)?;
    println!("Bob's eligibility: {}", is_eligible);

    let decrypted_signature = bob_client.decrypt_signature(signature);

    // 奨学金の受け取り
    // convert (StaticUnsignedBigInt<4>, StaticUnsignedBigInt<4>) to Vec<u8>
    // let _result = bob_client.interact_with_smart_contract(decrypted_signature);

    Ok(())
}

fn main() {
    if let Err(e) = simulate_scholarship_application() {
        eprintln!("Error: {}", e);
    }
}
