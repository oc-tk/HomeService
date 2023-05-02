use std::error::Error;
use warp::{Filter, Rejection, Reply};
use wake_on_lan;

#[derive(Debug)]
enum ApiError {
    ComputerNotFound,
    ErrorWhileSendingPacket,
}

type Computers = Vec<Computer>;
impl warp::reject::Reject for ApiError {}

#[derive(Debug, Clone)]
struct Computer {
    id: i32,
    mac_address: String,
}

async fn get_computer_by_id(id: i32, computers: Computers) -> Result<impl Reply, Rejection> {
    if computers.iter().any(|c| c.id == id) {
        match computers.into_iter().nth(id as usize) {
            Some(comp) => {
                match wake_computer(comp.mac_address.clone()) {
                    Ok(_) => {
                        println!("Packet sent successfully.");
                        Ok(warp::reply::json(&true))
                    },
                    Err(error) => {
                        println!("Error sending packet: {}", error);
                        Err(warp::reject::custom(ApiError::ErrorWhileSendingPacket))
                    },
                }
            }
            None => Err(warp::reject::custom(ApiError::ComputerNotFound))
        }

        
    } else {
        Err(warp::reject::custom(ApiError::ComputerNotFound))
    }
}

fn wake_computer(mac_address: String) -> Result<bool, Box<dyn Error>> {
    let converted_mac_address: [u8; 6] = match mac_address.as_bytes().try_into() {
        Ok(array) => array,
        Err(error) => return Err(format!("Error during converting into byte array, {}", error).into()),
    };

    let magic_packet = wake_on_lan::MagicPacket::new(&converted_mac_address);
    match magic_packet.send() {
        Ok(()) => Ok(true),
        Err(error) => return Err(format!("Error sending packet, {}", error).into()),
    }
}

fn get_computers() -> Computers{
    let mut comps: Computers = Vec::new();

    let comp1 = Computer {
        id: 1,
        mac_address: String::from("00:00:00:00:00:00"),
    };

    let comp2 = Computer {
        id: 2,
        mac_address: String::from("00:00:00:00:00:00"),
    };

    comps.push(comp1);
    comps.push(comp2);

    comps
}

#[tokio::main]
async fn main() {
    let computers: Computers = get_computers();
    let computers_filter = warp::any().map(move || computers.clone());

    let wake_computer_by_id = warp::path!("wol_computer" / i32)
        .and(warp::get())
        .and(computers_filter.clone())
        .and_then(get_computer_by_id);


    let routes = wake_computer_by_id;

    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;
}