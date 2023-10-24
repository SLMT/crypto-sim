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
