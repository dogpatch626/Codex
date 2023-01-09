pub fn encrypt(plain_text: &str, key: &str) -> String{

    let mut result:String = String::from("");
    let key_len = key.len();
    if key_len == 0{  return plain_text.to_string()}; 
    if plain_text.len() == 0{ return plain_text.to_string() }

    let filtered_key = key.to_ascii_lowercase(); 
    let filtered_plain_text = plain_text.to_ascii_lowercase(); 

    filtered_plain_text.chars().map(|c| {
    
    });

    return result; 
}