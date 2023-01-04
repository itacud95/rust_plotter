use plotters::prelude::*;
use std::cmp::Ord;
use std::{cmp::max, collections::BTreeMap, env, fs};

fn read_file(filename: &str) -> BTreeMap<String, Vec<f32>> {
    // Read the contents of the file into a string
    let contents = fs::read_to_string(filename).expect("Unable to read file");

    // Split the string into a vector of lines
    let lines: Vec<String> = contents.split('\n').map(|s| s.to_string()).collect();
    // let lines = contents.split('\n').collect::<Vec<&str>>();

    // Create a BTreeMap to store the key-value pairs
    let mut map = BTreeMap::new();

    for line in lines {
        let words = line.split_whitespace().collect::<Vec<&str>>();

        if words.len() != 2 {
            continue;
        }

        let key = String::from(words[0]);
        let val = words[1];
        let num: f32 = val.parse().unwrap();

        if !map.contains_key(&key) {
            map.insert(key, vec![num]);
            continue;
        }

        let iter = map.get_mut(&key).unwrap();
        iter.push(num);
    }
    return map;
}

fn plot(title: &String, dev_timings: Vec<f32>, org_timings: Vec<f32>) {
    let path = OUT_FILE_NAME.to_owned() + &title.to_string() + ".png";
    let root_area = BitMapBackend::new(&path, (1024*2, 768*2)).into_drawing_area();

    root_area.fill(&WHITE).unwrap();

    let root_area = root_area.titled(&title, ("sans-serif", 20)).unwrap();

    // let (upper, _lower) = root_area.split_vertically(512);

    let dev_max = dev_timings
        .iter()
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();
    let org_max = org_timings
        .iter()
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();

    let dev_sum: f32 = dev_timings.iter().sum();
    let dev_avg = dev_sum / dev_timings.len() as f32;
    let org_sum: f32 = org_timings.iter().sum();
    let org_avg = org_sum / org_timings.len() as f32;

    let y_valuesi32: Vec<i32> = (0..dev_timings.len() as i32).collect(); // last element will be n-1
    let y_values: Vec<f32> = y_valuesi32.iter().map(|x| *x as f32).collect();

    let x_scale = dev_timings.len() as f32 + 5f32;
    let y_scale = if dev_avg > org_avg {
        dev_avg.to_owned() * 2f32
    } else {
        org_avg.to_owned() * 2f32
    };

    let mut cc = ChartBuilder::on(&root_area)
        .margin(5)
        .set_all_label_area_size(50)
        // .caption("%", ("sans-serif", 40))
        .build_cartesian_2d(0f32..x_scale, 0f32..y_scale)
        .unwrap();

    cc.configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()
        .unwrap();

    let dev_iter = y_values.iter().cloned().zip(dev_timings.iter().cloned());

    // for (x,y) in iter.clone() {
    //     println!("{}, {}", x, y);
    // }

    let dev_series = LineSeries::new(dev_iter, &RGBColor(255, 0, 0));

    cc.draw_series(dev_series)
        .unwrap()
        .label("Devel")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    let org_iter = y_values.iter().cloned().zip(org_timings.iter().cloned());
    let org_series = LineSeries::new(org_iter, &BLUE);

    cc.draw_series(org_series)
        .unwrap()
        .label("Master")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    cc.configure_series_labels()
        .border_style(&BLACK)
        .draw()
        .unwrap();

    /*
    // It's possible to use a existing pointing element
     cc.draw_series(PointSeries::<_, _, Circle<_>>::new(
        (-3.0f32..2.1f32).step(1.0).values().map(|x| (x, x.sin())),
        5,
        Into::<ShapeStyle>::into(&RGBColor(255,0,0)).filled(),
    ))?;*/

    // To avoid the IO failure being ignored silently, we manually call the present function
    root_area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", path);
}

const OUT_FILE_NAME: &'static str = "plotters-doc-data/";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dev_values = read_file("dev_time");
    let org_values = read_file("org_time");

    for (key, values) in dev_values.iter() {
        let sum: f32 = values.iter().sum();
        let avg = sum / values.len() as f32;
        if values.len() < 5 {
            continue;
        }
        println!("{key}: count: {} avg: {}", values.len(), avg);
        let org_values = org_values.get(key).unwrap();
        plot(key, values.to_vec(), org_values.to_vec());
        // break;
    }

    Ok(())
}
