use anchor_lang::prelude::*;

pub mod nft_transfer;
pub mod sol_transfer;
pub mod open_offer;
pub mod close_offer;

use nft_transfer::*;
use sol_transfer::*;
use open_offer::*;
use close_offer::*;


declare_id!("GREPj1wkRt5dh8NeyFa6AuL8fxsiSVk5eMvKBaW9SbQ3");


#[program]
pub mod P2P_credit_platform {

    use super::*;

    pub fn nft_transfer(
        ctx: Context<NftTransfer>,
        sale_lamports: u64
    ) -> Result<()> {
        nft_transfer::nft_transfer(
            ctx,
            sale_lamports
        )
    }

    pub fn sol_transfer(
        ctx: Context<SolTransfer>,
        sale_lamports: u64
    ) -> Result<()> {
        sol_transfer::sol_transfer(
            ctx,
            sale_lamports,
        )
    }

    pub fn open_offer(
        ctx: Context<OpenOffer>,
        creditor: String,
        debtor: String,
        time_mark: i64,
        mint_account: String,
        money_count: f64
    ) -> Result<()> {
        open_offer::open_offer(
            ctx,
            creditor,
            debtor,
            time_mark,
            mint_account,
            money_count
        )
    }

    pub fn close_offer(
        ctx: Context<CloseOffer>
    ) -> Result<()> {
        close_offer::close_offer(
            ctx
        )
    }
}