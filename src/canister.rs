

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::{query, update};
use std::cell::RefCell;

thread_local! {
    static BALANCE: RefCell<u64> = RefCell::new(0);
}

#[derive(CandidType, Deserialize)]
struct TransferArgs {
    to: String,
    amount: u64,
}

#[update]
async fn send_tokens(args: TransferArgs) -> Result<(), String> {
    // Simple token sending logic, adjust for real implementation
    if *BALANCE.with(|balance| *balance.borrow()) < args.amount {
        return Err("Insufficient balance".into());
    }

    BALANCE.with(|balance| {
        *balance.borrow_mut() -= args.amount;
    });

    // Normally, you'd interact with another canister to send the tokens.
    // For simplicity, this example assumes success.
    Ok(())
}

#[update]
async fn receive_tokens(amount: u64) -> Result<(), String> {
    BALANCE.with(|balance| {
        *balance.borrow_mut() += amount;
    });
    Ok(())
}

#[query]
fn get_balance() -> u64 {
    BALANCE.with(|balance| *balance.borrow())
}
