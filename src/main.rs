extern crate jsonrpc;
extern crate serde;
#[macro_use] extern crate serde_derive;

use serde_json::json;

#[derive(Serialize)]
struct GetBlockHashParameter {
    height : i32,
}

fn main() {
    let client = jsonrpc::client::Client::new("http://seed.evonet.networks.dash.org:3000".to_owned(), None, None);
    let mut request = client.build_request("getBestBlockHash", &[]);
    match client.send_request(&request).and_then(|res| res.into_result::<String>()) {
        Ok(result) => println!("getBestBlockHash {}", result),
        Err(e) => println!("error {}", e),
    }

    let param = GetBlockHashParameter { height : 1 };
    let get_block_hash_parameter = vec![json!({"params" : param})];

    request = client.build_request("getBlockHash", &get_block_hash_parameter);
    //println!("params {}", request.params[0]);
    match client.send_request(&request).and_then(|res| res.into_result::<String>()) {
        Ok(result) => println!("getBlockHash {}", result),
        Err(e) => println!("error {}", e),
    }
}
