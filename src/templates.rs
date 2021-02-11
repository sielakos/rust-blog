use tera::Tera;

pub fn init_tera(pattern: &str) -> Tera {
    match Tera::new(pattern) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    }
}
