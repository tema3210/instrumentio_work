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

        'coins: for (i,(nominal, ammount)) in COINS.iter().zip(self.0.iter()).enumerate() {
            let target = (sum - curr_sum); // what we want on this iteration

            for attempt in 0..*ammount {
                let next = *ammount.min(&(attempt + 1)); //the next possible ammount of coins

                match (attempt * nominal < target, next * nominal < target) {
                    // current try and the next try are less the needed
                    (true, true) if next > attempt => continue,
                    // we've reached a peak ammount of coins of current nominal, so take them
                    (true, true) if next == attempt => {
                        curr_coins[i] = attempt;
                        curr_sum += attempt * nominal;
                        break;
                    },
                    // well, attempt is greater than next attempt...
                    (true,true) => unreachable!("next < attempt !!!!"),
                    // we found right ammount of coins of given nominal
                    // cut the most of the price
                    (true,false) => {
                        curr_coins[i] = attempt;
                        curr_sum += attempt * nominal;
                        break;
                    },
                    // go on
                    (false, true) => continue,
                    // price is too big
                    (false,false) => {
                       break 'coins;
                    }
                }
            }
        }
        assert!(curr_sum <= sum, "sanity");

        if curr_sum < sum {
            return None;
        };

        Some(InCoins(curr_coins))
    }


    /// the fisrt is modified self the second is change
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

        let change_coins = coins_got.coins_for_sum(change)?;

        let coins_rest = InCoins({ //coins we have got in vending machine
            let mut ret = [0;6];
            for i in 0..6 {
                ret[i] = coins_got.0[i] - change_coins.0[i];
            };
            ret
        });

        Some([coins_rest,change_coins])

    }

    fn total(&self) -> u32 {
        COINS.iter().enumerate().fold(0, |acc, (i,amm)| acc + COINS[i] * *amm )
    }
}

struct VendingMachine {
    stored: HashMap<Product,u8>,
    has: InCoins
}

impl VendingMachine {
    fn purchase(&mut self,what: Product, money: InCoins) -> Option<([Product;1],InCoins)> {
        if self.stored[&what] > 1 {
            if money.total() < what.price {
                return None
            };
            let [inner,change] = self.has.trade(&money, what.price)?;
            self.has = inner;
            Some(([what],change))
        } else {
            None
        }
    }
}

fn main() {
    println!("Implement me!");
}
