use ecdsa::numeral::Numeral;
use std::{error::Error, sync::Arc};

use tfhe::integer::{BooleanBlock, ClientKey, RadixCiphertext, U256};

use ethers::{
    prelude::*,
    providers::{Http, Provider},
    signers::LocalWallet,
};

abigen!(
    ScholarshipFund,
    r#"[
        function deposit() public payable
        function requestScholarship(bytes memory signature) public
        function withdraw() public
        event ScholarshipApproved(address indexed applicant)
    ]"#,
);

pub struct BobClient {
    client_key: ClientKey,
}

impl BobClient {
    pub fn new(client_key: ClientKey) -> Result<Self, Box<dyn Error>> {
        Ok(Self { client_key })
    }

    pub fn encrypt_balance(&self, balance: u8) -> Result<RadixCiphertext, Box<dyn Error>> {
        const NUM_BLOCK: usize = 128;

        Ok(self.client_key.encrypt_radix(balance, NUM_BLOCK))
    }

    pub fn decrypt_result(&self, encrypted_result: BooleanBlock) -> Result<bool, Box<dyn Error>> {
        let decrypted_value: bool = self.client_key.decrypt_bool(&encrypted_result);
        Ok(decrypted_value)
    }

    // todo: decrypt_signature
    pub fn decrypt_signature(&self, signature: (RadixCiphertext, RadixCiphertext)) -> (U256, U256) {
        let signature = (
            U256::decrypt(&signature.0, &self.client_key),
            U256::decrypt(&signature.1, &self.client_key),
        );
        return signature;
    }

    pub async fn interact_with_smart_contract(
        &self,
        signature: Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        // Provider の設定（ローカルの場合）
        let provider = Provider::<Http>::try_from("http://localhost:8545")?;

        // 秘密鍵からウォレットを作成（注意: 実際の秘密鍵を直接コードに書かないでください）
        let wallet: LocalWallet = "0x...".parse()?;
        let client = SignerMiddleware::new(provider, wallet);
        let client = Arc::new(client);

        // コントラクトアドレスの設定
        let contract_address = "CONTRACT_ADDRESS_HERE".parse::<Address>()?;
        let contract = ScholarshipFund::new(contract_address, client.clone());

        // 奨学金の申請
        let tx = contract.request_scholarship(signature.into());
        let pending_tx = tx.send().await?;
        let receipt = pending_tx.await?;

        let logs = receipt.unwrap().logs;
        for log in logs {
            if let Ok(decoded) = contract.decode_event::<ScholarshipApprovedFilter>(
                "ScholarshipApproved",
                log.topics,
                log.data,
            ) {
                println!("Scholarship approved for: {:?}", decoded.applicant);
            }
        }
        // 奨学金の引き出し
        let withdraw_tx = contract.withdraw();
        let pending_withdraw_tx = withdraw_tx.send().await?;
        let withdraw_receipt = pending_withdraw_tx.await?;

        println!("Withdraw receipt: {:?}", withdraw_receipt);

        Ok(())
    }
}
