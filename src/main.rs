use clap::Parser;
use std::io::stdin;
use std::io::{stdout, Write};

mod io_main;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// 計算モード: 直列(s)または並列(p)
    #[arg(value_name = "MODE")]
    calc_mode: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let calc_mode = match cli.calc_mode {
        Some(mode) if mode == "s" || mode == "p" => mode,
        _ => {
            print!("Which calculation mode do you want to run, in Parallel(p) or Series(s)? > ");
            stdout().flush().unwrap();
            select_calc_mode()
        }
    };

    if calc_mode == "p" {
        println!("Mode : Parallel Calculation");
        let register_vec: Vec<f64> = input_register_loop();
        let result = calculate_register_parallel(register_vec);
        println!("Combind Parallel resistance[ohm]: {}", result);
    }

    if calc_mode == "s" {
        println!("Mode : Serial Calculation");
        let register_vec: Vec<f64> = input_register_loop();
        let result = calculate_register_serial(register_vec);
        println!("Combind Serial resistance[ohm]: {}", result);
    }
}

fn input_register_loop() -> Vec<f64> {
    let mut register_vec = Vec::new();
    let mut num_of_resister: u8 = 1;

    while let Some(line) = input_read_line(num_of_resister) {
        let action = match input_parse_line(&line) {
            None => break,
            Some(action) => input_str2float(action),
        };
        register_vec.push(action);
        num_of_resister += 1;
    }
    register_vec
}
fn input_str2float(s: String) -> f64 {
    let mut text = s;
    std::io::stdin().read_line(&mut text).expect("入力エラー");
    return text.trim().parse().expect("convert error");
}
fn input_parse_line(line: &str) -> Option<String> {
    match line.is_empty() {
        true => None,
        false => Some(line.to_string()),
    }
}

fn input_read_line(num: u8) -> Option<String> {
    print!("R{}>", num);
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
    loop {
        let mut mode = String::new();
        std::io::stdin().read_line(&mut mode).expect("入力エラー");

        return mode.trim().parse().expect("convert error");
    }
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
        assert_eq!(calculate_register_serial(vec![200.0, 100.0, 300.5]), 600.5);
    }

    #[test]
    fn test_calc_serial_2() {
        assert_eq!(calculate_register_serial(vec![100.0, 100.0,]), 100.0);
    }
    #[test]
    fn test_calc_parallel_1() {
        assert_eq!(calculate_register_parallel(vec![100.0, 100.0,]), 50.0);
    }
}
