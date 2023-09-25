use serde_json;
use std::env;

// Available if you need it!
struct Decoded {
    value: serde_json::Value,
    length: usize,
}

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> Decoded {
    // If encoded_value starts with a digit, it's a number
    if encoded_value.chars().next().unwrap().is_digit(10) {
        // Example: "5:hello" -> "hello"
        let colon_index = encoded_value.find(':').unwrap();
        let number_string = &encoded_value[..colon_index];
        let number = number_string.parse::<i64>().unwrap();
        let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
        Decoded {
            length: string.len() + colon_index + 1,
            value: serde_json::Value::String(string.to_string()),
        }
    } else if encoded_value.chars().next().unwrap() == 'i' {
         //i.45 =  45
        let e_index = encoded_value.find('e').unwrap();
        let number_string = &encoded_value[1..e_index];
        let number = number_string.parse::<i64>().unwrap();
        Decoded {
            length: number_string.len() +2,
            value: serde_json::Value::Number(serde_json::Number::from(number)),
        }
    } else if encoded_value.starts_with('l') {
        // l5:helloi52ee  -> l5:helloi52ee ["hello", 52] 
        let mut list = Vec::new();
        let mut index = 1;
        while encoded_value.chars().nth(index).unwrap() != 'e' {
            let decoded_value = decode_bencoded_value(&encoded_value[index..]);
            list.push(decoded_value.value);
            index += decoded_value.length;
        }

    Decoded { 
        value: serde_json::Value::Array(list), 
        length: index + 1, 
    } 
    } else if encoded_value.starts_with('d') {
        // d3:foo3:bar5:helloi52ee  -> {"hello": 52, "foo":"bar"} 
        let mut map = serde_json::Map::new();
        let mut index = 1;
        while encoded_value.chars().nth(index).unwrap() != 'e' {
            let encoded_value = &encoded_value[index..];
            let decoded_value = decode_bencoded_value(&encoded_value[index..]);
            let decoded_key = decode_bencoded_value(encoded_value);
            index += decoded_value.length + decoded_key.length;
        }
        Decoded {
            value: serde_json::Value::Object(map),
            length : index + 1,
        }
    } else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        // println!("Logs from your program will appear here!");

        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value).value;
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1]) 
    }
}
