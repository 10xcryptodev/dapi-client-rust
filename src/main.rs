extern crate jsonrpc;
extern crate serde_derive;
extern crate serde;

fn main() {
    let client = jsonrpc::client::Client::new("http://seed.evonet.networks.dash.org:3000".to_owned(), None, None);
    let request = client.build_request("getBestBlockHash", &[]);
    match client.send_request(&request).and_then(|res| res.into_result::<String>()) {
        Ok(result) => println!("getBestBlockHash {}", result),
        Err(e) => println!("error {}", e),
    }
}
