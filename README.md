# Rust Unit Converter CLI

Rust Unit Converter CLI application to allow for fast numerical convertion between units in various fields.
Each convertion is stored in .yaml file for customization and scalability purposes.


# Usage

rust_unit_converter --help
rust_unit_converter convert --field_name <field_name> --unit1 <unit1> --unit2 <unit2>
cargo run -- convert --field_name <field_name> --unit1 <unit1> --unit2 <unit2>

Check [evalexpr](https://docs.rs/evalexpr/latest/evalexpr/) crate for supported operations

# Example

rust_unit_converter convert --field_name "thermodynamics" --unit1 "299.15K" --unit2 "ºC"


# Future Updates
