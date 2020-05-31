pub use codebreaker::Codebreaker;

pub fn decrypt(destaddr: u32, addrval: u32) {
    let mut cb = Codebreaker::new();
    let decrypted_code = cb.decrypt_code(destaddr, addrval);

    println!("Code decrypted: 0x{:x}, 0x{:x}", decrypted_code.0, decrypted_code.1);
}

pub fn encrypt(destaddr: u32, addrval: u32) {
    let mut cb = Codebreaker::new();
    let encrypted_code = cb.encrypt_code(destaddr, addrval);

    println!("Code encrypted: 0x{:x}, 0x{:x}", encrypted_code.0, encrypted_code.1);
}