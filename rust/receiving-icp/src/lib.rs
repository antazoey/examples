use candid::Principal;
use ic_ledger_types::{AccountBalanceArgs, AccountIdentifier, Subaccount};

// This is the ledger principal for TESTICP
// To use real ICP, use `ryjl3-tyaaa-aaaaa-aaaba-cai` instead.
const LEDGER_PRINCIPAL: &str = "xafvr-biaaa-aaaai-aql5q-cai";

/// Retrieves the canister's main account.
#[ic_cdk::query]
async fn account() -> String {
    AccountIdentifier::new(&ic_cdk::api::canister_self(), &Subaccount([0; 32])).to_string()
}

/// Retrieves the canister's subaccount based on upper and lower values.
#[ic_cdk::query]
async fn subaccount(upper: u128, lower: u128) -> String {
    // Create a 32-byte array by combining the little endian representation of upper and lower
    let mut subaccount_bytes = [0u8; 32];
    subaccount_bytes[0..16].copy_from_slice(&upper.to_le_bytes());
    subaccount_bytes[16..32].copy_from_slice(&lower.to_le_bytes());

    AccountIdentifier::new(&ic_cdk::api::canister_self(), &Subaccount(subaccount_bytes)).to_string()
}

/// Retrieves own balance from the ledger.
#[ic_cdk::update]
async fn get_balance() -> u64 {
    let ledger = Principal::from_text(LEDGER_PRINCIPAL).expect("invalid ledger principal");

    // Compute the canister's account identifier.
    let account = AccountIdentifier::new(&ic_cdk::api::canister_self(), &Subaccount([0; 32]));

    // Retrieves the account's balance from the ledger.
    let balance = ic_ledger_types::account_balance(ledger, &AccountBalanceArgs { account })
        .await
        .expect("call to get balance failed");

    balance.e8s()
}
