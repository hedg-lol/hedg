//! FYRST Protocol CLI
//!
//! Command-line tool for interacting with the FYRST on-chain program.

use clap::{Parser, Subcommand};
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::read_keypair_file;
use std::str::FromStr;

const DEFAULT_RPC: &str = "https://api.devnet.solana.com";
const PROGRAM_ID: &str = "CcyByKGzRDK17icyNGAgdUN4q7WzbL1BPi4BNzqytyMP";

#[derive(Parser)]
#[command(name = "fyrst")]
#[command(about = "FYRST Protocol CLI - Responsible Token Launchpad")]
#[command(version)]
struct Cli {
    /// Solana RPC endpoint URL
    #[arg(long, default_value = DEFAULT_RPC)]
    rpc: String,

    /// Path to the payer keypair file
    #[arg(long, default_value = "~/.config/solana/id.json")]
    keypair: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show protocol information and statistics
    Info,

    /// Show escrow vault details for a token
    Escrow {
        /// Token mint address
        #[arg(long)]
        mint: String,

        /// Deployer wallet address
        #[arg(long)]
        deployer: String,
    },

    /// Show bonding curve state for a token
    Curve {
        /// Token mint address
        #[arg(long)]
        mint: String,
    },

    /// Show buyer record for a wallet and token
    Buyer {
        /// Buyer wallet address
        #[arg(long)]
        wallet: String,

        /// Token mint address
        #[arg(long)]
        mint: String,
    },

    /// Calculate buy cost for a given amount
    Quote {
        /// Token mint address
        #[arg(long)]
        mint: String,

        /// SOL amount to spend
        #[arg(long)]
        sol: f64,
    },
}

fn expand_tilde(path: &str) -> String {
    if path.starts_with('~') {
        if let Ok(home) = std::env::var("HOME") {
            return path.replacen('~', &home, 1);
        }
    }
    path.to_string()
}

fn derive_escrow_pda(deployer: &Pubkey, mint: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"escrow", deployer.as_ref(), mint.as_ref()],
        program_id,
    )
}

fn derive_curve_pda(mint: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"curve", mint.as_ref()], program_id)
}

fn derive_buyer_pda(buyer: &Pubkey, mint: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"record", buyer.as_ref(), mint.as_ref()],
        program_id,
    )
}

fn lamports_to_sol(lamports: u64) -> f64 {
    lamports as f64 / 1_000_000_000.0
}

fn main() {
    let cli = Cli::parse();

    let rpc = RpcClient::new_with_commitment(
        cli.rpc.clone(),
        CommitmentConfig::confirmed(),
    );

    let program_id = Pubkey::from_str(PROGRAM_ID).expect("Invalid program ID");

    let keypair_path = expand_tilde(&cli.keypair);
    let _payer = read_keypair_file(&keypair_path).ok();

    match cli.command {
        Commands::Info => {
            println!("FYRST Protocol");
            println!("==============");
            println!("Program ID: {}", program_id);
            println!("RPC:        {}", cli.rpc);

            match rpc.get_account(&program_id) {
                Ok(account) => {
                    println!("Status:     Deployed");
                    println!("Owner:      {}", account.owner);
                    println!("Data len:   {} bytes", account.data.len());
                }
                Err(_) => {
                    println!("Status:     Not deployed on this cluster");
                }
            }
        }

        Commands::Escrow { mint, deployer } => {
            let mint_pk = Pubkey::from_str(&mint).expect("Invalid mint address");
            let deployer_pk = Pubkey::from_str(&deployer).expect("Invalid deployer address");
            let (pda, bump) = derive_escrow_pda(&deployer_pk, &mint_pk, &program_id);

            println!("Escrow Vault");
            println!("============");
            println!("PDA:      {}", pda);
            println!("Bump:     {}", bump);
            println!("Deployer: {}", deployer_pk);
            println!("Mint:     {}", mint_pk);

            match rpc.get_account(&pda) {
                Ok(account) => {
                    println!("Balance:  {} SOL", lamports_to_sol(account.lamports));
                    println!("Data len: {} bytes", account.data.len());
                }
                Err(_) => {
                    println!("Status:   Not found (escrow may not exist)");
                }
            }
        }

        Commands::Curve { mint } => {
            let mint_pk = Pubkey::from_str(&mint).expect("Invalid mint address");
            let (pda, bump) = derive_curve_pda(&mint_pk, &program_id);

            println!("Bonding Curve");
            println!("=============");
            println!("PDA:  {}", pda);
            println!("Bump: {}", bump);
            println!("Mint: {}", mint_pk);

            match rpc.get_account(&pda) {
                Ok(account) => {
                    println!("Balance:  {} SOL", lamports_to_sol(account.lamports));
                    println!("Data len: {} bytes", account.data.len());
                }
                Err(_) => {
                    println!("Status:   Not found (curve may not exist)");
                }
            }
        }

        Commands::Buyer { wallet, mint } => {
            let wallet_pk = Pubkey::from_str(&wallet).expect("Invalid wallet address");
            let mint_pk = Pubkey::from_str(&mint).expect("Invalid mint address");
            let (pda, bump) = derive_buyer_pda(&wallet_pk, &mint_pk, &program_id);

            println!("Buyer Record");
            println!("============");
            println!("PDA:    {}", pda);
            println!("Bump:   {}", bump);
            println!("Buyer:  {}", wallet_pk);
            println!("Mint:   {}", mint_pk);

            match rpc.get_account(&pda) {
                Ok(account) => {
                    println!("Data len: {} bytes", account.data.len());
                }
                Err(_) => {
                    println!("Status:   No record found");
                }
            }
        }

        Commands::Quote { mint, sol } => {
            let mint_pk = Pubkey::from_str(&mint).expect("Invalid mint address");
            let (pda, _) = derive_curve_pda(&mint_pk, &program_id);
            let lamports = (sol * 1_000_000_000.0) as u64;

            println!("Price Quote");
            println!("===========");
            println!("Mint:      {}", mint_pk);
            println!("SOL Input: {:.4}", sol);
            println!("Lamports:  {}", lamports);

            match rpc.get_account(&pda) {
                Ok(_) => {
                    let fee = lamports / 100;
                    let net = lamports - fee;
                    println!("Fee (1%):  {} lamports ({:.4} SOL)", fee, lamports_to_sol(fee));
                    println!("Net input: {} lamports ({:.4} SOL)", net, lamports_to_sol(net));
                    println!("(Exact token amount depends on current supply)");
                }
                Err(_) => {
                    println!("Status:    Curve not found for this mint");
                }
            }
        }
    }
}
