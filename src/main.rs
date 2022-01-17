use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::process;
use toy_pay::client::Client;
use toy_pay::transaction::{Transaction, InputTransaction};

fn main() {
    // fname was passed and exists
    let fname = match std::env::args().nth(1) {
        Some(f) => f,
        _ => {
            eprintln!("cargo run -- path_to_input.csv");
            process::exit(exitcode::USAGE)
        }
    };

    let file = match File::open(&fname) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("{fname} failed to open");
            process::exit(exitcode::NOINPUT);
        }
    };

    let reader = BufReader::new(file);
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(reader);

    let mut clients: HashMap<u16, Client> = HashMap::new();

    for line in csv_reader.deserialize() {
        let record: InputTransaction = line.unwrap();
        let transaction = match Transaction::from_input_transaction(&record) {
            Ok(tx) => tx,
            // Swallowing errors to keep processing without obscuring stdout
            Err(_) => continue,
        };

        // Swallowing errors to keep processing without obscuring stdout
        match clients.get_mut(&record.client) {
            Some(c) => {
                #[allow(unused_must_use)]
                let _ = c.process_transaction(transaction);
            },
            // create a new client, have it process the transaction and add it to the record
            None => {
                let mut new_client = Client::new(record.client.clone());
                let _ = new_client.process_transaction(transaction);
                clients.insert(record.client, new_client);

            },
        }
    }

    for client in clients {
        println!("{:?}", client);
    }
}

