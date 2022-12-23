const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let mut missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;

    // with type
    // let mut missiles: i32 = STARTING_MISSILES;
    // let ready: i32 = READY_AMOUNT;

    // This will have error rust can't auto convert integer types
    //let ready: i64 = READY_AMOUNT;

    // in one line
    //let mut (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    
    // in one line with types
    //let mut (missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    
    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;
    println!("{} missiles left", missiles);

    println!();
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles - ready);
}