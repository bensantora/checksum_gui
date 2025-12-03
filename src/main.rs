slint::include_modules!();

use sha2::{Digest, Sha256};
use std::fs;

fn main() {
    let ui = MainWindow::new().unwrap();

    ui.on_compute_checksum({
        let ui = ui.as_weak();
        move || {
            let ui = ui.unwrap();
            let path = ui.get_file_path().to_string();

            // Read file
            let data = match fs::read(&path) {
                Ok(d) => d,
                Err(e) => {
                    ui.set_result(format!("Error: {}", e).into());
                    return;
                }
            };

            // Compute SHA-256
            let mut hasher = Sha256::new();
            hasher.update(&data);
            let digest = hasher.finalize();
            ui.set_result(format!("{:x}", digest).into());
        }
    });

    ui.run().unwrap();
}
