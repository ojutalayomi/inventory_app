use iso4217::CurrencyCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrencyOption {
    pub code: &'static str,
    pub name: &'static str,
}

impl std::fmt::Display for CurrencyOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.code, self.name)
    }
}

pub fn options() -> Vec<CurrencyOption> {
    iso4217::all()
        .iter()
        .map(|code| CurrencyOption {
            code: code.alpha3,
            name: code.name,
        })
        .collect()
}

pub fn find_option(code: &str) -> Option<CurrencyOption> {
    iso4217::alpha3(code).map(|currency| CurrencyOption {
        code: currency.alpha3,
        name: currency.name,
    })
}

pub fn format_currency(amount: f64, currency: &str) -> String {
    format!("{} {:.2}", currency, amount)
}

pub fn currency_exp(currency: &str) -> Option<i8> {
    iso4217::alpha3(currency).map(|currency| currency.exp)
}

pub fn format_amount(amount: f64, currency: &str) -> String {
    if let Some(exp) = currency_exp(currency) {
        if exp >= 0 {
            return format!("{amount:.precision$}", precision = exp as usize);
        }
    }
    format!("{amount:.2}")
}

pub fn format_currency_with_exp(amount: f64, currency: &str) -> String {
    if let Some(exp) = currency_exp(currency) {
        if exp >= 0 {
            return format!("{currency} {amount:.precision$}", precision = exp as usize);
        }
    }
    format_currency(amount, currency)
}
