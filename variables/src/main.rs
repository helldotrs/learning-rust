const STARTING_MISSLES: i32 = 8;
const READY_AMOUNT: i32     = 2;

fn main() {
    let mut missiles            = STARTING_MISSLES;
    let ready: i32              = READY_AMOUNT;
    println!("Firing {} of {} missiles...", ready, missiles);
    missiles -= ready;
    println!("{} missiles left", missiles);
}
