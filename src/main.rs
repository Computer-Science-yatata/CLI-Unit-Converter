use std;
use std::io::prelude::*; // Import the Read trait
use clap::{Command, Arg};
use serde;
use serde_yaml;


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Relation {

    //#[serde(rename = "unit1")]
    unit1: String,
    //#[serde(rename = "unit2")]
    unit2: String,
    //#[serde(rename = "equation")]
    equation: String
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct  Fields{

    //#[serde(rename = "field_name")]
    field_name: String,
    //#[serde(rename = "relations")]
    relations: Vec<Relation>,

}






fn main() {



    const FILENAME:&str = "relations.yaml";



    let matches = Command::new("Rust Unit Converter")
        .version("0.1.0")
        .author("Yata-ta (https://github.com/Yata-ta)")
        .about("Allows the convertion of Units and Values in Math/Physics Scenarios")
        .allow_missing_positional(false)
        

        .subcommand(
            Command::new("convert")
                .about("Converts units based on field name")


                .arg(Arg::new("field_name")
                    .required(true)
                    .default_value("Trignometry")
                    //.index(1)
                    .short('f')
                    .long("field_name")
                    .action(clap::ArgAction::Set)
                    .help("In what field of Math/Physics does the convertion occur"))
                
                .arg(Arg::new("unit1")
                    .required(false)
                    .default_value("None")
                    //.index(2)
                    .short('u')
                    .long("unit1")
                    .action(clap::ArgAction::Set)
                    .help("The value together with its unit letter for convertion"))
                
                .arg(Arg::new("unit2")
                    .required(false)
                    .default_value("None")
                    //.index(1)
                    .short('v')
                    .long("unit2")
                    .action(clap::ArgAction::Set)
                    .help("The unit to which the first value with be converted to")),

                    
                
        ).get_matches();

    
    
    match matches.subcommand_matches("convert") {
        Some(sub_m) => {
            let field_name = sub_m.get_one::<String>("field_name").unwrap();
            let unit1 = sub_m.get_one::<String>("unit1").unwrap();
            let unit2 = sub_m.get_one::<String>("unit2").unwrap();

            println!("Selected field_name: {}", field_name);
            println!("Selected unit1: {}", unit1);
            println!("Selected unit2: {}", unit2);







            // Deserialize a YAML file into a struct
            let mut file = std::fs::File::open(FILENAME).expect("Failed to open file");

            let mut file_values = String::new();
            
            file.read_to_string(&mut file_values).expect("Failed to read file");


            //print!("FIle Values\n {}", file_values);
            //assert_eq!(file_values, "field_name: thermodynamics\nrelations:\n- unit1: ºC\nunit2: K\nequation: x + 274.15");

            let fields: Vec<Fields> = serde_yaml::from_str(&file_values).expect("Failed to deserialize YAML");



            //print!("{}\n", fields[1].field_name)



        }
        _ => {
            eprintln!("Subcommand not recognized.");
        }
    }



}