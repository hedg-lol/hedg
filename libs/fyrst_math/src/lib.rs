//! Mathematical primitives for the FYRST protocol.
//!
//! This crate provides pure arithmetic functions used by the
//! on-chain bonding curve program. All operations use checked
//! math and u128 intermediates to prevent overflow.

/// Calculate the cost of buying `amount` tokens starting at `current_supply`.
///
/// Uses the linear bonding curve integral:
/// cost = base_price * amount + slope * (2 * current_supply * amount + amount^2) / 2
///
/// Returns `None` on overflow.
pub fn buy_cost(base_price: u64, slope: u64, current_supply: u64, amount: u64) -> Option<u64> {
    let bp = base_price as u128;
    let s = slope as u128;
    let cs = current_supply as u128;
    let a = amount as u128;

    let linear_part = bp.checked_mul(a)?;
    let quadratic_num = s.checked_mul(
        cs.checked_mul(2)?
            .checked_mul(a)?
            .checked_add(a.checked_mul(a)?)?
    )?;
    let quadratic_part = quadratic_num.checked_div(2)?;
    let total = linear_part.checked_add(quadratic_part)?;

    if total > u64::MAX as u128 {
        None
    } else {
        Some(total as u64)
    }
}

/// Calculate the return from selling `amount` tokens starting at `current_supply`.
///
/// Uses the inverse bonding curve integral:
/// return = base_price * amount + slope * (2 * current_supply * amount - amount^2) / 2
///
/// Returns `None` on overflow or underflow.
pub fn sell_return(base_price: u64, slope: u64, current_supply: u64, amount: u64) -> Option<u64> {
    if amount > current_supply {
        return None;
    }

    let bp = base_price as u128;
    let s = slope as u128;
    let cs = current_supply as u128;
    let a = amount as u128;

    let linear_part = bp.checked_mul(a)?;
    let term_a = cs.checked_mul(2)?.checked_mul(a)?;
    let term_b = a.checked_mul(a)?;
    let quadratic_num = s.checked_mul(term_a.checked_sub(term_b)?)?;
    let quadratic_part = quadratic_num.checked_div(2)?;
    let total = linear_part.checked_add(quadratic_part)?;

    if total > u64::MAX as u128 {
        None
    } else {
        Some(total as u64)
    }
}

/// Calculate the current spot price at a given supply level.
///
/// price = base_price + slope * supply
pub fn spot_price(base_price: u64, slope: u64, supply: u64) -> Option<u64> {
    let increment = (slope as u128).checked_mul(supply as u128)?;
    let price = (base_price as u128).checked_add(increment)?;
    if price > u64::MAX as u128 {
        None
    } else {
        Some(price as u64)
    }
}

/// Calculate the trade fee from a gross amount.
///
/// fee = amount * fee_bps / 10_000
pub fn calculate_fee(amount: u64, fee_bps: u64) -> Option<u64> {
    let fee = (amount as u128)
        .checked_mul(fee_bps as u128)?
        .checked_div(10_000)?;
    if fee > u64::MAX as u128 {
        None
    } else {
        Some(fee as u64)
    }
}

/// Calculate the deployer share of the trade fee.
///
/// deployer_fee = trade_fee / 2  (50% split)
pub fn deployer_fee(trade_fee: u64) -> u64 {
    trade_fee / 2
}

/// Calculate the protocol treasury share of the trade fee.
///
/// protocol_fee = trade_fee - deployer_fee  (remaining 50%)
pub fn protocol_fee(trade_fee: u64) -> u64 {
    trade_fee - deployer_fee(trade_fee)
}

/// Calculate the pro-rata refund amount for a single buyer.
///
/// refund = (buyer_sol_spent * escrow_balance) / total_sol_collected
///
/// Returns `None` on overflow or division by zero.
pub fn refund_amount(
    buyer_sol_spent: u64,
    escrow_balance: u64,
    total_sol_collected: u64,
) -> Option<u64> {
    if total_sol_collected == 0 {
        return None;
    }

    let numerator = (buyer_sol_spent as u128).checked_mul(escrow_balance as u128)?;
    let result = numerator.checked_div(total_sol_collected as u128)?;

    if result > u64::MAX as u128 {
        None
    } else {
        Some(result as u64)
    }
}

/// Check whether a bonding curve should graduate.
///
/// Graduation occurs when `reserve_balance >= threshold`.
pub fn should_graduate(reserve_balance: u64, threshold: u64) -> bool {
    reserve_balance >= threshold
}

/// Lamports to SOL conversion (display only).
pub fn lamports_to_sol(lamports: u64) -> f64 {
    lamports as f64 / 1_000_000_000.0
}

/// SOL to lamports conversion.
pub fn sol_to_lamports(sol: f64) -> u64 {
    (sol * 1_000_000_000.0) as u64
}

/// Calculate the average buy price for a range of tokens.
///
/// avg_price = total_cost / amount
pub fn average_price(base_price: u64, slope: u64, start_supply: u64, amount: u64) -> Option<u64> {
    if amount == 0 {
        return Some(0);
    }
    let cost = buy_cost(base_price, slope, start_supply, amount)?;
    Some(cost / amount)
}

/// Calculate the market cap at a given supply and spot price.
///
/// market_cap = supply * current_price
pub fn market_cap(base_price: u64, slope: u64, supply: u64) -> Option<u64> {
    let price = spot_price(base_price, slope, supply)?;
    let cap = (supply as u128).checked_mul(price as u128)?;
    if cap > u64::MAX as u128 {
        None
    } else {
        Some(cap as u64)
    }
}

/// Calculate fully diluted value at a theoretical max supply.
///
/// fdv = max_supply * price_at_max_supply
pub fn fully_diluted_value(
    base_price: u64,
    slope: u64,
    max_supply: u64,
) -> Option<u64> {
    market_cap(base_price, slope, max_supply)
}

/// Calculate the price impact of a buy order.
///
/// Returns the percentage price increase (in basis points) caused by buying `amount` tokens.
pub fn price_impact_bps(
    base_price: u64,
    slope: u64,
    current_supply: u64,
    amount: u64,
) -> Option<u64> {
    let price_before = spot_price(base_price, slope, current_supply)?;
    let price_after = spot_price(base_price, slope, current_supply + amount)?;

    if price_before == 0 {
        return None;
    }

    let impact = ((price_after - price_before) as u128)
        .checked_mul(10_000)?
        .checked_div(price_before as u128)?;

    Some(impact as u64)
}

/// Calculate the total reserve value locked in the bonding curve.
///
/// This is the integral of the price function from 0 to current_supply.
pub fn total_reserve_value(base_price: u64, slope: u64, supply: u64) -> Option<u64> {
    buy_cost(base_price, slope, 0, supply)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buy_cost_basic() {
        let cost = buy_cost(1000, 10, 0, 100).unwrap();
        assert_eq!(cost, 100_000 + 10 * 100 * 100 / 2);
    }

    #[test]
    fn test_buy_cost_nonzero_supply() {
        let cost = buy_cost(1000, 10, 50, 100).unwrap();
        let expected = 1000 * 100 + 10 * (2 * 50 * 100 + 100 * 100) / 2;
        assert_eq!(cost, expected);
    }

    #[test]
    fn test_sell_return_basic() {
        let ret = sell_return(1000, 10, 100, 50).unwrap();
        let expected = 1000 * 50 + 10 * (2 * 100 * 50 - 50 * 50) / 2;
        assert_eq!(ret, expected);
    }

    #[test]
    fn test_sell_exceeds_supply() {
        assert!(sell_return(1000, 10, 50, 100).is_none());
    }

    #[test]
    fn test_spot_price() {
        assert_eq!(spot_price(1000, 10, 0).unwrap(), 1000);
        assert_eq!(spot_price(1000, 10, 100).unwrap(), 2000);
    }

    #[test]
    fn test_calculate_fee() {
        assert_eq!(calculate_fee(10_000, 100).unwrap(), 100);
        assert_eq!(calculate_fee(10_000, 50).unwrap(), 50);
    }

    #[test]
    fn test_fee_split() {
        let fee = calculate_fee(10_000, 100).unwrap();
        assert_eq!(deployer_fee(fee), 50);
        assert_eq!(protocol_fee(fee), 50);
    }

    #[test]
    fn test_refund_amount() {
        let refund = refund_amount(500, 1000, 2000).unwrap();
        assert_eq!(refund, 250);
    }

    #[test]
    fn test_refund_zero_collected() {
        assert!(refund_amount(500, 1000, 0).is_none());
    }

    #[test]
    fn test_graduation() {
        assert!(!should_graduate(80_000_000_000, 85_000_000_000));
        assert!(should_graduate(85_000_000_000, 85_000_000_000));
        assert!(should_graduate(90_000_000_000, 85_000_000_000));
    }

    #[test]
    fn test_lamport_conversion() {
        assert!((lamports_to_sol(1_000_000_000) - 1.0).abs() < f64::EPSILON);
        assert_eq!(sol_to_lamports(1.0), 1_000_000_000);
    }

    #[test]
    fn test_overflow_protection() {
        assert!(buy_cost(u64::MAX, u64::MAX, u64::MAX, u64::MAX).is_none());
        assert!(spot_price(u64::MAX, u64::MAX, u64::MAX).is_none());
    }

    #[test]
    fn test_buy_sell_symmetry() {
        let base = 1000;
        let slope = 5;
        let supply = 100;
        let amount = 50;

        let cost = buy_cost(base, slope, supply, amount).unwrap();
        let ret = sell_return(base, slope, supply + amount, amount).unwrap();
        assert_eq!(cost, ret, "Buy cost should equal sell return at same range");
    }

    #[test]
    fn test_fee_precision() {
        assert_eq!(calculate_fee(1_000_000_000, 100).unwrap(), 10_000_000);
        assert_eq!(calculate_fee(1_000_000_000, 50).unwrap(), 5_000_000);
        assert_eq!(calculate_fee(999, 100).unwrap(), 9);
        assert_eq!(calculate_fee(100, 100).unwrap(), 1);
        assert_eq!(calculate_fee(99, 100).unwrap(), 0);
    }

    #[test]
    fn test_deployer_protocol_fee_sum() {
        for total in [100, 1000, 999, 1_000_000] {
            let d = deployer_fee(total);
            let p = protocol_fee(total);
            assert_eq!(d + p, total, "Fee split must sum to total");
        }
    }

    #[test]
    fn test_refund_distribution() {
        let escrow = 10_000_000_000u64;
        let total_collected = 20_000_000_000u64;

        let r1 = refund_amount(5_000_000_000, escrow, total_collected).unwrap();
        let r2 = refund_amount(15_000_000_000, escrow, total_collected).unwrap();

        assert_eq!(r1, 2_500_000_000);
        assert_eq!(r2, 7_500_000_000);
        assert!(r1 + r2 <= escrow, "Total refunds must not exceed escrow");
    }

    #[test]
    fn test_graduation_boundary() {
        let threshold = 85_000_000_000u64;
        assert!(!should_graduate(threshold - 1, threshold));
        assert!(should_graduate(threshold, threshold));
        assert!(should_graduate(threshold + 1, threshold));
    }

    #[test]
    fn test_lamport_sol_roundtrip() {
        let sol_values = [0.1, 0.5, 1.0, 2.5, 10.0, 100.0];
        for sol in sol_values {
            let lamports = sol_to_lamports(sol);
            let back = lamports_to_sol(lamports);
            assert!((back - sol).abs() < 1e-9, "Roundtrip failed for {}", sol);
        }
    }

    #[test]
    fn test_spot_price_monotonic() {
        let base = 1000;
        let slope = 10;
        let mut prev = 0u64;
        for supply in (0..1000).step_by(100) {
            let price = spot_price(base, slope, supply).unwrap();
            assert!(price >= prev, "Price must be monotonically increasing");
            prev = price;
        }
    }

    #[test]
    fn test_zero_amount_operations() {
        assert_eq!(buy_cost(1000, 10, 100, 0).unwrap(), 0);
        assert_eq!(sell_return(1000, 10, 100, 0).unwrap(), 0);
        assert_eq!(calculate_fee(0, 100).unwrap(), 0);
    }
}
