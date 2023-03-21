use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use chrono::{Datelike, Days, NaiveDate};

const COLLECTOR_CAPACITY: f32 = 25000.;

struct Ex2Data {
    date: NaiveDate,
    water_refilled: f32,
}

struct Period {
    start: NaiveDate,
    duration_days: u64,
}

struct Ex5Data {
    period: Period,
    last_temp: f32,
}

fn main() {
    let file = File::open("pogoda.txt").expect("File 'pogoda.txt' not found");

    let start_date = NaiveDate::from_ymd_opt(2015, 3, 31).expect("Error parsing start date");

    let mut collector = COLLECTOR_CAPACITY;

    let mut days_with_temp_less_than_15 = 0;
    let mut days_with_temp_over_15_and_rain_less_than_0_6 = 0;
    let mut days_with_temp_over_15_and_rain_over_0_6 = 0;

    let mut first_day_when_collector_was_refilled = None;

    let mut monthly_water_use = vec![0.; 12];

    let mut biggest_rain_water_loss = 0.;

    let mut current_period_of_rainless_days_with_temp_rising = Ex5Data {
        period: Period {
            start: start_date,
            duration_days: 0,
        },
        last_temp: 0.,
    };

    let mut longest_period_of_rainless_days_with_temp_rising = Period {
        start: start_date,
        duration_days: 0,
    };

    let mut current_rain_sum = 0.;
    let mut biggest_rain_sum = 0.;

    let mut current_date = start_date;

    for line in BufReader::new(file).lines().into_iter().skip(1) {
        let line = line.expect("Error reading line");
        let mut line = line.split_whitespace();

        current_date = current_date
            .succ_opt()
            .expect("Error adding days to current date");

        let temp = line
            .next()
            .expect("Error reading temperature")
            .replace(",", ".");
        let temp = temp
            .parse::<f32>()
            .expect(format!("Error parsing temperature: {}", temp).as_str());

        let rain = line
            .next()
            .expect("Error reading pressure")
            .replace(",", ".");
        let rain = rain
            .parse::<f32>()
            .expect(format!("Error parsing pressure: {}", rain).as_str());

        if rain > 0. {
            let collector_with_rain = collector + 700. * rain as f32;

            if collector_with_rain - COLLECTOR_CAPACITY > biggest_rain_water_loss {
                biggest_rain_water_loss = collector_with_rain - COLLECTOR_CAPACITY;
            }

            collector = collector_with_rain.min(COLLECTOR_CAPACITY).ceil();

            // Ex.5.
            if current_period_of_rainless_days_with_temp_rising
                .period
                .duration_days
                > longest_period_of_rainless_days_with_temp_rising.duration_days
            {
                longest_period_of_rainless_days_with_temp_rising =
                    current_period_of_rainless_days_with_temp_rising.period;
            }

            current_period_of_rainless_days_with_temp_rising = Ex5Data {
                period: Period {
                    start: current_date,
                    duration_days: 0,
                },
                last_temp: f32::MAX,
            };

            // Ex. 6.
            current_rain_sum += rain;
        } else {
            let collector_with_evaporation = collector - 0.0003 * temp.powf(1.5) * collector;

            collector = collector_with_evaporation.max(0.).ceil();

            // Ex. 5.
            if temp > current_period_of_rainless_days_with_temp_rising.last_temp {
                current_period_of_rainless_days_with_temp_rising
                    .period
                    .duration_days += 1;
            } else {
                if current_period_of_rainless_days_with_temp_rising
                    .period
                    .duration_days
                    > longest_period_of_rainless_days_with_temp_rising.duration_days
                {
                    longest_period_of_rainless_days_with_temp_rising =
                        current_period_of_rainless_days_with_temp_rising.period;
                }

                current_period_of_rainless_days_with_temp_rising = Ex5Data {
                    period: Period {
                        start: current_date,
                        duration_days: 0,
                    },
                    last_temp: 0.,
                };
            }

            // Ex. 6.
            if current_rain_sum > biggest_rain_sum {
                biggest_rain_sum = current_rain_sum;
            }

            current_rain_sum = 0.;
        }

        current_period_of_rainless_days_with_temp_rising.last_temp = temp;

        // How much water is needed to water the plants
        let water_needed = if temp <= 30. { 12000. } else { 24000. };

        // If there isn't enough water in the collector, it gets filled to the top
        if collector < water_needed {
            let water_refilled = COLLECTOR_CAPACITY - collector;

            if first_day_when_collector_was_refilled.is_none() {
                first_day_when_collector_was_refilled = Some(Ex2Data {
                    date: current_date,
                    water_refilled,
                });
            }

            monthly_water_use[current_date.month0() as usize] += water_refilled;

            collector = COLLECTOR_CAPACITY;
        }

        // Water the plants
        collector -= water_needed;

        if collector < 0. {
            panic!("The water needed to water the plants ({}) was greater than the capacity of the collector ({})", water_needed, COLLECTOR_CAPACITY);
        }

        // Ex. 1.
        if temp < 15. {
            days_with_temp_less_than_15 += 1;
        }

        if temp > 15. && rain < 0.6 {
            days_with_temp_over_15_and_rain_less_than_0_6 += 1;
        }

        if temp > 15. && rain > 0.6 {
            days_with_temp_over_15_and_rain_over_0_6 += 1;
        }
    }

    if current_rain_sum > biggest_rain_sum {
        biggest_rain_sum = current_rain_sum;
    }

    println!("Ex. 1.");
    println!(
        "\tDays with temperature less than 15: {}",
        days_with_temp_less_than_15
    );
    println!(
        "\tDays with temperature over 15 and rain less than 0.6: {}",
        days_with_temp_over_15_and_rain_less_than_0_6
    );
    println!(
        "\tDays with temperature over 15 and rain over 0.6: {}",
        days_with_temp_over_15_and_rain_over_0_6
    );

    println!("Ex. 2.");
    let first_day_when_collector_was_refilled =
        first_day_when_collector_was_refilled.expect("Collector was never refilled");
    println!(
        "\tFirst day when collector was refilled: {}",
        first_day_when_collector_was_refilled
            .date
            .format("%Y-%m-%d")
    );
    println!(
        "\tWater refilled: {}",
        first_day_when_collector_was_refilled.water_refilled
    );

    println!("Ex. 3.");
    for i in 3..8 {
        let water_used = (monthly_water_use[i] / 1000.).ceil();
        let water_price = 11.74;

        let water_cost = water_used * water_price;

        println!(
            "\t{}: {} m^3, {:.2} PLN",
            NaiveDate::from_ymd_opt(2015, i as u32 + 1, 1)
                .expect("Error parsing date")
                .format("%B"),
            water_used,
            water_cost
        );
    }

    println!("Ex. 4.");
    println!(
        "\tBiggest rain water loss: {}",
        biggest_rain_water_loss.ceil()
    );
    println!(
        "\tTo accomodate for this loss, the collector should have a capacity of {} m^3",
        (COLLECTOR_CAPACITY + biggest_rain_water_loss).ceil()
    );

    println!("Ex. 5.");

    let start_date = longest_period_of_rainless_days_with_temp_rising.start;
    let end_date = start_date
        .checked_add_days(Days::new(
            longest_period_of_rainless_days_with_temp_rising.duration_days,
        ))
        .unwrap();

    println!(
        "\tLongest period of rainless days with temperature rising: {} - {} ({} days)",
        start_date.format("%Y-%m-%d"),
        end_date.format("%Y-%m-%d"),
        longest_period_of_rainless_days_with_temp_rising.duration_days
    );

    println!("Ex. 6.");
    println!("\tBiggest rain sum: {}", biggest_rain_sum);
}
