pub enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    pub fn to_string(&self) -> String {
        match self {
            IpAddr::V4(a, b, c, d) => format!("{a}.{b}.{c}.{d}"),
            IpAddr::V6(ip) => ip.to_string(),
        }
    }
}

#[cfg(test)]
mod ipaddr_tests {
    use super::IpAddr;

    #[test]
    fn ipv4() {
        let home = IpAddr::V4(127, 0, 0, 1);

        assert_eq!(home.to_string(), "127.0.0.1")
    }

    #[test]
    fn ipv6() {
        let loopback = IpAddr::V6("::0".to_string());

        assert_eq!(loopback.to_string(), "::0")
    }
}

pub fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(v) => Some(v + 1),
    }
}

#[cfg(test)]
mod option_tests {
    use super::plus_one;

    #[test]
    fn none_plus_one_is_none() {
        let num: Option<i32> = None;
        let res = plus_one(num);

        assert_eq!(res, None)
    }

    #[test]
    fn some_plus_one_is_some() {
        let num: Option<i32> = Some(5);
        let res = plus_one(num);

        assert_eq!(res, Some(6))
    }

    #[test]
    fn if_let_control() {
        let config_max: Option<u8> = Some(3u8);
        if let Some(v) = config_max {
            assert_eq!(v, 3u8)
        } else {
            assert!(false)
        }
    }
}
