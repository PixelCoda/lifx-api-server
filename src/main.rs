extern crate lifx_api_server;

fn main() {

    let config = lifx_api_server::Config { 
        secret_key: format!("xxx"),
        port: 8089
    };

    lifx_api_server::start(config);

    // Now you can use curl to access the api
    // curl -X PUT "http://localhost:8089/v1/lights/all/state"      -H "Authorization: Bearer xxx"      -d "color=kelvin:9000"
    // or rust


    println!("sync");

    loop {
        
    }
}