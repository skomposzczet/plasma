pub mod db;

use thiserror::Error;

const DATABASE: &'static str = "plasma";

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed connecting to db")]
    CouldNotConnectToDB,
}
