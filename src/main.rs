#![warn(clippy::all, clippy::pedantic)]



//##################
//#   STRUCTURES   #
//##################



#[derive(Debug)]
enum OrderStatus {
    Paid { paid_date: String },
    Sent { paid_date: String, sent_date: String },
    Delivered { paid_date: String, sent_date: String, deliver_date: String },
    Disputed { paid_date: String, sent_date: String, deliver_date: String, reason: String }
} impl OrderStatus {
    fn info(&self) {
        match self {
            Self::Paid { paid_date } => { 
                println!("{}", paid_date) 
            },
            Self::Sent { paid_date, sent_date } => { 
                println!("{}\n{}", paid_date, sent_date) 
            },
            Self::Delivered { paid_date, sent_date, deliver_date } => { 
                println!("{}\n{}\n{}", paid_date, sent_date, deliver_date) 
            },
            Self::Disputed { paid_date, sent_date, deliver_date, reason } => { 
                println!("{}\n{}\n{}\n{}", paid_date, sent_date, deliver_date, reason) 
            }
        }
    }
}

struct Order {
    customer: String,
    status: OrderStatus
}



//############
//#   MAIN   #
//############



fn main() {
    let mut order = Order {
        customer: "David".to_string(),
        status: OrderStatus::Paid{ 
            paid_date: "12.12.12".to_string()
        }
    };
}



//#################
//#   FUNCTIONS   #
//#################







//#############
//#   TESTS   #
//#############



#[cfg(test)]
mod tests {
    //use super::*;



    #[test]
    fn test_1() {

    }
}