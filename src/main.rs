extern crate i2c_linux;

use std::{fs, format, thread, time::Duration};
use i2c_linux::{I2c, Message};

fn read_cpu_temp_to_float(sensor_path: &String) -> i16 {
	let mut temperature_string = fs::read_to_string(sensor_path).expect("Unable to read CPU temperature");
	temperature_string.pop();

	let temperature_int: i32 = temperature_string.parse().expect("Error parsing CPU temperature");

	(temperature_int / 1000) as i16
}

fn cpu_temp_to_fan_percent(temperature: i16, min: i16, max: i16) -> u8 {
	let minrel_temp = (temperature - min) as f32;
	let div = (max - min) as f32;

	100_u8.min(1_u8.max((minrel_temp / div * 100.0) as u8))
}

fn thread_tempcontrol() {
	let sensor_path = "/sys/class/thermal/thermal_zone0/temp".to_string();
	let temperature_min = 30;
	let temperature_max = 65;

	let i2c_device = "/dev/i2c-1";
	let i2c_address: u16 = 0x1a;

	let waittime = 2500;

	let waittime_duration = Duration::from_millis(waittime);

	let mut i2c = I2c::from_path(i2c_device).expect("Unable to open I2C");
	i2c.smbus_set_slave_address(i2c_address, false).expect(format!("Unable to use address {}", i2c_address).as_str());

	let (_, write_flags) = i2c.i2c_transfer_flags().expect("Error getting transfer flags");

	loop {
		let temperature = read_cpu_temp_to_float(&sensor_path);
		let fanspeed = cpu_temp_to_fan_percent(temperature, temperature_min, temperature_max);

		// println!("{}°C -> {}%", temperature, fanspeed);
		i2c.i2c_transfer(&mut [Message::Write {
			address: i2c_address,
			data: &[fanspeed],
			flags: write_flags
		}]);

		thread::sleep(waittime_duration);
	}
}

fn main() {
	let scheduler = thread::spawn(thread_tempcontrol);

	scheduler.join().expect("Scheduler panicked");
}

