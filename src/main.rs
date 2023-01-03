use std::{collections::BTreeMap, env, fs, slice::Iter};
// use plotters::data::range;
use plotters::prelude::*;

fn read_file(filename: &String) -> BTreeMap<String, Vec<f32>> {
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

fn plot(valuesy: Vec<f32>) {
    let root_area = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();

    root_area.fill(&WHITE).unwrap();

    let root_area = root_area.titled("_", ("sans-serif", 60)).unwrap();

    let (upper, lower) = root_area.split_vertically(512);

    // let x_axis = (0f32..x).step(1.0);
    let values = vec![0.0f32, 2., 4., 6., 8., 10., 5.];
    let max_value = values
        .iter()
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap()
        .clone();
    let mut y_valuesi32: Vec<i32> = (0..values.len() as i32).collect(); // last element will be n-1
    let y_values: Vec<f32> = y_valuesi32.iter().map(|x| *x as f32).collect();

    let x = values.len() as f32;
    let y = max_value + 5f32;

    let mut cc = ChartBuilder::on(&upper)
        .margin(5)
        .set_all_label_area_size(50)
        .caption("%", ("sans-serif", 40))
        .build_cartesian_2d(0f32..x, 0f32..y)
        .unwrap();
    // .build_cartesian_2d(-3.4f32..3.4, -1.2f32..1.2f32)?;

    cc.configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()
        .unwrap();

    // for (x, y) in values.iter().zip(y_values.iter()) {}

    let iter = y_values.iter().cloned().zip(values.iter().cloned());

    for (x,y) in iter.clone() {
        println!("{}, {}", x, y);
    }

    // let iter = values.iter().zip(y_values.iter());
    let series = LineSeries::new(iter, &RGBColor(255, 0, 0));
    // let seriesY = LineSeries::new(values.into_iter().map(|x| (x, x)), &RGBColor(255, 0, 0));

    cc.draw_series(series)
        .unwrap()
        .label("Red label")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // cc.draw_series(LineSeries::new(
    //     x_axis.values().map(|x| (x, x.cos())),
    //     &BLUE,
    // ))?
    // .label("Cosine")
    // .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

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
    println!("Result has been saved to {}", OUT_FILE_NAME);
}

const OUT_FILE_NAME: &'static str = "plotters-doc-data/sample.png";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Give file to read")
    }
    let filename = &args[1];
    println!("Reading file {}", filename);
    let dev_values = read_file(filename);

    for (key, values) in dev_values.iter() {
        let sum: f32 = values.iter().sum();
        let avg = sum / values.len() as f32;
        if values.len() < 5 {
            continue;
        }
        println!("{key}: count: {} avg: {}", values.len(), avg);
        // values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        plot(values.to_vec());
        break;
    }

    Ok(())
}
