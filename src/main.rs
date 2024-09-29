use reqwest::Client ;
use serde::{Deserialize , Serialize};
use std::error::Error;
use std::io::{self , Write};
use std::sync::Arc;
use tokio::sync::Mutex;


#[derive(Serialize, Deserialize, Debug)]
struct Coord{
    lon : f64,
    lat : f64
}

#[derive(Serialize, Deserialize, Debug)]
struct WeatherInfor{
    id : i32,
    main : String,
    description : String,
    icon : String
}

#[derive(Serialize, Deserialize, Debug)]
struct Main{
    temp : f64 ,
    feels_like : f64 , 
    temp_min : f64 ,
    temp_max : f64 ,
    pressure : i32 ,
    humidity: i32 , 
    sea_level : i32 ,
    grnd_level : i32 ,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind{
    speed : f64,
    deg : i32,
    gust : f64
}

#[derive(Serialize, Deserialize, Debug)]
struct Rain{
    #[serde(rename = "1h")]
    onehourse: f64
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds{
    all : i32
}


#[derive(Serialize, Deserialize, Debug)]
struct Sys{
    r#type : i32,
    id : i32,
    country : String,
    sunrise : u64,
    sunset : u64
}


#[derive(Serialize, Deserialize, Debug)]
struct Weather{
    coord :Coord,
    weather : Vec<WeatherInfor>,
    base : String,
    main :  Main,
    visibility : i32,
    wind : Wind,
    rain : Rain ,
    clouds : Clouds,
    dt   : u64,
    sys : Sys,
    timezone : i32,
    id : i32,
    name : String ,
    cod : i32
}


#[tokio::main]
async fn main() -> Result<(),reqwest::Error> {

    //Share data     
    let shared_data = Arc::new(Mutex::new(String::from("")));

    // Thread 1
    let shared_data_clone = Arc::clone(&shared_data);
    let handle1 = tokio::spawn(async move {
        let result = async {
            loop{
                let long = "105.83";
                let lat = "21.02";
                let api_key = APIKEY;
                let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",lat,long,api_key);
                
                let request = reqwest::get(&url).await?;
                
                if request.status().is_success() {
                    let weather_data : Weather = request.json().await?;
                    
                    //cord
                    println!("Longitude: {}", weather_data.coord.lon);
                    println!("Latitude: {}", weather_data.coord.lat);
                    //weather
                    for i in &weather_data.weather {
                        println!("Id: {}", i.id);
                        println!("Main: {}", i.main);
                        println!("Description: {}", i.description);
                        println!("Icon: {}", i.icon);
                    }
                    //base
                    println!("Base: {}", weather_data.base);
                    //main
                    println!("Temp: {}", weather_data.main.temp);
                    println!("Feels like: {}", weather_data.main.feels_like);
                    println!("Temp min: {}", weather_data.main.temp_min);
                    println!("Temp max: {}", weather_data.main.temp_max);
                    println!("Pressure: {}", weather_data.main.pressure);
                    println!("Humidity: {}", weather_data.main.humidity);
                    println!("Sea level: {}", weather_data.main.sea_level);
                    println!("Grnd level: {}", weather_data.main.grnd_level);
                    //wind
                    println!("Speed: {}", weather_data.wind.speed);
                    println!("Deg: {}", weather_data.wind.deg);
                    println!("Gust: {}", weather_data.wind.gust);
                    //dt
                    println!("Dt: {}", weather_data.dt);
                    //rain
                    println!("1h: {}", weather_data.rain.onehourse);
                    //clouds
                    println!("All: {}", weather_data.clouds.all);
                    //sys
                    println!("Type: {}", weather_data.sys.r#type);
                    println!("Id: {}", weather_data.sys.id);
                    println!("Country: {}", weather_data.sys.country);
                    println!("Sunrise: {}", weather_data.sys.sunrise);
                    println!("Sunset: {}", weather_data.sys.sunset);
                    //
                    println!("Timezone: {}", weather_data.timezone);
                    println!("Id: {}", weather_data.id);
                    println!("Name: {}", weather_data.name);
                    println!("Cod: {}", weather_data.cod);
                    // Cập nhật biến chung trong mutex
                    let mut data = shared_data_clone.lock().await;
                    *data = String::from(weather_data.name);
                    println!("Luồng 1 - Giá trị shared_data: {}", *data);
                } else {
                    println!("Yêu cầu không thành công: {}", request.status());
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }   
            Ok::<(), reqwest::Error>(()) 
        };
        if let Err(e) = result.await {
            eprintln!("Error in handle1: {:?}", e);
        }     
    });


    //Thread 2
    let shared_data_clone = Arc::clone(&shared_data);
    let handle2 = tokio::spawn(async move {
        loop {
            let mut data = shared_data_clone.lock().await;
            println!("Luồng 2 - Giá trị shared_data: {}", *data);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });
    
    //Thread 3  
    let shared_data_clone = Arc::clone(&shared_data);
    let handle3 = tokio::spawn(async move {
        loop {
            let mut data = shared_data_clone.lock().await;
            println!("Luồng 3 - Giá trị shared_data: {}", *data);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });


    //Handle Error
    if let Err(e) = handle1.await {
        eprintln!("Lỗi trong luồng 1: {:?}", e);
    }

    if let Err(e) = handle2.await {
        eprintln!("Lỗi trong luồng 2: {:?}", e);
    }

    if let Err(e) = handle3.await {
        eprintln!("Lỗi trong luồng 3: {:?}", e);
    }

    Ok(())
}



// #[derive(Serialize, Deserialize, Debug)]
// #[serde(untagged)] // Cho phép JSON không có nhãn
// enum WeatherInfo {
//     Regular {
//         id: u32,
//         main: String,
//         description: String,
//         icon: String,
//     },
//     Status {
//         status: bool,
//     },
// }

// for weather_info in &weather_data.weather {
//     match weather_info {
//         WeatherInfo::Regular { id, main, description, icon } => {
//             println!("Weather ID: {}", id);
//             println!("Main: {}", main);
//             println!("Description: {}", description);
//             println!("Icon: {}", icon);
//         }
//         WeatherInfo::Status { status } => {
//             println!("Status: {}", status);
//         }
//     }
// }
