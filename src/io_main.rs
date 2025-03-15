#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn run() {
    input_string_search();
    input_resistance();
}

#[allow(dead_code)]
pub fn input_resistance() {
    println!("Enter 'input_resistance' func");

    println!("Please Insert Number");
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).ok();
    //let n: i32 = number.trim().parse().ok().unwrap();

    let n: Result<i32, _> = number.trim().parse();

    match n {
        Ok(v) => {
            println!("input number :{:?}", v);
            println!("read ok");
        }

        Err(_) => println!("read ng"),
    }

    println!("input number :{:?}", n);
}

fn _search_si_prefix(number: String) -> SIprefix {
    let mut si_prefix: SIprefix = SIprefix::None;

    if number.contains("p") {
        si_prefix = SIprefix::Pico;
    }
    if number.contains("n") {
        si_prefix = SIprefix::Nano;
    }
    if number.contains("u") {
        si_prefix = SIprefix::Micro;
    }
    if number.contains("m") {
        si_prefix = SIprefix::Milli;
    }
    if number.contains("k") {
        si_prefix = SIprefix::Kilo;
    }
    if number.contains("M") {
        si_prefix = SIprefix::Mega;
    }
    if number.contains("G") {
        si_prefix = SIprefix::Giga;
    }
    if number.contains("T") {
        si_prefix = SIprefix::Tera;
    }

    si_prefix
}

fn _verify_counter_of_si_prefix(mut counter: u8) -> bool {
    counter += 1;

    if counter > 1 {
        // SI単位系文字が1つ以上ある
        false
    } else {
        true
    }
}

fn _search_si_prefix2(number: String) -> Option<SIprefix> {
    let mut si_prefix: SIprefix = SIprefix::None;
    let count_si_prefix: u8 = 0;

    if number.contains("p") {
        si_prefix = SIprefix::Pico;
        _verify_counter_of_si_prefix(count_si_prefix);
    }
    if number.contains("n") {
        si_prefix = SIprefix::Nano;
    }
    if number.contains("u") {
        si_prefix = SIprefix::Micro;
    }
    if number.contains("m") {
        si_prefix = SIprefix::Milli;
    }
    if number.contains("k") {
        si_prefix = SIprefix::Kilo;
    }
    if number.contains("M") {
        si_prefix = SIprefix::Mega;
    }
    if number.contains("G") {
        si_prefix = SIprefix::Giga;
    }
    if number.contains("T") {
        si_prefix = SIprefix::Tera;
    }

    Some(si_prefix)
}

pub fn input_string_search() {
    println!("Enter 'input_string_search' func");
    println!("Please Insert Resistance(SI Prefix OK)");

    let mut number = String::new();
    std::io::stdin().read_line(&mut number).ok();

    let si_prefix = _search_si_prefix(number);

    match si_prefix {
        SIprefix::Pico => println!("Pico"),
        SIprefix::Nano => println!("Nano"),
        SIprefix::Micro => println!("Micro"),
        SIprefix::Milli => println!("Milli"),
        SIprefix::Kilo => println!("Kilo"),
        SIprefix::Mega => println!("Mega"),
        SIprefix::Giga => println!("Giga"),
        SIprefix::Tera => println!("Tera"),
        _ => println!("None"),
    }

    //  let n: i32 = number.trim().parse().ok().unwrap();
}
