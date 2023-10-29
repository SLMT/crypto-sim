use std::collections::HashMap;

use rust_decimal::Decimal;

use crate::option::CryptoOption;

#[derive(Debug)]
struct Holding {
    amount: Decimal,
    cost: Decimal,
}

pub struct Portofolio {
    holdings: HashMap<CryptoOption, Holding>,
}

impl Portofolio {
    pub fn get_options(&self) -> Vec<&CryptoOption> {
        self.holdings.keys().collect()
    }
}

#[derive(Debug, Default)]
pub struct PortofolioBuilder {
    holdings: HashMap<CryptoOption, Holding>,
}

impl PortofolioBuilder {
    pub fn new() -> PortofolioBuilder {
        Self::default()
    }

    pub fn add(
        mut self,
        option: CryptoOption,
        amount: Decimal,
        cost: Decimal,
    ) -> PortofolioBuilder {
        self.holdings.insert(option, Holding { amount, cost });
        self
    }

    pub fn build(self) -> Portofolio {
        Portofolio {
            holdings: self.holdings,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Local;

    use crate::option::{CryptoOptionBuilder, OptionType};

    use super::*;

    fn create_dummy_option(option_type: OptionType) -> CryptoOption {
        let builder = CryptoOptionBuilder::new();
        let exp = Local::now();
        let price = Decimal::new(2000, 0);
        builder
            .base_coin("ETH".to_owned())
            .expiration_date(exp)
            .strike_price(price)
            .option_type(option_type)
            .build()
            .unwrap()
    }

    #[test]
    fn test_builder() {
        let option1 = create_dummy_option(OptionType::Call);
        let option2 = create_dummy_option(OptionType::Put);
        let builder = PortofolioBuilder::new();
        let portofolio = builder
            .add(option1.clone(), Decimal::ONE, Decimal::new(50, 0))
            .add(option2.clone(), Decimal::ONE, Decimal::new(100, 0))
            .build();
        let options = portofolio.get_options();
        assert!(options.contains(&&option1));
        assert!(options.contains(&&option2));
    }
}
