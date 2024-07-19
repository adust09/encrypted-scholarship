use ethers::{
    prelude::*,
    providers::{Http, Provider},
    signers::LocalWallet,
    utils::hex,
    utils::keccak256,
};
use eyre::Result;

use std::convert::TryFrom;
use std::sync::Arc;

abigen!(
    HashLockWithdraw,
    r#"[
        function deposit() public payable
        function setHash(address depositor, uint256 depositId, bytes32 _hash) public
        function withdraw(address depositor, uint256 depositId, bytes32 _random) public
        event DepositCreated(address indexed depositor, uint256 amount, uint256 depositId)
        event HashSet(address indexed depositor, uint256 depositId, bytes32 hash)
        event WithdrawMade(address indexed depositor, address indexed recipient, uint256 amount, uint256 depositId)
    ]"#,
);

async fn setup_contract() -> Result<(Arc<SignerMiddleware<Provider<Http>, LocalWallet>>, Address)> {
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    let wallet: LocalWallet = "0x...".parse()?; // Replace with actual private key
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    let contract_address = "CONTRACT_ADDRESS_HERE".parse::<Address>()?;

    Ok((client, contract_address))
}

async fn alice_deposit(
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    contract_address: Address,
) -> Result<U256> {
    let contract = HashLockWithdraw::new(contract_address, client.clone());
    let amount = U256::from(1_000_000_000_000_000_000u64); // 1 ETH
    let tx = contract.deposit().value(amount);
    let pending_tx = tx.send().await?;
    let receipt = pending_tx.await?;

    for log in receipt.unwrap().logs {
        if let Ok(decoded) =
            contract.decode_event::<DepositCreatedFilter>("DepositCreated", log.topics, log.data)
        {
            println!("Deposit created: {:?}", decoded);
            return Ok(decoded.deposit_id);
        }
    }

    Err(eyre::eyre!("Deposit event not found"))
}
async fn third_party_set_hash(
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    contract_address: Address,
    alice_address: Address,
    deposit_id: U256,
) -> Result<[u8; 32]> {
    let contract = HashLockWithdraw::new(contract_address, client.clone());
    let random = rand::random::<[u8; 32]>();
    let hash = keccak256(random);

    let tx = contract.set_hash(alice_address, deposit_id, hash);
    let pending_tx = tx.send().await?;
    let receipt = pending_tx.await?;

    for log in receipt.unwrap().logs {
        if let Ok(decoded) = contract.decode_event::<HashSetFilter>("HashSet", log.topics, log.data)
        {
            println!("Hash set: {:?}", decoded);
            if decoded.depositor == alice_address
                && decoded.deposit_id == deposit_id
                && decoded.hash == hash
            {
                return Ok(random);
            }
        }
    }

    Err(eyre::eyre!("HashSet event not found or mismatch"))
}

async fn bob_withdraw(
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    contract_address: Address,
    alice_address: Address,
    deposit_id: U256,
    random: [u8; 32],
) -> Result<()> {
    let contract = HashLockWithdraw::new(contract_address, client.clone());

    let tx = contract.withdraw(alice_address, deposit_id, random);
    let pending_tx = tx.send().await?;
    let receipt = pending_tx.await?;

    for log in receipt.unwrap().logs {
        if let Ok(decoded) =
            contract.decode_event::<WithdrawMadeFilter>("WithdrawMade", log.topics, log.data)
        {
            println!("Withdraw made: {:?}", decoded);
            return Ok(());
        }
    }

    Err(eyre::eyre!("Withdraw event not found"))
}

#[tokio::main]
async fn main() -> Result<()> {
    let (client, contract_address) = setup_contract().await?;

    // Alice deposits
    let deposit_id = alice_deposit(client.clone(), contract_address).await?;

    // Third party sets hash
    let alice_address = client.address();
    let random =
        third_party_set_hash(client.clone(), contract_address, alice_address, deposit_id).await?;

    // Simulate sending random to Bob (in reality, this would be done off-chain)
    println!("Random sent to Bob: {:?}", hex::encode(random));

    // Bob withdraws
    bob_withdraw(
        client.clone(),
        contract_address,
        alice_address,
        deposit_id,
        random,
    )
    .await?;

    Ok(())
}
