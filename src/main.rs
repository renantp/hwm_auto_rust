use std::process::Command;
use std::{env, fs, io};

fn main() -> std::io::Result<()> {
    println!("Type 'help' to see more information");
    loop {
        let mut s = String::new();
        io::stdin()
            .read_line(&mut s)
            .expect("Failed to read in line");
        let s = s.trim();
        match s {
            "help" => help(),
            "cs" => create_shortcut(),
            "create-shortcut" => create_shortcut(),
            "stb" => show_task_bar(),
            "htb" => hide_task_bar(),
            "reboot" => reboot(),
            "exit" => break Ok(()),
            _ => println!("Command not found. Type 'help' to see more information"),
        };
    }
}
fn help() {
    let mut path = env::current_dir().expect("Something wrong");
    path.push("data/help");
    let contents = fs::read_to_string(&path).expect("Some thing went wrong");
    println!("{}", contents);
    // dbg!(path);
}
fn create_shortcut() {
    let mut path = env::current_dir().expect("Something wrong");
    path.push("data/chienowa.desktop");
    fs::copy(&path, "/home/pi/.config/autostart/chienowa.desktop").expect("Something wrong");
    fs::copy(&path, "/usr/share/applications/chienowa.desktop").expect("Something wrong");
}
fn show_task_bar() {
    let temp_directory = env::temp_dir();
    let temp_file = temp_directory.join("file");
    let buf = String::from("@lxpanel --profile LXDE-pi\n@pcmanfm --desktop --profile LXDE-pi\n@xscreensaver -no-splash\n");
    fs::write(&temp_file, buf).expect("Something wrong");
    fs::copy(&temp_file, "/etc/xdg/lxsession/LXDE-pi/autostart").expect("Something wrong");
    reboot();
}
fn hide_task_bar() {
    let temp_directory = env::temp_dir();
    let temp_file = temp_directory.join("file");
    let buf = String::from("#@lxpanel --profile LXDE-pi\n@pcmanfm --desktop --profile LXDE-pi\n@xscreensaver -no-splash\n");
    fs::write(&temp_file, buf).expect("Something wrong");
    fs::copy(&temp_file, "/etc/xdg/lxsession/LXDE-pi/autostart").expect("Something wrong");
    let mut cmd = Command::new("reboot");
    match cmd.output() {
        Ok(output) => {
            println!("Ok {}", output.status);
        }
        Err(_) => println!("Error"),
    }
}

fn reboot() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("reboot")
            .output()
            .expect("failed to execute process")
    };
    output.stdout;
}
