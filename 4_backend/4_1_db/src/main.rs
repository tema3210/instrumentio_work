use std::path::PathBuf;
use clap::Parser;
use diesel_async::AsyncConnection;

mod schema;

type Connection = diesel_async::AsyncPgConnection;


#[derive(clap::Parser,Debug)]
#[command(version, about, long_about = None)]
/// CRUD to a simple DB
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand,Debug)]
enum Commands {
    CreateUser { name: String, age: u16 },
    CreateRole { at: PathBuf, permit: u8, desc: Option<String> }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv::dotenv().ok();

    let args = Args::parse();

    let db_url = std::env::var("DATABASE_URL").map_err(|e| format!("{e:?}"))?;

    let conn = Connection::establish(&db_url).await.map_err(|e| format!("{e:?}"))?;

    println!("{:?}",&args);
    Ok(())
}
