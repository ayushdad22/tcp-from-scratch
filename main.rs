extern crate libc;

use std::ptr;
use std::mem;
use std::net::Ipv4Addr;

#[repr(C, packed)]
struct IpHeader {
    ver_ihl: u8, // version 4
    dscp_ecn: u8, 
    total_length: u16,
    identification: u16,
    flags_fragment: u16,
    ttl: u8,
    protocol: u8,
    checksum: u16,
    src_addr: u32,
    dest_addr: u32
}

impl IpHeader {
    fn new(src: u32, dest: u32, payload_len: u16, protocol: u8) -> Self {
        let mut hdr = IpHeader {
            ver_ihl: (4 << 4) | 5, // IPv4 + 5*4=20 byte header
            dscp_ecn: 0,
            total_length: (20 + payload_len).to_be(),
            identification: 0,
            flags_fragment: 0,
            ttl: 64,
            protocol,
            checksum: 0,
            src_addr: src.to_be(),
            dest_addr: dest.to_be(), //converts to big edian
        };
        hdr.checksum = ip_checksum(&hdr);
        hdr
    }
}

// Calculate checksum
fn ip_checksum(header: &IpHeader) -> u16 {
    let ptr = header as *const _ as *const u16;
    let mut sum = 0u32;
    for i in 0..10 {
        sum += unsafe { *ptr.add(i) } as u32;
    }
    while (sum >> 16) != 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    }
    !(sum as u16)
}

fn ip_to_u32(ip: &str) -> u32 {
    Ipv4Addr::from(ip.parse::<Ipv4Addr>().unwrap()).into()
}

fn main() {
    unsafe {
        let sock = libc::socket(libc::AF_INET, libc::SOCK_RAW, libc::IPPROTO_RAW);
        if sock < 0 {
            eprintln!("Failed to create raw socket");
            return;
        }

        let src_ip = ip_to_u32("192.168.0.2");
        let dst_ip = ip_to_u32("192.168.0.1");

        let ip_header = IpHeader::new(src_ip, dst_ip, 0, 6); // TCP = 6
        let header_ptr = &ip_header as *const _ as *const libc::c_void;

        // Destination socket address
        let sockaddr = libc::sockaddr_in {
            sin_family: libc::AF_INET as u16,
            sin_port: 0,
            sin_addr: libc::in_addr { s_addr: dst_ip },
            sin_zero: [0; 8],
        };

        let ret = libc::sendto(
            sock,
            header_ptr,
            std::mem::size_of::<IpHeader>(),
            0,
            &sockaddr as *const _ as *const libc::sockaddr,
            mem::size_of::<libc::sockaddr_in>() as u32,
        );

        if ret < 0 {
            eprintln!("Failed to send packets");
        } else {
            println!("Sent {} bytes", ret);
        }
    }
}
