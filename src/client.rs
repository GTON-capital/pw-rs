use std::str::FromStr;
use std::string::ToString;

use std::fs;
use std::io;
use std::path::Path;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use bigdecimal::{BigDecimal, ToPrimitive};

use serde::{Deserialize, Serialize};

use either::Either;

use futures_retry::{FutureRetry, RetryPolicy};
use hex;
use web3::{contract::Contract, contract::Options, types::Address, types::U256, *};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub t: String,
    pub d: u8,
    pub name: String,
    pub symbol: String,
    pub ts: U256,
}

impl Asset {
    pub fn new(t: String, d: u8, name: String, symbol: String, ts: U256) -> Asset {
        Asset {
            t,
            d,
            name,
            symbol,
            ts,
        }
    }
}

fn handle_network_err<E>(_e: E) -> RetryPolicy<io::Error> {
    RetryPolicy::WaitRetry(Duration::from_millis(70))
}

async fn try_fetch_token_data(
    token_contract: &Contract<transports::Http>,
) -> std::result::Result<Asset, web3::contract::Error> {
    let result = tokio::try_join!(
        token_contract.query("decimals", (), None, Options::default(), None),
        token_contract.query("name", (), None, Options::default(), None),
        token_contract.query("symbol", (), None, Options::default(), None),
        token_contract.query("totalSupply", (), None, Options::default(), None),
    );
    match result {
        Ok(v) => {
            let (d, name, symbol, ts) = v;
            Ok(Asset::new(
                hex::encode(token_contract.address()).to_string(),
                d,
                name,
                symbol,
                ts,
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

#[derive(Clone)]
pub struct Props {
    pub node_rpc: String, //
}

#[derive(Clone)]
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

    pub fn build_contract(
        &self,
        path: Either<&str, Vec<u8>>,
        address: &str,
    ) -> Contract<web3::transports::Http> {
        // let endpoint = std::env::var("RPC").unwrap();
        let transport = transports::Http::new(self.props.node_rpc.as_str()).unwrap();
        let web3 = Web3::new(transport);

        let file_abi = match path {
            Either::Left(path) => {
                let file_abi = fs::read(Path::new(path)).unwrap();
                file_abi
            }
            Either::Right(file_abi) => file_abi,
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

        let uniswap_pair_abi = tokio::fs::read(Path::new("./abi/UniswapV2Pair.json"))
            .await
            .unwrap();
        let erc20_abi = tokio::fs::read(Path::new("./abi/ERC20.json"))
            .await
            .unwrap();

        Client {
            web3,
            props,
            uniswap_pair_abi,
            erc20_abi,
        }
    }

    // fgSpiLP = { tid: '0x25f5b3840d414a21c4fc46d21699e54d48f75fdd', dec: 18 }
    // fToken = { tid: '0x21be370d5312f44cb42ce377bc9b8a0cef1a4c83', dec: 18 }
    // gToken = { tid: '0xC1Be9a4D5D45BeeACAE296a7BD5fADBfc14602C4', dec: 18 }
    // uToken = { tid: '0x04068da6c83afcfa0e13ba15a6696662335d5b75', dec: 6 }
    // fuSpoLP = { tid: '0x2b4c76d0dc16be1c31d4c1dc53bf9b45987fc75c', dec: 18 }
    // ugSpoLP = { tid: '0xb9b452A71Dd1cfB4952d90e03bf701A6C7Ae263b', dec: 18 }

    const fgSpiLP: &'static InternalToken =
        &InternalToken("0x25f5b3840d414a21c4fc46d21699e54d48f75fdd", 18);
    const fToken: &'static InternalToken =
        &InternalToken("0x21be370d5312f44cb42ce377bc9b8a0cef1a4c83", 18);
    const gToken: &'static InternalToken =
        &InternalToken("0xC1Be9a4D5D45BeeACAE296a7BD5fADBfc14602C4", 18);
    const uToken: &'static InternalToken =
        &InternalToken("0x04068da6c83afcfa0e13ba15a6696662335d5b75", 6);
    const fuSpoLP: &'static InternalToken =
        &InternalToken("0x2b4c76d0dc16be1c31d4c1dc53bf9b45987fc75c", 18);
    const ugSpoLP: &'static InternalToken =
        &InternalToken("0xb9b452A71Dd1cfB4952d90e03bf701A6C7Ae263b", 18);

    const gcAccs: &'static [&'static str; 3] = &[
        "0xB3D22267E7260ec6c3931d50D215ABa5Fd54506a",
        "0xbb652A9FAc95B5203f44aa3492200b6aE6aD84e0",
        "0xd196B496425Be880BA63AcE90C60258b9A52b044",
    ];

    async fn try_fetch_balance_of(
        token_contract: Contract<web3::transports::Http>,
        holder: &str,
    ) -> std::result::Result<U256, web3::contract::Error> {
        let r = token_contract
            .query(
                "balanceOf",
                Self::decode_addr(holder),
                None,
                Options::default(),
                None,
            )
            .await;
        r
    }

    /** RPC FUNCTIONS **/
    async fn get_erc20token_balance<'a>(&self, token: &'a str, holder: &'a str) -> f64 {
        let token_contract =
            self.build_contract(Either::Right(self.uniswap_pair_abi.clone()), token);
        let token_data = fetch_token_data(&token_contract).await;

        let result = Self::try_fetch_balance_of(token_contract.clone(), holder)
            .await
            .unwrap();

        let r = result as U256;
        let r = BigDecimal::from_str(r.to_string().as_str()).unwrap();
        let r = r.to_f64().unwrap();
        let mult = 10f64.powf(token_data.d as f64);

        r / mult
    }

    async fn get_erc20token_supply<'a>(&self, token: &'a str) -> f64 {
        let token_contract =
            self.build_contract(Either::Right(self.uniswap_pair_abi.clone()), token);
        let token_data = fetch_token_data(&token_contract).await;

        let r = token_data.ts as U256;
        let r = BigDecimal::from_str(r.to_string().as_str()).unwrap();
        let r = r.to_f64().unwrap();
        let mult = 10f64.powf(token_data.d as f64);

        r / mult
    }

    pub async fn get_wftm_price(&self) -> f64 {
        let u_token = Self::uToken.clone();
        let fu_spo_lp = Self::fuSpoLP.clone();
        let f_token = Self::fToken.clone();

        println!("{:?}", u_token.0);
        println!("{:?}", fu_spo_lp.0);
        println!("{:?}", f_token.0);

        let amount_u = self.get_erc20token_balance(u_token.0, fu_spo_lp.0).await;
        let amount_f = self.get_erc20token_balance(f_token.0, fu_spo_lp.0).await;

        amount_u / amount_f
    }

    pub async fn get_wftm_gton_gc_pool_lp(&'static self) -> f64 {
        let fg_spi_LP = Self::fgSpiLP.clone();
        let gc_accs = Self::gcAccs.clone();

        let n = gc_accs.len();

        let mut handles = Vec::new();
        let sum_of = Arc::new(RwLock::new(0.0));

        for i in 0..n {
            let sum_of_c = Arc::clone(&sum_of);

            let handle = tokio::task::spawn(async move {
                let x = gc_accs[i];
                let r = self.get_erc20token_balance(fg_spi_LP.0, x).await;
                *sum_of_c.write().unwrap() += r;
            });

            handles.push(handle);
        }

        for h in handles {
            h.await.unwrap();
        }

        Arc::try_unwrap(sum_of).unwrap().into_inner().unwrap()
    }

    pub async fn get_usdc_gton_gc_pool_lp(&'static self) -> f64 {
        let ug_spo_lp = Self::ugSpoLP.clone();
        let gc_accs = Self::gcAccs.clone();

        let n = gc_accs.len();

        let mut handles = Vec::new();
        let sum_of = Arc::new(RwLock::new(0.0));

        for i in 0..n {
            let sum_of_c = Arc::clone(&sum_of);

            let handle = tokio::task::spawn(async move {
                let x = gc_accs[i];
                let r = self.get_erc20token_balance(ug_spo_lp.0, x).await;
                *sum_of_c.write().unwrap() += r;
            });

            handles.push(handle);
        }

        for h in handles {
            h.await.unwrap();
        }

        Arc::try_unwrap(sum_of).unwrap().into_inner().unwrap()
    }

    pub async fn get_ftm_gton_liq(&self) -> f64 {
        let fg_spi_lp = Self::fgSpiLP.clone();
        let f_token = Self::fToken.clone();

        let wftm_price = self.get_wftm_price().await;
        let f_token_balance_of_fg_spi_lp =
            self.get_erc20token_balance(f_token.0, fg_spi_lp.0).await;

        2.0 * wftm_price * f_token_balance_of_fg_spi_lp
    }

    pub async fn get_usdc_gton_liq(&self) -> f64 {
        let g_token = Self::gToken.clone();
        let ug_spo_lp = Self::ugSpoLP.clone();

        let balance_of = self.get_erc20token_balance(g_token.0, ug_spo_lp.0).await;

        2.0 * balance_of
    }

    // def getFtmGtonLP(self):
    //   tid = self.tid
    //   dec = self.dec
    //   fgSpiLP = self.fgSpiLP

    //   return self.apiFtmSanGetTokenSupply(fgSpiLP[tid], fgSpiLP[dec])

    pub async fn get_ftm_gton_lp(&self) -> f64 {
        let fg_spi_lp = Self::fgSpiLP.clone();

        self.get_erc20token_supply(fg_spi_lp.0).await
    }

    pub async fn get_usdc_gton_lp(&self) -> f64 {
        let ug_spo_lp = Self::ugSpoLP.clone();

        self.get_erc20token_supply(ug_spo_lp.0).await
    }

    // def getGCpol(self):
    // return sum([self.getFtmGtonLiq()*self.getFtmGtonGCpolLP()/self.getFtmGtonLP(), self.getUsdGtonLiq()*self.getUsdGtonGCpolLP()/self.getUsdGtonLP()])

    pub async fn get_gc_pol(&'static self) -> f64 {
        let (ftm_gton_liq, ftm_gton_gc_pol_lp, ftm_gton_lp, usdc_gton_liq, usdc_gton_lp, usdc_gton_gc_pol_lp): (
            f64,
            f64,
            f64,
            f64,
            f64,
            f64
        ) = tokio::join!(
            self.get_ftm_gton_liq(),
            self.get_wftm_gton_gc_pool_lp(),
            self.get_ftm_gton_lp(),
            self.get_usdc_gton_liq(),
            self.get_usdc_gton_lp(),
            self.get_usdc_gton_gc_pool_lp()
        );

        let sum_of = vec![
            ftm_gton_liq * ftm_gton_gc_pol_lp / ftm_gton_lp,
            usdc_gton_liq * usdc_gton_gc_pol_lp / usdc_gton_lp,
        ]
        .into_iter()
        .sum();

        sum_of
    }

    // # lib api
    // def pwModelPegWithPolMln(
    //     self,
    //     _pol,
    //     gcFloor = 2.05, # with current liq is
    //     gcBias = 1.7, # with current liq is
    //     gcMaxP = 600.0,
    //     gcMaxL = 550.0,
    // ):
    //     # print(_pol)
    //     # print(gcFloor)
    //     # print(gcBias)
    //     # print(gcMaxP)
    //     # print(gcMaxL)
    //     return max(gcFloor, gcBias + (gcMaxP*_pol/gcMaxL))

    pub async fn get_pw_model_with_pol_mln(
        &self,
        _pol: f64,
        gc_floor: f64,
        gc_bias: f64,
        gc_max_p: f64,
        gc_max_l: f64,
    ) -> f64 {
        f64::max(gc_floor, gc_bias + (gc_max_p * _pol / gc_max_l))
    }

    pub async fn get_gc_pw_current_peg_usd(&'static self) -> f64 {
        self.get_pw_model_with_pol_mln(
            self.get_gc_pol().await / 10f64.powf(6.0),
            2.05,
            1.7,
            600.0,
            550.0,
        )
        .await
    }

    pub async fn get_gc_pw_current_peg_ftm(&'static self) -> f64 {
        let (gc_pol, wftm_price) = tokio::join!(self.get_gc_pol(), self.get_wftm_price());

        let pw_model_pol = self.get_pw_model_with_pol_mln(
            gc_pol / 10f64.powf(6.0),
            2.05,
            1.7,
            600.0,
            550.0,
        )
        .await;

        pw_model_pol / wftm_price
    }

    pub async fn get_gton_usdc_price(&'static self) -> f64 {
        let (amount_u, amount_g) = tokio::join!(
            self.get_erc20token_balance(Self::uToken.0, Self::ugSpoLP.0),
            self.get_erc20token_balance(Self::gToken.0, Self::ugSpoLP.0)
        );

        amount_u / amount_g
    }

    pub async fn get_gton_wftm_price(&'static self) -> f64 {
        let (amount_g, amount_f) = tokio::join!(
            self.get_erc20token_balance(Self::gToken.0, Self::fgSpiLP.0),
            self.get_erc20token_balance(Self::fToken.0, Self::fgSpiLP.0)
        );

        amount_f / amount_g
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_wftm_price() {
        let client = Client::new(Props {
            node_rpc: String::from("https://rpcapi-tracing.fantom.network"),
        })
        .await;
        println!("get_wftm_price: {:?}", client.get_wftm_price().await);
    }

    #[tokio::test]
    async fn test_get_wftm_gton_gc_pool_lp() {
        let client = Box::leak(Box::new(
            Client::new(Props {
                node_rpc: String::from("https://rpcapi-tracing.fantom.network"),
            })
            .await,
        ));
        println!(
            "get_wftm_gton_gc_pool_lp: {:?}",
            client.get_wftm_gton_gc_pool_lp().await
        );
    }

    #[tokio::test]
    async fn test_get_usdc_gton_gc_pool_lp() {
        let client = Box::leak(Box::new(
            Client::new(Props {
                node_rpc: String::from("https://rpcapi-tracing.fantom.network"),
            })
            .await,
        ));
        println!(
            "get_usdc_gton_gc_pool_lp: {:?}",
            client.get_usdc_gton_gc_pool_lp().await
        );
    }

    #[tokio::test]
    async fn test_get_ftm_gton_liq() {
        let client = Box::leak(Box::new(
            Client::new(Props {
                node_rpc: String::from("https://rpcapi-tracing.fantom.network"),
            })
            .await,
        ));
        println!("get_ftm_gton_liq: {:?}", client.get_ftm_gton_liq().await);
    }

    #[tokio::test]
    async fn test_get_usdc_gton_liq() {
        let client = Box::leak(Box::new(
            Client::new(Props {
                node_rpc: String::from("https://rpcapi-tracing.fantom.network"),
            })
            .await,
        ));
        println!("get_usdc_gton_liq: {:?}", client.get_usdc_gton_liq().await);
    }

    #[tokio::test]
    async fn test_get_ftm_gton_lp() {
        let client = Box::leak(Box::new(
            Client::new(Props {
                node_rpc: String::from("https://rpcapi-tracing.fantom.network"),
            })
            .await,
        ));
        println!("get_ftm_gton_lp: {:?}", client.get_ftm_gton_lp().await);
    }

    #[tokio::test]
    async fn test_get_usdc_gton_lp() {
        let client = Box::leak(Box::new(
            Client::new(Props {
                node_rpc: String::from("https://rpcapi-tracing.fantom.network"),
            })
            .await,
        ));
        println!("get_usdc_gton_lp: {:?}", client.get_usdc_gton_lp().await);
    }

    #[tokio::test]
    async fn test_get_gc_pol() {
        let client = Box::leak(Box::new(
            Client::new(Props {
                node_rpc: String::from("https://rpcapi-tracing.fantom.network"),
            })
            .await,
        ));
        println!("get_gc_pol: {:?}", client.get_gc_pol().await);
    }

    #[tokio::test]
    async fn test_get_gton_usdc_price() {
        let client = Box::leak(Box::new(
            Client::new(Props {
                node_rpc: String::from("https://rpcapi-tracing.fantom.network"),
            })
            .await,
        ));
        println!(
            "get_gton_usdc_price: {:?}",
            client.get_gton_usdc_price().await
        );
    }

    #[tokio::test]
    async fn test_get_gton_wftm_price() {
        let client = Box::leak(Box::new(
            Client::new(Props {
                node_rpc: String::from("https://rpcapi-tracing.fantom.network"),
            })
            .await,
        ));
        println!(
            "get_gton_wftm_price: {:?}",
            client.get_gton_wftm_price().await
        );
    }

    #[tokio::test]
    async fn test_get_gc_pw_current_peg_usd() {
        let client = Box::leak(Box::new(
            Client::new(Props {
                node_rpc: String::from("https://rpcapi-tracing.fantom.network"),
            })
            .await,
        ));
        println!(
            "get_gc_pw_current_peg_usd: {:?}",
            client.get_gc_pw_current_peg_usd().await
        );
    }

    #[tokio::test]
    async fn test_get_gc_pw_current_peg_ftm() {
        let client = Box::leak(Box::new(
            Client::new(Props {
                node_rpc: String::from("https://rpcapi-tracing.fantom.network"),
            })
            .await,
        ));
        println!(
            "get_gc_pw_current_peg_ftm: {:?}",
            client.get_gc_pw_current_peg_ftm().await
        );
    }
}
