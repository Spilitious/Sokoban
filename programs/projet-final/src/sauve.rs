/* use anchor_lang::prelude::*;

declare_id!("FYjcKSeCtxwWi161uNjmN8cs2ykVtA2YWdpnsyAWjuHK");

#[program]
pub mod projet_final {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, width: u8, height: u8) -> Result<()> {
        let ground = &mut ctx.accounts.ground;
        ground.width = width;
        ground.height = height;
        ground.data = vec![0; 10];
        Ok(())
    }

    pub fn add_box_start_position(ctx: Context<ModifyGround>, x: u8, y: u8) -> Result<()> {
        let ground = &mut ctx.accounts.ground;
        let index = (y as usize) * (ground.width as usize) + (x as usize);
        if index < ground.data.len() {
            ground.data[index] = 1;
            return Ok(());
        } else {
            Err(ErrorCode::IndexOutOfBounds.into())
        }
        
    }

    pub fn add_box_end_position(ctx: Context<ModifyGround>, x: u8, y: u8) -> Result<()> {
        let ground = &mut ctx.accounts.ground;
        let index = (y as usize) * (ground.width as usize) + (x as usize);
        if index < ground.data.len() {
            ground.data[index] = 2;
            return Ok(());
        } else {
            Err(ErrorCode::IndexOutOfBounds.into())
        }
        
    }

    pub fn add_wall(ctx: Context<ModifyGround>, x: u8, y: u8) -> Result<()> {
        let ground = &mut ctx.accounts.ground;
        let index = (y as usize) * (ground.width as usize) + (x as usize);
        if index < ground.data.len() {
            ground.data[index] = 3;
            Ok(())
        } else {
            Err(ErrorCode::IndexOutOfBounds.into())
        }
        
    }


}

//impl Ground {
  //  const LEN: usize = ;
//}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space =  8 + 1 + 1 + 4 + (4* 1024 * 1024))] 
    pub ground: Account<'info, Ground>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct ModifyGround<'info> {
    #[account(mut)]
    pub ground: Account<'info, Ground>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct Ground {
    pub width: u8,
    pub height:u8,
    pub data: Vec<u8>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Index out of bounds.")]
    IndexOutOfBounds,
}

/*

pub fn convert_to_index(ctx: Context<Level>, x:u8, y:u8) -> u16 {
    let ground = ctx.accounts.ground;
    let w = ground.width;
    let h = ground.height;
    let index = y*h+x;

    if index < ground.data.len() {
        index;          
    } else {
        Err(ErrorCode::IndexOutOfBounds.into())
    } 
}

pub fn convert_to_point(ctx: Context<Level>, p:u16) -> u16 {
    if p < ground.data.len() {
        let ground = ctx.accounts.ground;
        let w = ground.width;
        let h = ground.height;
        index = p / w + p % w;       
    } else {
        Err(ErrorCode::IndexOutOfBounds.into())
    } 
}*/