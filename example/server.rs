use std::error::Error;
use tfhe::integer::{BooleanBlock, PublicKey, RadixCiphertext, ServerKey};

use crate::ecdsa::ecdsa_sign;
use crate::helper::u256_from_decimal_string;
use crate::ops::secp256k1::prelude::{FQ_MODULO, FR_MODULO, GENERATOR};
pub struct FheServer {
    server_key: ServerKey,
    public_key: PublicKey,
}

// Start of Selection
impl FheServer {
    pub fn new(server_key: ServerKey, public_key: PublicKey) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            server_key,
            public_key,
        })
    }

    pub fn review_application(
        &self,
        encrypted_balance: RadixCiphertext,
    ) -> Result<BooleanBlock, Box<dyn Error>> {
        let threshold = 100u64;
        let is_eligible = self
            .server_key
            .scalar_lt_parallelized(&encrypted_balance, threshold);
        Ok(is_eligible)
    }

    pub fn sign_result(&self, result: &BooleanBlock) -> (RadixCiphertext, RadixCiphertext) {
        let sk = u256_from_decimal_string(
            "32670510020758816978083085130507043184471273380659243275938904335757337482424",
        );
        let nonce = u256_from_decimal_string(
            "158972629851468960855479098042189567798917817837573660423710583832714848",
        );
        // resultの代わりに明示的なメッセージを使う
        let msg = u256_from_decimal_string("1");
        const NUM_BLOCK: usize = 128;
        let enc_sk = self.public_key.encrypt_radix(sk, NUM_BLOCK);
        let enc_k = self.public_key.encrypt_radix(nonce, NUM_BLOCK);

        let signature = ecdsa_sign::<NUM_BLOCK, _>(
            &enc_sk,
            &enc_k,
            msg, // resultをBooleanBlockからPにできる？0or1
            *GENERATOR,
            *FQ_MODULO,
            *FR_MODULO,
            &self.server_key,
        );
        return signature;
    }
}