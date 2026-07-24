#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pool {
    pub rate: u32,
    pub units: Vec<String>,
    pub reroll: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn normal_plus_banner() -> Self {
        Self {
            name: "Normal+".to_string(),
            short_name: "np".to_string(),
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
                    "Superfeline",
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

    pub fn catfruit_banner() -> Self {
        Self {
            name: "Catfruit".to_string(),
            short_name: "cf".to_string(),
            rate_cum_sum: vec![400, 2400, 9400, 10000],
            pools: vec![
                Pool {
                    rate: 400,
                    units: vec!["5K XP".to_string()],
                    reroll: false,
                },
                Pool {
                    rate: 2000,
                    units: vec![
                        "Speed Up",
                        "Cat CPU",
                        "10K XP",
                        "30K XP",
                        "50K XP",
                        "Purple Catfruit Seed",
                        "Red Catfruit Seed",
                        "Blue Catfruit Seed",
                        "Green Catfruit Seed",
                        "Yellow Catfruit Seed",
                    ]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                    reroll: true,
                },
                Pool {
                    rate: 7000,
                    units: vec![
                        "Rich Cat",
                        "Cat Jobs",
                        "Sniper the Cat",
                        "100K XP",
                        "200K XP",
                        "Purple Catfruit",
                        "Red Catfruit",
                        "Blue Catfruit",
                        "Green Catfruit",
                        "Yellow Catfruit",
                    ]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                    reroll: false,
                },
                Pool {
                    rate: 600,
                    units: vec!["Treasure Radar", "500K XP", "Epic Catfruit"]
                        .into_iter()
                        .map(String::from)
                        .collect(),
                    reroll: false,
                },
            ],
        }
    }

    pub fn catseye_banner() -> Self {
        Self {
            name: "Catseye".to_string(),
            short_name: "ce".to_string(),
            rate_cum_sum: vec![500, 7400, 9400, 9900, 10000],
            pools: vec![
                Pool {
                    rate: 500,
                    units: vec!["5K XP".to_string()],
                    reroll: false,
                },
                Pool {
                    rate: 6900,
                    units: vec!["10K XP", "30K XP", "Special Catseye", "Rare Catseye"]
                        .into_iter()
                        .map(String::from)
                        .collect(),
                    reroll: true,
                },
                Pool {
                    rate: 2000,
                    units: vec!["100K XP", "Super Rare Catseye"]
                        .into_iter()
                        .map(String::from)
                        .collect(),
                    reroll: false,
                },
                Pool {
                    rate: 500,
                    units: vec!["Uber Rare Catseye".to_string()],
                    reroll: false,
                },
                Pool {
                    rate: 100,
                    units: vec!["Dark Catseye".to_string()],
                    reroll: false,
                },
            ],
        }
    }

    pub fn lucky_ticket_banner() -> Self {
        Self {
            name: "Lucky Ticket".to_string(),
            short_name: "lt".to_string(),
            rate_cum_sum: vec![0, 7400, 9500, 10000],
            pools: vec![
                Pool {
                    rate: 0,
                    units: vec![],
                    reroll: false,
                },
                Pool {
                    rate: 7400,
                    units: vec![
                        "Li'l Titan Cat",
                        "Li'l Lizard Cat",
                        "Li'l Fish Cat",
                        "Li'l Bird Cat",
                        "Li'l Cow Cat",
                        "Li'l Gross Cat",
                        "Li'l Axe Cat",
                        "Li'l Tank Cat",
                        "Li'l Cat",
                        "Speed Up",
                        "Speed Up",
                        "Speed Up",
                        "Cat CPU",
                        "Cat CPU",
                        "10K XP",
                        "10K XP",
                        "10K XP",
                        "30K XP",
                        "30K XP",
                        "30K XP",
                    ]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                    reroll: true,
                },
                Pool {
                    rate: 2100,
                    units: vec!["Rich Cat", "Cat Jobs", "Sniper the Cat"]
                        .into_iter()
                        .map(String::from)
                        .collect(),
                    reroll: false,
                },
                Pool {
                    rate: 500,
                    units: vec!["Treasure Radar".to_string()],
                    reroll: false,
                },
            ],
        }
    }

    pub fn lucky_ticket_g_banner() -> Self {
        Self {
            name: "Lucky Ticket G".to_string(),
            short_name: "ltg".to_string(),
            rate_cum_sum: vec![0, 5100, 8600, 10000],
            pools: vec![
                Pool {
                    rate: 0,
                    units: vec![],
                    reroll: false,
                },
                Pool {
                    rate: 5100,
                    units: vec![
                        "Catamin A",
                        "Catamin A",
                        "Catamin A",
                        "100K XP (β)",
                        "100K XP (β)",
                        "100K XP (β)",
                    ]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                    reroll: true,
                },
                Pool {
                    rate: 3500,
                    units: vec!["Catamin B", "Catamin B", "Catamin B", "500K XP"]
                        .into_iter()
                        .map(String::from)
                        .collect(),
                    reroll: false,
                },
                Pool {
                    rate: 1400,
                    units: vec!["Catamin C", "Catamin C", "Catamin C", "1M XP"]
                        .into_iter()
                        .map(String::from)
                        .collect(),
                    reroll: false,
                },
            ],
        }
    }

    pub fn all_banners() -> Vec<Self> {
        vec![
            Self::normal_banner(),
            Self::normal_plus_banner(),
            Self::catfruit_banner(),
            Self::catseye_banner(),
            Self::lucky_ticket_banner(),
            Self::lucky_ticket_g_banner(),
        ]
    }

    pub fn all_units(&self) -> Vec<String> {
        self.pools
            .iter()
            .flat_map(|pool| pool.units.clone())
            .collect()
    }
}
