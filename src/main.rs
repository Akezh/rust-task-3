extern crate sha2;

use sha2::{Sha256, Digest};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "N")]
    n: i32,

    #[structopt(short = "F")]
    f: i32,
}


fn main() {
    let opt = Opt::from_args();

    let mut res_str: String = "".to_string();
    for i in 0..opt.n {
        res_str += "0";
    }

    let mut count = 0;
    let mut current_num = 0;

    while count < opt.f {
        let mut hashing_func = Sha256::new();
        hashing_func.update(current_num.to_string());
        let result = hashing_func.finalize();
        let hash_str = format!("{:x}", result);

        if hash_str.ends_with(&res_str) {
            println!("{}, {}", current_num, hash_str);
            count += 1;
        }

        current_num += 1;
    }
}
