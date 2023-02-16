use clap::{Args, Parser, Subcommand};
mod requests;

/// Program to test the Meanscout API
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
   /// Number of times to do
   #[arg(short, long, default_value_t = 1)]
   count: u8,

   /// Change the link if needed
   #[arg(short, long, default_value="https://data.team4198.org:8000/scouting")]
   link: String,

   /// Password to be used with API
   #[arg(short, long, default_value="ChangeMe!")]
   password: String,

//    #[command(subcommand)]
//     command: Option<Commands>,

}

// #[derive(Subcommand, Debug)]
// enum Commands {
//     /// Adds
//     Add(Add),
//     /// does a thing
//     Remove(Add),
// }

// #[derive(Args, Debug)]
// struct Add {
//     name: Option<String>,
// }

fn main() {
    let args = Cli::parse();

    for _i in 0..args.count {
        let request = requests::post_data(&args.link, &args.password);
        if request.is_ok() {
            println!("it worked");
        }
        else {
            println!("Something went wrong");
            break
        }
    }
    
    // This is just some testing with commands
    // if args.command.is_some() {
    //     match &args.command.as_ref().unwrap() {
    //         Commands::Add(name) => {
    //             println!("'meantest add' was used, name is: {:?}", name.name.as_ref().unwrap())
    //         },
    //         Commands::Remove(name) => {
    //             println!("'meantest remove' was used, name is: {:?}", name.name.as_ref().unwrap())
    //         }
    //     }
    // }

//    for _ in 0..args.count {
//        println!("{}", args.password);
        // println!("{:?}", &args.command.is_some());
//    }
}