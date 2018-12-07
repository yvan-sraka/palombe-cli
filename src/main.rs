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
