use std::fs::File;
use std::io::{BufReader, BufRead, stdin};

use crate::letter;

#[derive(Clone)]
pub struct Ahorcado {
    word: Vec<letter::Letter>,
    tries: u32,
    guesses: Vec<char>,
    guessed_word: bool
}

impl Ahorcado {
 
    pub fn create_ahorcado(palabra:Vec<char>) -> Ahorcado{
        Ahorcado {
            word: letter::Letter::create_word(palabra),
            tries : 5,
            guesses: Vec::new(),
            guessed_word: false
        }
    }
    
    pub fn print_word(&self) {

        let mut cw;
        for i in 0..self.word.len(){
            
            cw = match self.word[i].get_value_for_print() {
                 Some(c) => c,
                 None => '_'
            };   
            print!(" {}",cw);
        }
        println!("");
    }

    pub fn load_word(path:&str) -> Vec<char>{
        let file = File::open(path).expect("error abriendo el archivo");

        let mut buffer = BufReader::new(file);
        let mut word = String::new();

        buffer.read_line(&mut word).expect("error leyendo el archivo");

        word.trim().chars().collect()
    }

    pub fn play_round(&mut self){

        self.print_round_message();

        let mut w:char = self.ask_for_a_guess();

        while w.to_ascii_lowercase().is_ascii_alphabetic() == false {
            println!("No se ingresó una letra valida");
            w = self.ask_for_a_guess();
        }

        self.make_guess(w);
        self.check_for_the_win();

    }

    pub fn print_round_message(&self){
        print!("La palabra hasta el momento es:");
        self.print_word();

        print!("Adivinaste las siguientes letras:");
        println!("{:?}", self.guesses);

        println!("te quedan {} intentos.", self.tries);
    }

    pub fn ask_for_a_guess(&self) -> char {
        println!("ingresá una letra");
        
        //creo un string donde guardar lo que se va a leer por stdin
        // y despues leo lo que se ingresa
        let mut v:String = String::new();
        stdin().read_line(&mut v).expect("Error leyendo la linea.");

        //aca no hago un trim porque solo me interesa el primer
        //caracter, asique lo paso a vector de chars y despues
        //devuelvo el primero
        let guess:Vec<char> = v.chars().collect();

        guess[0]
    }

    fn make_guess(&mut self, guess:char) {
        let mut correct:i32 = 0;
        
        for i in 0..self.word.len() {
            if self.word[i].check_guess(guess){
                correct = correct + 1;
            }
        }
        // resto una vida si la letra no forma parte de la palabras
        if correct == 0 {
            self.tries -= 1;
        }
        //agrego la letra a la lista de palabras ingresadas
        if !self.guesses.contains(&guess){
            self.guesses.push(guess);
        }
    }
/*
    pub fn remaining_lives(&self) -> u32 {
        self.tries
    }
*/
    pub fn is_ongoing(&self) -> bool {
        
        !self.guessed_word && self.tries > 0
    }

    pub fn check_for_the_win(&mut self) {
        let mut printed_guess: Vec<char> = Vec::new();
        let mut a: Vec<char> = Vec::new();

        for i in 0..self.word.len(){
            
            printed_guess.push( match self.word[i].get_value_for_print() {
                                    Some(c) => c,
                                    None => '_'
                                });
            
            a.push(self.word[i].get_letter());
        }

        let s: String = a.iter().collect();
        let v: String = printed_guess.iter().collect();

        let matching = s.chars().zip(v.chars()).filter(|&(s, v)| s == v).count();

        if matching == self.clone().word.len(){
            self.guessed_word = true;
        }
    }
}