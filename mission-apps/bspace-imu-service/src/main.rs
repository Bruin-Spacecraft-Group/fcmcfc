mod model;
mod schema;

use failure::Error;
use kubos_service::{Config, Logger, Service};
use model::{ReadData, Subsystem};
use schema::{MutationRoot, QueryRoot};
use std::sync::Arc;

fn main() -> Result<(), Error> {
  Logger::init("rust-mission-app").unwrap();

  Service::new(
    Config::new("bspace-imu-service")
      .map_err(|err| {
        error!("Failed to load service config: {:?}", err);
        err
      })
      .unwrap(),
    Subsystem::new(),
    QueryRoot,
    MutationRoot,
  )
  .start();
}
