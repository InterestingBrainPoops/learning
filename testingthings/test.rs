fn F(n:u64)->u64{let mut y = 1;let mut z = 1;let mut w = 0;for x in 1..n {w=y+z;y=z;z=w}w}fn main(){println!("{}",F(80));}
37889062373143906