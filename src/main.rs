use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use chrono::{NaiveDate, Local};
use std::fs;
use std::env;
use std::path::{Path, PathBuf};

/// Look through letters/ folder and check if any letters should be sent today
fn main() {
    let letters_path = env::current_exe().unwrap().parent().unwrap().join("letters");
    let letters = fs::read_dir(&letters_path).unwrap();

    // TODO: write a json that ties filenames to metadata (ex: date to send, date written, potentially photos)

    // for each file, check if it matches today's date; send email if so
    for letter in letters {
        let file_name = letter.unwrap().file_name().to_str().unwrap().to_string();
        let base_file_name = file_name.split(".").next().unwrap();

        let curr_date = Local::now().date_naive();
        let send_date = NaiveDate::parse_from_str(base_file_name, "%Y-%m-%d");

        if let Ok(d) = send_date {
            if d == curr_date {
                println!("found a match!");
                send_email(letters_path.join(&file_name), letters_path.parent().unwrap(), base_file_name);
            }
        }
        println!("Done!")
    }
}

/// Sends email from the past to yourself 
/// 
/// * `file_name`: letter's full path 
/// * `env_path`: project root path to find `.env` file
/// * `curr_date`: day the letter originates from
fn send_email(file_name: PathBuf, env_path: &Path, curr_date: &str) {
    dotenv::from_path(env_path.join(".env")).ok();

    let address = std::env::var("EMAIL").expect("Email address must be set");
    let password = std::env::var("PASSWORD").expect("Email password must be set");

    let email_body = fs::read_to_string(file_name).expect("Unable to read file");

    let email = Message::builder()
        .from(format!("Timey <{address}>").parse().unwrap())
        .to(format!("Timey <{address}>").parse().unwrap())
        .subject(format!("Unearthed time capsule from {curr_date}"))
        .header(ContentType::TEXT_PLAIN)
        .body(email_body)
        .unwrap();
    
    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(Credentials::new(address, password))
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}