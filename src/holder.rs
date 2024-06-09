/*    let pool_api_url = "http://15.204.211.130:4000/api/pools/ErgoSigmanauts";
    let price_api_url = "https://api.spectrum.fi/v1/price-tracking/cmc/markets";
    let hashrate_api = "https://api.ergoplatform.com/info";

    let pool_data: serde_json::Value = Client::new().get(pool_api_url).send().await?.json().await?;

    let mut stats = Stats {
        network: NetworkStats::default(),
        pool: PoolStats::default(),
        miner: MinerStats::default(),
    };

    //Format block height
    let block_height = pool_data["pool"]["networkStats"]["blockHeight"]
        .clone()
        .as_u64();

    //Only update the data if a new block is added to the chain
    if block_height.unwrap() != stats.network.height {
        match block_height {
            Some(block_height) => stats.network.height = block_height,
            None => println!("No data available for Block Height"),
        }

        let price_data: serde_json::Value = Client::new()
            .get(price_api_url)
            .send()
            .await?
            .json()
            .await?;
        let hashrate_data: serde_json::Value =
            Client::new().get(hashrate_api).send().await?.json().await?;

        // Network Hashrate
        let network_hashrate = hashrate_data["hashRate"].clone().as_f64();

        match network_hashrate {
            Some(network_hashrate) => {
                let network_hashrate =
                    ((network_hashrate / 1_000_000_000_000.0) * 100.0).round() / 100.0;
                stats
                    .network
                    .hashrate
                    .push_back((block_height.unwrap() as f64, network_hashrate));
            }

            None => println!("No data available for Network Hashrate"),
        }

        // Network Difficulty
        let network_difficulty = pool_data["pool"]["networkStats"]["networkDifficulty"]
            .clone()
            .as_f64();

        match network_difficulty {
            Some(network_difficulty) => {
                //round to 2 decimals
                let network_difficulty =
                    ((network_difficulty / 1_000_000_000_000_000.0) * 100.0).round() / 100.0;
                stats.network.difficulty = network_difficulty;
            }

            None => println!("No data available for Network Difficulty"),
        }

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
                                            stats.network.price =
                                                ((1.0 / price) * 100.0).round() / 100.0;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        //Pool hashrate
        let pool_hashrate = pool_data["pool"]["poolStats"]["poolHashrate"]
            .clone()
            .as_f64();

        match pool_hashrate {
            Some(pool_hashrate) => {
                let pool_hashrate = ((pool_hashrate / 1_000_000_000.0) * 100.0).round() / 100.0;
                stats
                    .pool
                    .hashrate
                    .push_back((stats.network.height as f64, pool_hashrate))
            }

            None => println!("No data available for Pool Hashrate"),
        }

        //Pool connected miners
        let connected_miners = pool_data["pool"]["poolStats"]["connectedMiners"]
            .clone()
            .as_u64();

        match connected_miners {
            Some(connected_miners) => stats.pool.connected_miners = connected_miners,

            None => println!("No data available for Connected Miners"),
        }

        //Pool effort
        let pool_effort = pool_data["pool"]["poolEffort"].clone().as_f64();

        match pool_effort {
            Some(pool_effort) => {
                let pool_effort = (pool_effort * 10000.0).round() / 100.0;
                stats.pool.effort = pool_effort;
            }

            None => println!("No data available for Pool Effort"),
        }

        //Pool total blocks
        let pool_total_blocks = pool_data["pool"]["totalBlocks"].clone().as_u64();

        match pool_total_blocks {
            Some(pool_total_blocks) => {
                stats.pool.total_blocks = pool_total_blocks;
            }

            None => println!("No data available for Pool Effort"),
        }

        //Pool confirming new block

        let block_data: serde_json::Value = Client::new()
            .get(format!("{}/blocks", pool_api_url))
            .send()
            .await?
            .json()
            .await?;

        let pool_block_confirmation: (&str, f64) = (
            block_data[0]["status"].as_str().unwrap(),
            (block_data[0]["confirmationProgress"].as_f64().unwrap()) * 100.0,
        );

        if pool_block_confirmation.0 == "pending" {
            stats.pool.confirming_new_block = pool_block_confirmation.1;
        } else {
            stats.pool.confirming_new_block = 100.0;
        }
    }

    //Store only the last 720 blocks (720 * 2min = 24h)
    if stats.network.hashrate.len() > 720 {
        stats.network.hashrate.pop_front();
        stats.pool.hashrate.pop_front();
    }
}*/
