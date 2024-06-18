use chrono::{DateTime, Timelike, Utc};
use reqwest::{self, Client};
use std::collections::VecDeque;
use std::fmt;

const POOL_API_URL: &str = "http://15.204.211.130:4000/api/pools/ErgoSigmanauts";
const NETWORK_API_URL: &str = "https://api.ergoplatform.com/info";
const PRICE_API_URL: &str = "https://api.spectrum.fi/v1/price-tracking/cmc/markets";

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Worker {
    pub name: String,
    pub hashrate: f64,
    pub shares_per_second: f64,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct NetworkStats {
    pub hashrate: f64,
    pub difficulty: f64,
    pub height: u64,
    pub reward: f64,
    pub reward_reduction: u8,
    pub price: f64,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PoolStats {
    pub data: serde_json::Value,
    pub hashrate: f64,
    pub connected_miners: u64,
    pub effort: f64,
    pub total_blocks: u64,
    pub confirming_new_block: f64,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MinerStats {
    pub data: serde_json::Value,
    pub address: String,
    pub hashrate_collection: VecDeque<(u32, f64)>,
    pub hashrate_current: f64,
    pub hashrate_6h: f64,
    pub hashrate_12h: f64,
    pub hashrate_24h: f64,
    pub pending_shares: f64,
    pub pending_balance: f64,
    pub round_contribution: f64,
    pub total_paid: f64,
    pub paid_24h: f64,
    pub workers: Vec<Worker>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Stats {
    pub network: NetworkStats,
    pub pool: PoolStats,
    pub miner: MinerStats,
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl fmt::Display for Worker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl fmt::Display for MinerStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}
/// Get data from Mining Core API
pub async fn get_data(address: String) -> Result<Stats, reqwest::Error> {
    let mut network_stats = NetworkStats::default().await;
    let mut pool_stats = PoolStats::default().await;
    let mut miner_stats = MinerStats::default(address).await;

    network_stats.provide_data().await.unwrap();
    pool_stats.provide_data().await.unwrap();
    miner_stats.provide_data().await.unwrap();

    //participation
    miner_stats.round_contribution =
        ((miner_stats.hashrate_current / (pool_stats.hashrate * 1_000.0)) * 10000.0).round()
            / 100.0;

    Ok(Stats {
        network: network_stats,
        pool: pool_stats,
        miner: miner_stats,
    })
}

/// Get data from Mining Core API
pub async fn get_landing_page_data(address: String) -> Result<Stats, reqwest::Error> {
    let mut network_stats = NetworkStats::default().await;
    let mut pool_stats = PoolStats::default().await;
    let mut miner_stats = MinerStats::default(address).await;

    network_stats.provide_data().await.unwrap();
    pool_stats.provide_data().await.unwrap();
    // miner_stats.provide_data().await.unwrap();

    //participation
    miner_stats.round_contribution =
        ((miner_stats.hashrate_current / (pool_stats.hashrate * 1_000.0)) * 10000.0).round()
            / 100.0;

    Ok(Stats {
        network: network_stats,
        pool: pool_stats,
        miner: miner_stats,
    })
}

impl NetworkStats {
    pub async fn default() -> Self {
        NetworkStats {
            hashrate: f64::default(),
            difficulty: f64::default(),
            height: u64::default(),
            reward: f64::default(),
            reward_reduction: u8::default(),
            price: f64::default(),
        }
    }

    pub async fn provide_data(&mut self) -> Result<(), reqwest::Error> {
        let hashrate_data: serde_json::Value = Client::new()
            .get(NETWORK_API_URL)
            .send()
            .await?
            .json()
            .await?;

        let info_data: serde_json::Value =
            Client::new().get(POOL_API_URL).send().await?.json().await?;

        let price_data: serde_json::Value = Client::new()
            .get(PRICE_API_URL)
            .send()
            .await?
            .json()
            .await?;

        self.hashrate = (hashrate_data["hashRate"]
            .clone()
            .as_f64()
            .expect(" network hashrate not awailable")
            / 10_000_000_000.0)
            .round()
            / 100.0;

        self.difficulty = (info_data["pool"]["networkStats"]["networkDifficulty"]
            .clone()
            .as_f64()
            .expect("network diff not avialable")
            / 10_0000_000_000_000.0)
            .round()
            / 100.0;

        self.height = info_data["pool"]["networkStats"]["blockHeight"]
            .clone()
            .as_u64()
            .expect("block height not available");

        // ERG Price
        if let serde_json::Value::Array(arr) = price_data {
            for obj in arr {
                if let serde_json::Value::Object(o) = obj {
                    if let Some(base_name) = o.get("base_name") {
                        if base_name == "ERG" {
                            if let Some(quote_name) = o.get("quote_name") {
                                if quote_name == "SigUSD" {
                                    if let Some(last_price) = o.get("last_price") {
                                        if let Some(price) = last_price.as_f64() {
                                            self.price = ((1.0 / price) * 100.0).round() / 100.0;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        self.get_block_reward().await.unwrap();

        Ok(())
    }

    async fn get_block_reward(&mut self) -> Result<(), reqwest::Error> {
        let block_data: serde_json::Value = Client::new()
            .get(format!("{}/blocks", POOL_API_URL))
            .send()
            .await?
            .json()
            .await?;

        self.reward = block_data[0]["reward"].as_f64().unwrap().round();
        Ok(())
    }
}

impl PoolStats {
    pub async fn default() -> PoolStats {
        PoolStats {
            data: serde_json::Value::default(),
            hashrate: f64::default(),
            connected_miners: u64::default(),
            effort: f64::default(),
            total_blocks: u64::default(),
            confirming_new_block: f64::default(),
        }
    }

    pub async fn provide_data(&mut self) -> Result<(), reqwest::Error> {
        self.fetch_data().await.unwrap();
        self.calculate_data().await.unwrap();
        Ok(())
    }

    async fn fetch_data(&mut self) -> Result<(), reqwest::Error> {
        self.data = Client::new().get(POOL_API_URL).send().await?.json().await?;
        Ok(())
    }

    async fn calculate_data(&mut self) -> Result<(), reqwest::Error> {
        /* Hashrate */
        self.hashrate = (self.data["pool"]["poolStats"]["poolHashrate"]
            .clone()
            .as_f64()
            .expect("no pool hashrate available")
            / 10_000_000.0)
            .round()
            / 100.0;

        /* Connected Miners */
        self.connected_miners = self.data["pool"]["poolStats"]["connectedMiners"]
            .clone()
            .as_u64()
            .expect("connected miners not available");

        /* Pool effort */
        self.effort = (self.data["pool"]["poolEffort"]
            .clone()
            .as_f64()
            .expect("pool effort not available")
            * 10000.0)
            .round()
            / 100.0;

        /* Total Blocks */
        self.total_blocks = self.data["pool"]["totalBlocks"]
            .clone()
            .as_u64()
            .expect("total blocks data not available");

        /* New block confirmation */
        let block_data: serde_json::Value = Client::new()
            .get(format!("{}/blocks", POOL_API_URL))
            .send()
            .await?
            .json()
            .await?;

        let pool_block_confirmation: (&str, f64, f64) = (
            block_data[0]["status"].as_str().unwrap(),
            (block_data[0]["confirmationProgress"].as_f64().unwrap()) * 100.0,
            block_data[0]["reward"].as_f64().unwrap().round(),
        );

        if pool_block_confirmation.0 == "pending" {
            self.confirming_new_block = pool_block_confirmation.1;
        } else {
            self.confirming_new_block = 100.0;
        }

        Ok(())
    }
}

impl MinerStats {
    pub async fn default(address: String) -> Self {
        MinerStats {
            data: serde_json::Value::default(),
            address: address,
            hashrate_collection: VecDeque::default(),
            hashrate_current: f64::default(),
            hashrate_6h: f64::default(),
            hashrate_12h: f64::default(),
            hashrate_24h: f64::default(),
            pending_balance: f64::default(),
            pending_shares: f64::default(),
            round_contribution: f64::default(),
            total_paid: f64::default(),
            paid_24h: f64::default(),
            workers: Vec::default(),
        }
    }
    pub async fn provide_data(&mut self) -> Result<(), reqwest::Error> {
        self.fetch_data().await.unwrap();
        self.calculate_hashrate_collection().await.unwrap();
        self.calculate_balance_shares_totalpaid_paid24h()
            .await
            .unwrap();
        self.calculate_workers().await.unwrap();
        self.calculate_hashrate().await.unwrap();
        Ok(())
    }

    async fn fetch_data(&mut self) -> Result<(), reqwest::Error> {
        let miner_api_url = format!("{}/{}/{}", POOL_API_URL, "miners", self.address);

        self.data = Client::new()
            .get(miner_api_url)
            .send()
            .await?
            .json()
            .await?;
        Ok(())
    }

    async fn calculate_hashrate_collection(&mut self) -> Result<(), reqwest::Error> {
        //hashrate_collection
        if let serde_json::Value::Array(sample_array) = self.data["performanceSamples"].clone() {
            for sample in sample_array.iter() {
                let time = sample["created"]
                    .as_str()
                    .expect("time for sample not available");

                let date_time: DateTime<Utc> = DateTime::parse_from_rfc3339(time).unwrap().into();
                let hour = date_time.hour();

                let mut hashrate: f64 = 0.0;

                for (_key, value) in sample["workers"].as_object().unwrap() {
                    hashrate += value["hashrate"]
                        .as_f64()
                        .expect("sample hashrate not available");
                }

                hashrate = (hashrate / 10_000.0).round() / 100.0;

                self.hashrate_collection.push_back((hour, hashrate));
            }
        }
        Ok(())
    }

    async fn calculate_balance_shares_totalpaid_paid24h(&mut self) -> Result<(), reqwest::Error> {
        self.pending_balance = (self.data["pendingBalance"]
            .as_f64()
            .expect("pendingBalance not available")
            * 100.0)
            .round()
            / 100.0;

        self.pending_shares = self.data["pendingShares"]
            .as_f64()
            .expect("pendingShares not available");

        // Round contribution TODO!

        self.total_paid = (self.data["totalPaid"]
            .as_f64()
            .expect("totalPaid not available")
            * 100.0)
            .round()
            / 100.0;

        self.paid_24h = (self.data["todayPaid"]
            .as_f64()
            .expect("todayPaid not available")
            * 100.0)
            .round()
            / 100.0;

        Ok(())
    }

    async fn calculate_workers(&mut self) -> Result<(), reqwest::Error> {
        for (key, value) in self.data["performance"]["workers"]
            .clone()
            .as_object()
            .unwrap()
        {
            self.workers.push(Worker {
                name: key.to_string(),
                hashrate: ((value["hashrate"]
                    .as_f64()
                    .expect("failed to load worker hashrate")
                    / 1_000_000.0)
                    * 100.0)
                    .round()
                    / 100.0,
                shares_per_second: value["sharesPerSecond"]
                    .as_f64()
                    .expect("failed to load worker shares per second"),
            })
        }
        Ok(())
    }

    async fn calculate_hashrate(&mut self) -> Result<(), reqwest::Error> {
        //Hashrate current
        let mut hashrate = 0.0;
        for worker in self.workers.iter() {
            hashrate += worker.hashrate;
        }
        self.hashrate_current = (hashrate * 100.0).round() / 100.0;

        //Hashrate 6h/24h
        let mut hashrate = 0.0;
        let mut itter = 0;
        for worker in self.hashrate_collection.iter() {
            hashrate += worker.1;

            match itter {
                5 => self.hashrate_6h = ((hashrate / 6.0) * 100.0).round() / 100.0,
                11 => self.hashrate_12h = ((hashrate / 12.0) * 100.0).round() / 100.0,
                23 => self.hashrate_24h = ((hashrate / 24.0) * 100.0).round() / 100.0,
                _ => {}
            }
            itter += 1;
        }
        Ok(())
    }
}
