#![allow(dead_code)]
use std::env;
use std::{error::Error, io, process};

// This lets us write `#[derive(Deserialize)]`.
use serde::Deserialize;

// We don't need to derive `Debug` (which doesn't require Serde), but it's a
// good habit to do it for all your types.
//
// Notice that the field names in this struct are NOT in the same order as
// the fields in the CSV data!
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    latitude: f64,
    longitude: f64,
    population: Option<u64>,
    city: String,
    state: String,
}

fn run(v: &mut Vec<Record>) -> Result<(), Box<dyn Error>> {
    //let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b':')
        .from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: Record = result?;
        v.push(record);
        //println!("{:?}", record);
        // Try this if you don't like each record smushed on one line:
        // println!("{:#?}", record);
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut record_v: Vec<Record> = vec![];
    if let Err(err) = run(&mut record_v) {
        println!("{}", err);
        process::exit(1);
    }
    println!("have {} entries in the vector", record_v.len());

    //now, what do i want to do with the values...
    //
    //group by...
    //find max, min, stddev.
    //

    //if the records are grouped by proximity to each other, try:
    // arg to chunks mut is how many chunks do you want...
    let chunk_sz: usize = args[1].parse().unwrap();
    for (i, chunk) in record_v.chunks_mut(chunk_sz).enumerate() {
        //if i wasn't working with floats...
        //let max = chunk.iter().map(|x| x.latitude).max().unwrap();
        //let min = chunk.iter().map(|x| x.latitude).min().unwrap();
        //since i am:
        println!("block {i} has {} records.", chunk.len());
        let chunk_lat = chunk.iter().map(|x| x.latitude);

        let max = chunk_lat.clone().reduce(f64::max).unwrap();
        let min = chunk_lat.clone().reduce(f64::min).unwrap();
        //let max = chunk.iter().map(|x| x.latitude).reduce(f64::max).unwrap();
        //let min = chunk.iter().map(|x| x.latitude).reduce(f64::min).unwrap();
        println!("\tmin and max of group: {min}, {max}");

        let data_mean = mean(chunk_lat.clone().collect::<Vec<f64>>().as_slice());
        println!("\tMean is {:?}", data_mean.unwrap());

        let data_std_deviation = std_deviation(chunk_lat.clone().collect::<Vec<f64>>().as_slice());
        println!("\tStandard deviation is {:?}", data_std_deviation.unwrap());

        let zscore = match (data_mean, data_std_deviation) {
            (Some(mean), Some(std_deviation)) => {
                let diff = chunk_lat.clone().collect::<Vec<f64>>()[0] as f64 - mean;

                Some(diff / std_deviation)
            }
            _ => None,
        };
        println!(
            "\tZ-score of data at index 0 (with value {}) is {:?}",
            chunk_lat.clone().collect::<Vec<f64>>()[0],
            zscore
        );
    }
}

fn mean(data: &[f64]) -> Option<f64> {
    let sum = data.iter().sum::<f64>() as f64;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    }
}

fn std_deviation(data: &[f64]) -> Option<f64> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - (*value as f64);

                    diff * diff
                })
                .sum::<f64>()
                / count as f64;

            Some(variance.sqrt())
        }
        _ => None,
    }
}
