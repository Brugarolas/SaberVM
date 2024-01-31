mod error_handling;
mod header;
mod parse;
mod pretty;
mod verify;

use std::fs;

use crate::header::ByteStream;
use crate::header::Error;
use crate::header::VerifiedStmt::*;

fn go(bytes: &ByteStream) -> Result<(), Error> {
    let unverified_stmts = parse::go(bytes)?;

    let verified_stmts = verify::go(unverified_stmts)?;

    // the following is just for debugging
    let first_func = &verified_stmts[0];
    let Func(_, func_type, ops) = first_func;
    dbg!(pretty::typ(&func_type));
    for op in ops {
        println!("{}", pretty::verified_op(&op))
    }

    Ok(())
}

fn main() {
    // get the bytes from the local bin.svm file (later this will be a CLI arg of course)
    let bytes = fs::read("bin.svm").unwrap();

    let mb_error = go(&bytes);

    error_handling::handle(mb_error);
}
