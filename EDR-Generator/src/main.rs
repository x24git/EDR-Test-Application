use clap::{Arg, App};
use crate::modules::logger::Logger;
use crate::modules::commander::TaskCommander;


mod modules;

fn main(){
    let matches = App::new("EDR Event Generator")
        .version("1.0")
        .author("Christopher Makarem")
        .about("Creates EDR events to verify detection and classification")
        .arg(Arg::with_name("Deliminator")
            .short("d")
            .long("deliminator")
            .value_name("CHARACTER")
            .help("Sets a custom deliminator for the specified input file (default value: ',')")
            .takes_value(true))
        .arg(Arg::with_name("Output File")
            .short("o")
            .long("outfile")
            .value_name("FILE")
            .help("Sets the output file location to log events (default value: 'log.csv')")
            .takes_value(true))
        .arg(Arg::with_name("INPUT")
            .value_name("FILE")
            .help("Sets the input file to use for event creation")
            .required(true)
            .index(1))
        .get_matches();
    let delim = matches.value_of("Deliminator").unwrap_or(",");
    let out_file = matches.value_of("Output File").unwrap_or("log.csv");
    let input_file = matches.value_of("INPUT").unwrap_or("windows_input.csv");
    let logger = Logger::new(&String::from(out_file));

    let mut commander = match TaskCommander::new(&input_file.to_string(), delim.as_bytes()[0], logger) {
        Ok(inner) => inner,
        Err(e) => {
            eprintln!("Encountered an unexpected error when setting up: {}", e);
            return
        }
    };
    let mut commands_processed = 0;
    while commander.read_next() {
        commands_processed = commands_processed + 1;
    }
    if commands_processed <= 0 {
        eprintln!("Input File was empty or was of bad format. No Commands Processed")
    } else {
        println!("Done. {} Instructions Found. Encountered {} error(s).", commands_processed, commander.get_num_errors())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_good_inputs() {
        let logger = Logger::new(&String::from("test.csv"));
        let mut commander =  TaskCommander::new(&"tests/good_test.csv".to_string(), ",".as_bytes()[0], logger).unwrap();
        let mut commands_processed = 0;
        while commander.read_next() {
            commands_processed = commands_processed + 1;
        }
        assert_eq!(0, commander.get_num_errors())
    }
    #[test]
    fn test_bad_inputs() {
        let logger = Logger::new(&String::from("test.csv"));
        let mut commander =  TaskCommander::new(&"tests/bad_test.csv".to_string(), ",".as_bytes()[0], logger).unwrap();
        let mut commands_processed = 0;
        while commander.read_next() {
            commands_processed = commands_processed + 1;
        }
        assert_eq!(9, commander.get_num_errors())
    }
}
