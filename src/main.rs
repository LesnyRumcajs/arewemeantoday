#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use chrono::{DateTime, Datelike, Utc};

use rocket::response::NamedFile;
use rocket_contrib::serve::StaticFiles;

use digest::generic_array::functional::FunctionalSequence;
use digest::Digest;
use sha2::Sha256;

static RESOURCE_DIRECTORY: &str = "static";
static SALT: &str = "f07Y4QY8Zwk1eUmQC6ZxayfT";

fn mean_check(date: DateTime<Utc>) -> bool {
    let mut hasher = Sha256::new();
    hasher.input(format!("{}{}{}{}", SALT, date.year(), date.month(), date.day()).as_bytes());
    hasher.result().fold(0, |acc, x| acc + x.count_ones()) % 2 == 0
}

#[get("/")]
fn are_we_mean_today() -> NamedFile {
    if mean_check(Utc::now()) {
        NamedFile::open(format!("{}/{}", RESOURCE_DIRECTORY, "mean.html"))
    } else {
        NamedFile::open(format!("{}/{}", RESOURCE_DIRECTORY, "not_mean.html"))
    }
    .expect("Invalid file!")
}

fn main() {
    rocket::ignite()
        .mount("/public", StaticFiles::from(RESOURCE_DIRECTORY))
        .mount("/", routes![are_we_mean_today])
        .launch();
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn check_we_are_not_too_mean_but_still_mean() {
        let date = Utc::now();
        let mean_days = (0..10000i64)
            .filter(|x| mean_check(date + Duration::days(*x)))
            .count();
        assert!(mean_days > 4500 && mean_days < 5500)
    }
}
