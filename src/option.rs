use std::default;

use chrono::{DateTime, Local};
use rust_decimal::Decimal;

use crate::error::{SimError, SimResult};

#[derive(Debug)]
pub enum OptionType {
    Call,
    Put,
}

#[derive(Debug)]
pub struct CryptoOption {
    pub base_coin: String,
    pub expiration_date: DateTime<Local>,
    pub strike_price: Decimal,
    pub option_type: OptionType,
}

#[derive(Debug, Default)]
pub struct CryptoOptionBuilder {
    base_coin: Option<String>,
    expiration_date: Option<DateTime<Local>>,
    strike_price: Option<Decimal>,
    option_type: Option<OptionType>,
}

impl CryptoOptionBuilder {
    pub fn new() -> CryptoOptionBuilder {
        Self::default()
    }

    pub fn base_coin(mut self, base_coin: String) -> Self {
        self.base_coin = Some(base_coin);
        self
    }

    pub fn expiration_date(mut self, expiration_date: DateTime<Local>) -> Self {
        self.expiration_date = Some(expiration_date);
        self
    }

    pub fn strike_price(mut self, strike_price: Decimal) -> Self {
        self.strike_price = Some(strike_price);
        self
    }

    pub fn option_type(mut self, option_type: OptionType) -> Self {
        self.option_type = Some(option_type);
        self
    }

    pub fn build(self) -> SimResult<CryptoOption> {
        Ok(CryptoOption {
            base_coin: self
                .base_coin
                .ok_or(SimError::option_field_not_set("base_coin"))?,
            expiration_date: self
                .expiration_date
                .ok_or(SimError::option_field_not_set("expiration_date"))?,
            strike_price: self
                .strike_price
                .ok_or(SimError::option_field_not_set("strike_price"))?,
            option_type: self
                .option_type
                .ok_or(SimError::option_field_not_set("option_type"))?,
        })
    }
}
