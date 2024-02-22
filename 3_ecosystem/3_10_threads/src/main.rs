#![feature(portable_simd)]
use std::simd::u32x64;

use crossbeam_channel::{unbounded, Receiver, Sender};
use rand::Rng;


type Element = u32;

/// i'll assume that 4096 is total matrix size, not per side
// this gives 64 els per side
type Matrix = [[Element;64];64];

type Simd = std::simd::Simd<Element,64>;

const RNG_N_LIMIT: Element = 500;

fn producer(sender: Sender<Matrix>, limit: u8) {
    let mut mat = [[0;64];64]; //4 KiB memory wasted

    let mut rng = rand::thread_rng();
    for _ in 0..limit {
        //regen
        for row in &mut mat {
            for col in &mut row[..] {
                *col = rng.gen::<u32>() % RNG_N_LIMIT;
            }
        }
        //send
        sender.send(mat).unwrap();
    };
}

fn consumer(receiver: Receiver<Matrix>,name: String) {

    fn fast_sum(x: &Matrix) -> Element {
        let mut sum = Simd::splat(0);
        for i in x {
            sum += Simd::from_slice(i);
        }
        sum.as_array().iter().sum()
    }
    
    for i in receiver {
        println!("{} sums to {:#?}",&name,fast_sum(&i));
    }
}


fn main() {
    std::thread::scope(|s| {
        let (snd, rcv) = unbounded();
        s.spawn(|| producer(snd, 32));
        {
            let rcv = rcv.clone();
            s.spawn(|| consumer(rcv,"cons1".into()));
        }
        {
            let rcv = rcv.clone();
            s.spawn(|| consumer(rcv,"cons2".into()));
        }
    });
}
