//! A macro to show the real reason for panicking.
//!
//! Use with `#[macro_use] extern crate picnic;` at the crate root and `picnic!()` wherever.

macro_rules! picnic {
    () => { panic!("\n    : ,\n(^^(^)^^^. .                 \'\n\\ o\\_/_)\' \\  YOUR PICNIC IS ON FIRE!\n.\\_________)     .  ") };
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn it_works() {
        picnic!();
    }
}
