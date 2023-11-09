#![allow(unused_parens)]
#![allow(non_snake_case)]


use std;
use std::io::prelude::*; // Import the Read trait
use clap::{Command, Arg};
use serde;
use serde_yaml;
use evalexpr::*;



#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Relation {

    unit1: String,
    unit2: String,
    equation: String
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct  Fields{

    field_name: String,
    relations: Vec<Relation>,

}





fn stripNumbers(word: &String) -> String{

    let mut word_modifiable = String::new();

    for c in word.chars(){

        if (c.is_numeric() == false && c != '.'){

            word_modifiable.push(c);
        }
    }


    return word_modifiable;

}






fn main() {



    const FILENAME:&str = "relations.yaml";



    let matches = Command::new("Rust Unit Converter")
        .version("0.1.0")
        .author("Yata-ta (https://github.com/Yata-ta)")
        .about("Allows the convertion of Units and Values in Math/Physics/Chemistry Scenarios")
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
                    .help("In what field of Math/Physics/Chemistry does the convertion occur"))
                
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


            let field_name_provided = sub_m.get_one::<String>("field_name").unwrap();
            let unit1_provided = sub_m.get_one::<String>("unit1").unwrap();
            let unit2_provided = sub_m.get_one::<String>("unit2").unwrap();



            // Deserialize a YAML file into a struct

            let mut file = std::fs::File::open(FILENAME).expect("Failed to open file");
            let mut file_values = String::new();
            file.read_to_string(&mut file_values).expect("Failed to read file");
            let fields: Vec<Fields> = serde_yaml::from_str(&file_values).expect("Failed to deserialize YAML");



           

            // Check if field_name provided exists in .yaml file

            let mut i:usize = 0;

            while(i < fields.len()){


                match fields.get(i){


                    Some(field) =>

                        if (field.field_name == *field_name_provided){

                            let unit1_provided_noNumbers = stripNumbers(&unit1_provided);
                            let unit2_provided_noNumbers = stripNumbers(&unit2_provided);


                            //  Find Relation

                            for relation in &field.relations{
                                                             
                                
                                if (relation.unit1 ==  unit1_provided_noNumbers && relation.unit2 == unit2_provided_noNumbers){
                                    

                                    let value:f64 = unit1_provided.replace(&unit1_provided_noNumbers, "").parse().unwrap();


                                    // Equation Evaluation

                                    let mut context = evalexpr::HashMapContext::new();
                                    context.set_value("x".into(), value.into()).unwrap();
                                    let result = eval_with_context(relation.equation.as_str(), &context);

                                    match result {
                                        Ok(value) => {
                                            println!("Result: {} {}", value,unit2_provided_noNumbers);
                                        }
                                        Err(error) => {
                                            eprintln!("Error: {:?}", error);
                                            return;
                                        }
                                    }


                                }
                            }
                            
                            
                            return;

                        }

                    None => {
                        //Error
                    }

                }
                i+=1;
            }

            if (i >= fields.len()){
                eprint!("Field Name NOT Found in: {}\n", FILENAME);
            }



        }
        _ => {
            eprintln!("Subcommand not recognized.");
        }
    }



}