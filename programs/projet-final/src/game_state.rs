use anchor_lang::prelude::*;
//use anchor_lang::solana_program::system_program;


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
       
        //Détermination de la case d'arrivée du joueur
        let  inc: i8;
        
        match direction {
            1 => inc = -(self.width as i8),  // Vers le haut
            2 => inc = 1,                    // Vers la droite
            3 => inc = self.width as i8,     // Vers le bas
            4 => inc = -1,                   // Vers la gauche
            _ => return false,               // Direction invalide
        }

        // Calculer la nouvelle position du joueur
         let new_position = (self.player_position as i32) + inc as i32;
    
        //Le joueur est sur une position d'arrivée 
        let mut reset = 0;
        if self.get_case_type(self.player_position) == 6 {
            reset = 4;
        }
       
        //Vérification de la possibilité du mouvement 
        if new_position < 0 || new_position >= (self.width as i32) * (self.height as i32) {
            return false;
            //return ErrorCode::IndexOutOfBounds;
        }
    
        // Vérifier de la présence d'un le mur
        if self.get_case_type(new_position as u16) == 1 {
            return false;
        }

        
        // Vérifier la présence d'une caisse
        if self.get_case_type(new_position as u16) == 3 || self.get_case_type(new_position as u16) == 5 {
            let new_position2 = new_position + inc as i32;
    
            if new_position2 < 0 {
               return false; 
               //return ErrorCode::IndexOutOfBounds;
            }
    
            // La caisse ne peut pas bouger car bloquer par un mur ou une caisse
            if self.get_case_type(new_position2 as u16) != 0 && self.get_case_type(new_position2 as u16) != 4 {
                return false;
            }
    

            // Déplacement du player  
            self.map_data[self.player_position as usize] = reset;
            self.player_position = new_position as u16;
            if self.get_case_type(new_position as u16) == 5 {
                self.map_data[new_position as usize] = 6;
            } else {
                self.map_data[new_position as usize] = 2;
            }
    
            //Déplacement de la caisse 
            if self.get_case_type(new_position2 as u16) == 0 {
                self.map_data[new_position2 as usize] = 3;
            } else {
                self.map_data[new_position2 as usize] = 5;
            }
            return true;
        }
    

        // Vérifier si la case est une position d'arrivée ou une case vide
        if self.get_case_type(new_position as u16) == 4 || self.get_case_type(new_position as u16) == 0 {
           
            self.map_data[self.player_position as usize] = reset;
            self.player_position = new_position as u16;            
            if self.get_case_type(new_position as u16) == 4 {
                self.map_data[new_position as usize] = 6  
            }
            else { 
                self.map_data[new_position as usize] = 2
            };
    
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