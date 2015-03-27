#![feature(test)]

extern crate sys_info;
extern crate test;

use sys_info::*;
use test::Bencher;

fn main() {

    println!("os: {} {}", os_type().unwrap(), os_release().unwrap());
    println!("cpu: {} cores, {} MHz", cpu_num().unwrap(), cpu_speed().unwrap());
    println!("proc total: {}", proc_total().unwrap());
    let load = loadavg().unwrap();
    println!("load: {} {} {}", load.one, load.five, load.fifteen);
    let mem = mem_info().unwrap();
    println!("mem: total {} KB, free {} KB, avail {} KB, buffers {} KB, cached {} KB",
             mem.total, mem.free, mem.avail, mem.buffers, mem.cached);
    println!("swap: total {} KB, free {} KB", mem.swap_total, mem.swap_free);
    let disk = disk_info().unwrap();
    println!("disk: total {} KB, free {} KB", disk.total, disk.free);
    println!("hostname: {}", hostname().unwrap());

}

#[bench]
fn bench_test(b: &mut Bencher) {
    b.iter(|| {
        let _ = loadavg();
        let _ = mem_info();
        let _ = disk_info();
    });
}
    

