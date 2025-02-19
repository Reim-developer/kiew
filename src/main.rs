use kiew::cmds::handles::handles_commands;

#[doc = "Kiew is a Command Line Interface, for scraping website"]
#[tokio::main]

async fn main() {
    handles_commands().await;
}
