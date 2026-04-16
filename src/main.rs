
//use std::fs;
use std::path::PathBuf;


use clap::{Parser};





#[derive(Parser)]
#[command(version, about)]
struct Args
{
    // Path to the config file
    #[arg(short, long)]
    recipes_file: PathBuf,
}


fn main() -> ()
{
    let args = Args::parse();

    println!("recipes_file: {:?}", args.recipes_file);



    if args.recipes_file.is_dir()
    {

    }




}



