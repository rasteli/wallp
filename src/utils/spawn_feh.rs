use std::process::Command;

pub fn spawn_feh(images: Vec<String>, preview: bool) {
    let mut args = vec!["--bg-fill".to_string()];

    if preview {
        args.push("--no-fehbg".to_string());
    }

    for img in images {
        args.push(img);
    }

    Command::new("feh")
        .args(args)
        .spawn()
        .expect("Failed to spawn feh");
}
