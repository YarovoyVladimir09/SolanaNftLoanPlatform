use {
    anchor_lang::{
        prelude::*
    }
};

//use std::fmt;
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
    ledger_account.status = 0;
    ledger_account.money_count = money_count;
    
    msg!("Ledger Account Address: {}", &ctx.accounts.ledger_account.key());  

    Ok(())
}

pub fn modify_status_offer(
    ctx: Context<ModifyOffer>,
    status: u8,
) -> Result<()> {
    msg!("Modify offer status. Ledger Account Address: {}", &ctx.accounts.ledger_account.key()); 
    let ledger_account = &mut ctx.accounts.ledger_account;
    msg!("Old status: {:?}", LedgerStatus::from_u8(&ledger_account.status)); 

    ledger_account.status = status;
    msg!("Creditor: {}, debtor: {} ",ledger_account.creditor, ledger_account.debtor);
    msg!("Time-mark: {}, mint account: {} ",ledger_account.time_mark, ledger_account.mint_account);
    msg!("Moneyback: {}, money_count {} ",ledger_account.status, ledger_account.money_count);
    msg!("New status: {:?}", LedgerStatus::from_u8(&ctx.accounts.ledger_account.status)); 
    Ok(())
}

pub fn modify_amount_offer(
    ctx: Context<ModifyOffer>,
    time_mark: u64,
    money_count: u64,
) -> Result<()> {
    msg!("Modify offer amount. Ledger Account Address: {}", &ctx.accounts.ledger_account.key()); 
    let ledger_account = &mut ctx.accounts.ledger_account;
    msg!("Old amount: money count - {}, time mark - {}", &ledger_account.money_count, &ledger_account.time_mark); 
    ledger_account.time_mark = time_mark;
    ledger_account.money_count = money_count;
    ledger_account.status = 1;
    msg!("Creditor: {}, debtor: {} ",ledger_account.creditor, ledger_account.debtor);
    msg!("Status: {:?}, mint account: {} ",LedgerStatus::from_u8(&1), ledger_account.mint_account);
    msg!("New money_count: {}, time_mark {} ",ledger_account.money_count, ledger_account.time_mark);
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

#[derive(Accounts)]
pub struct ModifyOffer<'info> {
    #[account(mut)]
    pub ledger_account: Account<'info, Ledger>,
    #[account(mut)]
    pub wallet: Signer<'info>,
}

#[account]
pub struct Ledger {
    pub creditor: Pubkey,
    pub debtor: Pubkey,
    pub time_mark: u64,
    pub mint_account: Pubkey,
    pub status: u8,
    pub money_count: u64
}

#[derive(Debug)]
enum LedgerStatus{
    Created = 0,
    Modify = 1,
    Accepted = 2,
    Closed = 3,
    Activated = 4,
    Finished = 5,
}

impl LedgerStatus{
    fn from_u8(value: &u8) -> LedgerStatus{
        match value {
            0 => LedgerStatus::Created,
            1 => LedgerStatus::Modify,
            2 => LedgerStatus::Accepted,
            3 => LedgerStatus::Closed,
            4 => LedgerStatus::Activated,
            5 => LedgerStatus::Finished,
            _ => panic!("Unknown value: {}", value),
        }
    }
}