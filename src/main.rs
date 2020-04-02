#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use chrono::{Datelike, Utc};

use rocket::response::NamedFile;
use rocket_contrib::serve::StaticFiles;

use digest::generic_array::functional::FunctionalSequence;
use digest::Digest;
use sha2::Sha256;

static RESOURCE_DIRECTORY: &str = "static";
static SALT: &str = "f07Y4QY8Zwk1eUmQC6ZxayfT";

fn mean_check() -> bool {
    let mut hasher = Sha256::new();
    let now = Utc::now();
    hasher.input(format!("{}{}{}{}", SALT, now.year(), now.month(), now.day()).as_bytes());
    hasher.result().fold(0, |acc, x| acc + x.count_ones()) % 2 == 0
}

#[get("/")]
fn are_we_mean_today() -> NamedFile {
    if mean_check() {
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
