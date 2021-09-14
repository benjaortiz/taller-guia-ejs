#[derive(Clone)]
pub struct Letter {
    pub c: char,
    pub guessed: bool,

}

impl Letter {
    // devuelve la letra que debe imprimirse por pantalla si es
    // que fue adivinada, 'None' en caso contrario 
    pub fn get_value_for_print(&self)-> Option<char>{
       
        if self.guessed {
            Some(self.c)
        
        }else{
            None 
        }
    }

    // revisa si se adivinó la letra, cambia su "estado" a adivinado
    // 
    pub fn check_guess(&mut self, guess:char) -> bool {
        let ret;
        if guess == self.c {
            self.guessed = true;
            ret = true;
        } else {
            ret = false;
        }
        ret
    }

    pub fn create_word(vec: Vec<char>) -> Vec<Letter>{
        let mut word = Vec::new();
    
        for i in vec{
        word.push(Letter{c: i,guessed: false});
        }
    
        word
    }

    pub fn get_letter(&self) -> char {
        self.c
    }
}