use queues::{Queue, IsQueue};
use std::sync::Mutex;
use core::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pulse {
    Low,
    High
}

type Destination = String;
type Source = String;
type Packet = (Source, Destination, Pulse);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction
}

pub trait AocModule {
    fn process_packet(&mut self, packet: Packet);
    fn add_destination(&mut self, dest: Destination);
    fn get_destinations(&self) -> Vec<Destination>;
    fn set_type(&mut self, module_type: ModuleType);
    fn get_type(&self) -> ModuleType;
    fn get_name(&self) -> String;
    // default add_input implementation is empty
    // only used in conjunction module
    fn add_input(&mut self, input_name: String) { }
}

#[derive(Debug, Clone)]
pub struct BroadcasterModule<'a> {
    module_type: ModuleType,
    wire: &'a Mutex<Queue<Packet>>,
    dest: Vec<Destination>,
    pulse: Pulse
}

impl<'a> BroadcasterModule<'a> {
    pub fn new(
        module_type: ModuleType,
        wire: &'a Mutex<Queue<Packet>>
    ) -> Self {
        let dest: Vec<Destination> = vec![];
        Self {
            module_type,
            wire,
            dest,
            pulse: Pulse::Low
        }
    } 
}

impl<'a> AocModule for BroadcasterModule<'a> {
    fn add_destination(&mut self, dest: Destination) {
        self.dest.push(dest);    
    }

    fn get_destinations(&self) -> Vec<Destination> {
        self.dest.clone()
    }

    fn process_packet(&mut self, packet: Packet) {
        self.pulse = packet.2;
        let mut guard = self.wire.lock().unwrap();
        for d in &self.dest {
            let packet: Packet = (self.get_name(), d.clone(), self.pulse);
            guard.add(packet).unwrap();
        }
    }

    fn set_type(&mut self, module_type: ModuleType) {
        self.module_type = module_type;
    }

    fn get_name(&self) -> String {
        "broadcaster".to_string()
    }

    fn get_type(&self) -> ModuleType {
        self.module_type
    }
}

#[derive(Debug, Clone)]
pub struct FlipFlopModule<'a> {
    name: String,
    module_type: ModuleType,
    status_on: bool,
    wire: &'a Mutex<Queue<Packet>>,
    pub dest: Vec<Destination>
}

impl<'a> FlipFlopModule<'a> {
    pub fn new(
        name: String,
        module_type: ModuleType,
        wire: &'a Mutex<Queue<Packet>>
    ) -> Self {
        let dest: Vec<Destination> = vec![];
        Self {
            name,
            module_type,
            status_on: false,
            wire,
            dest
        }
    } 
}

impl<'a> AocModule for FlipFlopModule<'a> {
    fn add_destination(&mut self, dest: Destination) {
        self.dest.push(dest);    
    }

    fn get_destinations(&self) -> Vec<Destination> {
        self.dest.clone()
    }

    fn process_packet(&mut self, packet: Packet) {
        if packet.2 == Pulse::Low {
            let mut guard = self.wire.lock().unwrap();
            match self.status_on {
                false => {
                    self.status_on = true;
                    for d in &self.dest {
                        let packet: Packet = (self.get_name(), d.clone(), Pulse::High);
                        guard.add(packet).unwrap();
                    }
                    },
                true => {
                    self.status_on = false;
                    for d in &self.dest {
                        let packet: Packet = (self.get_name(), d.clone(), Pulse::Low);
                        guard.add(packet).unwrap();
                    }
                }
            }
            
        }
    }

    fn set_type(&mut self, module_type: ModuleType) {
        self.module_type = module_type;
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> ModuleType {
        self.module_type
    }
}


#[derive(Debug, Clone)]
pub struct ConjunctionModule<'a> {
    name: String,
    module_type: ModuleType,
    wire: &'a Mutex<Queue<Packet>>,
    dest: Vec<Destination>,
    input: Vec<Source>,
    input_mem: Vec<Pulse>
}

impl<'a> ConjunctionModule<'a> {
    pub fn new(
        name: String,
        module_type: ModuleType,
        wire: &'a Mutex<Queue<Packet>>
    ) -> Self {
        let dest: Vec<Destination> = vec![];
        let input_mem: Vec<Pulse> = vec![];
        let input: Vec<Destination> = vec![];
        Self {
            name,
            module_type,
            wire,
            dest,
            input,
            input_mem
        }
    } 
}

impl<'a> AocModule for ConjunctionModule<'a> {
    fn add_destination(&mut self, dest: Destination) {
        self.dest.push(dest);
    }

    fn get_destinations(&self) -> Vec<Destination> {
        self.dest.clone()
    }

    fn process_packet(&mut self, packet: Packet) {
        let input_name = packet.0;
        if let Some(input_idx) = self.input.iter().position(|d| **d == input_name) {
            self.input_mem[input_idx] = packet.2;
        }
        let mut guard = self.wire.lock().unwrap();
        let all_high = self.input_mem.iter().all(|p| *p == Pulse::High);
        let pulse = if all_high { Pulse::Low } else { Pulse::High };
        for d in &self.dest {
            let packet: Packet = (self.get_name(), d.clone(), pulse);
            guard.add(packet).unwrap();
        }
    }

    fn set_type(&mut self, module_type: ModuleType) {
        self.module_type = module_type;
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> ModuleType {
        self.module_type
    }

    fn add_input(&mut self, input_name: String) {
        self.input.push(input_name);
        self.input_mem.push(Pulse::Low);
    }
}
