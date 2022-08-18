use {
    anchor_lang::{
        prelude::*,
        system_program,
    }
};

//pub mod sol_transfer;
//pub mod "../sol_transfer.rs";

// #[path = "./sol_transfer.rs"]
// pub mod sol_transfer;
// use sol_transfer::*;

//use crate::sol_transfer;


pub fn open_offer(
    ctx: Context<OpenOffer>,
    creditor: String,
    debtor: String,
    time_mark: i64,
    mint_account: String,
    //moneyback: bool,
    money_count: f64
) -> Result<()> {
    msg!("Open offer creditor {}, debtor {} ",creditor, debtor);
    let ledger_account = &mut ctx.accounts.ledger_account;
    ledger_account.creditor = creditor;
    ledger_account.debtor = debtor;
    ledger_account.time_mark = time_mark;
    ledger_account.mint_account = mint_account;
    ledger_account.moneyback = false;
    ledger_account.money_count = money_count;
    
    msg!("Ledger Account Address: {}", &ctx.accounts.ledger_account.key());  

    Ok(())
}


#[derive(Accounts)]
#[instruction(
    creditor: String,
    debtor: String,
    time_mark: i64,
)]
pub struct OpenOffer<'info> {
    #[account(
        init,
        payer = wallet,
        space = 8 + 113,
        seeds = [
            wallet.key().as_ref(),
            creditor.as_ref(),
            b"_",
            debtor.as_ref(),
            b"_",
            time_mark.to_string().as_ref()
        ],
        bump
    )]
    pub ledger_account: Account<'info, Ledger>,
    #[account(mut)]
    pub wallet: Signer<'info>,
    pub system_program: Program<'info, System>,

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