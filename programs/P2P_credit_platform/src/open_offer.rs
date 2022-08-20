use {
    anchor_lang::{
        prelude::*
    }
};
use std::mem::size_of;

pub fn open_offer(
    ctx: Context<OpenOffer>,
    time_mark: u64,
    money_count: u64
) -> Result<()> {
    msg!("Open offer creditor {}, debtor {} ",ctx.accounts.creditor.key().to_string(), ctx.accounts.debtor.key().to_string());
    let ledger_account = &mut ctx.accounts.ledger_account;
    ledger_account.creditor = ctx.accounts.creditor.key();
    ledger_account.debtor = ctx.accounts.debtor.key();
    ledger_account.time_mark = time_mark;
    ledger_account.mint_account = ctx.accounts.mint_account.key();
    ledger_account.moneyback = false;
    ledger_account.money_count = money_count;
    
    msg!("Ledger Account Address: {}", &ctx.accounts.ledger_account.key());  

    Ok(())
}


#[derive(Accounts)]
pub struct OpenOffer<'info> {
    #[account(
        init,
        payer = wallet,
        space = 8 + size_of::<Ledger>(),
        seeds = [
            creditor.key().as_ref(),
            mint_account.key().as_ref(),
            debtor.key().as_ref(),
        ],
        bump
    )]
    pub ledger_account: Account<'info, Ledger>,
    #[account(mut)]
    pub wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
/// CHECK: This is not dangerous because we don't read or write from this account
    pub creditor: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub debtor: AccountInfo<'info>,
        /// CHECK: This is not dangerous because we don't read or write from this account
    pub mint_account: AccountInfo<'info>

}

#[account]
pub struct Ledger {
    pub creditor: Pubkey,
    pub debtor: Pubkey,
    pub time_mark: u64,
    pub mint_account: Pubkey,
    pub moneyback: bool,
    pub money_count: u64
}