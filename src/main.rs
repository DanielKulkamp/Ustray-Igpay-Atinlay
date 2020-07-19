//use std::io;

fn main() {
    println!("Elcomway otay Ustray Igpay Atinlay");
    println!("Enterhay thehay entenceay otay ranslatetay");
    let mut input = String::new();
    loop {
    	match std::io::stdin().read_line(&mut input) {
    		Err(_) => {
    			println!("rytay againhay...");
    			continue;
    		}
    		Ok(_) => {
    			break;
    		}
    	}
    }
    let mut words : Vec<String> = Vec::new();
    for word in input.split_whitespace() {
    	words.push(word.to_string());
    }
    for word in words.iter_mut() {
    	let mut new_word = String::new();
    	let mut chars = word.char_indices();
    	let mut sufix = String::new();
    	match chars.next(){
    		Some((_, character)) => {
    			match character.to_ascii_lowercase() {
    				'a' | 'e' | 'i' | 'o' | 'u' => {
    					sufix = "hay".to_string();
    					new_word.push(character);
    				},
    				_ => {
    					sufix.push(character);
    					sufix.push_str("ay");
    				}
     			}
     			match chars.next(){
     				Some((i, _)) => {
     					new_word.push_str(&word[i..]);
     					new_word.push_str(&sufix);
     				},
     				None => new_word.push_str("ay"),
     			}
    		},
    		None => (),
     	}
     	*word = new_word;
    }
    for word in words.iter() {
     	print!("{} ", word);
    }
    println!("");

}

   