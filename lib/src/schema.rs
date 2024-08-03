use serde::{Deserialize, Serialize};
// use rust_decimal::prelude::Decimal;

#[derive(Debug,Serialize,Deserialize)]
pub struct OfferData {
    // #[serde(with="rust_decimal::serde::str")]
    // pub price: Decimal,
    // #[serde(with="rust_decimal::serde::str")]
    // pub size: Decimal,
    pub price: String,
    pub size: String,
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthStreamData {
    pub last_update_id: usize,
    pub bids: [OfferData; 20],
    pub asks: [OfferData; 20],
}

#[derive(Debug,Serialize,Deserialize)]
pub struct DepthStreamMessage {
    pub stream: String,
    pub data: DepthStreamData,
}