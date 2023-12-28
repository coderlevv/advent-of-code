use std::sync::Mutex;
use std::collections::HashMap;
use queues::{Queue, IsQueue};
use crate::module::{AocModule, ModuleType, Pulse};
use crate::module::{BroadcasterModule, FlipFlopModule, ConjunctionModule};
type Destination = String;
type Source = String;
type Packet = (Source, Destination, Pulse);


fn parse_destinations(dest_str: &str) -> Vec<String> {
    dest_str
        .split(',')
        .map(|d| d.trim().to_string())
        .collect::<Vec<String>>()
}

pub fn solve(input: &[String]) {
    let wire: Mutex<Queue<Packet>> = Mutex::new(Queue::new());
    let mut modules: Vec<Box<dyn AocModule>> = vec![];
    let mut module: Box<dyn AocModule>;
    let mut conj: HashMap<String, Vec<String>> = HashMap::new();
    for line in input {
        let spl: Vec<_> = line
            .split("->")
            .map(|s| s.trim())
            .collect();
        if spl[0] == "broadcaster" {
            module = Box::new(BroadcasterModule::new(ModuleType::Broadcaster, &wire));
            for d in parse_destinations(spl[1]) {
                module.add_destination(d);
            }
            modules.push(module);
            continue;
        }
        if spl[0].starts_with('%') {
            let module_name = spl[0][1..].to_string();
            module = Box::new(FlipFlopModule::new(module_name, ModuleType::FlipFlop, &wire));
            for d in parse_destinations(spl[1]) {
                module.add_destination(d);
            }
            modules.push(module);
            continue;
        }
        if spl[0].starts_with('&') {
            let module_name = spl[0][1..].to_string();
            conj.insert(module_name.clone(), Vec::new());
            module = Box::new(ConjunctionModule::new(module_name, ModuleType::Conjunction, &wire));
            for d in parse_destinations(spl[1]) {
                module.add_destination(d);
            }
            modules.push(module);
            continue;
        }
    }

    // set up the conjunction inputs
    for module in &modules {
        for dest in module.get_destinations() {
            if conj.contains_key(&dest) {
                conj.entry(dest).or_default().push(module.get_name());
            }
        }
    }
    for (conj_name, input) in conj {
        modules.iter_mut()
            .filter(|m| m.get_name() == conj_name)
            .for_each(|m| {
                for i in &input {
                    m.add_input(i.clone());
                }
            });
    }

    let mut low_count: u64 = 0;
    let mut high_count: u64 = 0;
  

    for _ in 0..1000 {
        // start the communication with a button press
        let packet = ("button".to_string(), "broadcaster".to_string(), Pulse::Low);
        let mut guard = wire.lock().unwrap();
        guard.add(packet).unwrap();
        drop(guard);
        // process packets
        loop {
            let mut guard = wire.lock().unwrap();
            let packet = guard.remove().unwrap();
            drop(guard);
            match packet.2 {
                Pulse::Low => low_count += 1,
                Pulse::High => high_count += 1
            }
            for module in modules.iter_mut() {
                let module_name = module.get_name();
                if module_name == packet.1 {
                    module.process_packet(packet.clone());
                }
            }
            let guard = wire.lock().unwrap();
            let done = guard.size() == 0;
            drop(guard);
            if done { break; }
        }
    }
    println!("{:?}", low_count * high_count);
}
