extern crate jsonrpc;
extern crate serde;
#[macro_use] extern crate serde_derive;

use serde_json::json;

#[derive(Serialize,Deserialize)]
struct GetBlockHashParameter {
    height : i32,
}

fn main() {
    let client = jsonrpc::client::Client::new("http://seed.evonet.networks.dash.org:3000".to_owned(), None);
    let mut request = client.build_request("getBestBlockHash", json!(""));
    request.id = json!(1);
    match client.send_request(&request).and_then(|res| res.into_result::<String>()) {
        Ok(result) => println!("getBestBlockHash {}", result),
        Err(e) => println!("error {}", e),
    }

    let param = GetBlockHashParameter { height : 5 };
    let get_block_hash_parameter = json!(param);

    request = client.build_request("getBlockHash", get_block_hash_parameter);
    //println!("params {}", request.params[0]);
    match client.send_request(&request).and_then(|res| res.into_result::<String>()) {
        Ok(result) => println!("getBlockHash {}", result),
        Err(e) => println!("error {}", e),
    }
    
}
