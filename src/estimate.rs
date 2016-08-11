use std::io;
use std::error::Error;

use clap;
use csv;

use libprosic::estimation::effective_mutation_rate;

pub fn effective_mutation_rate(matches: &clap::ArgMatches) -> Result<(), Box<Error+Send+Sync>> {
    let min_af = value_t!(matches, "min-af", f64).unwrap_or(0.12);
    let max_af = value_t!(matches, "max-af", f64).unwrap_or(0.25);
    let mut reader = csv::Reader::from_reader(io::stdin());
    let freqs = try!(reader.decode().collect::<Result<Vec<f64>, _>>());
    let estimate = effective_mutation_rate::Estimator::train(freqs.into_iter().filter(|&f| f >= min_af && f <= max_af));

    // print estimated mutation rate to stdout
    println!("{}", estimate.effective_mutation_rate());

    // if --observations is given, print observations
    if let Some(path) = matches.value_of("observations") {
        let mut writer = try!(csv::Writer::from_file(path));
        for (fr, mf) in estimate.observations() {
            try!(writer.write([format!("{}", fr), format!("{}", mf)].iter()));
        }
    }
    Ok(())
}