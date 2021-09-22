use std::collections::HashMap;


pub struct Repositorio {
    repo: HashMap<String,i32>
}

impl Repositorio {
    pub fn crear_repo() -> Repositorio {
        Repositorio {
            repo : HashMap::new()
        }
    }

    pub fn guardar_palabras(&mut self,linea: &Vec<String>) {

        for i in linea {
            *self.repo.entry(i.to_string()).or_insert(0) += 1;
        }
    }

    pub fn obtener_listado_de_elementos(&mut self) -> Vec<(String,i32)> {

        self.repo.iter().map(|(key,value)|{
            (key.clone().to_string(), *value)
        }).collect::<Vec<(String,i32)>>()
    }

    pub fn imprimir_frecuencia(&mut self){
        let mut palabras = self.obtener_listado_de_elementos();

        palabras.sort_by(|a,b| b.1.cmp(&a.1));

        for (word,freq) in palabras {
            println!("{} => {}", word, freq);
        }

    }
}