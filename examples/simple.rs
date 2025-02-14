use std::convert::TryInto;

use astarte_sdk::{types::AstarteType, AstarteOptions};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    // Realm name
    #[structopt(short, long)]
    realm: String,
    // Device id
    #[structopt(short, long)]
    device_id: String,
    // Credentials secret
    #[structopt(short, long)]
    credentials_secret: String,
    // Pairing URL
    #[structopt(short, long)]
    pairing_url: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let Cli {
        realm,
        device_id,
        credentials_secret,
        pairing_url,
    } = Cli::from_args();

    let mut sdk_options =
        AstarteOptions::new(&realm, &device_id, &credentials_secret, &pairing_url);

    sdk_options
        .add_interface_files("./examples/interfaces")
        .unwrap();

    sdk_options.build().await.unwrap();

    let mut device = sdk_options.connect().await.unwrap();

    let w = device.clone();
    tokio::task::spawn(async move {
        let mut i: f64 = 0.0;
        loop {
            let data: AstarteType = i.try_into().unwrap();
            w.send("com.test.Everything", "/double", data)
                .await
                .unwrap();
            println!("Sent {}", i);

            i += 1.1;

            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    });

    loop {
        if let Ok(data) = device.poll().await {
            println!("incoming: {:?}", data);

            if let astarte_sdk::Aggregation::Individual(var) = data.data {
                if data.path == "/1/enable" {
                    if var == true {
                        println!("sensor is ON");
                    } else {
                        println!("sensor is OFF");
                    }
                }
            }
        }
    }
}
