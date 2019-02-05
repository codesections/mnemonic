use crate::state::State;

pub fn new_mn_exists(mn: &str, state: &State) -> bool {
    state
        .filesystem()
        .mnemonic_files()
        .iter()
        .any(|file| file == mn)
}
