use clap::Parser;
use std::io::stdin;
use std::io::{stdout, Write};

mod io_main;
use io_main::{format_with_si_prefix, get_si_prefix_str, input_resistance_with_si_prefix};

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Calculate combined resistance for series/parallel circuits (SI prefix supported)\n抵抗の直列/並列接続時の合成抵抗を計算するCLIツール（SI接頭辞対応）",
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
        let formatted_result = format_with_si_prefix(result);
        if is_en {
            println!(
                "Combined Parallel resistance: {:.1}{}Ω",
                formatted_result.value,
                get_si_prefix_str(&formatted_result.prefix)
            );
        } else {
            println!(
                "並列合成抵抗: {:.1}{}Ω",
                formatted_result.value,
                get_si_prefix_str(&formatted_result.prefix)
            );
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
        let formatted_result = format_with_si_prefix(result);
        if is_en {
            println!(
                "Combined Series resistance: {:.1}{}Ω",
                formatted_result.value,
                get_si_prefix_str(&formatted_result.prefix)
            );
        } else {
            println!(
                "直列合成抵抗: {:.1}{}Ω",
                formatted_result.value,
                get_si_prefix_str(&formatted_result.prefix)
            );
        }
    }
}

fn input_register_loop(is_en: bool) -> Vec<f64> {
    let mut register_vec = Vec::new();
    let mut num_of_resister: u8 = 1;
    let mut continue_input = true;

    while continue_input {
        if is_en {
            print!("R{} > ", num_of_resister);
        } else {
            print!("抵抗{}> ", num_of_resister);
        }
        stdout().flush().unwrap();

        let mut input = String::new();
        if stdin().read_line(&mut input).is_ok() {
            let input = input.trim();
            if input.is_empty() {
                if is_en {
                    print!("Do you want to run the calculations?(y/n) > ");
                } else {
                    print!("計算を実行しますか？(y/n) > ");
                }
                stdout().flush().unwrap();

                let mut confirm = String::new();
                if stdin().read_line(&mut confirm).is_ok() && confirm.trim().to_lowercase() == "y" {
                    continue_input = false;
                }
            } else {
                match input_resistance_with_si_prefix(input.to_string()) {
                    Ok(value) => {
                        register_vec.push(value);
                        num_of_resister += 1;
                    }
                    Err(_) => {
                        if is_en {
                            println!("Invalid input. Please enter a number with optional SI prefix (e.g., 100k, 1M)");
                        } else {
                            println!(
                                "不正な入力です。数値とSI接頭辞を入力してください（例：100k、1M）"
                            );
                        }
                    }
                }
            }
        }
    }

    if is_en {
        println!("Execution calculating...");
    } else {
        println!("計算を実行中...");
    }
    register_vec
}

fn select_calc_mode() -> String {
    let mut mode = String::new();
    std::io::stdin().read_line(&mut mode).expect("入力エラー");
    mode.trim().to_string()
}

fn calculate_register_serial(reg: Vec<f64>) -> f64 {
    // 直列回路の計算
    reg.iter().sum()
}

fn calculate_register_parallel(reg: Vec<f64>) -> f64 {
    if reg.is_empty() {
        return 0.0;
    }
    if reg.iter().any(|&x| x == 0.0) {
        return 0.0; // 並列回路に0Ωが含まれる場合は0Ω
    }
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

    #[test]
    fn test_input_validation() {
        // SI接頭辞の解析
        assert_eq!(
            input_resistance_with_si_prefix("1k".to_string()),
            Ok(1000.0)
        );
        assert_eq!(
            input_resistance_with_si_prefix("1M".to_string()),
            Ok(1000000.0)
        );
        assert_eq!(input_resistance_with_si_prefix("1m".to_string()), Ok(0.001));

        // 不正な入力
        assert!(input_resistance_with_si_prefix("abc".to_string()).is_err());
        assert!(input_resistance_with_si_prefix("1kk".to_string()).is_err()); // 重複するSI接頭辞
        assert!(input_resistance_with_si_prefix("1kM".to_string()).is_err()); // 重複するSI接頭辞
        assert!(input_resistance_with_si_prefix("-1k".to_string()).is_ok()); // 負の値は許容
    }

    #[test]
    fn test_serial_calculation() {
        // 基本的な計算
        assert_eq!(calculate_register_serial(vec![100.0, 200.0]), 300.0);

        // SI接頭辞を含む値の計算
        let values = vec![
            input_resistance_with_si_prefix("1k".to_string()).unwrap(),
            input_resistance_with_si_prefix("500".to_string()).unwrap(),
        ];
        assert_eq!(calculate_register_serial(values), 1500.0);

        // エッジケース
        assert_eq!(calculate_register_serial(vec![0.0]), 0.0);
        assert_eq!(calculate_register_serial(vec![]), 0.0);
    }

    #[test]
    fn test_parallel_calculation() {
        // 基本的な計算
        assert_eq!(calculate_register_parallel(vec![100.0, 100.0]), 50.0);

        // SI接頭辞を含む値の計算
        let values = vec![
            input_resistance_with_si_prefix("1k".to_string()).unwrap(),
            input_resistance_with_si_prefix("1k".to_string()).unwrap(),
        ];
        assert_eq!(calculate_register_parallel(values), 500.0);

        // エッジケース
        assert_eq!(calculate_register_parallel(vec![0.0]), 0.0); // 0Ωの処理
        assert_eq!(calculate_register_parallel(vec![]), 0.0); // 空の入力
        assert_eq!(calculate_register_parallel(vec![100.0, 0.0]), 0.0); // 0Ωを含む場合
    }

    #[test]
    fn test_resistance_formatting() {
        // 各スケールでの表示確認
        let test_cases = vec![
            (0.000001, "1.0µ"),
            (0.001, "1.0m"),
            (1.0, "1.0"),
            (1000.0, "1.0k"),
            (1000000.0, "1.0M"),
            (1000000000.0, "1.0G"),
        ];

        for (value, expected) in test_cases {
            let formatted = format_with_si_prefix(value);
            assert_eq!(
                format!(
                    "{:.1}{}",
                    formatted.value,
                    get_si_prefix_str(&formatted.prefix)
                ),
                expected
            );
        }
    }

    #[test]
    fn test_language_settings() {
        // CLIの言語設定テスト
        let cli = Cli::parse_from(&["app", "--lang", "en"]);
        assert_eq!(cli.lang, "en");

        let cli = Cli::parse_from(&["app", "--lang", "ja"]);
        assert_eq!(cli.lang, "ja");

        // 不正な言語設定
        let cli = Cli::parse_from(&["app", "--lang", "fr"]);
        assert_ne!(cli.lang, "ja"); // デフォルト値の確認
    }

    #[test]
    fn test_cli_arguments() {
        // モード指定
        let cli = Cli::parse_from(&["app", "s"]);
        assert_eq!(cli.calc_mode, Some("s".to_string()));

        let cli = Cli::parse_from(&["app", "p"]);
        assert_eq!(cli.calc_mode, Some("p".to_string()));

        // 不正なモード
        let cli = Cli::parse_from(&["app", "x"]);
        assert_eq!(cli.calc_mode, Some("x".to_string())); // バリデーションの確認
    }
}
