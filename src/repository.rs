use std::collections::HashMap;

use rust_decimal::prelude::Decimal;



pub struct Database {
    pub books: HashMap<String, Book>,
}

pub struct Book {
    pub bids: Page,
    pub asks: Page,
}
pub struct Page {
    pub offers: [Offer;20],
    pub cumm_cap: [Decimal;20],
    pub cumm_vol: [Decimal;20],
}
#[derive(Clone)]
pub struct Offer {
    pub price: Decimal,
    pub size: Decimal,
}

impl Page {
    fn new(offers: [Offer;20]) -> Page {
        let mut cumm_vol: [Decimal;20] = [Default::default(); 20];
        let mut cumm_cap: [Decimal;20] = [Default::default(); 20];

        cumm_cap[0] = offers[0].price*offers[0].size;
        cumm_vol[0] = offers[0].size;


        for i in 1..20 {
            cumm_cap[i] = cumm_cap[i-1] + offers[i-1].price*offers[i-1].size;
            cumm_vol[i] = cumm_vol[i-1] + offers[i-1].size;
        }

        Page {
            offers,
            cumm_cap,
            cumm_vol,
        }
    }
}
impl Page {
    fn effective_price(&self, size: Decimal) -> Result<Decimal,String> {
        let p = self.cumm_vol.iter().position(|&x| x < size);
        let i = match p {
            Some(i) => i,
            None => return Err("Size requested is greater than the volume in the book".to_string()),
        };

        let marginal_volume = size - self.cumm_vol[i-1];
        let marginal_cap = self.offers[i].price * marginal_volume;

        let total_cap = self.cumm_cap[i] + marginal_cap;

        Ok(total_cap / size)
    }
}

pub struct Tip {
    pub bid: Offer,
    pub ask: Offer,
}

impl Book {
    fn tip(&self) -> Tip{
        Tip {
            bid: self.bids.offers[0].clone(),
            ask: self.asks.offers[0].clone(),
        }
    }
}