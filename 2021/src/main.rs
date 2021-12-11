use std::io::Read;

use ac2021::{client, solution::solve};
use structopt::StructOpt;

#[derive(StructOpt)]
enum Opt {
    Get {
        #[structopt(short, long, default_value = "2021")]
        year: i32,
        #[structopt(short, long)]
        day: i32,
    },
    Solve {
        #[structopt(short, long, default_value = "2021")]
        year: i32,
        #[structopt(short, long)]
        day: i32,
        #[structopt(short)]
        stdin: bool,
        #[structopt(short, long)]
        large: bool,
    },
}

#[actix_rt::main]
async fn main() -> Result<(), anyhow::Error> {
    match Opt::from_args() {
        Opt::Get { year, day } => {
            let problem = client::client().await?.get(year, day).await?;
            print!("{}", &problem);
        }
        Opt::Solve {
            year,
            day,
            stdin,
            large,
        } => {
            let mut input = String::new();
            if stdin {
                std::io::stdin().read_to_string(&mut input)?;
            } else {
                input = client::client().await?.get(year, day).await?;
            }
            let res = solve(input.trim(), day, large)?;
            println!("{}", res)
        }
    }
    Ok(())
}
