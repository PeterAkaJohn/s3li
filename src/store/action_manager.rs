use std::collections::HashMap;

use time::OffsetDateTime;

use crate::{action::Action, logger::LOGGER};

#[derive(Debug, Clone)]
struct Operation {
    timestamp: String,
    action: Action,
}

#[derive(Default, Debug, Clone)]
pub struct ActionManager {
    actions: Vec<Operation>,
}

impl ActionManager {
    pub fn push(&mut self, action: Action) {
        let now = OffsetDateTime::now_utc();
        let operation = Operation {
            timestamp: format!("{}", now),
            action,
        };

        LOGGER.info(&format!("Running Action {:#?}", operation));
        self.actions.push(operation);
    }
}
