mod code_handling;
use structopt::StructOpt;
use std::io;
use std::process;

#[derive(StructOpt)]
#[structopt(name = "RustCBR")]
struct Options {
    /// Decrypt a code.
    #[structopt(short = "d", long = "decrypt")]
    decrypt: bool,

    /// Encrypt a code.
    #[structopt(short = "e", long = "encrypt")]
    encrypt: bool,

    /// Print help info on how codes are formatted
    #[structopt(short = "c", long = "code-help")]
    code_help: bool,
}

fn main() {
    let opt = Options::from_args();

    if opt.decrypt {
        println!("Please input the address the code writes to (left part of the code).");

        let mut destaddr = String::new();

        io::stdin()
        .read_line(&mut destaddr)
        .expect("Error reading input");

        if destaddr == "\n" || destaddr.len() > 10 {
            eprintln!("Invalid destination address input given");
            process::exit(1);
        }

        println!("Now input the value that the code inserts into the address (right part of the code).");

        let mut addrval = String::new();

        io::stdin()
        .read_line(&mut addrval)
        .expect("Error reading input");

        if addrval == "\n" || addrval.len() > 10 {
            eprintln!("Invalid address value input given.");
            process::exit(1);
        }

        println!("Decrypting code..");

        let destaddr = u32::from_str_radix(destaddr.trim().trim_start_matches("0x"), 16)
            .expect("Failed to parse code");

        let addrval = u32::from_str_radix(addrval.trim().trim_start_matches("0x"), 16)
            .expect("Failed to parse code.");

        code_handling::decrypt(destaddr, addrval);

    } else if opt.encrypt {
        println!("Please input the address the code writes to (left part of the code).");

        let mut destaddr = String::new();

        io::stdin()
        .read_line(&mut destaddr)
        .expect("Error reading input");

        if destaddr == "\n" || destaddr.len() > 10 {
            eprint!("No destination address input given.");
            process::exit(1);
        }

        println!("Now input the value that the code inserts into the address (right part of the code)");

        let mut addrval = String::new();

        io::stdin()
        .read_line(&mut addrval)
        .expect("Error reading input");

        if addrval == "\n" || addrval.len() > 10 {
            eprint!("No address value input given");
        }

        println!("Encrypting code..");

        let destaddr = u32::from_str_radix(destaddr.trim().trim_start_matches("0x"), 16)
            .expect("Failed to parse code");

        let addrval = u32::from_str_radix(addrval.trim().trim_start_matches("0x"), 16)
            .expect("Failed to parse code");

        code_handling::encrypt(destaddr, addrval);

    } else if opt.code_help {
        println!("As with most cheat codes, the left part of the code is the memory address that is being written to,\nwhereas the right part is the value that gets written to the address. Both in are in hexadecimal notation.\nexample: 0x342143ef, 0x3293298f");
    } else {
        eprintln!("No valid input given. Pass the '--help' flag to get a list of the available commands.");
    }
}