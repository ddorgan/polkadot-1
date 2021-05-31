use polkadot_cli::Cli;
use sc_cli::SubstrateCli;
use structopt::StructOpt;
use polkadot_runtime_test::{PolkadotChainInfo, Block, Executor, SelectChain, BlockImport, dispatch_with_pallet_democracy};
use test_runner::{Node, ChainInfo, NodeConfig};
use sc_service::{TFullBackend, TFullClient, Configuration, TaskManager, TaskExecutor};
use polkadot_runtime::{Runtime, RuntimeApi};
use std::sync::Arc;
use sp_keystore::SyncCryptoStorePtr;
use sp_inherents::{InherentDataProviders, InherentDataProvider};
use sc_consensus_manual_seal::ConsensusDataProvider;

pub struct PolkadotSimnetChainInfo;

impl ChainInfo for PolkadotSimnetChainInfo {
    type Block = Block;
    type Executor = Executor;
    type Runtime = Runtime;
    type RuntimeApi = RuntimeApi;
    type SelectChain = SelectChain;
    type BlockImport = BlockImport<
        Self::Block,
        TFullBackend<Self::Block>,
        TFullClient<Self::Block, RuntimeApi, Self::Executor>,
        Self::SelectChain,
    >;
    type SignedExtras = polkadot_runtime::SignedExtra;
    type InherentDataProviders = ();

    fn signed_extras(from: <Runtime as frame_system::Config>::AccountId) -> Self::SignedExtras {
        PolkadotChainInfo::signed_extras(from)
    }

    fn config(task_executor: TaskExecutor) -> Configuration {
        let cmd = <Cli as StructOpt>::from_args();
        cmd.create_configuration(&cmd.run.base, task_executor).unwrap()
    }

    fn create_client_parts(config: &Configuration) -> Result<
        (
            Arc<TFullClient<Self::Block, RuntimeApi, Self::Executor>>,
            Arc<TFullBackend<Self::Block>>,
            SyncCryptoStorePtr,
            TaskManager,
            InherentDataProviders,
            Option<
                Box<
                    dyn ConsensusDataProvider<
                        Self::Block,
                        Transaction = sp_api::TransactionFor<
                            TFullClient<Self::Block, RuntimeApi, Self::Executor>,
                            Self::Block,
                        >,
                    >,
                >,
            >,
            Self::SelectChain,
            Self::BlockImport,
        ),
        sc_service::Error,
    > {
        PolkadotChainInfo::create_client_parts(config)
    }

    fn dispatch_with_root(call: <Runtime as frame_system::Config>::Call, node: &mut Node<Self>) {
        dispatch_with_pallet_democracy(call, node)
    }
}