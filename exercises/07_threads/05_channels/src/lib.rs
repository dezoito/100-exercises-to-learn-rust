use std::sync::mpsc::{Receiver, Sender};

use data::TicketDraft;
use store::TicketStore;

pub mod data;
pub mod store;

pub enum Command {
    Insert(TicketDraft),
}

// Start the system by spawning the server the thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.

/*
* You really have to look at tests/insert.rs to solve this one
* as there the lesson doesn't help at all
*
* 1- Here, we create an enum "Command", that defines an action
*    where the payload is a draft.
*    launch() creates the channel, sender and receiver
* 2- On insert.rs, we invoke the sender and use it to send the
*    "Insert" command for a new draft
* 3- Here, on server, we loop while we receive commands,
*    and if one matches the "Insert" enum, we add the draft to
*    the ticket store
* https://chatgpt.com/share/5fb2212a-e840-4757-ac37-dd5b2d211af5
*/
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: The server task should **never** stop.
//  Enter a loop: wait for a command to show up in
//  the channel, then execute it, then start waiting
//  for the next command.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    while let Ok(command) = receiver.recv() {
        match command {
            Command::Insert(ticket_draft) => {
                store.add_ticket(ticket_draft);
            }
        }
    }
}
