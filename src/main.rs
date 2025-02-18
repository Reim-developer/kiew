use cmds::handles::handles_commands;

mod cli;
mod cmds;
mod core;

#[tokio::main]
async fn main() {
    handles_commands().await;
}
