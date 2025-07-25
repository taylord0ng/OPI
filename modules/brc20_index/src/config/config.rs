use bitcoin::network::Network;
use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref FIRST_INSCRIPTION_HEIGHTS: HashMap<Network, i32> = {
        let mut map = HashMap::new();
        map.insert(Network::Bitcoin, 767430);
        map.insert(Network::Testnet, 2413343);
        map.insert(Network::Testnet4, 0);
        map.insert(Network::Regtest, 0);
        map.insert(Network::Signet, 112402);
        map
    };

    pub static ref FIRST_BRC20_HEIGHTS: HashMap<Network, i32> = {
        let mut map = HashMap::new();
        map.insert(Network::Bitcoin, 779832);
        map.insert(Network::Testnet, 2413343);
        map.insert(Network::Testnet4, 0);
        map.insert(Network::Regtest, 0);
        map.insert(Network::Signet, 112402);
        map
    };

    /// During phase 1, only 6 byte tickers can get deposited into programmable module.
    pub static ref FIRST_BRC20_PROG_PHASE_1_HEIGHTS: HashMap<Network, i32> = {
        let mut map = HashMap::new();
        map.insert(Network::Bitcoin, 909969);
        map.insert(Network::Testnet, 0);
        map.insert(Network::Testnet4, 0);
        map.insert(Network::Regtest, 0);
        map.insert(Network::Signet, 230000);
        map
    };

    pub static ref FIRST_BRC20_PROG_PHASE_2_HEIGHTS: HashMap<Network, i32> = {
        let mut map = HashMap::new();
        map.insert(Network::Bitcoin, 914888);
        map.insert(Network::Testnet, 0);
        map.insert(Network::Testnet4, 0);
        map.insert(Network::Regtest, 0);
        map.insert(Network::Signet, 230000);
        map
    };
}

pub const DB_HOST_KEY: &str = "DB_HOST";
pub const DB_HOST_DEFAULT: &str = "localhost";

pub const DB_PORT_KEY: &str = "DB_PORT";
pub const DB_PORT_DEFAULT: &str = "5432";

pub const DB_USER_KEY: &str = "DB_USER";
pub const DB_USER_DEFAULT: &str = "postgres";

pub const DB_PASSWORD_KEY: &str = "DB_PASSWD";
pub const DB_PASSWORD_DEFAULT: &str = "";

pub const DB_DATABASE_KEY: &str = "DB_DATABASE";
pub const DB_DATABASE_DEFAULT: &str = "postgres";

pub const DB_SSL_KEY: &str = "DB_SSL";
pub const DB_SSL_DEFAULT: &str = "false";

pub const REPORT_TO_INDEXER_KEY: &str = "REPORT_TO_INDEXER";
pub const REPORT_TO_INDEXER_DEFAULT: &str = "true";

pub const REPORT_URL_KEY: &str = "REPORT_URL";
pub const REPORT_URL_DEFAULT: &str = "https://api.opi.network/report_block";

pub const REPORT_RETRIES_KEY: &str = "REPORT_RETRIES";
pub const REPORT_RETRIES_DEFAULT: &str = "10";

pub const REPORT_NAME_KEY: &str = "REPORT_NAME";
pub const REPORT_NAME_DEFAULT: &str = "opi_brc20_indexer";

pub const NETWORK_TYPE_KEY: &str = "NETWORK_TYPE";
pub const NETWORK_TYPE_DEFAULT: &str = "mainnet";

pub const BRC20_PROG_ENABLED_KEY: &str = "BRC20_PROG_ENABLED";
pub const BRC20_PROG_ENABLED_DEFAULT: &str = "false";

pub const BRC20_PROG_RPC_URL_KEY: &str = "BRC20_PROG_RPC_URL";
pub const BRC20_PROG_RPC_URL_DEFAULT: &str = "http://localhost:18545";

pub const BRC20_PROG_RPC_USER_KEY: &str = "BRC20_PROG_RPC_USER";
pub const BRC20_PROG_RPC_PASSWORD_KEY: &str = "BRC20_PROG_RPC_PASSWORD";

pub const BRC20_PROG_BALANCE_SERVER_URL_KEY: &str = "BRC20_PROG_BALANCE_SERVER_URL";
pub const BRC20_PROG_BALANCE_SERVER_URL_DEFAULT: &str = "127.0.0.1:18546";

pub const PROTOCOL_KEY: &str = "p";
pub const PROTOCOL_BRC20: &str = "brc-20";
pub const PROTOCOL_BRC20_PROG: &str = "brc20-prog";
pub const PROTOCOL_BRC20_MODULE: &str = "brc20-module";

pub const BRC20_MODULE_BRC20PROG: &str = "BRC20PROG";

// BRC20 specific keys
pub const LIMIT_PER_MINT_KEY: &str = "lim";
pub const MAX_SUPPLY_KEY: &str = "max";
pub const DECIMALS_KEY: &str = "dec";
pub const AMOUNT_KEY: &str = "amt";
pub const OPERATION_KEY: &str = "op";
pub const MODULE_KEY: &str = "module";
pub const TICKER_KEY: &str = "tick";
pub const SELF_MINT_KEY: &str = "self_mint";
pub const SALT_KEY: &str = "salt";
pub const HASH_KEY: &str = "hash";

// BRC20 prog specific keys
pub const DATA_KEY: &str = "d";
pub const BASE64_DATA_KEY: &str = "b";
pub const CONTRACT_ADDRESS_KEY: &str = "c";
pub const INSCRIPTION_ID_KEY: &str = "i";

pub const OPERATION_DEPLOY: &str = "deploy";
pub const OPERATION_PREDEPLOY: &str = "predeploy";
pub const OPERATION_WITHDRAW: &str = "withdraw";
pub const OPERATION_MINT: &str = "mint";
pub const OPERATION_TRANSFER: &str = "transfer";

pub const OPERATION_BRC20_PROG_DEPLOY: &str = "deploy";
pub const OPERATION_BRC20_PROG_DEPLOY_SHORT: &str = "d";

pub const OPERATION_BRC20_PROG_CALL: &str = "call";
pub const OPERATION_BRC20_PROG_CALL_SHORT: &str = "c";

pub const OPERATION_BRC20_PROG_TRANSACT: &str = "transact";
pub const OPERATION_BRC20_PROG_TRANSACT_SHORT: &str = "t";

pub const BRC20_PROG_OP_RETURN_PKSCRIPT: &str = "6a09425243323050524f47";
pub const OP_RETURN: &str = "6a";

pub const NO_WALLET: &str = "";

pub const MAX_DECIMALS: u8 = 18;
pub const MAX_AMOUNT: u128 = (2u128.pow(64) - 1) * 10u128.pow(18);

pub const PREDEPLOY_BLOCK_HEIGHT_DELAY: i32 = 3;
pub const PREDEPLOY_BLOCK_HEIGHT_ACCEPTANCE_DELAY: i32 = 10;

pub const BRC20_PROG_MINE_BATCH_SIZE: i32 = 50000;

pub const EVENT_SEPARATOR: &str = "|";

pub const SELF_MINT_ENABLE_HEIGHT: i32 = 837090;

// Versions used for database migrations and version checks
// These should be updated when the database schema changes
pub const DB_VERSION: i32 = 6;
pub const EVENT_HASH_VERSION: i32 = 3;
pub const BRC20_PROG_VERSION: &str = "0.10.1";
pub const INDEXER_VERSION: &str = "opi-brc20-full-node v0.5.0";

fn get_bitcoin_network_type(network_type: &str) -> Network {
    match network_type {
        "mainnet" => Network::Bitcoin,
        "testnet" => Network::Testnet,
        "testnet4" => Network::Testnet4,
        "regtest" => Network::Regtest,
        "signet" => Network::Signet,
        _ => panic!("Invalid network type"),
    }
}

pub struct Brc20IndexerConfig {
    pub db_host: String,
    pub db_port: String,
    pub db_user: String,
    pub db_password: String,
    pub db_database: String,
    pub db_ssl: bool,

    pub report_to_indexer: bool,
    pub report_url: String,
    pub report_retries: i32,
    pub report_name: String,

    pub network_type_string: String,

    pub first_inscription_height: i32,
    pub first_brc20_height: i32,
    /// Phase 1 adds support for contracts and depositing tickers with 6 byte length.
    pub first_brc20_prog_phase_one_height: i32,
    /// Phase 2 adds support for all tickers.
    pub first_brc20_prog_all_tickers_height: i32,

    pub brc20_prog_enabled: bool,
    pub brc20_prog_rpc_url: String,
    pub brc20_prog_rpc_user: Option<String>,
    pub brc20_prog_rpc_password: Option<String>,

    pub brc20_prog_balance_server_url: String,
}

impl Default for Brc20IndexerConfig {
    fn default() -> Self {
        let network_type_string =
            &std::env::var(NETWORK_TYPE_KEY).unwrap_or_else(|_| NETWORK_TYPE_DEFAULT.to_string());
        let network_type = get_bitcoin_network_type(&network_type_string);

        Brc20IndexerConfig {
            db_host: std::env::var(DB_HOST_KEY).unwrap_or_else(|_| DB_HOST_DEFAULT.to_string()),
            db_port: std::env::var(DB_PORT_KEY).unwrap_or_else(|_| DB_PORT_DEFAULT.to_string()),
            db_user: std::env::var(DB_USER_KEY).unwrap_or_else(|_| DB_USER_DEFAULT.to_string()),
            db_password: std::env::var(DB_PASSWORD_KEY)
                .unwrap_or_else(|_| DB_PASSWORD_DEFAULT.to_string()),
            db_database: std::env::var(DB_DATABASE_KEY)
                .unwrap_or_else(|_| DB_DATABASE_DEFAULT.to_string()),
            db_ssl: std::env::var(DB_SSL_KEY).unwrap_or_else(|_| DB_SSL_DEFAULT.to_string())
                == "true",

            report_to_indexer: std::env::var(REPORT_TO_INDEXER_KEY)
                .unwrap_or_else(|_| REPORT_TO_INDEXER_DEFAULT.to_string())
                == "true"
                && network_type != Network::Regtest,

            report_url: std::env::var(REPORT_URL_KEY)
                .unwrap_or_else(|_| REPORT_URL_DEFAULT.to_string()),
            report_retries: std::env::var(REPORT_RETRIES_KEY)
                .unwrap_or_else(|_| REPORT_RETRIES_DEFAULT.to_string())
                .parse::<i32>()
                .unwrap(),
            report_name: std::env::var(REPORT_NAME_KEY)
                .unwrap_or_else(|_| REPORT_NAME_DEFAULT.to_string()),

            network_type_string: network_type_string.to_string(),

            first_inscription_height: *FIRST_INSCRIPTION_HEIGHTS
                .get(&network_type)
                .unwrap_or_else(|| panic!("Invalid network type: {}", network_type)),
            first_brc20_height: *FIRST_BRC20_HEIGHTS
                .get(&network_type)
                .unwrap_or_else(|| panic!("Invalid network type: {}", network_type)),
            first_brc20_prog_phase_one_height: *FIRST_BRC20_PROG_PHASE_1_HEIGHTS
                .get(&network_type)
                .unwrap_or_else(|| panic!("Invalid network type: {}", network_type)),
            first_brc20_prog_all_tickers_height: *FIRST_BRC20_PROG_PHASE_2_HEIGHTS
                .get(&network_type)
                .unwrap_or_else(|| panic!("Invalid network type: {}", network_type)),

            brc20_prog_enabled: std::env::var(BRC20_PROG_ENABLED_KEY)
                .unwrap_or_else(|_| BRC20_PROG_ENABLED_DEFAULT.to_string())
                == "true",
            brc20_prog_rpc_url: std::env::var(BRC20_PROG_RPC_URL_KEY)
                .unwrap_or_else(|_| BRC20_PROG_RPC_URL_DEFAULT.to_string()),
            brc20_prog_rpc_user: std::env::var(BRC20_PROG_RPC_USER_KEY)
                .ok()
                .filter(|s| !s.is_empty()),
            brc20_prog_rpc_password: std::env::var(BRC20_PROG_RPC_PASSWORD_KEY)
                .ok()
                .filter(|s| !s.is_empty()),

            brc20_prog_balance_server_url: std::env::var(BRC20_PROG_BALANCE_SERVER_URL_KEY)
                .unwrap_or_else(|_| BRC20_PROG_BALANCE_SERVER_URL_DEFAULT.to_string()),
        }
    }
}
