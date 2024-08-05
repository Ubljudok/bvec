mod bvec;
mod buffer;

use bvec::BVEC;
use buffer::BVECBuffer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut instance = BVEC::new();
        println!("Zero!");
        instance.run();
        println!("Uno!");
        std::thread::sleep(std::time::Duration::from_micros(1000));
        instance.end();
        println!("Dos!");
    }
}
