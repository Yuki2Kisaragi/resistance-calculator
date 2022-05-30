use regex::Regex;
use std::io::stdin;
use std::io::{stdout, Write};

enum SIprefix {
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

struct ResistanceValue {
    str_resistance: String,
    real_resistance: f64,
    si_prefix: SIprefix,
}
impl ResistanceValue {
    fn _si_prefix_search(&mut self) -> &Self {
        if self.str_resistance.contains("p") {
            self.si_prefix = SIprefix::Pico;
        }
        if self.str_resistance.contains("n") {
            self.si_prefix = SIprefix::Nano;
        }
        if self.str_resistance.contains("u") {
            self.si_prefix = SIprefix::Micro;
        }
        if self.str_resistance.contains("m") {
            self.si_prefix = SIprefix::Milli;
        }
        if self.str_resistance.contains("k") {
            self.si_prefix = SIprefix::Kilo;
        }
        if self.str_resistance.contains("M") {
            self.si_prefix = SIprefix::Mega;
        }
        if self.str_resistance.contains("G") {
            self.si_prefix = SIprefix::Giga;
        }
        if self.str_resistance.contains("T") {
            self.si_prefix = SIprefix::Tera;
        }

        self
    }
    fn _set_real_resistance(&mut self) -> &Self {
        //TODO
        // 1) 文字(SI接頭辞)と数字を分割する
        // 2) 数字 * SI接頭辞の積をself.real_resistanceに拘束する
        self
    }
}

pub fn run() {
    //input_string_search();
    //input_resistance();

    let re = Regex::new(r"^[1-9][0-9]*[pnumkMGT]?$").unwrap();
    println!("{:?}", re.is_match("2000u"));
    println!("{:?}", re.is_match("10"));
}

pub fn input_resistance() {
    println!("Enter 'input_resistance' func");
    println!("Please Insert Number");
    let mut num_of_loop = 1;
    loop {
        print!("R{} >", num_of_loop);
        stdout().flush().unwrap();
        num_of_loop += 1;
    }

    let mut number = String::new();
    stdin().read_line(&mut number).ok();

    //let n: Result<i32, _> = number.trim().parse();
    //println!("input number :{:?}", n);
}

pub fn input_string_search() {
    println!("Enter 'input_string_search' func");
    println!("Please Insert Resistance(SI Prefix OK)");

    let mut number = String::new();
    stdin().read_line(&mut number).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_kiro() {
        assert_eq!("a", "a");
    }
}
