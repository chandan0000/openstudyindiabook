use migration::cli;
use openstudyindiabook::run;

use dotenv::dotenv;

 
#[tokio::main]
async fn main() {
    dotenv().ok();
    cli::run_cli(migration::Migrator).await;

    run().await;
}

// cargo watch -q -c -w src/ -x run
// sea-orm-cli generate entity  -o entity/src                                                