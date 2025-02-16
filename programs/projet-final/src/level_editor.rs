use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeNftId<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub nft_id_counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(height: u8, width: u8)]
pub struct CreateNft<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 2 + 32 + 4 + 1 + 100+ (height as usize * width as usize)
    )]
    pub nft_account: Account<'info, NftAccount>,
  
    #[account(mut)]
    pub nft_id_counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct NftAccount {
    pub owner: Pubkey,
    pub id: u64,
    pub height: u8,
    pub width: u8,
    pub data: Vec<u8>,
}

#[account]
pub struct Counter {
    pub count: u64,
}

pub fn initialize_nft_id(ctx: Context<InitializeNftId>) -> Result<()> {
    let nft_id_counter = &mut ctx.accounts.nft_id_counter;
    nft_id_counter.count = 0;
    Ok(())
}

pub fn create_nft(
    ctx: Context<CreateNft>,
    height: u8,
    width: u8,
    data: Vec<u8>,
) -> Result<()> {
    let nft_account = &mut ctx.accounts.nft_account;

    let mint_counter = &mut ctx.accounts.nft_id_counter;
    mint_counter.count += 1;

    nft_account.id = mint_counter.count;
    nft_account.owner = *ctx.accounts.user.key;
    nft_account.height = height;
    nft_account.width = width;
    nft_account.data = data;
    Ok(())
}
