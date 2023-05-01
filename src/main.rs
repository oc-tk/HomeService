use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply};


#[derive(Debug, Serialize, Deserialize, Clone)]
struct Computer {
    id: u32,
    mac_address: String,
}

type Computers = Vec<Computer>;

#[derive(Debug)]
enum ApiError {
    UserNotFound,
}

impl warp::reject::Reject for ApiError {}

async fn get_computer_by_id(id: u32, computers: Computers) -> Result<impl Reply, Rejection> {
    if computers.iter().any(|c| c.id == id) {
        match computers.into_iter().nth(id as usize) {
            Some(comp) => {
                Ok(warp::reply::json(&comp.mac_address))
            }
            None => {
                Err(warp::reject::custom(ApiError::UserNotFound))
            }
        }

        
    } else {
        Err(warp::reject::custom(ApiError::UserNotFound))
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

    let wake_computer_by_id = warp::path!("computer" / u32)
        .and(warp::get())
        .and(computers_filter.clone())
        .and_then(get_computer_by_id);


    let routes = wake_computer_by_id;

    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;
}