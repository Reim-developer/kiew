//! Kiew is a Command Line Interface, for scraping website.
use kiew::cmds::handles::handles_commands;

#[doc = "Kiew"]
#[tokio::main]

async fn main() {
    handles_commands().await;
}
