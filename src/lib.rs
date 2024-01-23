#![feature(try_trait_v2)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
pub mod command;
pub mod error;
pub mod response;

pub(crate) use error::Error;

pub fn version() -> std::collections::HashMap<String, String> {
    let mut version = std::collections::HashMap::new();
    version.insert("im-sdk".to_string(), env!("CARGO_PKG_VERSION").to_string());

    version
}

#[cfg(test)]
mod test {
    #[test]
    fn version() {
        println!("{:#?}", crate::version());
    }
}
