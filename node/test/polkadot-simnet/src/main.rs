use log::LevelFilter;
use std::error::Error;
use test_runner::{Node, ChainInfo, NodeConfig};
mod chain_info;

use chain_info::PolkadotSimnetChainInfo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = NodeConfig {
        log_targets: vec![
            ("yamux", LevelFilter::Off),
            ("multistream_select", LevelFilter::Off),
            ("libp2p", LevelFilter::Off),
            ("jsonrpc_client_transports", LevelFilter::Off),
            ("sc_network", LevelFilter::Off),
            ("tokio_reactor", LevelFilter::Off),
            ("parity-db", LevelFilter::Off),
            ("sub-libp2p", LevelFilter::Off),
            ("peerset", LevelFilter::Off),
            ("ws", LevelFilter::Off),
            ("sc_service", LevelFilter::Off),
            ("sc_basic_authorship", LevelFilter::Off),
            ("telemetry-logger", LevelFilter::Off),
            ("sc_peerset", LevelFilter::Off),
            ("rpc", LevelFilter::Off),

            ("sync", LevelFilter::Debug),
            ("sc_network", LevelFilter::Debug),
            ("runtime", LevelFilter::Trace),
            ("babe", LevelFilter::Debug)
        ],
    };
    let node = Node::<PolkadotSimnetChainInfo>::new(config)?;

    // wait for ctrl_c signal, then drop node.
    tokio::signal::ctrl_c().await?;

    drop(node);

    Ok(())
}
