#[derive(Debug, Clone)]
pub struct Pool {
    pub rate: u32,
    pub units: Vec<String>,
    pub reroll: bool,
}

#[derive(Debug, Clone)]
pub struct BannerData {
    pub name: String,
    pub short_name: String,
    pub rate_cum_sum: Vec<u32>,
    pub pools: Vec<Pool>,
}

impl BannerData {
    pub fn normal_banner() -> Self {
        Self {
            name: "Normal".to_string(),
            short_name: "n".to_string(),
            rate_cum_sum: vec![10000],
            pools: vec![Pool {
                rate: 10000,
                units: vec![
                    "Cat",
                    "Tank Cat",
                    "Axe Cat",
                    "Gross Cat",
                    "Cow Cat",
                    "Bird Cat",
                    "Fish Cat",
                    "Lizard Cat",
                    "Titan Cat",
                    "Cat Cannon Attack",
                    "Cat Cannon Charge",
                    "Worker Cat Rate",
                    "Worker Cat Wallet",
                    "Base Defense",
                    "Research",
                    "Accounting",
                    "Study",
                    "Cat Energy",
                ]
                .into_iter()
                .map(String::from)
                .collect(),
                reroll: true,
            }],
        }
    }
}
