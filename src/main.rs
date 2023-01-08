mod ceaser_cipher;
mod vigenere_cipher;
fn main() {
   let idk =  ceaser_cipher::decrypt("ifmmp", 1);
   let vig = vigenere_cipher::encrypt("idk", "");
   println!("{}", vig);
    // let mut i = 0; 
    // while(i<5){
    // let eek:String = String::from("idk"); 
    // let toPrint:String = eek.chars().collect();
    // println!("{:#?}", toPrint);
    // i += 1; 
    // }
    
}