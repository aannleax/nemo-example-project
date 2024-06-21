use std::env;

use nemo::model::Identifier;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        eprintln!("Usage {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    // Setup Nemo Engine and start the reasoning

    let mut engine = nemo::api::load(file_path.into()).unwrap();
    if let Err(error) = nemo::api::reason(&mut engine) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }

    // Read content of a table into a vec of rows
    // .predicage_rows() returns an iterator over [AnyDataValue]
    // which may contain data of all supported data types in nemo
    //
    // Here, we simply call .to_string() to print it

    let result_predicate = Identifier::new(String::from("result"));
    let result_table = engine
        .predicate_rows(&result_predicate)
        .expect("Error in Nemo")
        .expect("Predicate doesn't exist")
        .map(|values| {
            values
                .into_iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("Result: {:?}", result_table);
}
