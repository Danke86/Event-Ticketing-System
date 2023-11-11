use std::{io, fmt::format};

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

//check if customer name is in the array
fn checkname_in_customerarray(name:&String, customers:&Vec<Customer>)->bool{
    let mut isinarray:bool = false;
    for i in customers {
        if i.name == *name{
            isinarray = true;
        }
    }
    isinarray
}

//return the customer with that name

//buys a ticket for the customer
fn buy_ticket(name:&String, eventidtobuy:&u32, events:&mut Vec<Artist_Event>, customers:&mut Vec<Customer>){
    for i in events.iter_mut() {
        if i.event_id == *eventidtobuy{
            //check if stock > 0
            if i.stock > 0{
                //check if customer exists
                let eventticketname:String = format!("{}_{}_{}",i.event_id,i.event_title,i.datetime);
                if checkname_in_customerarray(name, customers) {
                    //if yes, just append the new ticket and update total cost
                    for j in customers.iter_mut(){
                        if j.name == *name{
                            j.tickets_bought.push(eventticketname.clone())
                        }
                    }
                }else{
                    //if not, append a new customer and append the new ticket
                }
                // decrement stock
                i.stock -=1;
                println!("Successfully bought!");
            }else{
                println!("No more stock left!");
            }
        }
    }
}

fn main() {
    let mut events: Vec<Artist_Event> = Vec::new();
    let mut customers: Vec<Customer> = Vec::new();
    let mut run:bool = true;
    while run{
        let mut choiceinput = String::new();
        print_menu();
        io::stdin().read_line(& mut choiceinput).expect("Error");
        let choiceinput:isize = choiceinput.trim().parse().expect("Error");
        if choiceinput == 7 {
            run = false
        }else if choiceinput == 1 { //Add event
            println!("");
            println!("Enter event id:");
            let mut eventidinput = String::new();
            io::stdin().read_line(& mut eventidinput).expect("Error");
            let eventidinput:u32 = eventidinput.trim().parse().expect("Error");
            let isineventsalready:bool = checkid_in_eventarray(&eventidinput, &events);
            if  isineventsalready == true{
                println!("Event ID already exists!");
            }else{
                // If it doesnt exist, we can add to the array

                // get user inputs for event
                println!("Enter event title:");
                let mut eventtitleinput = String::new();
                io::stdin().read_line(& mut eventtitleinput).expect("Error");
                eventtitleinput = eventtitleinput.trim().parse().expect("Error");
                println!("Enter artist:");
                let mut artistinput = String::new();
                io::stdin().read_line(& mut artistinput).expect("Error");
                artistinput = artistinput.trim().parse().expect("Error");
                println!("Enter date and time:");
                let mut datetimeinput = String::new();
                io::stdin().read_line(& mut datetimeinput).expect("Error");
                datetimeinput = datetimeinput.trim().parse().expect("Error");
                println!("Enter ticket price:");
                let mut ticketpriceinput = String::new();
                io::stdin().read_line(& mut ticketpriceinput).expect("Error");
                let ticketpriceinput:f64 = ticketpriceinput.trim().parse().expect("Error");
                let mut stockinput = String::new();
                println!("Enter stock:");
                io::stdin().read_line(& mut stockinput).expect("Error");
                let stockinput:u32 = stockinput.trim().parse().expect("Error");

                // instantiate new artist event
                let newevent:Artist_Event = Artist_Event { event_id: eventidinput, event_title: eventtitleinput, artist: artistinput, datetime: datetimeinput, ticket_price: ticketpriceinput, stock: stockinput};
                //push to the array
                events.push(newevent);
                
                println!("Succesfully Added Event ");
            }
        }else if choiceinput == 2 { //Customer Buys Ticket
            println!("");
            if events.len() == 0{ //check if there are no events avaialble
                println!("There are no events available!"); 
            }else{
                // if there events, proceed to let the customer buy a ticket
                println!("Enter customer name:");
                let mut customernameinput = String::new();
                io::stdin().read_line(& mut customernameinput).expect("Error");

                //print all events available
                println!("");
                println!("-------- EVENTS AVAILABLE --------");
                for event in &events{
                    println!("[{}] {} ({}) - {}", event.event_id, event.event_title, event.artist ,event.ticket_price);
                }

                //ask what ticket to buy
                println!("");
                println!("Enter event id to buy :");

                let mut eventidtobuyinput = String::new();
                io::stdin().read_line(& mut eventidtobuyinput).expect("Error");
                let eventidtobuyinput:u32 = eventidtobuyinput.trim().parse().expect("Error");

                //check if input is in events 
                if checkid_in_eventarray(&eventidtobuyinput, &events){
                    buy_ticket(&customernameinput, &eventidtobuyinput, &mut events, &mut customers)
                    // let mut eventtobuy = get_event_by_id(&eventidtobuyinput, &events);
                    // //check if stock > 0
                    // if events[eventidtobuyinput] > 0{
                    //     //check if customer exists
                        
                    //     //if yes, just append the new ticket and update total cost
                        
                    //     //if not, append a new customer and append the new ticket
                    // }else{
                    //     println!("No more stock left!");
                    // }
                }else {
                    println!("Event ID not found!");
                }
            }
        }else if choiceinput == 3 { //Edit Event Details

        }else if choiceinput == 4 { //Delete Event

        }else if choiceinput == 5 { //View All Events
            
        }else if choiceinput == 6 { //View All Customers
            println!("");
            if customers.len() == 0{ //check if there are no customers
                println!("There are no customers yet!"); 
            }else{
                
            }
        }else{
            println!("Invalid Input!");
        }
        println!("");
    }
    
}
