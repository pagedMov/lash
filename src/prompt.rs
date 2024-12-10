use tokio::{io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader}, sync::mpsc};
use log::debug;

use crate::event::{ShellError,ShellEvent};
use crate::parser;

pub async fn prompt(sender: mpsc::Sender<ShellEvent>) {
    debug!("Reached prompt");

    let mut stdout = io::stdout();
    stdout.write_all(b"> ").await.unwrap();
    stdout.flush().await.unwrap();

    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);

    let mut input = String::new();
    reader.read_line(&mut input).await.unwrap();

    let trimmed_input = input.trim().to_string();
    let mut parser = parser::RshParser::new(&trimmed_input);
    if let Err(e) = parser.tokenize() {
        let _ = sender.send(ShellEvent::CatchError(ShellError::ParsingError(e))).await;
        let _ = sender.send(ShellEvent::Prompt).await;
    } else {
        let _ = sender.send(ShellEvent::NewAST(parser.get_ast())).await;
        let _ = sender.send(ShellEvent::Prompt).await;
    }
}
