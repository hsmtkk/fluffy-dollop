use pnet::datalink::NetworkInterface;
use serde::{Serialize, Deserialize};

fn main() {
    let intfs = pnet::datalink::interfaces();
    for intf in intfs {
        list_interface(intf);
    }
}

#[derive(Serialize, Deserialize)]
struct NetworkInterfaceJson {
    name: String,
    description: String,
    index: u32,
    mac: String,
    ips: Vec<String>,
}

impl NetworkInterfaceJson {
    fn new(name:String, description:String, index:u32, mac:String, ips:Vec<String>) -> NetworkInterfaceJson {
        NetworkInterfaceJson{
            name,
            description,
            index,
            mac,
            ips,
        }
    }
}

fn list_interface(intf:NetworkInterface){
    let mac: String = match intf.mac {
        Some(mac) => {mac.to_string()},
        None => "".to_string(),
    };
    let mut ips: Vec<String> = Vec::new();
    for ip in intf.ips {
        ips.push(ip.to_string());
    }
    let nfj = NetworkInterfaceJson::new(intf.name, intf.description, intf.index, mac, ips);
    let yml = serde_yaml::to_string(&nfj).expect("yml encode");
    println!("{}", yml);   
}