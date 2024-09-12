fn main() {
    let mut time_second: f64 = 0.0;
    let time_step_second: f64 = 1e-7;
    let simulation_time_second: f64 = 11.0;
    let time_step_show_second: f64 = 0.1;

    let mut matter1 = Matter {
        position_meter: 0.0,
        length_meter: 1.0,
        mass_kilogram: 1.0,
        acceleration_meter_per_second_squared: 0.0,
        velocity_meter_per_second: 0.0,
    };

    let mut matter2 = Matter {
        position_meter: 10.0,
        length_meter: 1.0,
        mass_kilogram: 1.0,
        acceleration_meter_per_second_squared: 0.0,
        velocity_meter_per_second: 0.0,
    };

    // Force
    let mut force_newton: f64;
    let duration_force_second: f64 = 10.0;

    let mut time_passed_after_last_show_second: f64 = 0.0;

    while time_second <= simulation_time_second {
        time_second = time_second + time_step_second;

        // Apply a force
        matter1.acceleration_meter_per_second_squared = 0.0;
        if time_second <= duration_force_second {
            force_newton = time_second; // Function of force
            matter1.acceleration_meter_per_second_squared = force_newton / matter1.mass_kilogram;
        }

        matter1.velocity_meter_per_second = matter1.velocity_meter_per_second
            + matter1.acceleration_meter_per_second_squared * time_step_second;

        matter1.position_meter =
            matter1.position_meter + matter1.velocity_meter_per_second * time_step_second;

        time_passed_after_last_show_second = time_passed_after_last_show_second + time_step_second;
        if time_passed_after_last_show_second > time_step_show_second {
            println!(
                "Time: {time_second} s\nPosition: {0} m\nAcceleration: {1} m/(s^2)\nVelocity: {2} m/s\n",
                matter1.position_meter,
                matter1.acceleration_meter_per_second_squared,
                matter1.velocity_meter_per_second
            );
            time_passed_after_last_show_second = 0.0;
        }
    }
}

struct Matter {
    position_meter: f64,
    length_meter: f64,
    mass_kilogram: f64,
    // linear_density_kilogram_per_meter: f64 = mass_kilogram / length_meter,
    acceleration_meter_per_second_squared: f64,
    velocity_meter_per_second: f64,
}
