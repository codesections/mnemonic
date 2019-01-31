use crate::input_state::FsState;

pub fn mn_exists(mn: &str, fs_state: &FsState) -> bool {
    match &fs_state.mn_files() {
        Some(mn_files) => mn_files.iter().any(|file| file == &format!("{}.md", mn)),
        None => false,
    }
}
