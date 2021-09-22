
pub fn formatear_linea(s: &String) -> Vec<String>{

    let fl = s.split_whitespace();
    let mut ret = Vec::new();

    for i in fl{
        ret.push(i.trim().to_lowercase());
    }
    ret
}