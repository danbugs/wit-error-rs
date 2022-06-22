use anyhow::{bail, Result};

wit_bindgen_rust::export!("../wit/hello-world.wit");

wit_error_rs::impl_error!(hello_world::Error);
// ^^^ implements std::fmt::Display, and std::error::Error
wit_error_rs::impl_from!(anyhow::Error, hello_world::Error::ErrorWithDescription);
// ^^^ implements From like:
// impl From<anyhow::Error> for hello_world::Error {
//     fn from(err: anyhow::Error) -> Self {
//         hello_world::Error::ErrorWithDescription(err.to_string())
//     }
// }

pub struct HelloWorld;

impl hello_world::HelloWorld for HelloWorld {
    fn cause_error() -> Result<(), hello_world::Error> {
        Ok(cause_anyhow_error()?)
    }
}

fn cause_anyhow_error() -> Result<()> {
    bail!("cause_anyhow_error caused an error");
}

#[cfg(test)]
mod tests {
    use crate::hello_world::HelloWorld;
    use crate::HelloWorld as HW;

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let _err = HW::cause_error()?;
        // vvv could have assert but want to see what this looks like failing
        // assert!(matches!(
        //     crate::hello_world::Error::ErrorWithDescription(
        //         "cause_anyhow_error caused an error".to_string()
        //     ),
        //     err
        // ));
        Ok(())
    }
}
