fn blarg(b: &u32) {
   println!("{}",b); 
   b = &66;
}
fn main() {
    let v1: u32 = 5;
    blarg(&v1);
    println!("{}", v1);
}
