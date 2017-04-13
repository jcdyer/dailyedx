# Daily EdX!

To get started:

1. [install rust](https://www.rust-lang.org/en-US/install.html)
2. Update to the nightly build so you can use rocket (the rust webserver this app is built in): `rustup install nightly-2017-04-11`*
3. Default to that nightly build: `rustup default nightly-2017-04-11`
4. run `npm install`
5. run `npm run watch`
6. run `cargo run` to get the app running
7. go to http://localhost:8000/dailyedx/

\* The docs tell you just to run `rustup install nightly`, but there's a [bug](https://github.com/rust-lang-nursery/rustup.rs/issues/1062) that currently prevents you from doing that.
