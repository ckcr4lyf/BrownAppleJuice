/// Thanks to the example for BLE advertising at https://github.com/bluez/bluer/blob/cfc9363160f1a66f312592ae96b5a746350e8f02/bluer/examples/le_advertise.rs
/// and the initial python script (and sample messages) from https://github.com/ECTO-1A/AppleJuice/blob/e6a61f6a199075f5bb5b1a00768e317571d25bb9/app.py

use std::{collections::BTreeMap, time::Duration};

use log::{debug, trace, info};
use tokio::{io::{BufReader, AsyncBufReadExt}, time::sleep};

#[tokio::main(flavor = "current_thread")]
async fn main() -> bluer::Result<()> {
    env_logger::init();

    // Try and create new session, get default adapter
    // and power it on
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let mut manufacturer_data: BTreeMap<u16, Vec<u8>> = BTreeMap::new();
    
    // Airpods
    // TODO: Add more!
    manufacturer_data.insert(0x004C, vec![0x07, 0x19, 0x07, 0x02, 0x20, 0x75, 0xaa, 0x30, 0x01, 0x00, 0x00, 0x45, 0x12, 0x12, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

    debug!("Advertising on Bluetooth adapter {} with address {}", adapter.name(), adapter.address().await?);
    let le_advertisement = bluer::adv::Advertisement {
        manufacturer_data: manufacturer_data,
        ..Default::default()
    };

    trace!("Prepared advertisement: {:?}", &le_advertisement);
    let handle = adapter.advertise(le_advertisement).await?;
    
    // cleanup
    info!("Advertising! Press enter to stop...");
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    let _ = lines.next_line().await;

    info!("Removing advertisement");
    drop(handle);
    sleep(Duration::from_secs(1)).await;

    Ok(())
}
