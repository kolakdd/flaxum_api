pub mod config;
pub mod domain;
pub mod routes;
pub use error::Error;
pub mod common;
pub mod db;
pub mod logger;
pub mod scalar;
pub mod state;
pub mod utils;

mod error;
