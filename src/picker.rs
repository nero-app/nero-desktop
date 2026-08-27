use std::path::PathBuf;

pub async fn player() -> Option<PathBuf> {
    rfd::AsyncFileDialog::new()
        .set_title("Select a video player")
        .pick_file()
        .await
        .map(|handle| handle.path().to_path_buf())
}

pub async fn folder() -> Option<PathBuf> {
    rfd::AsyncFileDialog::new()
        .pick_folder()
        .await
        .map(|handle| handle.path().to_path_buf())
}

pub async fn extension() -> Option<PathBuf> {
    rfd::AsyncFileDialog::new()
        .set_title("Select an extension")
        .add_filter("WebAssembly component", &["wasm"])
        .pick_file()
        .await
        .map(|handle| handle.path().to_path_buf())
}
