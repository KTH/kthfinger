use clap::Parser;
use reqwest::blocking::get;
use serde::{Deserialize, Deserializer};
use std::error::Error;
use std::fmt::{self, Display};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::from_args();
    for username in &args.names {
        match get_user(username) {
            Ok(user) => println!("{user}"),
            Err(err) => println!("Failed to get user {username:?}: {err}\n"),
        }
    }
    Ok(())
}

#[derive(Parser)]
#[clap(about, author, version)]
struct Args {
    /// User names or kthids to search for.
    #[clap(required = true)]
    names: Vec<String>,
}

fn get_user(username: &str) -> Result<User, Box<dyn Error>> {
    Ok(
        get(format!("https://api.kth.se/api/profile/1.1/{username}"))?
            .error_for_status()?
            .json()?,
    )
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
    given_name: String,
    family_name: String,
    email: String,
    url: String,
    works_for: Vec<Department>,
    #[serde(deserialize_with = "empty_string_is_none")]
    job_title: Option<String>,
    #[serde(deserialize_with = "empty_string_is_none")]
    work_location: Option<String>,
    #[serde(deserialize_with = "empty_string_is_none")]
    telephone: Option<String>,
}

impl Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "\x1b]8;;{}\x1b\\{} {}\x1b]8;;\x1b\\ <{}>",
            self.url, self.given_name, self.family_name, self.email
        )?;
        for dept in &self.works_for {
            writeln!(f, "    {}", dept.name)?;
        }
        write_opt(f, "Titel:", &self.job_title)?;
        write_opt(f, "Plats:", &self.work_location)?;
        write_opt(f, "Tel:  ", &self.telephone)?;
        Ok(())
    }
}
fn write_opt(f: &mut fmt::Formatter, label: &str, value: &Option<String>) -> fmt::Result {
    if let Some(value) = value.as_deref() {
        writeln!(f, "    {label} {value}")?;
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Department {
    name: String,
}

fn empty_string_is_none<'de, D: Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
    Ok(Option::deserialize(d)?.filter(|s: &String| !s.is_empty()))
}
