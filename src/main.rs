use serde_bencode::{de, value::Value as BencodeValue};
use serde_bytes::ByteBuf;
use serde_json::{Map, Value as JsonValue};
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Torrent {
    announce: String,
    info: Info,
}

#[derive(Serialize, Deserialize, Debug)]
struct Info {
    length: usize,
    name: String,
    #[serde(rename = "piece length")]
    pieces_length: usize,
    pieces: ByteBuf,
}

fn to_json(value: &BencodeValue) -> JsonValue {
    match value {
        BencodeValue::Bytes(bytes) => JsonValue::String(String::from_utf8_lossy(bytes).to_string()),
        BencodeValue::Int(num) => JsonValue::Number(num.to_owned().into()),
        BencodeValue::List(list) => JsonValue::Array(list.iter().map(|v| to_json(v)).collect()),
        BencodeValue::Dict(dict) => {
            let mut json_dict = Map::new();
            for (key, val) in dict.iter(){
                let key = String::from_utf8(key.clone().into()).unwrap().to_string();
                let val = to_json(val);
                json_dict.insert(key, val);
            }
            JsonValue::Object(json_dict)}
    }
}
// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value: BencodeValue = de::from_str(encoded_value).unwrap();
        println!("{}", to_json(&decoded_value)); 
    } else if command  == "Info" {
        let torrent_file = &args[2];
        let file_buf = std::fs::read(torrent_file).unwrap();
        let torrent = de::from_bytes::<Torrent>(&file_buf).unwrap(); 

        println!("Tracker URL: {}", torrent.announce);
        println!("Length: {}", torrent.info.length);
    } else {
        println!("unknown command: {}", args[1]) 
    }
}