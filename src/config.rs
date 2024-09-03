use std::net::IpAddr;
use std::str::FromStr;

// usage: port_sniffer flag n_threads ip_addr
// flag: -j | -h | --help
// ip_addr: ipv4

pub const MAX_PORT: u16 = 65535;

#[derive(Debug)]
pub struct Config {
    flag: String,
    ip_addr: IpAddr,
    n_threads: u16
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
        
        const ERR_MESSAGE: &str = "usage: port_sniffer flag n_threads ip_addr";
        
        args.next();
  
        let first_arg = if let Some(arg) = args.next() {
            arg
        }else {
            return Err(ERR_MESSAGE.to_string());
        };
        
        if first_arg.contains("-h") 
        || first_arg.contains("--help")
        || !first_arg.contains("-j") {
            return Err(ERR_MESSAGE.to_string());
        }
        
        let second_arg = if let Some(arg) = args.next() {
            arg
        }else{
            return Err(ERR_MESSAGE.to_string());
        };
        
        let n_threads = match second_arg.trim().parse::<u16>() {
            Ok(number) => number,
            Err(_) => return Err(ERR_MESSAGE.to_string())
        };
        
        let third_arg = if let Some(arg) = args.next() {
            arg
        }else {
            return Err(ERR_MESSAGE.to_string());
        };
        
        let ip_addr = match IpAddr::from_str(&third_arg[..]) {
            Ok(ip_addr) => ip_addr,
            Err(_) => return Err(ERR_MESSAGE.to_string())
        };
        
        Ok(
            Config { 
                flag: first_arg, 
                ip_addr, 
                n_threads
            }
        )
    }
    
    pub fn props(&self) -> (IpAddr, u16){
        (self.ip_addr.clone(), self.n_threads)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn test_build_with_no_arguments() {
        let args: Vec<String> = vec![];
        let result = Config::build(args.into_iter());
        
        match result {
            Ok(_)=> {
                panic!("build should return Err variant");
            }
            Err(val) => {
                assert_eq!("usage: port_sniffer flag n_threads ip_addr".to_string(), val);
            }
        }
    }
    
    #[test]
    fn test_build_with_help_flag() {
        
        let args: Vec<String> = vec![
            "port_scanner".to_string(),
            "-h".to_string()
        ];
        
        let result = Config::build(args.into_iter());
        
        match result {
            Ok(_)=> {
                panic!("build should return Err variant");
            }
            Err(val) => {
                assert_eq!("usage: port_sniffer flag n_threads ip_addr".to_string(), val);
            }
        }
        
    }
    
    #[test]
    fn test_build_with_invalid_flag() {
        let args: Vec<String> = vec![
            "port_scanner".to_string(),
            "-k".to_string()
        ];
        
        let result = Config::build(args.into_iter());
        
        match result {
            Ok(_)=> {
                panic!("build should return Err variant");
            }
            Err(val) => {
                assert_eq!("usage: port_sniffer flag n_threads ip_addr".to_string(), val);
            }
        }
    }
}