//! Anoma client CLI.

use color_eyre::eyre::Result;
use namada_apps::cli;
use namada_apps::cli::cmds::*;
use namada_apps::client::{rpc, tx, utils};

pub async fn main() -> Result<()> {
    match cli::anoma_client_cli()? {
        cli::AnomaClient::WithContext(cmd_box) => {
            let (cmd, ctx) = *cmd_box;
            use AnomaClientWithContext as Sub;
            match cmd {
                // Ledger cmds
                Sub::TxCustom(TxCustom(args)) => {
                    tx::submit_custom(ctx, args).await;
                }
                Sub::TxTransfer(TxTransfer(args)) => {
                    tx::submit_transfer(ctx, args).await;
                }
                Sub::TxUpdateVp(TxUpdateVp(args)) => {
                    tx::submit_update_vp(ctx, args).await;
                }
                Sub::TxInitAccount(TxInitAccount(args)) => {
                    tx::submit_init_account(ctx, args).await;
                }
                Sub::TxInitValidator(TxInitValidator(args)) => {
                    tx::submit_init_validator(ctx, args).await;
                }
                Sub::TxInitProposal(TxInitProposal(args)) => {
                    tx::submit_init_proposal(ctx, args).await;
                }
                Sub::TxVoteProposal(TxVoteProposal(args)) => {
                    tx::submit_vote_proposal(ctx, args).await;
                }
                Sub::Bond(Bond(args)) => {
                    tx::submit_bond(ctx, args).await;
                }
                Sub::Unbond(Unbond(args)) => {
                    tx::submit_unbond(ctx, args).await;
                }
                Sub::Withdraw(Withdraw(args)) => {
                    tx::submit_withdraw(ctx, args).await;
                }
                // Ledger queries
                Sub::QueryEpoch(QueryEpoch(args)) => {
                    rpc::query_epoch(args).await;
                }
                Sub::QueryBlock(QueryBlock(args)) => {
                    rpc::query_block(args).await;
                }
                Sub::QueryBalance(QueryBalance(args)) => {
                    rpc::query_balance(ctx, args).await;
                }
                Sub::QueryBonds(QueryBonds(args)) => {
                    rpc::query_bonds(ctx, args).await;
                }
                Sub::QueryVotingPower(QueryVotingPower(args)) => {
                    rpc::query_voting_power(ctx, args).await;
                }
                Sub::QuerySlashes(QuerySlashes(args)) => {
                    rpc::query_slashes(ctx, args).await;
                }
                Sub::QueryResult(QueryResult(args)) => {
                    rpc::query_result(ctx, args).await;
                }
                Sub::QueryRawBytes(QueryRawBytes(args)) => {
                    rpc::query_raw_bytes(ctx, args).await;
                }

                Sub::QueryProposal(QueryProposal(args)) => {
                    rpc::query_proposal(ctx, args).await;
                }
                Sub::QueryProposalResult(QueryProposalResult(args)) => {
                    rpc::query_proposal_result(ctx, args).await;
                }
                Sub::QueryProtocolParameters(QueryProtocolParameters(args)) => {
                    rpc::query_protocol_parameters(ctx, args).await;
                }
            }
        }
        cli::AnomaClient::WithoutContext(cmd, global_args) => match cmd {
            // Utils cmds
            Utils::JoinNetwork(JoinNetwork(args)) => {
                utils::join_network(global_args, args).await
            }
            Utils::FetchWasms(FetchWasms(args)) => {
                utils::fetch_wasms(global_args, args).await
            }
            Utils::InitNetwork(InitNetwork(args)) => {
                utils::init_network(global_args, args)
            }
            Utils::InitGenesisValidator(InitGenesisValidator(args)) => {
                utils::init_genesis_validator(global_args, args)
            }
        },
    }
    Ok(())
}
