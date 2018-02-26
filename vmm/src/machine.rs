/// Machine configuration specifics

use std::net::Ipv4Addr;
use std::path::PathBuf;

pub struct MachineCfg {
    pub vcpu_count: u8,         //has default value
    pub mem_size: usize,        //has default value
    pub root_blk_file: Option<PathBuf>,
    pub host_ip: Option<Ipv4Addr>,
    pub subnet_mask: Ipv4Addr, //has default value
    pub vsock_guest_cid: Option<u64>,
}

impl MachineCfg {
    pub fn new(
        vcpu_count: u8,
        mem_size: usize,
        root_blk_file: Option<PathBuf>,
        host_ip: Option<Ipv4Addr>,
        subnet_mask: Ipv4Addr,
        vsock_guest_cid: Option<u64>,
    ) -> Self {
        MachineCfg {
            vcpu_count,
            mem_size,
            root_blk_file,
            host_ip,
            subnet_mask,
            vsock_guest_cid,
        }
    }
}
