#![allow(dead_code)]

use chrono::{DateTime, Duration, Utc};
use thiserror::Error;

#[derive(Debug, Error)]
enum PassError {
    #[error("expired pass")]
    PassExpired,
    #[error("insufficient funds: {0}")]
    InsufficientFunds(isize),
    #[error("pass read error: {0}")]
    ReadError(String),
}

struct SubwayPass {
    id: usize,
    funds: isize,
    expires: DateTime<Utc>,
}

fn swipe_card() -> Result<SubwayPass, PassError> {
    // Err(PassError::ReadError("magstrip failure".to_owned()))
    Ok(SubwayPass {
        id: 0,
        funds: 200,
        expires: Utc::now() + Duration::weeks(52),
    })
}

fn use_pass(pass: &mut SubwayPass, cost: isize) -> Result<(), PassError> {
    if Utc::now() > pass.expires {
        Err(PassError::PassExpired)
    } else if pass.funds - cost < 0 {
        Err(PassError::InsufficientFunds(pass.funds))
    } else {
        pass.funds -= -cost;
        Ok(())
    }
}

fn main() {
    let can_board = swipe_card().and_then(|mut pass| use_pass(&mut pass, 3));
    match can_board {
        Ok(_) => println!("ok to board"),
        Err(e) => match e {
            PassError::ReadError(_) => (),
            PassError::PassExpired => (),
            PassError::InsufficientFunds(_) => (),
        },
    };
}
