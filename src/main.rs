use clap::Parser;
use std::io::stdin;
use std::io::{stdout, Write};

mod io_main;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Calculate combined resistance for series/parallel circuits\n抵抗の直列/並列接続時の合成抵抗を計算するCLIツール",
    long_about = "A CLI tool to calculate combined resistance for series/parallel circuits.\n\
                  Use 's' option for series connection, 'p' option for parallel connection.\n\
                  Interactive mode will be activated if no option is specified.\n\n\
                  抵抗の直列/並列接続時の合成抵抗を計算するCLIツールです。\n\
                  直列接続の場合は 's' オプション、並列接続の場合は 'p' オプションを指定してください。\n\
                  オプション未指定の場合は対話モードで動作します。"
)]
struct Cli {
    /// Calculation mode: Series(s) or Parallel(p)
    /// 計算モード: 直列(s)または並列(p)
    #[arg(value_name = "MODE")]
    calc_mode: Option<String>,

    /// Language setting: en/ja (default: ja)
    /// 言語設定: en/ja (デフォルト: ja)
    #[arg(short, long, default_value = "ja")]
    lang: String,
}

fn main() {
    let cli = Cli::parse();
    let is_en = cli.lang == "en";

    let calc_mode = match cli.calc_mode {
        Some(mode) if mode == "s" || mode == "p" => mode,
        _ => {
            if is_en {
                print!(
                    "Which calculation mode do you want to run, in Parallel(p) or Series(s)? > "
                );
            } else {
                print!("計算モードを選択してください。並列(p)または直列(s)? > ");
            }
            stdout().flush().unwrap();
            select_calc_mode()
        }
    };

    if calc_mode == "p" {
        if is_en {
            println!("Mode: Parallel Calculation");
        } else {
            println!("モード: 並列計算");
        }
        let register_vec: Vec<f64> = input_register_loop(is_en);
        let result = calculate_register_parallel(register_vec);
        if is_en {
            println!("Combined Parallel resistance[Ω]: {:.3}", result);
        } else {
            println!("並列合成抵抗[Ω]: {:.3}", result);
        }
    }

    if calc_mode == "s" {
        if is_en {
            println!("Mode: Series Calculation");
        } else {
            println!("モード: 直列計算");
        }
        let register_vec: Vec<f64> = input_register_loop(is_en);
        let result = calculate_register_serial(register_vec);
        if is_en {
            println!("Combined Series resistance[Ω]: {:.3}", result);
        } else {
            println!("直列合成抵抗[Ω]: {:.3}", result);
        }
    }
}

fn input_register_loop(is_en: bool) -> Vec<f64> {
    let mut register_vec = Vec::new();
    let mut num_of_resister: u8 = 1;

    while let Some(line) = input_read_line(num_of_resister, is_en) {
        let action = match input_parse_line(&line) {
            None => break,
            Some(action) => input_str2float(action, is_en),
        };
        register_vec.push(action);
        num_of_resister += 1;
    }
    register_vec
}

fn input_str2float(s: String, is_en: bool) -> f64 {
    s.trim().parse().unwrap_or_else(|_| {
        panic!(
            "{}",
            if is_en {
                "conversion error"
            } else {
                "数値変換エラー"
            }
        )
    })
}

fn input_parse_line(line: &str) -> Option<String> {
    match line.is_empty() {
        true => None,
        false => Some(line.to_string()),
    }
}

fn input_read_line(num: u8, is_en: bool) -> Option<String> {
    if is_en {
        print!("R{} > ", num);
    } else {
        print!("抵抗{}> ", num);
    }
    stdout().flush().unwrap();

    let mut result = String::new();
    match stdin().read_line(&mut result) {
        Ok(size) => {
            if size == 0 {
                None
            } else {
                let result = result.trim_end();
                Some(result.to_string())
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    }
}

fn select_calc_mode() -> String {
    let mut mode = String::new();
    std::io::stdin().read_line(&mut mode).expect("入力エラー");
    mode.trim().to_string()
}

fn calculate_register_serial(reg: Vec<f64>) -> f64 {
    reg.iter().sum()
}

fn calculate_register_parallel(reg: Vec<f64>) -> f64 {
    let inverse_reg: Vec<_> = reg.iter().map(|x| 1.0 / x).collect();
    let tmp_combind_reg: f64 = inverse_reg.iter().sum();
    1.0 / tmp_combind_reg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_serial_1() {
        assert_eq!(
            (calculate_register_serial(vec![200.0, 100.0, 300.5]) * 1000.0).round() / 1000.0,
            600.5
        );
    }

    #[test]
    fn test_calc_serial_2() {
        assert_eq!(
            (calculate_register_serial(vec![100.0, 100.0,]) * 1000.0).round() / 1000.0,
            200.0
        );
    }

    #[test]
    fn test_calc_parallel_1() {
        assert_eq!(
            (calculate_register_parallel(vec![100.0, 100.0,]) * 1000.0).round() / 1000.0,
            50.0
        );
    }

    #[test]
    fn test_calc_parallel_2() {
        assert_eq!(
            (calculate_register_parallel(vec![100.0, 100.0, 100.0]) * 1000.0).round() / 1000.0,
            33.333
        );
    }

    #[test]
    fn test_calc_parallel_3() {
        assert_eq!(
            (calculate_register_parallel(vec![100.0, 100.0, 0.0]) * 1000.0).round() / 1000.0,
            0.0
        );
    }
}
