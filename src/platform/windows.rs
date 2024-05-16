use super::Opt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub fn build_routes(gateway: IpAddr, opt: &Opt) -> Vec<Route> {
    let mut routes = Vec::with_capacity(2);
    if opt.gateway_ipv4.is_some() {
        let mut r1 = Route::new(Ipv4Addr::UNSPECIFIED.into(), 1)
            .with_gateway(gateway)
            .with_metric(0);
        if let Some(luid) = opt.luid {
            r1 = r1.with_luid(luid);
        }
        if let Some(if_index) = opt.if_index {
            r1 = r1.with_ifindex(if_index);
        }
        routes.push(r1);
    }

    if opt.gateway_ipv6.is_some() {
        let mut r2 = Route::new(Ipv6Addr::UNSPECIFIED.into(), 1)
            .with_gateway(gateway)
            .with_metric(0);
        if let Some(luid) = opt.luid {
            r2 = r2.with_luid(luid);
        }
        if let Some(if_index) = opt.if_index {
            r2 = r2.with_ifindex(if_index);
        }
        routes.push(r2);
    }

    routes
}
