#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SIprefix {
    None = 0,
    Pico,
    Nano,
    Micro,
    Milli,
    Kilo,
    Mega,
    Giga,
    Tera,
}

pub struct ResistanceValue {
    pub value: f64,
    pub prefix: SIprefix,
}

pub fn search_si_prefix(number: &str) -> Result<SIprefix, String> {
    let mut found_prefix = SIprefix::None;

    // SI接頭辞の検出
    let prefixes = [
        ('p', SIprefix::Pico),
        ('n', SIprefix::Nano),
        ('u', SIprefix::Micro),
        ('m', SIprefix::Milli),
        ('k', SIprefix::Kilo),
        ('M', SIprefix::Mega),
        ('G', SIprefix::Giga),
        ('T', SIprefix::Tera),
    ];

    for (c, prefix) in prefixes.iter() {
        let count = number.chars().filter(|&x| x == *c).count();
        if count > 0 {
            if count > 1 {
                return Err(format!("Invalid: SI prefix '{}' appears multiple times", c));
            }
            if found_prefix != SIprefix::None {
                return Err("Multiple different SI prefixes found".to_string());
            }
            found_prefix = *prefix;
        }
    }

    Ok(found_prefix)
}

pub fn input_resistance_with_si_prefix(input: String) -> Result<f64, String> {
    let si_prefix = search_si_prefix(&input)?;
    let numeric_str: String = input
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
        .collect();

    match numeric_str.parse::<f64>() {
        Ok(value) => {
            let multiplier = match si_prefix {
                SIprefix::Pico => 1e-12,
                SIprefix::Nano => 1e-9,
                SIprefix::Micro => 1e-6,
                SIprefix::Milli => 1e-3,
                SIprefix::None => 1.0,
                SIprefix::Kilo => 1e3,
                SIprefix::Mega => 1e6,
                SIprefix::Giga => 1e9,
                SIprefix::Tera => 1e12,
            };
            Ok(value * multiplier)
        }
        Err(_) => Err("Invalid number format".to_string()),
    }
}

pub fn format_with_si_prefix(value: f64) -> ResistanceValue {
    let abs_value = value.abs();
    let (prefix, multiplier) = if abs_value >= 1e12 {
        (SIprefix::Tera, 1e12)
    } else if abs_value >= 1e9 {
        (SIprefix::Giga, 1e9)
    } else if abs_value >= 1e6 {
        (SIprefix::Mega, 1e6)
    } else if abs_value >= 1e3 {
        (SIprefix::Kilo, 1e3)
    } else if abs_value >= 1.0 {
        (SIprefix::None, 1.0)
    } else if abs_value >= 1e-3 {
        (SIprefix::Milli, 1e-3)
    } else if abs_value >= 1e-6 {
        (SIprefix::Micro, 1e-6)
    } else if abs_value >= 1e-9 {
        (SIprefix::Nano, 1e-9)
    } else {
        (SIprefix::Pico, 1e-12)
    };

    ResistanceValue {
        value: value / multiplier,
        prefix,
    }
}

pub fn get_si_prefix_str(prefix: &SIprefix) -> &'static str {
    match prefix {
        SIprefix::Pico => "p",
        SIprefix::Nano => "n",
        SIprefix::Micro => "µ",
        SIprefix::Milli => "m",
        SIprefix::None => "",
        SIprefix::Kilo => "k",
        SIprefix::Mega => "M",
        SIprefix::Giga => "G",
        SIprefix::Tera => "T",
    }
}
