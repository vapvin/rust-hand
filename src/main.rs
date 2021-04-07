use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random U8: {}", n1);
    println!("Random U16: {}", n2);
    println!("Random U32: {}", rng.gen::<u32>());
}
