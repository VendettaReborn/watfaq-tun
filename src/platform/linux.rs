use std::net::{Ipv4Addr, Ipv6Addr};

use net_route::{Handle, Route};

use super::Opt;

fn build_rules(table: u32, ipv4: bool, ipv6: bool) -> Vec<net_route::Rule> {
    let ipv4_rules = if ipv4 {
        vec![
            // will lookup the table main fristly,
            // and ignore the default route(with prefix of 0) in it
            net_route::Rule {
                table_id: Some(254),
                priority: Some(7000),
                suppress_prefixlength: Some(0),
                ..Default::default()
            },
            // will lookup the table, in which the default gateway shall be set
            net_route::Rule {
                table_id: Some(table),
                priority: Some(7001),
                ..Default::default()
            },
        ]
    } else {
        vec![]
    };

    let ipv6_rules = if ipv6 {
        vec![
            net_route::Rule {
                table_id: Some(254),
                priority: Some(7000),
                suppress_prefixlength: Some(0),
                v6: true,
                ..Default::default()
            },
            net_route::Rule {
                table_id: Some(table),
                priority: Some(7001),
                v6: true,
                ..Default::default()
            },
        ]
    } else {
        vec![]
    };

    [ipv4_rules, ipv6_rules].concat()
}

pub async fn add_rules(
    table: u32,
    ipv4: bool,
    ipv6: bool,
    clear_before_add: bool,
) -> std::io::Result<()> {
    let rules = build_rules(table, ipv4, ipv6);
    let handle = Handle::new().unwrap();
    if clear_before_add {
        let _ = handle.delete_rules(rules.clone()).await;
    }
    handle.add_rules(rules).await
}

pub(crate) fn build_routes(opt: &Opt) -> Vec<Route> {
    let mut routes = Vec::with_capacity(2);

    if opt.gateway_ipv4.is_some() {
        routes.push(
            Route::new(Ipv4Addr::UNSPECIFIED.into(), 0)
                .with_ifindex(opt.if_index)
                .with_table(opt.table),
        );
    }
    if opt.gateway_ipv6.is_some() {
        routes.push(
            Route::new(Ipv6Addr::UNSPECIFIED.into(), 0)
                .with_ifindex(opt.if_index)
                .with_table(opt.table),
        );
    }

    routes
}
