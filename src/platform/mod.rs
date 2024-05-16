use net_route::Handle;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::add_rules;
#[cfg(target_os = "linux")]
use linux::build_routes;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub use macos::build_routes;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::build_routes;

/// Options for adding routes and rules(linux only)
#[derive(Clone, Debug)]
pub struct Opt {
    /// if specified, will only add routes in the preset
    pub preset: Vec<(IpAddr, u8)>,
    /// table id that the rule will by added
    #[cfg(target_os = "linux")]
    pub table: u32,
    #[cfg(target_os = "windows")]
    // the tun device's luid
    pub luid: Option<u64>,
    pub if_index: u32,
    /// the tun device's ip address that will be used as the gateway
    pub gateway_ipv4: Option<Ipv4Addr>,
    pub gateway_ipv6: Option<Ipv6Addr>,
}

pub async fn add_route(opt: &Opt) -> std::io::Result<()> {
    let handle = Handle::new()?;
    let routes = if opt.preset.is_empty() {
        build_routes(opt)
    } else {
        opt.preset
            .iter()
            .cloned()
            .map(|(dst, prefix)| net_route::Route::new(dst, prefix))
            .collect::<Vec<_>>()
    };

    for route in routes {
        tracing::trace!("adding route:{:?},", route);
        const MAX_RETRY: usize = 3;
        for _ in 0..MAX_RETRY {
            if let Err(e) = handle.add(&route).await {
                tracing::warn!("Failed to add route: {:?}, err: {:?}", route, e);
                if e.kind() == std::io::ErrorKind::AlreadyExists {
                    // ignore the error in delete
                    let _ = handle.delete(&route).await;
                }
            } else {
                tracing::debug!("add route success: {:?},", route);
                break;
            }
        }
    }

    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
#[cfg(test)]
#[allow(unused)]
mod tests {

    use super::build_routes;
}
