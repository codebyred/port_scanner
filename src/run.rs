use std::{io::{self, Write}, net::{IpAddr, TcpStream}, sync::mpsc::{self, Sender}, thread};

use crate::config::{Config, MAX_PORT};

pub fn run(config: Config) {
    
    let mut open_ports = vec![];
    let (ip_addr, n_threads) = config.props();
    let (tx, rx) = mpsc::channel::<u16>();
    
    for i in 0..n_threads {
        
        let tx_cloned = tx.clone();
        
        thread::spawn(move || {
            
            scan_port_and_send_report(i, ip_addr, n_threads, tx_cloned); 
            
        });
        
    }
    
    drop(tx);
    
    for r in rx {
        open_ports.push(r);
    }
    
    open_ports.sort();
    
    println!("");
    
    for port in open_ports {
        println!("{port}");
    }

    
}

fn scan_port_and_send_report(start_port: u16, ip_address: IpAddr, n_threads: u16, tx: Sender<u16>) {
    
    
    let mut port = start_port + 1;
    
    loop {
        match TcpStream::connect((ip_address, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }
        
        if (MAX_PORT - port) <= n_threads {
            break;
        }
        
        port += n_threads;
    }

    
}