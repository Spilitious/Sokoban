/* use anchor_lang::prelude::*;

declare_id!("FYjcKSeCtxwWi161uNjmN8cs2ykVtA2YWdpnsyAWjuHK");

#[program]
pub mod projet_final {
    use super::*;

    let mut player_position:u16;
    let mut width:u8;
    let mut height:u8;
    let data: Vec<u8>;
    //let boxes:vec<u16>

    // 0 vide
    // 1 wall
    // 2 player 
    // 3 caisse 
    // 4 position d'arrivée
    // 5 caisse + position d'arrivée
    // 6 bonhomme + position d'arrivée 


    pub fn initialize(ctx: Context<Level>, w:u8, h:u8, g:vect<u8>) -> Result {
            
        data = ; 
        width = w;
        height = h;
        
        for i in g {
            if ground.data[i] = 2 || ground.data[i] == 6
                player_position = i; 
        }

    }


    fn get_case_type(x: usize) -> Result<u8, ErrorCode> {
        if x < data.len() {
            Ok(data[x])
        } else {
            Err(ErrorCode::IndexOutOfBounds)
        }
    }


    //  0 up
    //  1 right
    //  2 down
    //  3 left
    fn move(direction:u8) ->-> Result<bool, ErrorCode> {

       
        let mut inc: i8 = 0;
        
        match direction {
            1 => inc = -(width as i8), // Vers le haut
            2 => inc = 1,              // Vers la droite
            3 => inc = width as i8,    // Vers le bas
            4 => inc = -1,             // Vers la gauche
            _ => return false,         // Direction invalide
            }

                    
        
        // Calculer la nouvelle position du joueur
        let new_position = (*player_position as i32) + inc;

        // Vérifier la sortie de la carte
        if new_position < 0 || new_position >= (width as i32) * (height as i32) {
            return Err(ErrorCode::IndexOutOfBounds);
        }

        //Mur 
        if get_case_type(new_position as u32 == 1) {
            return Ok(false);
        }

        //Déjà une caisse
        if  get_case_type(new_position as u32) == 3 || get_case_type(new_position as u32) == 5    
        {
            let new_position2 = new_position + inc as i32;

            if new_position2 < 0 {
                return Err(ErrorCode::IndexOutOfBounds.into());
            }

            //La caisse ne peux pas bouger
            if get_case_type(new_position as u32) != 0 && get_case_type(new_position as u32) != 4 {
                return Ok(false);
            }

            //Sinon on bouge le bonhomme et la caisse
            *player_position = new_position;

            return Ok(true);
        }
        
        // Vérifier si la case est une position d'arrivée ou une case vide
        if get_case_type(new_position as u32) == 4 || et_case_type(new_position as u32) == 0 {
            
            *player_position = new_position as u32;
             // Mettre à jour la case avec bonhomme ou position d'arrivée
            ground.data[new_position as usize] = if get_case_type(new_position as usize)? == 4 { 6 } else { 2 };

            true
        }

    }
}

#[derive(Accounts)]
pub struct Solve<'info> {
    #[account(init, payer = user, space =  8)] 
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[error_code]
pub enum ErrorCode {
    #[msg("Index out of bounds.")]
    IndexOutOfBounds,
    #[msg("Move impossible.")]
    MoveImpossible,
}





pub fn move_to(direction: u8) -> bool {
    let mut inc: i8 = 0;

    match direction {
        1 => inc = -(game.width as i8), // Vers le haut
        2 => inc = 1,                    // Vers la droite
        3 => inc = game.width as i8,     // Vers le bas
        4 => inc = -1,                   // Vers la gauche
        _ => return ErrorCode::UnknownDirection, // Direction invalide
    }

    // Calculer la nouvelle position du joueur
    let new_position = (game.player_position as i32) + inc as i32;

    // Vérifier la sortie de la carte
    if new_position < 0 || new_position >= (game.width as i32) * (game.height as i32) {
        return ErrorCode::IndexOutOfBounds;
    }

    // Vérifier le mur
    if get_case_type(new_position as usize)? == 1 {
        return false;
    }

    // Vérifier la présence d'une caisse
    if get_case_type(new_position as usize)? == 3 || get_case_type(new_position as usize)? == 5 {
        let new_position2 = new_position + inc as i32;

        if new_position2 < 0 {
            return ErrorCode::IndexOutOfBounds;
        }

        // La caisse ne peut pas bouger
        if get_case_type(new_position as usize)? != 0 && get_case_type(new_position as usize)? != 4 {
            return false;
        }

        // Déplacer le bonhomme et la caisse
        game.player_position = new_position as u16;

        return true;
    }

    // Vérifier si la case est une position d'arrivée ou une case vide
    if get_case_type(new_position as usize)? == 4 || get_case_type(new_position as usize)? == 0 {
        game.player_position = new_position as u16;
        game.map_data[new_position as usize] = if get_case_type(new_position as usize)? == 4 { 6 } else { 2 };

        return true;
    }

    false
}

fn get_case_type( x: usize) -> Result<u8, ErrorCode> {
    if x < game.map_data.len() {
        Ok(game.map_data[x])
    } else {
        Err(ErrorCode::IndexOutOfBounds)
    }
}

}

use anchor_lang::prelude::*;

declare_id!("FYjcKSeCtxwWi161uNjmN8cs2ykVtA2YWdpnsyAWjuHK");

#[program]
pub mod projet_final {
   // use super::*;
   



    impl GameState {
        pub fn new(width: u8, height: u8, map_data: Vec<u8>) -> Self {
            let mut player_position = 0;
            for (index, &value) in map_data.iter().enumerate() {
                if value == 2 || value == 6 {
                    player_position = index as u16;
                    break;
                }
            }
            Self {
                player_position,
                width,
                height,
                map_data,
            }
        }
    }
    
  
    pub fn initialize(ctx: Context<Level>, w: u8, h: u8, g: Vec<u8>) -> Result<> {
        game = GameState::new(w, h, g);
        Ok(())
    }
    
} 


   

 


#[derive(Accounts)]
pub struct Level<'info> {
    #[account(init, payer = user, space = 8)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Index out of bounds.")]
    IndexOutOfBounds,
    #[msg("Impossible move.")]
    MoveImpossible,
    #[msg("Unknown direction.")]
    UnknownDirection,
}
    */