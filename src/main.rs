//! Palombe
//! =======
//!
//! Palombe lets you send and receive messages synchronously through
//! different processes using named pipes.
//!
//! ```shell
//! palombe send foo bar
//! ```
//!
//! ```shell
//! palombe receive foo # bar
//! ```
//!
//! Aknowlegments
//! -------------
//!
//! :warning: This tool is not suited for building soytware, it is intend to
//! be use only in rapid prototyping and first product development steps!
//!
//! If you looking for a better / faster / saffer way to share typed (yes
//! you want that) data across different processes, take a look at [Google
//! Protocal Buffer](https://developers.google.com/protocol-buffers/) or
//! even better at [Cap'n Proto](https://capnproto.org/) (which is
//! infinitely faster).
//!
//! Supported environments
//! ----------------------
//!
//! The tool is embed into modules targeting several environments:
//!
//! -   ECMAScript: [npm](https://www.npmjs.com/package/palombe) \|
//!     [Yarn](https://yarnpkg.com/fr/package/palombe)
//!     ([*Sources*](https://github.com/yvan-sraka/palombe-node))
//! -   Python: [PyPI](https://pypi.org/project/palombe/)
//!     ([*Sources*](https://github.com/yvan-sraka/palombe-python))
//! -   Ruby: [RubyGem.org](https://rubygems.org/gems/palombe)
//!     ([*Sources*](https://github.com/yvan-sraka/palombe-ruby))
//! -   Rust: [Crates.io](https://crates.io/crates/palombe)
//!     ([*Sources*](https://github.com/yvan-sraka/palombe-rust))
//!
//! Contributing
//! ------------
//!
//! Please read
//! [CONTRIBUTING.md](https://github.com/yvan-sraka/Palombe/blob/master/CONTRIBUTING.md)
//! for details on our code of conduct, and the process for submitting pull
//! requests to us.
//!
//! Authors
//! -------
//!
//! -   [Yvan Sraka](https://github.com/yvan-sraka)
//!
//! See also the list of
//! [contributors](https://github.com/yvan-sraka/Palombe/graphs/contributors)
//! who participated in this project.
//!
//! License
//! -------
//!
//! This project is licensed under the 3rd version of GPL License - see the
//! [LICENSE](https://github.com/yvan-sraka/Palombe/blob/master/LICENSE)
//! file for details.

extern crate palombe;

extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Palombe")
        .version("0.1.0")
        .author("Yvan Sraka <yvan@sraka.pw>")
        .about("Palombe lets you send and receive messages synchronously through different processes using named pipes")
        .subcommand(SubCommand::with_name("send")
            .about("Send a message")
            .arg(Arg::with_name("NAME")
                .help("Sets the name of the value to send")
                .required(true)
                .index(1))
            .arg(Arg::with_name("VALUE")
                .help("Sets the value to send")
                .required(true)
                .index(2)))
        .subcommand(SubCommand::with_name("receive")
            .about("Receive a message")
            .arg(Arg::with_name("NAME")
                .help("Sets the name of the value to receive")
                .required(true)
                .index(1)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("send") {
        let name = matches.value_of("NAME").unwrap();
        let value = matches.value_of("VALUE").unwrap();
        palombe::send(name.to_string(), value.to_string());
    }
    if let Some(matches) = matches.subcommand_matches("receive") {
        let name = matches.value_of("NAME").unwrap();
        print!("{}", palombe::receive(name.to_string()));
    }
}
