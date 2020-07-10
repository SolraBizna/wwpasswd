use rand::{prelude::*, rngs::OsRng};
use dialog::DialogBox;

fn get_username() -> String {
    match dialog::Input::new("Please enter your username")
    .title("Username").show().unwrap() {
        None => std::process::exit(0),
        Some(x) => x
    }
}

fn get_password() -> String {
    loop {
        let password_a = dialog::Password::new("Please enter your new \
                                                password")
            .title("New Password").show().unwrap();
        let password_a = match password_a {
            None => std::process::exit(0),
            Some(x) => {
                if x == "" { std::process::exit(0) }
                x
            }
        };
        let password_b = dialog::Password::new("Please confirm your new \
                                                password")
            .title("Confirm New Password").show().unwrap();
        let password_b = match password_b {
            None => std::process::exit(0),
            Some(x) => {
                if x == "" { std::process::exit(0) }
                x
            }
        };
        if password_a == password_b { break password_a }
        else {
            let _ = dialog::Message::new("Passwords did not match. \
                                          Please try again.")
                        .title("Password mismatch").show();
        }
    }
}

fn main() {
    let username = get_username();
    let password = get_password();
    let mut salt = [0; 32];
    OsRng.fill_bytes(&mut salt);
    let mut sha = lsx::sha256::BufSha256::new();
    sha.update(&salt[..]);
    sha.update(password.as_bytes());
    sha.update(&salt[..]);
    sha.update(username.as_bytes());
    sha.update(&salt[..]);
    println!("Paste this to the WW admin:\nSalt: {}\nHash: {}",
             base64::encode(&salt[..]), base64::encode(&sha.finish(&[])[..]));
}
