use rand::Rng;
use std::error::Error;
use tfhe::prelude::FheOrd;
use tfhe::ServerKey;
use tfhe::{set_server_key, FheBool, FheUint8};

pub struct FheServer {
    server_key: ServerKey,
}

// Start of Selection
impl FheServer {
    pub fn new(server_key: ServerKey) -> Result<Self, Box<dyn Error>> {
        set_server_key(server_key.clone());
        Ok(Self { server_key })
    }

    pub fn review_application(
        &self,
        encrypted_balance: FheUint8,
    ) -> Result<FheBool, Box<dyn Error>> {
        let threshold = 100u8;
        let is_eligible = encrypted_balance.clone().lt(threshold);
        Ok(is_eligible)
    }

    pub fn sign_result(&self, result: &FheBool) -> Vec<u8> {
        // 実際の署名ロジックをここに実装
        let mut rng = rand::thread_rng();
        (0..64).map(|_| rng.gen::<u8>()).collect()
    }
}
