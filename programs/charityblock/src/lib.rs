use anchor_lang::prelude::*;
declare_id!("7zkgjDgZReMvhMuJtMU6Rx9ijsqKVpCH1sf6GcCp2dnW");

#[program]
pub mod charityblock {
    use super::*;

    pub fn create_campaign(ctx: Context<CreateCampaign>, name: String, goal: u64, price_per_token: u64) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        campaign.name = name;
        campaign.goal = goal;
        campaign.price_per_token = price_per_token;
        campaign.collected = 0;
        campaign.creator = *ctx.accounts.creator.key;
        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        let payer = &mut ctx.accounts.payer;
        let system_program = &ctx.accounts.system_program;

        // Transfer SOL to the campaign creator
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &payer.key(),
            &campaign.creator,
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                payer.to_account_info(),
                ctx.accounts.creator.to_account_info(),
                system_program.to_account_info(),
            ],
        )?;

        // Update state
        campaign.collected += amount;
        Ok(())
    }
}

#[account]
pub struct Campaign {
    pub name: String,
    pub goal: u64,
    pub price_per_token: u64,
    pub collected: u64,
    pub creator: Pubkey,
}

#[derive(Accounts)]
pub struct CreateCampaign<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 64 + 64 + 64 + 32)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: Safe because this is only the SOL recipient
    #[account(mut)]
    pub creator: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
