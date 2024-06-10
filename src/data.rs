use chrono::{round, DateTime, TimeZone, Timelike, Utc};
use dioxus::prelude::SuperInto;
use reqwest::{self, Client};
use std::collections::VecDeque;
use std::fmt;

const POOL_API_URL: &str = "http://15.204.211.130:4000/api/pools/ErgoSigmanauts";

#[derive(Debug, Default, Clone)]
pub struct Worker {
    pub name: String,
    pub hashrate: f64,
    pub shares_per_second: f64,
}

#[derive(Debug, Default, Clone)]
pub struct NetworkStats {
    pub hashrate: VecDeque<(f64, f64)>,
    pub difficulty: f64,
    pub height: u64,
    pub reward: u8,
    pub reward_reduction: u8,
    pub price: f64,
}

#[derive(Debug, Default, Clone)]
pub struct PoolStats {
    pub hashrate: VecDeque<(f64, f64)>,
    pub connected_miners: u64,
    pub effort: f64,
    pub total_blocks: u64,
    pub confirming_new_block: f64,
}

#[derive(Debug, Default, Clone)]
pub struct MinerStats {
    pub hashrate_collection: VecDeque<(u32, f64)>,
    pub hashrate_current: f64,
    pub hashrate_6h: f64,
    pub hashrate_24h: f64,
    pub pending_shares: f64,
    pub pending_balance: f64,
    pub round_contribution: f64,
    pub total_paid: f64,
    pub paid_24h: f64,
    pub workers: Vec<Worker>,
}

#[derive(Debug, Default, Clone)]
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
    let mut miner_stats = MinerStats::default().await;

    miner_stats.get_data(address).await.unwrap();

    Ok(Stats {
        network: network_stats,
        pool: pool_stats,
        miner: miner_stats,
    })
}

impl NetworkStats {
    pub async fn default() -> Self {
        NetworkStats {
            hashrate: VecDeque::default(),
            difficulty: f64::default(),
            height: u64::default(),
            reward: u8::default(),
            reward_reduction: u8::default(),
            price: f64::default(),
        }
    }
}

impl PoolStats {
    pub async fn default() -> PoolStats {
        PoolStats {
            hashrate: VecDeque::default(),
            connected_miners: u64::default(),
            effort: f64::default(),
            total_blocks: u64::default(),
            confirming_new_block: f64::default(),
        }
    }
}
impl MinerStats {
    pub async fn default() -> Self {
        MinerStats {
            hashrate_collection: VecDeque::default(),
            hashrate_current: f64::default(),
            hashrate_6h: f64::default(),
            hashrate_24h: f64::default(),
            pending_balance: f64::default(),
            pending_shares: f64::default(),
            round_contribution: f64::default(),
            total_paid: f64::default(),
            paid_24h: f64::default(),
            workers: Vec::default(),
        }
    }

    pub async fn get_data(&mut self, address: String) -> Result<(), reqwest::Error> {
        let miner_api_url = format!("{}/{}/{}", POOL_API_URL, "miners", address);

        let data: serde_json::Value = Client::new()
            .get(miner_api_url)
            .send()
            .await?
            .json()
            .await?;

        //hashrate_collection
        if let serde_json::Value::Array(sample_array) = data["performanceSamples"].clone() {
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

        self.pending_balance = data["pendingBalance"]
            .as_f64()
            .expect("pendingBalance not available");

        self.pending_shares = data["pendingShares"]
            .as_f64()
            .expect("pendingShares not available");

        // Round contribution

        self.pending_balance = data["pendingBalance"]
            .as_f64()
            .expect("pendingBalance not available");

        self.total_paid = data["totalPaid"].as_f64().expect("totalPaid not available");

        self.paid_24h =
            (data["todayPaid"].as_f64().expect("todayPaid not available") * 100.0).round() / 100.0;

        for (key, value) in data["performance"]["workers"].clone().as_object().unwrap() {
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
                23 => self.hashrate_24h = ((hashrate / 24.0) * 100.0).round() / 100.0,
                _ => {}
            }
            itter += 1;
        }
        Ok(())
    }
}
