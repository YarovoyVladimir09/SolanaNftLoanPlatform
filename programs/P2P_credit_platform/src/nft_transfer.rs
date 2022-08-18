use {
    anchor_lang::{
        prelude::*,
        system_program,
    },
    anchor_spl::{
        associated_token,
        token,
    },
};

//pub mod sol_transfer;
//pub mod "../sol_transfer.rs";

// #[path = "./sol_transfer.rs"]
// pub mod sol_transfer;
// use sol_transfer::*;

//use crate::sol_transfer;


pub fn nft_transfer(
    ctx: Context<NftTransfer>,
    sale_lamports: u64
) -> Result<()> {

    msg!("Initiating transfer of {} lamports...", sale_lamports);
    if sale_lamports != 0 {
        msg!("Purchaser (sending lamports): {}", &ctx.accounts.buyer_authority.key());
        msg!("Seller (receiving lamports): {}", &ctx.accounts.owner_authority.key());
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.buyer_authority.to_account_info(),
                    to: ctx.accounts.owner_authority.to_account_info(),
                }
            ),
            sale_lamports
        )?;

        msg!("Lamports transferred successfully.");        
    }

    
    // let mut sol_accounts = SolTransfer {
    //     owner_authority: ctx.accounts.buyer_authority,
    //     buyer_authority: ctx.accounts.owner_authority,
    //     system_program: ctx.accounts.system_program
    // };

    // sol_transfer(       
    //     &mut sol_accounts,
    //     0
    // );

    msg!("Creating buyer token account...");
    msg!("Buyer Token Address: {}", &ctx.accounts.buyer_token_account.key());    
    associated_token::create(
        CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.buyer_authority.to_account_info(),
                associated_token: ctx.accounts.buyer_token_account.to_account_info(),
                authority: ctx.accounts.buyer_authority.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
    )?;

    msg!("Transferring NFT...");
    msg!("Owner Token Address: {}", &ctx.accounts.owner_token_account.key());    
    msg!("Buyer Token Address: {}", &ctx.accounts.buyer_token_account.key());    
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.owner_token_account.to_account_info(),
                to: ctx.accounts.buyer_token_account.to_account_info(),
                authority: ctx.accounts.owner_authority.to_account_info(),
            }
        ),
        1
    )?;
    msg!("NFT transferred successfully.");
    
    msg!("Sale completed successfully!");

    Ok(())
}


#[derive(Accounts)]
pub struct NftTransfer<'info> {
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub owner_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub owner_authority: Signer<'info>,
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub buyer_token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub buyer_authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}

// impl<'info> NftTransfer<'info> {
//     pub fn set_sol_ctx(&self) -> Context<'_, 'info> {
//         let cpi_program = self.system_program.to_account_info();
//         let sol_accounts = SolTransfer {
//             owner_authority: self.buyer_authority,
//             buyer_authority: self.owner_authority,
//             system_program: self.system_program
//         };
//         Context::new(cpi_program,
//             & mut sol_accounts)
//     }
// }