
pub struct ItemWithMetrics {
    pub item: MarketItem,
    // pub metrics: HashMap<String,
}


pub struct ItemsChunk {
    pub idx: u64,
    pub items: Vec<MarketItem>,
    pub revision: u64,
}


pub trait Metric {
    pub fn name(&self) -> &str;
    // pub fn html
}

