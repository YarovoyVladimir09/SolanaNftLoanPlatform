use anchor_lang::prelude::*;

pub mod nft_transfer;
pub mod sol_transfer;
pub mod open_offer;
pub mod close_offer;

use nft_transfer::*;
use sol_transfer::*;
use open_offer::*;
use close_offer::*;


declare_id!("7nwV8W1EJrsJ2QFdPVuUMbXhWbpRpo82cVfzDiB9LJhc");


#[program]
pub mod credit_platform {

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
        time_mark: u64,
        money_count: u64
    ) -> Result<()> {
        open_offer::open_offer(
            ctx,
            time_mark,
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