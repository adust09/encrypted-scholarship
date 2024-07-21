use std::error::Error;
use tfhe::integer::{
    bigint::StaticUnsignedBigInt, keycache::IntegerKeyCache, IntegerKeyKind, PublicKey,
};
use tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2;

use crate::client::BobClient;
use crate::server::FheServer;

fn to_le_bytes<const N: usize>(bigint: &StaticUnsignedBigInt<N>) -> Vec<u8> {
    let mut bytes = vec![0u8; N * 8];
    bigint.copy_to_le_byte_slice(&mut bytes);
    bytes
}

pub fn main() -> Result<(), Box<dyn Error>> {
    // setup server and client
    let (client_key, server_key) =
        IntegerKeyCache.get_from_params(PARAM_MESSAGE_2_CARRY_2, IntegerKeyKind::Radix);
    let public_key = PublicKey::new(&client_key);
    let fhe_server = FheServer::new(server_key, public_key)?;
    let bob_client = BobClient::new(client_key)?;

    // submit application
    let bob_balance = 50u8;
    let enc_balance = bob_client.encrypt_balance(bob_balance)?;

    // review application and return the result
    let enc_result = fhe_server.review_application(enc_balance)?;
    let enc_signature = fhe_server.sign_result(&enc_result);

    // get result and claim scholarship
    let is_eligible = bob_client.decrypt_result(enc_result)?;
    if is_eligible {
        println!("Congratulations! You are eligible for the scholarship.");
        let dec_signature = bob_client.decrypt_signature(enc_signature);
        let (x, y) = dec_signature;
        let mut signature_vec = Vec::new();
        signature_vec.extend_from_slice(&to_le_bytes(&x));
        signature_vec.extend_from_slice(&to_le_bytes(&y));
        let claim_result = bob_client.claim_scholarship(signature_vec);
    } else {
        println!("Sorry, you are not eligible for the scholarship.");
    }

    Ok(())
}
