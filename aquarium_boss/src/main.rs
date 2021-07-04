use rand::Rng;        // rng to enable random aquarium events
use std::io::BufRead; // enable reading from standard input
use std::time::Duration;
use std::thread::sleep;
use std::io::stdout;
use std::io::Write;

#[allow(unused_must_use)] // mwahahaha

fn title() {
    println!("\n  ==================================================================================");
    println!("  ==================================================================================");
    println!("  === AQUARIUM BOSS                                     ===== v0.1 =================");
    println!("  ===                                                   ============================");
}

fn print_week(week:&mut i32) {
     //de-reference and print week
    println!("  === WEEK: {}", *week);
}

fn first_week() {

    //

    let intro1 = {"
        Hey, you must be the new boss!!!"};
    for c in intro1.chars() {
       print!("{}", c);
       stdout().flush();
       sleep(Duration::from_millis(10));
    };
    sleep(Duration::from_millis(1700));

    //

    let intro2 = {"
        We have a bit of a problem:
        the aquarium doesn't have any sea animals!
        The last guy killed them all."};
    for c in intro2.chars() {
       print!("{}", c);
       stdout().flush();
       sleep(Duration::from_millis(17));
    };

    //

    let intro3_a = {"\n
        Well, "};
    print!("{}", intro3_a);
    sleep(Duration::from_millis(1000));

    //

    let intro3_b = {"except for the snails."};
    print!("{}", intro3_b);
    stdout().flush().unwrap();
    sleep(Duration::from_millis(500));

    //

    let intro4 = {"
                                        (Ohhhh god the snails...)"};
    for c in intro4.chars() {
       print!("{}", c);
       stdout().flush();
       sleep(Duration::from_millis(20));
    };

    //

    sleep(Duration::from_millis(2000));
    let intro5 = {"
            ANYWAY!"};
    println!("{}", intro5);
    sleep(Duration::from_millis(2000));

    //

    let intro6 = {"
        Just buy some fish! Feed them!
        How hard can it be?!\n\n"};
    for c in intro6.chars() {
       print!("{}", c);
       stdout().flush();
       sleep(Duration::from_millis(17));
    };
    sleep(Duration::from_millis(2000));

    let line = ("\n  ==================================================================================\n\n");
    for c in line.chars() {
       print!("{}", c);
       stdout().flush();
       sleep(Duration::from_millis(13));
    };
}

fn tanks() {
    let tanks = {"
  Aquarium Tanks:
 ___________________  ___________________  ___________________  ___________________
 |^^^^^^^^^^^^^^^^^|  |                 |  |^^^^^^^^^^^^^^^^^|  |                 |
 |                 |  |~~~~~~~~~~~~~~~~~|  |                 |  |~~~~~~~~~~~~~~~~~|
 |                 |  |                 |  |                 |  |                 |
 |_________________|  |_________________|  |_________________|  |_________________|

    Ocean Exhibit       Coastal Wetlands     <Empty Exhibit>      <Empty Exhibit>
  _________________    _________________    _________________   __________________
    \n"};
    println!("{}", tanks);
}

fn bank(balance:&mut i32) {
    // to-do: need to refactor this printing style into its own function
    let line = ("  ==================================================================================\n\n");
    for c in line.chars() {
       print!("{}", c);
       stdout().flush();
       sleep(Duration::from_millis(13));
    };
    println!("  === BANK BALANCE: ${}", *balance);
}

// each week you have the chance for a random event
fn event() {
    let mut rng = rand::thread_rng();             // threadsafe rng object assigned to variable
    let random = rng.gen_range(1..11);            // inclusive min to non-inclusive upper bound
    // match random {
    //     1 => println!("A quiet week..."),
    //     2 => println!("Visit boom! Extra income"),
    //     3 => println!("Tank leaks..."),
    //     4 => println!("Algae attack!"),
    //     5 => println!("Penguins lead a rebellion!"),
    //     6 => println!("Cute sea otter babies bring extra visitors!"),
    //     7 => println!("A shark attacks its tankmates!"),
    //     8 => println!("Snails take over"),
    //     9 => println!("Animal gets sick"),
    //     10 => println!("idk?!"),
    // };
}

fn main() {
    let mut balance:i32 = 15000;                     // starting bank balance
    let mut week:i32 = 1;                            // declare non-constant variable
    title();
    if week == 1 {
        first_week();
    }
    print_week(&mut week);                           // pass mutable var "week" by reference
    tanks();
    bank(&mut balance);

    if week > 1 {
        event();
    }
}
