use std::collections::HashMap;

#[derive(Debug)]
struct UndergroundSystem {
    records: HashMap<String, HashMap<String, Vec<i32>>>,
    checkin: HashMap<i32, CheckInInfo>,
}

#[derive(Debug)]
struct CheckInInfo {
    station_name: String,
    t: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {
    fn new() -> Self {
        Self {
            records: HashMap::new(),
            checkin: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.checkin.insert(id, CheckInInfo { station_name, t });
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        if let Some(info) = self.checkin.get(&id) {
            let spend = t - info.t;
            let start = info.station_name.clone();
            if let Some(rec) = self.records.get_mut(&start) {
                if let Some(res) = rec.get_mut(&station_name) {
                    res.push(spend);
                } else {
                    rec.insert(station_name, vec![spend]);
                }
            } else {
                self.records
                    .insert(start, HashMap::from([(station_name, vec![spend])]));
            }
        }
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let rec = self.records.get(&start_station).unwrap();
        let rec = rec.get(&end_station).unwrap();
        let sum: i32 = rec.iter().sum();
        sum as f64 / rec.len() as f64
    }
}

#[test]
fn test() {
    let mut system = UndergroundSystem::new();
    system.check_in(45, "Leyton".to_string(), 3);
    system.check_in(32, "Paradise".to_string(), 8);
    system.check_in(27, "Leyton".to_string(), 10);
    system.check_out(45, "Waterloo".to_string(), 15); // Customer 45 "Leyton" -> "Waterloo" in 15-3 = 12
    system.check_out(27, "Waterloo".to_string(), 20); // Customer 27 "Leyton" -> "Waterloo" in 20-10 = 10
    system.check_out(32, "Cambridge".to_string(), 22); // Customer 32 "Paradise" -> "Cambridge" in 22-8 = 14
    system.get_average_time("Paradise".to_string(), "Cambridge".to_string()); // return 14.00000. One trip "Paradise" -> "Cambridge", (14) / 1 = 14
    system.get_average_time("Leyton".to_string(), "Waterloo".to_string()); // return 11.00000. Two trips "Leyton" -> "Waterloo", (10 + 12) / 2 = 11
    system.check_in(10, "Leyton".to_string(), 24);
    system.get_average_time("Leyton".to_string(), "Waterloo".to_string()); // return 11.00000
    system.check_out(10, "Waterloo".to_string(), 38); // Customer 10 "Leyton" -> "Waterloo" in 38-24 = 14
    system.get_average_time("Leyton".to_string(), "Waterloo".to_string()); // return 12.00000. Three trips "Leyton" -> "Waterloo", (10 + 12 + 14) / 3 = 12
    dbg!(system);
}
