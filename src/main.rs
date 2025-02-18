use cmds::handles::handles_commands;

mod cli;
mod cmds;
mod core;
mod styles;

#[tokio::main]
async fn main() {
    handles_commands().await;
}
