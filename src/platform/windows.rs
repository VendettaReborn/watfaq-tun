use super::Opt;
use net_route::Route;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub fn build_routes(opt: &Opt) -> Vec<Route> {
    let mut routes = Vec::with_capacity(2);
    if let Some(gateway_ipv4) = opt.gateway_ipv4 {
        let mut r1 = Route::new(Ipv4Addr::UNSPECIFIED.into(), 1)
            .with_gateway(IpAddr::V4(gateway_ipv4))
            .with_metric(0);
        if let Some(luid) = opt.luid {
            r1 = r1.with_luid(luid);
        }
        r1 = r1.with_ifindex(opt.if_index);
        routes.push(r1);
    }

    if let Some(gateway_ipv6) = opt.gateway_ipv6 {
        let mut r2 = Route::new(Ipv6Addr::UNSPECIFIED.into(), 1)
            .with_gateway(IpAddr::V6(gateway_ipv6))
            .with_metric(0);
        if let Some(luid) = opt.luid {
            r2 = r2.with_luid(luid);
        }
        r2 = r2.with_ifindex(opt.if_index);
        routes.push(r2);
    }

    routes
}
