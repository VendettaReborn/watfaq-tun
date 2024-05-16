use std::net::{IpAddr, Ipv4Addr};
use net_route::Route;

use super::Opt;

pub fn build_routes(gateway: IpAddr, opt: &Opt) -> Vec<Route> {
    return [
        Route::new("1.0.0.0".parse().unwrap(), 8).with_gateway(gateway),
        Route::new("2.0.0.0".parse().unwrap(), 7).with_gateway(gateway),
        Route::new("4.0.0.0".parse().unwrap(), 6).with_gateway(gateway),
        Route::new("8.0.0.0".parse().unwrap(), 5).with_gateway(gateway),
        Route::new("16.0.0.0".parse().unwrap(), 4).with_gateway(gateway),
        Route::new("32.0.0.0".parse().unwrap(), 3).with_gateway(gateway),
        Route::new("64.0.0.0".parse().unwrap(), 2).with_gateway(gateway),
        Route::new("128.0.0.0".parse().unwrap(), 1).with_gateway(gateway),
    ]
    .into();
}
