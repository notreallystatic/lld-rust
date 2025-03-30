/*
    Problem Statement:
        You have some IoT devices like fan and a light bulb.
        Now you want to check if a light bulb is on or not, or switch it on/off.
        Same for fan, you want to check if it is switched on, or control its speed from 0-5.
        But, different clients have bought these sensors from different manufactures.
        So, the APIs to handle these control vary across the manufactures.
        Solve this problem using the abstract factory design pattern.
*/

use std::{
    error::Error,
    net::{IpAddr, Ipv6Addr},
    panic,
};

trait LightBulb {
    fn is_switched_on(&self) -> Result<bool, Box<dyn Error>>;
    fn switch(&mut self, command: bool) -> Result<bool, Box<dyn Error>>;
}

#[derive(PartialEq, PartialOrd, Debug)]
enum FanSpeed {
    Speed0 = 0,
    Speed1 = 1,
    Speed2 = 2,
    Speed3 = 3,
    Speed4 = 4,
    Speed5 = 5,
}

trait Fan {
    fn is_switched_on(&self) -> Result<bool, Box<dyn Error>>;
    fn switch(&mut self, command: FanSpeed) -> Result<bool, Box<dyn Error>>;
}

trait DeviceFactory {
    fn create_light_bulb(&self) -> Result<Box<dyn LightBulb>, Box<dyn Error>>;
    fn create_fan(&self) -> Result<Box<dyn Fan>, Box<dyn Error>>;
}

#[derive(Debug)]
struct PhilipsLightBulb {
    sensor_config: PhilipsSensor,
    state: bool,
}

impl LightBulb for PhilipsLightBulb {
    fn is_switched_on(&self) -> Result<bool, Box<dyn Error>> {
        println!("state :: {:?}", self);
        Ok(self.state)
    }

    fn switch(&mut self, command: bool) -> Result<bool, Box<dyn Error>> {
        println!("prev state :: {:?}", self);
        self.state = command;
        println!("new state :: {:?}", self);
        Ok(true)
    }
}
#[derive(Debug)]
struct PhilipsFan {
    sensor_config: PhilipsSensor,
    state: FanSpeed,
}

impl Fan for PhilipsFan {
    fn is_switched_on(&self) -> Result<bool, Box<dyn Error>> {
        println!("state :: {:?}", self);
        Ok(self.state > FanSpeed::Speed0)
    }

    fn switch(&mut self, command: FanSpeed) -> Result<bool, Box<dyn Error>> {
        println!("prev state :: {:?}", self);
        self.state = command;
        println!("new state :: {:?}", self);
        Ok(true)
    }
}

#[derive(Debug, Clone)]
struct PhilipsSensor {
    brand: String,
    ip: IpAddr,
    port: u16,
}

impl DeviceFactory for PhilipsSensor {
    fn create_light_bulb(&self) -> Result<Box<dyn LightBulb>, Box<dyn Error>> {
        Ok(Box::new(PhilipsLightBulb {
            sensor_config: self.clone(),
            state: false,
        }))
    }

    fn create_fan(&self) -> Result<Box<dyn Fan>, Box<dyn Error>> {
        Ok(Box::new(PhilipsFan {
            sensor_config: self.clone(),
            state: FanSpeed::Speed0,
        }))
    }
}

#[derive(Debug)]
struct SamsungLightBulb {
    sensor_config: SamsungSensor,
    state: bool,
}

impl LightBulb for SamsungLightBulb {
    fn is_switched_on(&self) -> Result<bool, Box<dyn Error>> {
        println!("state :: {:?}", self);
        Ok(self.state)
    }

    fn switch(&mut self, command: bool) -> Result<bool, Box<dyn Error>> {
        println!("prev state :: {:?}", self);
        self.state = command;
        println!("new state :: {:?}", self);
        Ok(true)
    }
}

#[derive(Debug)]
struct SamsungFan {
    sensor_config: SamsungSensor,
    state: FanSpeed,
}

impl Fan for SamsungFan {
    fn is_switched_on(&self) -> Result<bool, Box<dyn Error>> {
        println!("prev state :: {:?}", self);
        Ok(self.state > FanSpeed::Speed0)
    }

    fn switch(&mut self, command: FanSpeed) -> Result<bool, Box<dyn Error>> {
        println!("prev state :: {:?}", self);
        self.state = command;
        println!("new state :: {:?}", self);
        Ok(true)
    }
}

#[derive(Debug, Clone)]
struct SamsungSensor {
    brand: String,
    ip: IpAddr,
    port: u16,
}

impl DeviceFactory for SamsungSensor {
    fn create_light_bulb(&self) -> Result<Box<dyn LightBulb>, Box<dyn Error>> {
        Ok(Box::new(SamsungLightBulb {
            sensor_config: self.clone(),
            state: false,
        }))
    }

    fn create_fan(&self) -> Result<Box<dyn Fan>, Box<dyn Error>> {
        Ok(Box::new(SamsungFan {
            sensor_config: self.clone(),
            state: FanSpeed::Speed0,
        }))
    }
}

pub fn run() {
    let application_list: Vec<(&str, IpAddr, u16)> = vec![
        (
            "Samsung",
            IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
            3000,
        ),
        (
            "Philips",
            IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 1, 1)),
            8080,
        ),
    ];
    for application in application_list {
        let device: Box<dyn DeviceFactory> = match application.0 {
            "Samsung" => Box::new(SamsungSensor {
                brand: String::from(application.0),
                ip: application.1,
                port: application.2,
            }),
            "Philips" => Box::new(PhilipsSensor {
                brand: String::from(application.0),
                ip: application.1,
                port: application.2,
            }),
            _ => {
                panic!("Invalid factory")
            }
        };
        let mut light_bulb = device.create_light_bulb().unwrap();
        let _ = light_bulb.is_switched_on();
        let _ = light_bulb.switch(true);
        let _ = light_bulb.is_switched_on();

        let mut fan = device.create_fan().unwrap();
        let _ = fan.is_switched_on();
        let _ = fan.switch(FanSpeed::Speed4);
        let _ = fan.is_switched_on();
    }
}
