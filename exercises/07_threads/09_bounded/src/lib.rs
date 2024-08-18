// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};

use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
pub mod data;
pub mod store;

// * The error type has to be manually defined
// * since we are using try_send
#[derive(Debug, thiserror::Error)]
#[error("The store is overloaded")]
pub struct OverloadedError;

// * Notice we have to use SyncSender
#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

// * we now have to map errors and return a Result (try_send can err)
impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, OverloadedError> {
        // * define sender and receiver for the SYNC channel (limited to 1 msg)
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(1);

        // * this defines the sender for a TicketStoreClient
        // * it is the client invoked on the tests
        self.sender
            .try_send(Command::Insert {
                draft,
                response_channel: response_sender,
            })
            .map_err(|_| OverloadedError)?;
        // * we have to Ok, since we are returning a result
        Ok(response_receiver.recv().unwrap())
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, OverloadedError> {
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(1);
        self.sender
            .try_send(Command::Get {
                id,
                response_channel: response_sender,
            })
            .map_err(|_| OverloadedError)?;
        Ok(response_receiver.recv().unwrap())
    }
}

// * capacity to define # of messages
pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);

    std::thread::spawn(move || server(receiver));
    // * Look at the return type!
    TicketStoreClient { sender }
}

// * Commands now use SyncSender
enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
