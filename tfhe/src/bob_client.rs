use std::error::Error;
use std::sync::Arc;
use tfhe::prelude::FheEncrypt;
use tfhe::{prelude::FheDecrypt, ClientKey};
use tfhe::{FheBool, FheUint8};

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

    pub fn encrypt_balance(&self, balance: u8) -> Result<FheUint8, Box<dyn Error>> {
        Ok(FheUint8::encrypt(balance, &self.client_key))
    }

    pub fn decrypt_result(&self, encrypted_result: FheBool) -> Result<bool, Box<dyn Error>> {
        let decrypted_value: bool = FheBool::decrypt(&encrypted_result, &self.client_key);
        Ok(decrypted_value)
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
