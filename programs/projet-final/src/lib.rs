use anchor_lang::prelude::*;

mod game_state;
mod error;
use crate::game_state::GameState; 
use crate::error::ErrorCode; 

declare_id!("FYjcKSeCtxwWi161uNjmN8cs2ykVtA2YWdpnsyAWjuHK");

#[program]
pub mod projet_final {
    use super::*;


    pub fn initialize(ctx: Context<Initialize>, width: u8, height: u8, map_data: Vec<u8>) -> Result<()> {
        let game = &mut ctx.accounts.game;
        
        if usize::from(height*width) > map_data.len() {
            return Err(ErrorCode::InitialisationFailed.into());
        }
        
        game.player_position = 0;
        game.width = width;
        game.height = height;
        game.map_data = map_data.clone();
        game.solved = false;
        
        for index in map_data.iter() {
            if game.get_case_type(*index as u16) == 2 || game.get_case_type(*index as u16) == 6 {
                game.player_position = *index as u16;
                break;
            }
        }

        Ok(())

    }

    pub fn solve(ctx: Context<Initialize>, directions: Vec<u8>) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.run_sequence(directions);
        game.verify();
        return Ok(());
    }


    pub fn add_item(ctx: Context<AddItem>, item:u8,  x: u8, y: u8) -> Result<()> {
        let game = &mut ctx.accounts.game;
        let index = (y as usize) * (game.width as usize) + (x as usize);
        if index < game.map_data.len() {
            game.map_data[index] = item;
            return Ok(());
        } else {
            Err(ErrorCode::IndexOutOfBounds.into())
        }
        
    }

    




}

#[derive(Accounts)]
#[instruction(height: u8, width: u8)]
pub struct Initialize<'info> {
   
    #[account(
        init,
        payer = signer,
        seeds = [signer.key().as_ref()],
        bump,
       
        space = 8 + 8 + 8 + 8 + 1 +4+ (height as usize * width as usize))]
   
    pub game: Account<'info, GameState>,
  
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddItem<'info> {
    #[account(mut)]
    pub game: Account<'info, GameState>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
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