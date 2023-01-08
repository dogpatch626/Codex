pub fn encrypt(plain_text: &str, key: &str) -> String{

    let mut result:String = String::from("");
    
    if key.len() == 0{  return plain_text.to_string()}; 
    if plain_text.len() == 0{ return plain_text.to_string() }
    return result; 
}