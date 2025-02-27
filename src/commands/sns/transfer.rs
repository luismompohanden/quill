use crate::commands::transfer::parse_tokens;
use crate::lib::{
    signing::{sign_ingress_with_request_status_query, IngressWithRequestId},
    AnyhowResult, AuthInfo, ParsedSubaccount,
};
use crate::lib::{ParsedAccount, ROLE_ICRC1_LEDGER};
use candid::Encode;
use clap::Parser;
use ic_icrc1::{endpoints::TransferArg, Memo};
use icp_ledger::Tokens;

use super::SnsCanisterIds;

/// Signs a ledger transfer update call.
#[derive(Parser)]
pub struct TransferOpts {
    /// The destination account.
    pub to: ParsedAccount,

    /// The subaccount of the destination account.
    #[clap(long)]
    pub to_subaccount: Option<ParsedSubaccount>,

    #[clap(long)]
    /// The subaccount to transfer from.
    pub from_subaccount: Option<ParsedSubaccount>,

    /// Amount of governance tokens to transfer (with up to 8 decimal digits after decimal point)
    #[clap(long, value_parser = parse_tokens)]
    pub amount: Tokens,

    /// An arbitrary number associated with a transaction. The default is 0
    #[clap(long)]
    pub memo: Option<u64>,

    /// The amount that the caller pays for the transaction, default is 0.0001 tokens. Specify this amount
    /// when using an SNS that sets its own transaction fee
    #[clap(long, value_parser = parse_tokens)]
    pub fee: Option<Tokens>,
}

pub fn exec(
    auth: &AuthInfo,
    sns_canister_ids: &SnsCanisterIds,
    opts: TransferOpts,
) -> AnyhowResult<Vec<IngressWithRequestId>> {
    let amount = opts.amount.get_e8s().into();
    let fee = opts.fee.map(|fee| fee.get_e8s().into());
    let memo = opts.memo.map(Memo::from);
    let ledger_canister_id = sns_canister_ids.ledger_canister_id;
    let mut to = opts.to.0;
    if let Some(to_subaccount) = opts.to_subaccount {
        to.subaccount = Some(to_subaccount.0 .0);
    }
    let args = TransferArg {
        memo,
        amount,
        fee,
        from_subaccount: opts.from_subaccount.map(|x| x.0 .0),
        created_at_time: None,
        to,
    };

    let msg = sign_ingress_with_request_status_query(
        auth,
        ledger_canister_id,
        ROLE_ICRC1_LEDGER,
        "icrc1_transfer",
        Encode!(&args)?,
    )?;

    Ok(vec![msg])
}
