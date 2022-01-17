use std::collections::HashMap;
use std::io;
use std::process;
use toy_pay::client::{Client, ClientRecord};
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

    let mut csv_reader = match csv::Reader::from_path(&fname) {
        Ok(rdr) => rdr,
        Err(_) => {
            eprintln!("{fname} failed to open");
            process::exit(exitcode::NOINPUT);
        }
    };

    let clients = clients_process_all_tx(&mut csv_reader);
    let clients = clients.values().map(|c| ClientRecord::from_client(c)).collect();
    let mut csv_writer = csv::WriterBuilder::new().has_headers(true).from_writer(io::stdout());

    match write_all_client_records(clients, &mut csv_writer) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Failed to write client data: {err:?}");
            process::exit(exitcode::IOERR);
        }
    };
}

/// use the csv reader to process all the transactions into clients
fn clients_process_all_tx<R: io::Read>(rdr: &mut csv::Reader<R>) -> HashMap<u16, Client> {
    let mut clients: HashMap<u16, Client> = HashMap::new();

    for line in rdr.deserialize() {
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
                let mut new_client = Client::new(record.client);
                let _ = new_client.process_transaction(transaction);
                clients.insert(record.client, new_client);

            },
        }
    }

    clients
}

fn write_all_client_records<W: io::Write>(clients: Vec<ClientRecord>, wtr: &mut csv::Writer<W>) -> Result<(), csv::Error> {
    for client in clients {
        wtr.serialize(client)?;
    }

    match wtr.flush() {
        Ok(_) => (),
        Err(_) => {
            eprintln!("unable to flush csv writer buffer");
            process::exit(exitcode::IOERR);
        }
    };

    Ok(())
}
