use serde_bencode::{de, value::Value as BencodeValue};
use serde_bytes::ByteBuf;
use serde_json::{Map, Value as JsonValue};
use serde::{Serialize, Deserialize};
use std::env;
use sha1::{Digest, Sha1};
use hex;
use url::form_urlencoded;

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
                let key = String::from_utf8(key.clone()).unwrap().to_string();
                let val = to_json(val);
                json_dict.insert(key, val);
            }
            JsonValue::Object(json_dict)}
    }
}

fn info_hash(info: &Info) -> String {
    let mut hasher = Sha1::new();
    hasher.update(serde_bencode::to_bytes(&info).unwrap());
    hex::encode(hasher.finalize())
}

fn make_tracker_request(tracker_url: &str, info_hash: &str, peer_id: &str, total_length: usize) -> Result<(), reqwest::Error> {
    let left = total_length.to_string();

    let mut params = form_urlencoded::Serializer::new(String::new());
    params.append_pair("info_hash", info_hash);
    params.append_pair("peer_id", peer_id);
    params.append_pair("port", "6881");
    params.append_pair("uploaded", "0");
    params.append_pair("downloaded", "0");
    params.append_pair("left", &left);
    params.append_pair("compact", "1");

    let request_url =format!("{}?{}", tracker_url, params.finish());

    //Mak the GET request to the tracker
    let response= reqwest::blocking::get(&request_url)?;

    if response.status().is_success() {
        let response_body = response.bytes()?;
        println!("Response body: {:?}", response_body);
    } else {
        println!("Error: Request failed with status {:?}", response.status());
    }
    Ok(())

}
// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    let peer_id = "00112233445566778899";
    let total_length = 12335;

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value: BencodeValue = de::from_str(encoded_value).unwrap();
        println!("{}", to_json(&decoded_value)); 
    } else if command  == "info" {
        let file_name = &args[2];
        let file_buf = std::fs::read(file_name).unwrap();
        let torrent = de::from_bytes::<Torrent>(&file_buf).unwrap(); 

        let tracker_url = &torrent.announce;
        let info_hash = info_hash(&torrent.info);

        println!("Tracker URL: {}", torrent.announce);
        println!("Length: {}", torrent.info.length);
        println!("Info Hash: {}", info_hash);
        println!("Piece Length: {}", torrent.info.pieces_length);

        println!("Piece Hash: ");
        for chunk in torrent.info.pieces.chunks(20) {
            let hash_hex: String = hex::encode(chunk);
            println!("{}", hash_hex);
        }

        if let Err(err) = make_tracker_request(tracker_url, &info_hash, peer_id, total_length) {
            println!("Error: {:?}", err);
        }
    } else {
        println!("unknown command: {}", args[1]) 
    }
}
