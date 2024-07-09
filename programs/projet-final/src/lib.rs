use anchor_lang::prelude::*;

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

/*
        for (index, &value) in map_data.iter().enumerate() {
            if value == 2 || value == 6 {
                game.player_position = index as u16;
                break;
            }
        }
        */
        Ok(())

    }

    pub fn solve(ctx: Context<Initialize>, directions: Vec<u8>) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.run_sequence(directions);
        game.verify();
        return Ok(());
    }


}

#[derive(Accounts)]
#[instruction(height: u8, width: u8)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8 + 8 + 8 + 1 +400000+ (height as usize * width as usize))]
    pub game: Account<'info, GameState>,
  
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GameState {
    pub player_position: u16,
    pub width: u8,
    pub height: u8,
    pub map_data: Vec<u8>,
    pub solved:bool,
}
// 0 vide
    // 1 wall
    // 2 player 
    // 3 caisse 
    // 4 position d'arrivée
    // 5 caisse + position d'arrivée
    // 6 bonhomme + position d'arrivée 


impl GameState {

    /*
    pub fn init(width: u8, height: u8, map_data: Vec<u8>) -> Self {
        let mut player_position = 0;
        for (index, &value) in map_data.iter().enumerate() {
            if value == 2 || value == 6 {
                player_position = index as u16;
                break;
            }
        }

        let solved:bool = false; 

        Self {
            player_position,
            width,
            height,
            map_data,
            solved,
        }
    }
        */

    pub fn get_case_type(&self, x:u16) -> u8 {
        if x < (self.width as u16) *(self.height as u16) {
            return self.map_data[x as usize];
        }
        return 1; 
    }
    
    pub fn move_to(&mut self, direction: u8) -> bool {
        let  inc: i8;
    
        match direction {
            1 => inc = -(self.width as i8), // Vers le haut
            2 => inc = 1,                    // Vers la droite
            3 => inc = self.width as i8,     // Vers le bas
            4 => inc = -1,                   // Vers la gauche
            _ => return false, // Direction invalide
        }
    
        // Calculer la nouvelle position du joueur
        let new_position = (self.player_position as i32) + inc as i32;
    
        // Vérifier la sortie de la carte
        //      if new_position < 0 || new_position >= (self.width as i32) * (self.height as i32) {
        //        return ErrorCode::IndexOutOfBounds;
        //   }
    
        // Vérifier le mur
        if self.get_case_type(new_position as u16)== 1 {
            return false;
        }
    
        // Vérifier la présence d'une caisse
        if self.get_case_type(new_position as u16) == 3 || self.get_case_type(new_position as u16) == 5 {
            let new_position2 = new_position + inc as i32;
    
        //     if new_position2 < 0 {
         //       return ErrorCode::IndexOutOfBounds;
          //  }
    
            // La caisse ne peut pas bouger
            if self.get_case_type(new_position as u16) != 0 && self.get_case_type(new_position as u16) != 4 {
                return false;
            }
    
            // Déplacer le bonhomme et la caisse
            self.player_position = new_position as u16;
            if self.get_case_type(new_position as u16) == 4 {
                self.map_data[new_position2 as usize] = 5;
            }else {
                self.map_data[new_position2 as usize] = 2;
            }
    
            return true;
        }
    
        // Vérifier si la case est une position d'arrivée ou une case vide
        if self.get_case_type(new_position as u16) == 4 || self.get_case_type(new_position as u16) == 0 {
            self.player_position = new_position as u16;
             
            if self.get_case_type(new_position as u16) == 4 {
                self.map_data[new_position as usize] = 6  
            }
           
            else { self.map_data[new_position as usize] = 2   };
    
            return true;
        }
    
        false
    }

    pub fn run_sequence(&mut self, directions:Vec<u8>) -> bool {
        for i in directions {
            if !self.move_to(i) {
              return false; }
        }   
        return true;
    }

    pub fn verify(&mut self) -> bool {
        for i in &self.map_data {
            if self.get_case_type(*i as u16) == 3 {
                self.solved = false;
                return false;
            }
        }

        self.solved = true;
        return true;
    }

}

#[error_code]
pub enum ErrorCode {
    #[msg("Index out of bounds.")]
    IndexOutOfBounds,
    #[msg("Impossible move.")]
    MoveImpossible,
    #[msg("Unknown direction.")]
    UnknownDirection,
    #[msg("Wrong data.")]
    InitialisationFailed,
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