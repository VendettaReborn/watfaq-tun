use net_route::Route;
use std::net::IpAddr;

use super::Opt;

pub fn build_routes(opt: &Opt) -> Vec<Route> {
    let routes_ipv4 = if let Some(gateway_ipv4) = opt.gateway_ipv4 {
        let gateway = IpAddr::V4(gateway_ipv4);
        [
            Route::new("1.0.0.0".parse().unwrap(), 8).with_gateway(gateway),
            Route::new("2.0.0.0".parse().unwrap(), 7).with_gateway(gateway),
            Route::new("4.0.0.0".parse().unwrap(), 6).with_gateway(gateway),
            Route::new("8.0.0.0".parse().unwrap(), 5).with_gateway(gateway),
            Route::new("16.0.0.0".parse().unwrap(), 4).with_gateway(gateway),
            Route::new("32.0.0.0".parse().unwrap(), 3).with_gateway(gateway),
            Route::new("64.0.0.0".parse().unwrap(), 2).with_gateway(gateway),
            Route::new("128.0.0.0".parse().unwrap(), 1).with_gateway(gateway),
        ]
        .into()
    } else {
        vec![]
    };

    let routes_ipv6 = if let Some(gateway_ipv6) = opt.gateway_ipv6 {
        let gateway = IpAddr::V6(gateway_ipv6);
        [
            Route::new("1::".parse().unwrap(), 8).with_gateway(gateway),
            Route::new("2::".parse().unwrap(), 7).with_gateway(gateway),
            Route::new("2::".parse().unwrap(), 6).with_gateway(gateway),
            Route::new("8::".parse().unwrap(), 5).with_gateway(gateway),
            Route::new("16::".parse().unwrap(), 4).with_gateway(gateway),
            Route::new("32::".parse().unwrap(), 3).with_gateway(gateway),
            Route::new("64::".parse().unwrap(), 2).with_gateway(gateway),
            Route::new("128::".parse().unwrap(), 1).with_gateway(gateway),
        ]
        .into()
    } else {
        vec![]
    };

    [routes_ipv4, routes_ipv6].concat()
}

#[test]
fn test_parse() {
    let addr = "1::".parse::<std::net::Ipv6Addr>();
    assert!(addr.is_ok());
}

#[test]
fn test_build_routes() {
    let opt = Opt {
        preset: vec![],
        if_index: 1,
        gateway_ipv4: Some("192.17.0.1".parse().unwrap()),
        gateway_ipv6: Some("2001:db8::1".parse().unwrap()),
    };
    let routes = build_routes(&opt);
    assert_eq!(routes.len(), 16);
}
