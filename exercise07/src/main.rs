use std::io;

struct Artist_Event {
    event_id : u32 ,
    event_title : String ,
    artist : String ,
    datetime : String ,
    ticket_price : f64 ,
    stock : u32
}
struct Customer {
    name : String ,
    tickets_bought : Vec <String> ,
    total_cost : f64
}



// prints the menu
fn print_menu(){
    println!("==================== MENU ====================");
    println!("[1] Add Event Details");
    println!("[2] Buy Ticket");
    println!("[3] Edit Event Details");
    println!("[4] Delete Event");
    println!("[5] View All Events");
    println!("[6] View All Customers");
    println!("[7] Exit");
    println!("");
    println!("Enter choice:");
}

// check if id is in array
fn checkid_in_eventarray(id:&u32, events:&Vec<Artist_Event>)->bool{
    let mut isinarray:bool = false;
    for i in events {
        if i.event_id == *id{
            isinarray = true;
        }
    }
    isinarray
}

fn main() {
    let events: Vec<Artist_Event> = Vec::new();
    let mut run:bool = true;
    while run{
        let mut choiceinput = String::new();
        print_menu();
        io::stdin().read_line(& mut choiceinput).expect("Error");
        let choiceinput:isize = choiceinput.trim().parse().expect("Error");
        if choiceinput == 7 {
            run = false
        }else if choiceinput == 1 {
            println!("");
            println!("Enter event id:");
            let mut eventidinput = String::new();
            io::stdin().read_line(& mut eventidinput).expect("Error");
            let eventidinput:u32 = eventidinput.trim().parse().expect("Error");
            let isineventsalready:bool = checkid_in_eventarray(&eventidinput, &events);
            if  isineventsalready == true{
                println!("Event ID already exists!")
            }else{
                println!("lezgo")
            }

        }else if choiceinput == 2 {

        }else if choiceinput == 3 {

        }else if choiceinput == 4 {

        }else if choiceinput == 5 {
            
        }else if choiceinput == 6 {
            
        }else{
            println!("Invalid Input!");
        }
        println!("");
    }
    
}
