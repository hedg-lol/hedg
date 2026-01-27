//! CLI configuration and environment handling.
//!
//! Manages RPC endpoints, keypair paths, and program ID resolution
//! for different Solana clusters.

use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Solana cluster configuration.
#[derive(Debug, Clone, PartialEq)]
pub enum Cluster {
    Devnet,
    Testnet,
    Mainnet,
    Custom(String),
}

impl Cluster {
    /// Parse a cluster from a string identifier.
    pub fn from_str_name(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "devnet" | "d" => Cluster::Devnet,
            "testnet" | "t" => Cluster::Testnet,
            "mainnet" | "mainnet-beta" | "m" => Cluster::Mainnet,
            url => Cluster::Custom(url.to_string()),
        }
    }

    /// Get the RPC URL for this cluster.
    pub fn rpc_url(&self) -> String {
        match self {
            Cluster::Devnet => "https://api.devnet.solana.com".to_string(),
            Cluster::Testnet => "https://api.testnet.solana.com".to_string(),
            Cluster::Mainnet => "https://api.mainnet-beta.solana.com".to_string(),
            Cluster::Custom(url) => url.clone(),
        }
    }

    /// Get the explorer URL base for this cluster.
    pub fn explorer_url(&self, address: &str) -> String {
        let cluster_param = match self {
            Cluster::Devnet => "?cluster=devnet",
            Cluster::Testnet => "?cluster=testnet",
            Cluster::Mainnet => "",
            Cluster::Custom(_) => "?cluster=custom",
        };
        format!("https://explorer.solana.com/address/{}{}", address, cluster_param)
    }
}

/// Format a public key for display, showing first and last 4 characters.
pub fn short_address(pubkey: &Pubkey) -> String {
    let s = pubkey.to_string();
    if s.len() > 12 {
        format!("{}...{}", &s[..4], &s[s.len()-4..])
    } else {
        s
    }
}

/// Parse a SOL amount string, supporting both decimal and lamport notation.
pub fn parse_sol_amount(input: &str) -> Result<u64, String> {
    if input.ends_with("lamports") || input.ends_with("l") {
        let num_str = input.trim_end_matches("lamports").trim_end_matches("l").trim();
        num_str.parse::<u64>().map_err(|e| format!("Invalid lamport amount: {}", e))
    } else {
        let sol: f64 = input.parse().map_err(|e| format!("Invalid SOL amount: {}", e))?;
        if sol < 0.0 {
            return Err("Amount cannot be negative".to_string());
        }
        Ok((sol * 1_000_000_000.0) as u64)
    }
}

/// Validate that a string is a valid base58 public key.
pub fn validate_pubkey(input: &str) -> Result<Pubkey, String> {
    Pubkey::from_str(input).map_err(|e| format!("Invalid public key '{}': {}", input, e))
}

/// Format lamports as a human-readable SOL string.
pub fn format_sol(lamports: u64) -> String {
    let sol = lamports as f64 / 1_000_000_000.0;
    if sol >= 1.0 {
        format!("{:.4} SOL", sol)
    } else if sol >= 0.001 {
        format!("{:.6} SOL", sol)
    } else {
        format!("{} lamports", lamports)
    }
}

/// Format a timestamp as a human-readable date string.
pub fn format_timestamp(unix_ts: i64) -> String {
    let secs = unix_ts;
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;

    if days > 0 {
        format!("{}d {}h {}m ago", days, hours, mins)
    } else if hours > 0 {
        format!("{}h {}m ago", hours, mins)
    } else {
        format!("{}m ago", mins)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cluster_parsing() {
        assert_eq!(Cluster::from_str_name("devnet"), Cluster::Devnet);
        assert_eq!(Cluster::from_str_name("d"), Cluster::Devnet);
        assert_eq!(Cluster::from_str_name("mainnet"), Cluster::Mainnet);
        assert_eq!(Cluster::from_str_name("mainnet-beta"), Cluster::Mainnet);
    }

    #[test]
    fn test_cluster_rpc_urls() {
        assert_eq!(Cluster::Devnet.rpc_url(), "https://api.devnet.solana.com");
        assert_eq!(Cluster::Mainnet.rpc_url(), "https://api.mainnet-beta.solana.com");
    }

    #[test]
    fn test_short_address() {
        let pk = Pubkey::from_str("CcyByKGzRDK17icyNGAgdUN4q7WzbL1BPi4BNzqytyMP").unwrap();
        let short = short_address(&pk);
        assert!(short.starts_with("CcyB"));
        assert!(short.ends_with("tyMP"));
        assert!(short.contains("..."));
    }

    #[test]
    fn test_parse_sol_amount() {
        assert_eq!(parse_sol_amount("1.0").unwrap(), 1_000_000_000);
        assert_eq!(parse_sol_amount("0.5").unwrap(), 500_000_000);
        assert_eq!(parse_sol_amount("1000000lamports").unwrap(), 1_000_000);
        assert!(parse_sol_amount("-1.0").is_err());
    }

    #[test]
    fn test_format_sol() {
        assert_eq!(format_sol(1_000_000_000), "1.0000 SOL");
        assert_eq!(format_sol(500_000_000), "0.500000 SOL");
        assert_eq!(format_sol(100), "100 lamports");
    }

    #[test]
    fn test_validate_pubkey() {
        assert!(validate_pubkey("CcyByKGzRDK17icyNGAgdUN4q7WzbL1BPi4BNzqytyMP").is_ok());
        assert!(validate_pubkey("not-a-pubkey").is_err());
        assert!(validate_pubkey("").is_err());
    }

    #[test]
    fn test_format_timestamp() {
        assert_eq!(format_timestamp(90061), "1d 1h 1m ago");
        assert_eq!(format_timestamp(3660), "1h 1m ago");
        assert_eq!(format_timestamp(120), "2m ago");
    }
}
