use anchor_lang::prelude::*;

mod soluce_checker;
mod level_editor;
// mod game_state;

// use crate::game_state::GameState; 
use crate::soluce_checker::*;
use crate::level_editor::*;


declare_id!("7pf6pTCeTrgkoETp5GD1f3v2W6pTPH6Wz7SSbRFhHkwb");

#[program]
pub mod game {
    use super::*;
    

    /* Re-export functions from game.rs
    pub fn initialize(ctx: Context<Initialize>, width: u8, height: u8, map_data: Vec<u8>) -> Result<()> {
        soluce_checker::initialize(ctx, width, height, map_data)
    }
*/
    pub fn solve(ctx: Context<Initialize>, width:u8, height:u8, id_nft:u32,  map_data:Vec<u8>, directions: Vec<u8>) -> Result<()> {
        soluce_checker::solve(ctx, width, height, id_nft, map_data, directions)
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        soluce_checker::claim(ctx)
    }
/*
    pub fn transfer(ctx: Context<Transfer>) -> Result<()> {
        soluce_checker::transfer(ctx)
    }

    pub fn add_item(ctx: Context<Initialize>, item: u8, x: u8, y: u8) -> Result<()> {
        soluce_checker::add_item(ctx, item, x, y)
    }

    pub fn movet(ctx: Context<MapModifier>, direction: u8) -> Result<()> {
        soluce_checker::movet(ctx, direction)
    }
    */
    // Re-export functions from nft.rs
    pub fn initialize_nft_id(ctx: Context<InitializeNftId>) -> Result<()> {
        level_editor::initialize_nft_id(ctx)
    }

    pub fn create_nft(ctx: Context<CreateNft>, height: u8, width: u8, data: Vec<u8>) -> Result<()> {
        level_editor::create_nft(ctx, height, width, data)
    }
}

