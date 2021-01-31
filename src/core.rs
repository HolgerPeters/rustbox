use super::config;
use super::err;

use slog::*;

/// aggregate root
pub fn update(cfg: config::Config) -> err::Result<()> {
    let log = &cfg.logger();
    info!(log, "Setup of processing");
    Ok(()).and_then(|x| {
        info!(log, "success");
        Ok(x)
    })
}
