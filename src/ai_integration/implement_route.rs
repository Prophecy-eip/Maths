//! Implement Route module
//!
//! This module contains the integration of the flask poc to link the IA and the dynamic library.
//!
use crate::web_server;
use reqwest::*;
// use serde_json::Value;
// use std::fs::File;
// use std::io::Read;

/// Take the two values given by the IA and round the closest value to its ceiling to and truncate the other to make
/// an even 20 every time in the total score.
///
/// # Parameters
/// val1 (f64): The first value given by the ai
/// val2 (f64): The second value given by the ai
///
/// # Return
/// (u8,u8) : a tuple of the modified value to send it back to the application
fn adjust_values(val1: f64, val2: f64) -> (u8, u8) {
    let diff1 = val1.ceil() - val1;
    let diff2 = val2.ceil() - val2;
    let target_sum: u8 = 20;

    if diff1 < diff2 {
        let adjusted_val1 = val1.ceil() as u8;
        let adjusted_val2 = (target_sum as f64 - val1.ceil()) as u8;
        (adjusted_val1, adjusted_val2)
    } else {
        let adjusted_val1 = (target_sum as f64 - val2.ceil()) as u8;
        let adjusted_val2 = val2.ceil() as u8;
        (adjusted_val1, adjusted_val2)
    }
}

/// The bridge between the request in the front and the call to the ai in the back, requested through a flask server
///
/// # Parameters
/// req (web_server::ProphecyRequestArmies): Request ...
///
/// # Return
/// Result<(u8,u8)> : a tuple of the formatted values, that will be send to the application
pub async fn handle_flask_request(req: web_server::ProphecyRequestArmies) -> Result<(u8, u8)> {
    // let mut host: String = String::from("0.0.0.0");
    // let mut port = 4242;

    // match File::open("../../ai/flask_config.json") {
    //     Ok(mut file) => {
    //         let mut contents = String::new();
    //         match file.read_to_string(&mut contents) {
    //             Ok(_) => match serde_json::from_str::<Value>(&contents) {
    //                 Ok(config) => {
    //                     host = config["client_rust"]
    //                         .as_str()
    //                         .unwrap_or("0.0.0.0")
    //                         .to_string();
    //                     port = config["port"].as_u64().unwrap_or(4242) as u16;
    //                 }
    //                 Err(e) => eprintln!("Error: Unable to parse the config file. {:?}", e),
    //             },
    //             Err(e) => eprintln!("Error: Unable to read the config file. {:?}", e),
    //         }
    //     }
    //     Err(e) => eprintln!("Error: Unable to open the config file. {:?}", e),
    // }

    let url = "http://dev.prophecy-eip.com/ai".to_string();
    let data = serde_json::to_value(&req).expect("Failed to serialize request to JSON.");
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&data)
        .send()
        .await
        .expect("Failed to send request.");

    if res.status().is_success() {
        let result: serde_json::Value = res.json().await?;
        println!("Original values: {:#}", result);
        if let Some(array) = result.as_array() {
            if array.len() == 2 {
                let val1 = array[0].as_f64().unwrap();
                let val2 = array[1].as_f64().unwrap();
                let (adjusted_val1, adjusted_val2) = adjust_values(val1, val2);
                println!("Adjusted values: {}, {}", adjusted_val1, adjusted_val2);
                return Ok((adjusted_val1, adjusted_val2));
            } else {
                eprintln!("Unexpected number of values in response array");
            }
        } else {
            eprintln!("Response is not an array");
        }
    } else {
        eprintln!("Failed to get a successful response: {}", res.status());
    }
    todo!("Return Error");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web_server::*;

    #[tokio::test]
    async fn test_call_request_flask() {
        // Create some mockup ProphecyUnit objects
        let unit1 = ProphecyUnit::new("Black Cloaks".to_string(), vec!["Great Weapon".to_string()]);
        let unit2 = ProphecyUnit::new("Black Cloaks".to_string(), vec!["Shield".to_string()]);

        // Create mockup ProphecyRequestArmies object
        let request_armies = ProphecyRequestArmies::new(
            "some-key".to_string(),
            vec![
                unit1.clone(),
                unit1.clone(),
                unit1.clone(),
                unit1.clone(),
                unit1.clone(),
                unit1.clone(),
                unit1.clone(),
                unit1.clone(),
                unit1.clone(),
                unit1.clone(),
                unit1.clone(),
                unit1.clone(),
                unit1.clone(),
                unit1.clone(),
                unit1.clone(),
                unit1.clone(),
                unit1.clone(),
                unit1.clone(),
                unit1.clone(),
                unit1.clone(),
                unit1.clone(),
                unit1.clone(),
            ],
            vec![unit2.clone()],
        );

        // Call handle_flask_request with the mockup data
        let result = handle_flask_request(request_armies).await.unwrap();
        assert_eq!(result, (11, 9)); // Replace with the expected result
    }
}
