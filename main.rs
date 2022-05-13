
use std::{fs::File, io::Read, collections::HashMap, time::{Instant, Duration}, ops::{Add, AddAssign}, thread};

use autopilot::key::{type_string, KeyCode, tap, Code};
use multiinput::{RawInputManager, DeviceType, State, KeyId};
use serde::{Deserialize};


fn get_key_str(key: &KeyId) -> String {
    format!("{:?}", key)
}

const BP: &Code = &Code(KeyCode::Backspace);

#[derive(Debug, Deserialize, Eq, PartialEq, Hash)]
struct Inputs(String, Option<String>);

#[derive(Debug, Deserialize)]
enum TO {
    Text(String),
    Timer(String, u64),
}


#[derive(Debug, Deserialize)]
struct Outputs(TO, u8);

impl Inputs {
    fn get_key(&self) -> Self {
        Self(self.0.to_string(), None)
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open(".\\kr.ron")?;
    let mut info = String::new();
    file.read_to_string(&mut info)?;

    drop(file);

    let t: HashMap<Inputs, Outputs> = ron::from_str(&info).unwrap();
    drop(info);

    println!("{:?}", t);

    let mut timer = Instant::now();

    let sleep = Duration::from_millis(2);
    let mut kr = false;


    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Keyboards);


    let mut pressed: String = String::new();

    loop {
        if let Some(event) = manager.get_event() {
            match event {
                multiinput::RawEvent::KeyboardEvent(_, key, State::Released) => {
                    if kr {
                        match on_key_release(&key, &mut pressed, &t, &timer) {
                            Err(v) => println!("{:?} {:?}", v, key),
                            _ => {}
                        }
                    }
                    if key == KeyId::Return { kr = !kr; }
                    if key == KeyId::F3 { timer = Instant::now(); }
                },

                _ => {}
            }
        }
        thread::sleep(sleep);
    }

}


fn on_key_release(key: &KeyId, pressed: &mut String, map: &HashMap<Inputs, Outputs>, timer: &Instant) -> Result<(), &'static str> {
    let k = get_key_str(key);
    let t = Inputs(k.to_string(), Some(pressed.to_string()));

    *pressed = k;


    if let Some(value) = map.get(&t) {
        for _i in 0..(value.1) {
            tap(BP, &[], 0, 0);
        }
        match &value.0 {
            TO::Text(v) => {
                type_string(v, &[], 0., 0.);
                *pressed = v.to_string();
            },
            TO::Timer(v, time) => {
                let mut timed = timer.elapsed();
                timed.add_assign(Duration::new(*time, 0));
                let final_time = (timed.as_secs() as f32) / 60.0;
                let final_sec = ((final_time - (final_time as u64 as f32)) * 60.0) as u64;
                type_string(&format!("{} {}:{:1.} ", v, final_time as u64, final_sec), &[], 0., 0.);
            },
        }

        return Ok(());
    } else if let Some(value) = map.get(&t.get_key()) {
        for _i in 0..(value.1) {
            tap(BP, &[], 0, 0);
        }
        match &value.0 {
            TO::Text(v) => {
                type_string(v, &[], 0., 0.);
                *pressed = v.to_string();
            },
            TO::Timer(v, time) => {
                let mut timed = timer.elapsed();
                timed.add_assign(Duration::new(*time, 0));
                let final_time = (timed.as_secs() as f32) / 60.0;
                let final_sec = ((final_time - (final_time as u64 as f32)) * 60.0) as u64;
                type_string(&format!("{} {}:{:1.} ", v, final_time as u64, final_sec), &[], 0., 0.);
            },
        }
        return Ok(());
    }
    println!("{:?}", pressed);


    return Err("Not found");


}