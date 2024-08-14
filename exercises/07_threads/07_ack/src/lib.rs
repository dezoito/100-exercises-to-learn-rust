use data::{Ticket, TicketDraft};
use std::sync::mpsc::{Receiver, Sender};
use store::TicketId;

use crate::store::TicketStore;

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.

/*
* The Insert command expects a draft and a sender for the id
* hence why its type is Sender<TicketId>
*
* The Get command expects an id (of type TicketId), and
* a sender for the Ticket... since tickets may or may not exist,
* we have to wrap them in an Option<>
*
*/
pub enum Command {
    Insert {
        draft: TicketDraft,
        response_sender: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_sender: Sender<Option<Ticket>>,
    },
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            // * add the correct params for the enum
            Ok(Command::Insert {
                draft,
                response_sender,
            }) => {
                // * get the ticket id and send it though the channel
                // * let _ is required by the compiler
                let id = store.add_ticket(draft);
                let _ = response_sender.send(id);
            }
            // * add the correct params for the enum
            Ok(Command::Get {
                id,
                response_sender,
            }) => {
                // * must return a ticket
                // * let _ is required by the compiler
                let ticket = store.get(id);
                let _ = response_sender.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
