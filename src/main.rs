extern crate lifx_api_server;
use std::env;

fn main() {

 
    let secret_key = env::var("SECRET_KEY").expect("$SECRET_KEY is not set");

    let config = lifx_api_server::Config { 
        secret_key: secret_key.to_string(),
        port: 8000
    };

    lifx_api_server::start(config);

    // Now you can use curl to access the api
    // curl -X PUT "http://localhost:8089/v1/lights/all/state"      -H "Authorization: Bearer xxx"      -d "color=kelvin:9000"
    // or rust


    // extern crate lifx_rs as lifx;

    // fn main() {
    
    //     let key = "xxx".to_string();
    //     let mut api_endpoints: Vec<String> = Vec::new();
    
    //     api_endpoints.push(format!("http://localhost:8089"));
    
    //     let config = lifx::LifxConfig{
    //         access_token: key.clone(),
    //         api_endpoints: api_endpoints
    //     };
        
    //     let mut off_state = lifx::State::new();
    //     off_state.power = Some(format!("off"));
    
    //     // Turn off all lights
    //     lifx::Light::set_state_by_selector(config.clone(), format!("all"), off_state);
    
    // }



    println!("sync");

    loop {
        
    }
}