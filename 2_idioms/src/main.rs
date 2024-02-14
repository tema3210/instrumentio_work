use std::collections::HashMap;

#[derive(Hash,PartialEq,Eq)]
struct Product {
    name: String,
    price: u32
}

const COINS: &[u32] = &[50,20,10,5,2,1];

#[derive(Clone,Copy)]
struct InCoins([u32;6]);

impl InCoins {
    /// this greedy algorithm works only with normalized monetary systems (that's why there are no others)
    fn money(mut sum: u32) -> Self {
        let mut coins = [0;6];
        for (idx,coin) in COINS.iter().enumerate() {
            let (amm,leftover) = (sum / coin,sum % coin);
            coins[idx] = amm;
            sum = leftover;
        };
        Self(coins)
    }

    /// this returns composition of coins for `sum` of `self`'s coins
    fn coins_for_sum(&self,sum: u32) -> Option<InCoins> {
        let mut curr_sum = 0;
        let mut curr_coins = [0;6];

        for (i,(nominal, ammount)) in COINS.iter().zip(self.0.iter()).enumerate() {
            let target = (sum - curr_sum); // what we want on this iteration

            for attempt in 0..*ammount {
                let next = *ammount.min(&(attempt + 1)); //the next possible ammount of coins

                match (attempt * nominal < target, next * nominal < target) {
                    (true, true) if next > attempt => continue,
                    (true, true) if next == attempt => {
                        curr_coins[i] = attempt;
                        curr_sum += attempt * nominal;
                        break;
                    },
                    (true,true) => unreachable!("next < attempt !!!!"),
                    (true,false) => {
                        curr_coins[i] = attempt;
                        curr_sum += attempt * nominal;
                        break;
                    },
                    (false, true) => unreachable!("impossible"),
                    (false,false) => {
                       break;
                    }
                }
            }


            if nominal * ammount > target {
                continue
            } else {
                
            };
        }

        unimplemented!()
    }

    fn trade(&self, other: &Self, price: u32) -> Option<[InCoins;2]> {
        if other.total() < price { return None };
        
        let change = other.total() - price; //what we owe
        
        let coins_got = InCoins({ //all the coins at dispose
            let mut ret = [0;6];
            for i in 0..6 {
                ret[i] = self.0[i] + other.0[i];
            };
            ret
        });





    }

    fn total(&self) -> u32 {
        COINS.iter().enumerate().fold(0, |acc, (i,amm)| acc + COINS[i] * *amm )
    }
}

struct Money {
    sum: u32,
    in_coins: InCoins, //symetrical to order of the coins above
}

struct VendingMachine {
    stored: HashMap<Product,u8>,
    has: InCoins

}

impl VendingMachine {
    fn purchase(&mut self,what: Product, money: InCoins) -> Option<(Product,InCoins)> {
        if self.stored[&what] > 1 {

        } else {
            None
        }
    }
}







fn main() {
    println!("Implement me!");
}
