use structopt::StructOpt;

use htp::test::{Test, TestConfig};

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "f", long = "file", help = "Input is a file name.")]
    file_input: bool,

    #[structopt(
        name = "INPUT",
        help = "Base64 encoded input, or a file name if the file option is given."
    )]
    input: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();

    for i in &opt.input {
        let input = if opt.file_input {
            std::fs::read_to_string(i).unwrap().into()
        } else {
            base64::decode(i).unwrap()
        };
        let mut t = Test::new(TestConfig());
        let res = t.run_slice(&input);
        println!("Test result: {:?} {:?}", res, t);
        println!("Connection Parser: {:?}", t.connp);
        println!("Number of transactions: {}", t.connp.tx_size());
        let mut i = 0;
        while let Some(tx) = t.connp.tx(i) {
            println!("TX {}: {:?}", i, tx);
            i += 1;
        }
    }
}
