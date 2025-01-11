use bevy::color::Color;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub mod drag_and_drop;
pub mod preview_plugins;

pub const COLOR_BUTTON: Color = Color::srgb(1.0, 0.5, 0.0);
pub const BORDER_COLOR_INACTIVE: Color = Color::srgb(0.25, 0.25, 0.25);
pub const BORDER_COLOR_ACTIVE: Color = Color::srgb(0.75, 0.52, 0.99);
pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub const BACKGROUND_COLOR: Color = Color::WHITE;

pub fn get_socket(server: String, port: String) -> SocketAddr {
    let split: Vec<u8> = server
        .split(".")
        .map(|s| s.parse::<u8>().unwrap())
        .collect();
    let port_num = port.parse::<u16>().unwrap();
    SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(split[0], split[1], split[2], split[3])),
        port_num,
    )
}
