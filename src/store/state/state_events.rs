use crate::store::notifications::types::Notification;

use super::ui_state::UIState;

pub enum StateEvents {
    UpdateState(UIState),
    Alert(Notification),
}
