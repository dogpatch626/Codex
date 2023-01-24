use wasm_bindgen::prelude::*;
// ceaser cypher encryption formula En(x) = (x+n)mod26
#[allow(dead_code)]
#[wasm_bindgen]
pub fn encrypt(plain_text: &str, key: u8) -> String {
    let result = plain_text
        .chars()
        .map(|c| {
            let character;
            if c.is_ascii_alphabetic() {
                character = if c.is_ascii_uppercase() {
                    ((((c as u8) + key - 65) % 26) + 65) as char
                } else {
                    ((((c as u8) + key - 97) % 26) + 97) as char
                };
                return character;
            } else {
                c
            }
        })
        .collect();
    return result;
}
// ceaser cypher decryption formula En(x) = (x-n)mod26

#[allow(dead_code)]
#[wasm_bindgen]
pub fn decrypt(plain_text: &str, key: u8) -> String {
    let mut result = String::from("");
    for element in plain_text.chars() {
        let mut character = element as u8;
        //encrypt

        if element.is_uppercase() {
            character = ((character - key - 65) % 26) + 65;

            result.push(character as char);
        } else {
            character = ((character - key - 97) % 26) + 97;

            result.push(character as char);
        }
    }
    return result;
}