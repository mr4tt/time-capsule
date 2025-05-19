use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use chrono::{NaiveDate, Local};
use std::fs;
use std::env;

fn main() {
    let curr_path = env::current_exe().unwrap().display().to_string() + "/../";
    let paths = fs::read_dir(format!("{curr_path}/letters")).unwrap();

    // TODO: write a json that ties filenames to metadata (ex: date to send, date written, potentially photos)

    // for each file, check if it matches today's date; send email if so
    for path in paths {
        let curr_date = Local::now().date_naive(); 

        let file_name = path.unwrap().file_name().to_str().unwrap().to_string();
        let base_file_name = file_name.split(".").next().unwrap();
        let send_date = NaiveDate::parse_from_str(base_file_name, "%Y-%m-%d");

        if let Ok(d) = send_date {
            if d == curr_date {
                println!("found a match!");
                send_email(format!("{curr_path}letters/{file_name}"), &curr_path, &base_file_name);
            }
        }
    }
}

/// Sends email from the past to yourself 
/// 
/// * `file_name`: letter's full path 
/// * `env_path`: project root path to find `.env` file
/// * `curr_date`: day the letter originates from
fn send_email(file_name: String, env_path: &str, curr_date: &str) {
    dotenv::from_path(format!("{env_path}/.env")).ok();

    let address = std::env::var("EMAIL").expect("Email address must be set");
    let password = std::env::var("PASSWORD").expect("Email password must be set");

    let email_body = fs::read_to_string(&file_name).expect("Unable to read file");

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