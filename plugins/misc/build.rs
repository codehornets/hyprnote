const COMMANDS: &[&str] = &[
    "get_git_hash",
    "get_fingerprint",
    "opinionated_md_to_html",
    "open_audio",
    "delete_session_folder",
];

fn main() {
    let gitcl = vergen_gix::GixBuilder::all_git().unwrap();
    vergen_gix::Emitter::default()
        .add_instructions(&gitcl)
        .unwrap()
        .emit()
        .unwrap();

    tauri_plugin::Builder::new(COMMANDS).build();
}
