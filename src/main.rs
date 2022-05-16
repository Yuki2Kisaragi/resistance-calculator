use clap::{App, Arg};
use std::io::stdin;

mod cli;
mod io_main;

fn main() {
    let matches = App::new("reg-calc")
        .version("0.1.0")
        .author("Masa-Hero")
        .about("Resister Calculator")
        .arg(
            Arg::new("calc_mode")
                .about("Select Calculation Pararell or Serial")
                .value_name("MODE")
                .index(1)
                .required(false),
        )
        .get_matches();

    match matches.value_of("calc_mode") {
        Some(mode) => {
            if mode == "s" {
                println!("Mode : Serial Calc")
            } else if mode == "p" {
                println!("Mode : Parallel Calc")
            }
        }
        None => {
            print!("Which calculation mode do you want to run, in Parallel(p) or Series(s)? > ")
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

fn insert_test() {
    let register_vec: Vec<f64> = vec![200.0, 100.0, 300.5];
    let result = calculate_register_serial(register_vec);
    println!("combind register[ohm]: {}", result);

    let register_vec: Vec<f64> = vec![100.0, 100.0];
    let result = calculate_register_parallel(register_vec);
    println!("Combind Parallel register[ohm]: {}", result);

    let insert = input();
    println!("insert strings:{insert}");
}

fn input() -> f64 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("入力エラー");

    return s.trim().parse().expect("convert error");
}

fn row_input() -> String {
    let mut s = String::new();

    std::io::stdin().read_line(&mut s).expect("insert error");

    return s.trim().parse().expect("convert error");
}

fn create_vector_register() -> Vec<f64> {
    let mut register_vec = Vec::new();
    loop {
        let s = input();
        register_vec.push(s);
    }

    register_vec
}

fn calculate_register_serial(reg: Vec<f64>) -> f64 {
    reg.iter().sum()
}

fn calculate_register_parallel(reg: Vec<f64>) -> f64 {
    let inverse_reg: Vec<_> = reg.iter().map(|x| 1.0 / x).collect();
    let tmp_combind_reg: f64 = inverse_reg.iter().sum();
    1.0 / tmp_combind_reg
}

fn shell_loop() -> Vec<String> {
    let mut register_vec = Vec::new();

    while let Some(line) = shell_read_line() {
        let action = match shell_parse_line(&line) {
            None => break,
            Some(action) => action,
        };
        register_vec.push(action);
    }
    register_vec
}

fn shell_parse_line(line: &str) -> Option<String> {
    match line.is_empty() {
        true => None,
        false => Some(line.to_string()),
    }
}

fn shell_read_line() -> Option<String> {
    let mut result = String::new();
    match stdin().read_line(&mut result) {
        Ok(size) => {
            if size == 0 {
                None
            } else {
                // 改行を削除
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
