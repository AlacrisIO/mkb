use std::net::{IpAddr, Ipv4Addr};
/*
pub fn get_ip_plus_port(ip_addr: &Vec<u8>, port: u16) -> String {
    let str0 : String = ip_addr[0].to_string();
    let str1 : String = ip_addr[1].to_string();
    let str2 : String = ip_addr[2].to_string();
    let str3 : String = ip_addr[3].to_string();
    let str4 : String = port.to_string();
    let ip_plus_port : String = str0 + "." + &str1 + "." + &str2 + "." + &str3 + ":" + &str4;
    ip_plus_port
}
*/


pub fn retrieve_v4_addr(ip_addr: Vec<u8>) -> IpAddr {
    IpAddr::V4(Ipv4Addr::new(ip_addr[0], ip_addr[1], ip_addr[2], ip_addr[3]))
}
