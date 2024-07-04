
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Asset {
    pub name: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Portfolio {
    pub assets: Vec<Asset>,
}

impl Portfolio {
    pub fn new() -> Self {
        Portfolio { assets: vec![] }
    }

    pub fn add_asset(&mut self, asset: Asset) {
        self.assets.push(asset);
    }

    pub fn remove_asset(&mut self, name: &str) {
        self.assets.retain(|asset| asset.name != name);
    }

    pub fn calculate_portfolio_value(&self) -> f64 {
        self.assets.iter().map(|asset| asset.value).sum()
    }
}
