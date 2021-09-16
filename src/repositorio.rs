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

    pub fn guardar_palabras(&mut self,linea: &Vec<String>) -> Result<i32, String> {

    }

}