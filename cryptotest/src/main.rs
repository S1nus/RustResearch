extern crate crypto;
fn main() {
    println!("Hello, world!");

    let res: ([u8; 64], [u8; 32]) = crypto::ed25519::keypair(&[1, 3, 5, 234 , 552 , 255254 ,52525, 252]);

    let privkey: [u8; 64] = res.0;
    let pubkey: [u8; 32] = res.1;
    println!("{:?}", privkey[0]);

    println!("Private key:");
    for c in privkey.iter() {
        print!("{}", c);
    }
    print!("{}", "\n");

    println!("Public key:");
    for c in pubkey.iter() {
        print!("{}", c);
    }
    print!("{}", "\n");
}
