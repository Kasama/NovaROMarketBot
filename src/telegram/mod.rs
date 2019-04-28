extern crate future;
extern crate telegram_bot_fork;
extern crate tokio;

use futures::{future::lazy, Stream};
use telegram_bot_fork::*;

pub fn start_bot(token: String) {
    Api::new(token)
        .and_then(|api: Api| {
            api.stream().then(|maybe_update| {
                let res: Result<Result<Update, Error>, ()> = Ok(maybe_update);
                res
            })
        })
}
