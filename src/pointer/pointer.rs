use crate::config::strategy::{BUY_TRIGGER_USD, SELL_TRIGGER_USD};

#[derive(Debug)]
pub enum PointerSignal {
    BuyStep,
    SellStep,
}

pub struct Pointer {
    reference_price: Option<f64>,
}

impl Pointer {
    pub fn new() -> Self {
        Self {
            reference_price: None,
        }
    }

    pub fn update(&mut self, current_price: f64) -> Option<PointerSignal> {
        // First price initializes the pointer
        let reference = match self.reference_price {
            Some(p) => p,
            None => {
                self.reference_price = Some(current_price);
                return None;
            }
        };

        let diff = current_price - reference;

        // BUY: fixed USD drop
        if diff <= -BUY_TRIGGER_USD {
            self.reference_price = Some(current_price);
            return Some(PointerSignal::BuyStep);
        }

        // SELL: fixed USD rise
        if diff >= SELL_TRIGGER_USD {
            self.reference_price = Some(current_price);
            return Some(PointerSignal::SellStep);
        }

        None
    }
}
