use {
    anchor_lang::{
        prelude::*
    }
};

//use std::fmt;
use std::mem::size_of;

pub fn open_book(
    ctx: Context<OpenBook>,
) -> Result<()> {
    msg!("Create book for {}", ctx.accounts.loan_participater.key().to_string());
    msg!("Ledger account {} ", ctx.accounts.ledger_account.key().to_string());
    let book_account = &mut ctx.accounts.book_account;
    book_account.data.push(ctx.accounts.ledger_account.key());
    msg!("Book Account Address: {}", &ctx.accounts.book_account.key());  

    Ok(())
}

pub fn modify_book(
    ctx: Context<ModifyBook>,
) -> Result<()> {
    msg!("Loan participater {}, ledger account {} ",ctx.accounts.loan_participater.key().to_string(), ctx.accounts.ledger_account.key().to_string());
    let book_account = &mut ctx.accounts.book_account;
    book_account.data.push(ctx.accounts.ledger_account.key());
    msg!("Book Account Address: {}", &ctx.accounts.book_account.key());  

    Ok(())
}


#[derive(Accounts)]
pub struct OpenBook<'info> {
    #[account(
        init,
        payer = wallet,
       // space = 8 + size_of::<LedgerBook>(),
       space = 1000,
        seeds = [
            wallet.key().as_ref(),
            b"_book_",
            loan_participater.key().as_ref(),
        ],
        bump
    )]
    pub book_account: Account<'info, LedgerBook>,
    #[account(mut)]
    pub wallet: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub loan_participater: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub ledger_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}



#[derive(Accounts)]
pub struct ModifyBook<'info> {
    #[account(mut)]
    pub book_account: Account<'info, LedgerBook>,
    #[account(mut)]
    pub wallet: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub loan_participater: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub ledger_account: AccountInfo<'info>,

}


#[account]
pub struct LedgerBook {
    pub data: Vec<Pubkey>,
}
