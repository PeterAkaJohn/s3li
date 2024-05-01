
#[derive(Debug)]
pub enum Action {
    Quit,
    Tick,
    Render,
    Key(String)
}