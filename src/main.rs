extern crate jsonrpc;
extern crate serde;
#[macro_use] extern crate serde_derive;

use serde_json::json;

#[derive(Serialize,Deserialize)]
struct GetBlockHashParameter {
    height : i32,
}

#[derive(Serialize,Deserialize,Debug)]
struct GetAddressSummaryParameter {
    address : Vec<String>,
}

#[derive(Serialize,Deserialize,Debug)]
#[allow(non_snake_case)]
struct GetAddressSummaryResult {
    addrStr : Vec<String>,
    balance : f64,
    balanceSat : i64,
    totalReceived : f64,
    totalReceivedSat : i64,
    totalSent : f64,
    totalSentSat : i64,
    unconfirmedBalance : f64,
    unconfirmedBalanceSat : i64,
    unconfirmedTxApperances : i64,
    unconfirmedAppearances : i64,
    txApperances : i64,
    txAppearances : i64,
    transactions : Vec<String>,
}

#[derive(Serialize,Deserialize,Debug)]
#[allow(non_snake_case)]
struct GetMnListDiffParameter {
    baseBlockHash : String,
    blockHash : String
}

#[derive(Serialize,Deserialize,Debug)]
#[allow(non_snake_case)]
struct Mn{
    proRegTxHash : String,
    confirmedHash : String,
    service : String,
    pubKeyOperator : String,
    votingAddress : String,
    isValid : bool
}

#[derive(Serialize,Deserialize,Debug)]
#[allow(non_snake_case)]
struct GetMnListDiffResult {
    baseBlockHash : String,
    blockHash : String,
    cbTxMerkleTree : String,
    cbTx : String,
    deletedMNs : Vec<Mn>,
    mnList : Vec<Mn>
}

fn main() {
    let client = jsonrpc::client::Client::new("http://seed.evonet.networks.dash.org:3000".to_owned(), None);

    let param = GetAddressSummaryParameter { address : vec![String::from("yPan6DeKoRSzvLBXvdWijh5rWzJGPYUr9B"), String::from("ySnJVXXx9FtKUBTkovPaPPqCkTMNzDLPCu")] };
    let get_address_summary_parameter = json!(param);
    let mut request = client.build_request("getAddressSummary", get_address_summary_parameter);
    match client.send_request(&request).and_then(|res| res.into_result::<GetAddressSummaryResult>()) {
        Ok(result) => println!("{:?}", result),
        Err(e) => println!("error {}", e),
    }

    request = client.build_request("getBestBlockHash", json!({}));
    request.id = json!(1);
    match client.send_request(&request).and_then(|res| res.into_result::<String>()) {
        Ok(result) => println!("getBestBlockHash {}", result),
        Err(e) => println!("error {}", e),
    }
    
    let param = GetBlockHashParameter { height : 5 };
    let get_block_hash_parameter = json!(param);

    request = client.build_request("getBlockHash", get_block_hash_parameter);
    match client.send_request(&request).and_then(|res| res.into_result::<String>()) {
        Ok(result) => println!("getBlockHash {}", result),
        Err(e) => println!("error {}", e),
    }

    let param = GetMnListDiffParameter { baseBlockHash : String::from("2eb140dac7f006a0fbb7e828ad9025e2f1505e394dcf69609ed9dbcd71c35cac")
    , blockHash : String::from("0000009ca7e9ab136d6a324f11cb4142a138fed6ccfa3f80511d7975b804a6df") };
    let get_mnlistdiff_parameter = json!(param);
    request = client.build_request("getMnListDiff", get_mnlistdiff_parameter);
    match client.send_request(&request).and_then(|res| res.into_result::<GetMnListDiffResult>()) {
        Ok(result) => println!("{:?}", result),
        Err(e) => println!("error {}", e),
    }

}
