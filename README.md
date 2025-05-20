# Time Capsule

This project lets you send an email to your future self. Currently only works with **gmail**.

## Run

- Ensure you have letters in a `letters/` folder

Windows
- `cargo build --release && mv target/release/time-capsule.exe .`
- `./time-capsule.exe`

Linux
- `cargo build --release && mv target/release/time-capsule .`
- `./time-capsule`
  
- note: ensure your desired timezone is set

## Setup

### .env / letters

- The `.env` file contains your email and Google app password, which is used to email you when the time comes.

1. Make a copy of `.env.example` and rename to `.env`
2. Fill in the address field with your email address
3. Fill in the password field with your gmail app password
   - More info about app passwords [here](https://support.google.com/accounts/answer/185833)
   - To create an app password, go [here](https://myaccount.google.com/apppasswords)
4. Create a `letters/` folder, where you can put emails you would like to send
    - For each email, make the file name the date you'd like to send it, in `%Y-%m-%d` format
    - ex: `2025-05-12.txt`

### cronjob

- Cron is a program that can schedule times for things (cron jobs) to run on a Unix system
- In this case, we use cron to run our code once a day to see if any emails should be sent

1. Go to your repo and run `cargo build --release && mv target/release/time-capsule .` to create the executable and move it to the project root directory
2. Run `crontab -e` to create a new crontab file (contains all cron job info) or open it
3. Append `5 0 * * * ./time-capsule/time-capsule` (or current location of your executable) to the crontab file
   - This runs `time-capsule` every day at 12:05am
   - note: cron jobs run from your home directory
   - If you want logs to be in syslog, use `5 0 * * * ./time-capsule/time-capsule 2>&1 | logger -t time-capsule`
