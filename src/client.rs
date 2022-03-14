
use std::str::FromStr;
use std::string::ToString;


use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::io;


use bytes::Bytes;

use futures::future::{BoxFuture, FutureExt};
use bigdecimal::{BigDecimal, ToPrimitive};

use serde::{Serialize, Deserialize};

use either::Either;

use hex;
use web3::{
    contract, contract::Contract, contract::Options, ethabi::Token, types::Address, types::U256, *,
};
use futures_retry::{RetryPolicy, StreamRetryExt, FutureRetry};



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
  pub t: String,
  pub d: u8,
  pub name: String,
  pub symbol: String,
}

impl Asset {
  pub fn new(t: String, d: u8, name: String, symbol: String) -> Asset {
      Asset { t, d, name, symbol }
  }
}

fn handle_network_err<E>(_e: E) -> RetryPolicy<io::Error> {
  RetryPolicy::Repeat
}

async fn try_fetch_token_data(
  token_contract: &Contract<transports::Http>,
) -> std::result::Result<Asset, web3::contract::Error> {
  let result = tokio::try_join!(
      token_contract.query("decimals", (), None, Options::default(), None),
      token_contract.query("name", (), None, Options::default(), None),
      token_contract.query("symbol", (), None, Options::default(), None),
  );
  match result {
      Ok(v) => {
          let (d, name, symbol) = v;
          Ok(Asset::new(
              hex::encode(token_contract.address()).to_string(),
              d,
              name,
              symbol,
          ))
      }
      Err(e) => Err(e),
  }
}

async fn fetch_token_data(token_contract: &Contract<transports::Http>) -> Asset {
  let (result, _) = FutureRetry::new(
      move || try_fetch_token_data(token_contract),
      handle_network_err,
  )
  .await
  .unwrap();
  result
}


pub struct Props {
  pub node_rpc: String, //
}


pub struct Client {
  props: Props,
  web3: Web3<web3::transports::Http>,
  uniswap_pair_abi: Vec<u8>,
  erc20_abi: Vec<u8>,
}


struct InternalToken(&'static str, u8);

impl Client {

  pub fn decode_addr(address: &str) -> Address {
    let address = if address[0..2] == String::from("0x") {
        &address[2..]
    } else {
        address
    };
    let dc = hex::decode(address).unwrap();
    Address::from_slice(dc.as_slice())
}

  pub fn build_contract(path: Either<&str, Vec<u8>>, address: &str) -> Contract<web3::transports::Http> {
    let endpoint = std::env::var("RPC").unwrap();
    let transport = transports::Http::new(endpoint.as_str()).unwrap();
    let web3 = Web3::new(transport);

    let file_abi = match path {
      Either::Left(path) => {
        let file_abi = fs::read(Path::new(path)).unwrap();
        file_abi
      },
      Either::Right(file_abi) => {
        file_abi
      }
    };

    let decoded_address = Self::decode_addr(address);
    Contract::from_json(web3.eth(), decoded_address, file_abi.as_slice()).unwrap()
  }

  pub async fn new(props: Props) -> Self {
    // let endpoint = std::env::var(props.node_rpc).unwrap();
    let transport = web3::transports::Http::new(props.node_rpc.as_str()).unwrap();
    let web3 = web3::Web3::new(transport);

    // const PANCAKESWAPV2_ROUTER: &str = "10ed43c718714eb63d5aa57b78b54704e256024e";
    // // let decoded_address = hex::decode(PANCAKESWAPV2_ROUTER).expect("Decoding failed");
    // let decoded_address = decode_addr(PANCAKESWAPV2_ROUTER);
    // let from = decode_addr("3718eCd4E97f4332F9652D0Ba224f228B55ec543");

    let uniswap_pair_abi = tokio::fs::read(Path::new("./abi/UniswapV2Pair.json")).await.unwrap();
    let erc20_abi = tokio::fs::read(Path::new("./abi/ERC20.json")).await.unwrap();


    Client { 
      web3,
      props,
      uniswap_pair_abi,
      erc20_abi
    }
  }


  // fgSpiLP = { tid: '0x25f5b3840d414a21c4fc46d21699e54d48f75fdd', dec: 18 }
  // fToken = { tid: '0x21be370d5312f44cb42ce377bc9b8a0cef1a4c83', dec: 18 }
  // gToken = { tid: '0xC1Be9a4D5D45BeeACAE296a7BD5fADBfc14602C4', dec: 18 }
  // uToken = { tid: '0x04068da6c83afcfa0e13ba15a6696662335d5b75', dec: 6 }
  // fuSpoLP = { tid: '0x2b4c76d0dc16be1c31d4c1dc53bf9b45987fc75c', dec: 18 }
  // ugSpoLP = { tid: '0xb9b452A71Dd1cfB4952d90e03bf701A6C7Ae263b', dec: 18 }

  const fgSpiLP: &'static InternalToken = &InternalToken("0x25f5b3840d414a21c4fc46d21699e54d48f75fdd", 18);
  const fToken: &'static InternalToken = &InternalToken("0x21be370d5312f44cb42ce377bc9b8a0cef1a4c83", 18);
  const gToken: &'static InternalToken = &InternalToken("0xC1Be9a4D5D45BeeACAE296a7BD5fADBfc14602C4", 18);
  const uToken: &'static InternalToken = &InternalToken("0x04068da6c83afcfa0e13ba15a6696662335d5b75", 6);
  const fuSpoLP: &'static InternalToken = &InternalToken("0x2b4c76d0dc16be1c31d4c1dc53bf9b45987fc75c", 18);
  const ugSpoLP: &'static InternalToken = &InternalToken("0xb9b452A71Dd1cfB4952d90e03bf701A6C7Ae263b", 18);

  const gcAccs: &'static [&'static str; 3] = &[
    "0xB3D22267E7260ec6c3931d50D215ABa5Fd54506a",
    "0xbb652A9FAc95B5203f44aa3492200b6aE6aD84e0",
    "0xd196B496425Be880BA63AcE90C60258b9A52b044"
  ];

  async fn try_fetch_balance_of(
    token_contract: Contract<web3::transports::Http>,
    holder: &str,
  ) -> std::result::Result<U256, web3::contract::Error> {
    let r = token_contract.query(
        "balanceOf",
        Self::decode_addr(holder),
        None,
        Options::default(),
        None,
    ).await;
    r
  } 

  /** RPC FUNCTIONS **/
  pub async fn get_erc20token_balance<'a>(&self, token: &'a str, holder: &'a str) -> f64 {
    let token_contract = Self::build_contract(Either::Right(self.uniswap_pair_abi.clone()), token);
    let token_data = fetch_token_data(&token_contract).await;

    // let pointer = Box::pin(token_contract.clone());
    let (result, _) = FutureRetry::new(
        move || { Self::try_fetch_balance_of(token_contract.clone(), holder) },
        handle_network_err,
    )
    .await
    .unwrap();

    // let resp: U256 = balance_of_holder.unwrap();

    let r = result as U256;
    let r = BigDecimal::from_str(r.to_string().as_str()).unwrap();
    let r = r.to_f64().unwrap();
    let mult = 10f64.powf(token_data.d as f64);

    r / mult
  }
}

