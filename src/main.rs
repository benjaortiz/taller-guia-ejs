mod letter;
mod ahorcado;
use ahorcado::Ahorcado;

fn main() {
    println!("Bienvenido al ahorcado!");

    let letters = ahorcado::Ahorcado::load_word("data/palabras.txt");
    
    //imprimo algunas boludeces
 //   println!("\nla palabra tiene {} letras\n", letters.len());

 //   println!("{:?}", letters);
   
    //me armo el ahorcado
    let mut game = Ahorcado::create_ahorcado(letters);

    while game.remaining_lives() > 0 {
        game.play_round();
    }
}
/*
    UNA RONDA:
    1. printeo la pantalla de inicio
   
    2. le pido una letra al jugador
        2.1 valido que sea una letra 
        2.2 valido que no la haya tirado antes
   
    3. Chequeo si la letra esta en la palabra
        3.1 si está:
            3.1.1 seteo como adivinada la/las letra/s
    
        3.2 Si no está:
            3.2.1 le resto 1 intento al jugador
        
        3.3 agrego la letra a la lista de adivinadas
    4. imprimo mensaje de fin de ronda
    5. finalizo ronda: 
        5.1 si quedan intentos vuelvo a "2." (LOOP)
        
        5.2 si no quedan intentos: 
            5.2.1 indicó que el jugador perdió el juego
            5.2.2 muestro cual era la palabra
*/

