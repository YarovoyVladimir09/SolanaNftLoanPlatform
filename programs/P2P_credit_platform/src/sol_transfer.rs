use {
    anchor_lang::{
        prelude::*,
        system_program,
    }
};


pub fn sol_transfer(
    ctx: Context<SolTransfer>,
    sale_lamports: u64
) -> Result<()> {

    msg!("Initiating transfer of {} lamports...", sale_lamports);
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
    
    Ok(())
}


#[derive(Accounts)]
pub struct SolTransfer<'info> {
    #[account(mut)]
    pub owner_authority: Signer<'info>,
    #[account(mut)]
    pub buyer_authority: Signer<'info>,
    pub system_program: Program<'info, System>,

}