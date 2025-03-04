use core::error;

use thiserror::Error;

use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Error)]
enum NetworkErr {
    #[error("Connection Timeout")]
    Timeout,
    #[error("Unreachable")]
    Unreachable,
}

#[derive(Debug, Error)]
enum LockErr {
    #[error("Mechanical err : {0}")]
    MechanicalErr(i32),
    #[error("{0}")]
    Network(#[from] NetworkErr),
}

fn hello() -> Result<(), LockErr> {
    Err(LockErr::Network(NetworkErr::Timeout))
}

// ^ New Section for Error handling

#[derive(Debug, Error)]
enum CardError {
    #[error("Insuffiecient Balance Need : {0} to get TIcket")]
    Insuffiecient(usize),
    #[error("Card Is Expired")]
    Expired,
    #[error("Unable to Read the Chip")]
    UnableToRead,
}

struct Card {
    id: usize,
    balace: usize,
    expiration: DateTime<Utc>,
}

fn swipe_card() -> Result<Card, CardError> {
    Ok(Card {
        id: 0,
        balace: 10,
        expiration: Utc::now() + Duration::weeks(12),
    })
}

fn purchase(card: &Card, cost: usize) -> Result<(), CardError> {
    if Utc::now() > card.expiration {
        Err(CardError::Expired)
    } else {
        if card.balace < cost {
            Err(CardError::Insuffiecient(cost - card.balace))
        } else {
            Ok(())
        }
    }
}

pub fn err_acc() {
    // match hello() {
    //     Ok(()) => println!("NA"),
    //     Err(val) => println!("{}", val),
    // }

    let new_person = swipe_card().and_then(|c| purchase(&c, 20));
    match new_person {
        Ok(()) => println!("Payment Done You are good to go"),
        Err(e) => println!("error : {}", e),
    }
}
