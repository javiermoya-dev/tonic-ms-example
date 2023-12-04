use rust_decimal::Decimal;
use std::str::FromStr;

pub fn convert_to_decimal(valor: f64) -> Decimal {
    let valor_str = valor.to_string();
    match Decimal::from_str(&valor_str) {
        Ok(decimal) => decimal,
        Err(_) => Decimal::ZERO,
    }
  }