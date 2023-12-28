use std::sync::Mutex;
use std::collections::HashMap;
use queues::{Queue, IsQueue};
use num::integer;
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

    let mut conj_monitor: HashMap<String, u64> = HashMap::new();
    // register conjunction modules to monitor
    conj_monitor.insert("vg".to_string(), 0);
    conj_monitor.insert("nb".to_string(), 0);
    conj_monitor.insert("vc".to_string(), 0);
    conj_monitor.insert("ls".to_string(), 0);
    let mut conj_high_count: HashMap<String, u32> = HashMap::new();
    let mut button_press: u64 = 0;
    loop {
        // start the communication with a button press
        button_press += 1;
        let packet = ("button".to_string(), "broadcaster".to_string(), Pulse::Low);
        let mut guard = wire.lock().unwrap();
        guard.add(packet).unwrap();
        drop(guard);
        // process packets
        loop {
            let mut guard = wire.lock().unwrap();
            let packet = guard.remove().unwrap();
            drop(guard);
            if conj_monitor.contains_key(&packet.0) && packet.2 == Pulse::High {
                *conj_high_count.entry(packet.0.clone()).or_default() += 1;
                if *conj_high_count.get(&packet.0).unwrap() == 1 {
                    conj_monitor.insert(packet.0.clone(), button_press);
                }
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
        if conj_high_count.len() == conj_monitor.len() && conj_high_count.values().all(|c| *c >= 2) {
            break;
        }
        //if button_press == 2 { break; }
    }
    let monitor_vals: Vec<_> = conj_monitor.values().copied().collect();
    let mut lcm: u64 = integer::lcm(monitor_vals[0], monitor_vals[1]);
    let mut k: usize = 2;
    while k < monitor_vals.len() {
        lcm = integer::lcm(lcm, monitor_vals[k]);
        k += 1;
    }
    println!("{}", lcm);
}
