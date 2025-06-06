// default account settings
pub const DEFAULT_NETWORK: &str = "testnet";
// test net coin addresses
pub struct TestNetAddresses {
    pub coin_address: &'static str,
    pub eth_address: &'static str,
    pub usdc_address: &'static str,
}

impl TestNetAddresses {
    pub fn new() -> Self {
        Self {
            coin_address: "01f3cc99688e6141355c53752418230211facf063c",
            eth_address: "01dce90b5a0b6eb598dd6b4250f0f5924eb4a4a818",
            usdc_address: "0117f2ccfcb0c56ce5b2ad440e879711a5ac8b64a6",
        }
    }
}

#[allow(dead_code)]
pub const PARTISIA_COIN_TYPE: u32 = 3757;
#[allow(dead_code)]
pub struct MainNetAddresses {
    pub coin_address: &'static str,
    pub eth_address: &'static str,
    pub usdc_address: &'static str,
}

#[allow(dead_code)]
impl MainNetAddresses {
    pub fn new() -> Self {
        Self {
            coin_address: "000000000000000000000000000000000000000000",
            eth_address: "000000000000000000000000000000000000000000",
            usdc_address: "000000000000000000000000000000000000000000",
        }
    }
}


// rpc endpoings
#[allow(dead_code)]
pub struct RpcEndpoints {
    pub testnet: &'static str,
    pub mainnet: &'static str,
}

#[allow(dead_code)]
impl RpcEndpoints {
    pub fn new() -> Self {
        Self {
            testnet: "https://node1.testnet.partisiablockchain.com",
            mainnet: "https://rpc.mainnet.partisia.io",
        }
    }
}
