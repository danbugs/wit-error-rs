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
wit_error_rs::impl_from!(MyError, hello_world::Error::ErrorWithDescription);

pub enum MyError {
    ErrorWithDescription(String)
}

impl std::fmt::Debug for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::ErrorWithDescription(e) => f
                .debug_tuple("MyError::ErrorWithDescription")
                .field(e)
                .finish()
        }    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl std::error::Error for MyError {}

pub struct HelloWorld;

impl hello_world::HelloWorld for HelloWorld {
    fn cause_error() -> Result<(), hello_world::Error> {
        let err = cause_my_error();
        Ok(err?)
        // Ok(cause_anyhow_error()?)
    }
}

fn cause_anyhow_error() -> Result<()> {
    bail!("cause_anyhow_error caused an error");
}

fn cause_my_error() -> Result<(), MyError> {
    Err(MyError::ErrorWithDescription("cause_my_error caused an error".to_string()))
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
        //         "cause_my_error caused an error".to_string()
        //     ),
        //     _err
        // ));
        Ok(())
    }
}
