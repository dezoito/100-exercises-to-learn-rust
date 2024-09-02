use crate::data::{Status, Ticket, TicketDraft};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(u64);

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Arc<Mutex<Ticket>>>,
    counter: u64,
}

impl TicketStore {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        // * in this case we have to wrap the ticket in bot Mutex and Arc
        self.tickets.insert(id, Arc::new(Mutex::new(ticket)));
        id
    }

    // The `get` method should return a handle to the ticket
    // which allows the caller to either read or modify the ticket.
    pub fn get(&self, id: TicketId) -> Option<Arc<Mutex<Ticket>>> {
        // * Here, the return type is already an Option wrapping and Arc and a Mutex!
        // * the cloned() method is used to return a clone of the Arc<Mutex<Ticket>> value stored in the HashMap. Let's break it down:
        self.tickets.get(&id).cloned()
    }
}
