use {
    anchor_lang::{
        prelude::*,
        system_program,
    }
};

pub fn close_offer(
    ctx: Context<CloseOffer>,
) -> Result<()> {
    msg!("Close offer. Ledger Account Address: {}", &ctx.accounts.ledger_account.key()); 
    let ledger_account = &mut ctx.accounts.ledger_account;
    ledger_account.moneyback = true;
    msg!("Creditor: {}, debtor: {} ",ledger_account.creditor, ledger_account.debtor);
    msg!("Time-mark: {}, mint account: {} ",ledger_account.time_mark, ledger_account.mint_account);
    msg!("Moneyback: {}, money_count {} ",ledger_account.moneyback, ledger_account.money_count);
    Ok(())
}


#[derive(Accounts)]
pub struct CloseOffer<'info> {
    #[account(mut)]
    pub ledger_account: Account<'info, Ledger>,
    #[account(mut)]
    pub wallet: Signer<'info>,
}

#[account]
pub struct Ledger {
    pub creditor: String,
    pub debtor: String,
    pub time_mark: i64,
    pub mint_account: String,
    pub moneyback: bool,
    pub money_count: f64
}