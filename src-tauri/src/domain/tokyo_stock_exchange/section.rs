#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Section(String);

/// ETF・ETN
/// PRO Market
/// REIT・ベンチャーファンド・カントリーファンド・インフラファンド
/// グロース（内国株式）
/// グロース（外国株式）
/// スタンダード（内国株式）
/// スタンダード（外国株式）
/// プライム（内国株式）
/// プライム（外国株式）
/// 出資証券

// Equity Contribution Securities
// ETFs/ ETNs
// Growth Market (Foreign)
// Growth Market(Domestic)
// Prime Market (Domestic)
// Prime Market(Foreign)
// PRO Market
// REIT, Venture Funds, Country Funds and Infrastructure Funds
// Standard Market(Domestic)
// Standard Market(Foreign)

impl core::ops::Deref for Section {
    type Target = String;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}

impl TryFrom<String> for Section {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, anyhow::Error> {
        Ok(Self(value))
    }
}
