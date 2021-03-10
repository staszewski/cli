use bat::PrettyPrinter;
use std::io;
use url::UrlBuilder;

mod url;

fn menu() -> u8 {
    println!("\nMENU");
    println!("----------------------------------------\n");
    println!("Please, select an option:");
    println!("\t 1. Creational patterns");
    println!("\t 2. Behavioral patterns");
    println!("\t 3. Structural patterns");
    println!("\t 0. Exit");
    let mut option: String = String::new();

    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");

    let option = option.trim().parse::<u8>();

    match option {
        Ok(o) => return o,
        Err(_) => {
            eprintln!("ERROR: Please, type a number");
            99
        }
    }
}

fn nested_menu() -> u8 {
    println!("\nMENU");
    println!("----------------------------------------\n");
    println!("Please, select an option:");
    println!("\t 1. Factory");
    let mut option: String = String::new();

    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");

    let option = option.trim().parse::<u8>();

    match option {
        Ok(o) => return o,
        Err(_) => {
            eprintln!("ERROR: Please, type a number");
            99
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    'cli: while {
        let option: u8 = menu();

        match option {
            1 => {
                /*
                        let resp =
                reqwest::blocking::get("https://raw.githubusercontent.com/lpxxn/rust-design-pattern/master/creational/singleton.rs")?.text()?;
                        PrettyPrinter::new()
                            .language("rust")
                            .line_numbers(true)
                            .input_from_bytes(resp.as_bytes())
                            .print()
                            .unwrap();
                        */
                let mut url = UrlBuilder::init();
                url.append("creational/");
                'asd: while {
                    let second_option: u8 = nested_menu();

                    match second_option {
                        1 => {
                            url.append("factory");
                            println!("{:?}", url);
                            let resp = reqwest::blocking::get(&url.url)?.text()?;
                            PrettyPrinter::new()
                                .language("rust")
                                .line_numbers(true)
                                .input_from_bytes(resp.as_bytes())
                                .print()
                                .unwrap();
                            break 'cli;
                        }
                        _ => todo!(),
                    }
                } {}
                break 'cli;
            }
            _ => {
                println!("lala");
                true
            }
        }
    } {}

    Ok(())
}
