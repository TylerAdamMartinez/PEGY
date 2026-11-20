use std::marker::PhantomData;
use std::ops::{Add, Div};

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use rusty_money::{iso, Money};

#[derive(Debug, Clone, Copy)]
struct Dimensionless;

#[derive(Debug, Clone, Copy)]
struct PercentPoints;

#[derive(Debug, Clone, Copy)]
struct Quantity<U> {
    value: Decimal,
    unit: PhantomData<U>,
}

impl<U> Quantity<U> {
    fn new(value: Decimal) -> Self {
        Self {
            value,
            unit: PhantomData,
        }
    }
}

type Percent = Quantity<PercentPoints>;
type Ratio = Quantity<Dimensionless>;

impl<U> Add for Quantity<U> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.value + rhs.value)
    }
}

// Money / Money -> Dimensionless
// Only matching currencies can be divided.
fn divide_money(
    lhs: &Money<'_, iso::Currency>,
    rhs: &Money<'_, iso::Currency>,
) -> Result<Ratio, &'static str> {
    if lhs.currency().iso_alpha_code != rhs.currency().iso_alpha_code {
        return Err("Currency mismatch");
    }

    lhs.amount()
        .checked_div(*rhs.amount())
        .map(Ratio::new)
        .ok_or("Division by zero or overflow")
}

// Ratio / Percent -> Ratio
impl Div<Percent> for Ratio {
    type Output = Option<Ratio>;

    fn div(self, rhs: Percent) -> Self::Output {
        self.value.checked_div(rhs.value).map(Ratio::new)
    }
}

fn main() {
    let stock_price = Money::from_decimal(dec!(150.00), iso::USD);
    let earnings_per_share = Money::from_decimal(dec!(10.00), iso::USD);

    let pe_ratio =
        divide_money(&stock_price, &earnings_per_share).expect("Invalid P/E calculation");

    let growth_rate = Percent::new(dec!(20));
    let dividend_yield = Percent::new(dec!(2));

    let peg_ratio = (pe_ratio / growth_rate).expect("Invalid PEG calculation");

    let pegy_ratio = (pe_ratio / (growth_rate + dividend_yield)).expect("Invalid PEGY calculation");

    println!("P/E:  {}", pe_ratio.value);
    println!("PEG:  {}", peg_ratio.value);
    println!("PEGY: {}", pegy_ratio.value);
}
