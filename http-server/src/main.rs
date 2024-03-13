#[macro_use] extern crate rocket;
// use rocket::{routes, launch, get};
use rocket::fs::{relative, NamedFile};
use std::path::{Path, PathBuf};

#[derive(FromFormField)]
enum Lang {
    #[field(value = "en")]
    English,
    #[field(value = "es")]
    Spanish
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![files])
        .mount("/v1", routes![user_usize, user_isize, user_str])
}


#[get("/<file..>", rank = 4)]
async fn files(file: PathBuf) -> NamedFile {
    if file == Path::new("") {
        // Redirige desde "/" a "/index.html"
        return NamedFile::open(Path::new(relative!("public/index.html"))).await.unwrap()
    }

    let path = Path::new(relative!("public/")).join(file);
    match NamedFile::open(path).await {
        Ok(correct_file) => correct_file,
        Err(_) => NamedFile::open(Path::new(relative!("public/error404.html"))).await.unwrap(),
    }
}



#[get("/<name>/<id>?<lang>")]
fn user_usize(name: String, id: usize, lang: Option<Lang>) -> String {
    match lang {
        Some(Lang::Spanish) => format!("Hola, usuario {} con id usize {}", name, id),
        _ => format!("Hello, user {} with usize id {}", name, id)
    }
}

#[get("/<name>/<id>", rank = 2)]
fn user_isize(name: String, id: isize) -> String {
    format!("Hello, user {} with isize id {}", name, id)
}

#[get("/<name>/<id>", rank = 3)]
fn user_str(name: String, id: String) -> String {
    format!("Hello, user {}, your id {} was not valid", name, id)
}
