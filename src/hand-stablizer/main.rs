use std::thread;
use std::time::Duration;

use image::imageops;
use nokhwa::{CameraFormat, FrameFormat, ThreadedCamera};
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;

use anthon_rs::common::PointerTracker;
use anthon_rs::mediapipe::{Mediapipe, Packet};

fn record(count: usize) -> Vec<Packet> {
    let mut camera = ThreadedCamera::new(
        0,
        Some(CameraFormat::new_from(
            640,
            480,
            FrameFormat::MJPEG,
            anthon_rs::FPS as u32,
        )),
    )
    .expect("Capturing device initialization failed.");
    let mediapipe = Mediapipe::default();

    if camera.open_stream(|_| ()).is_err() {
        panic!("Opening capture stream failed.");
    }

    (0..count)
        .map(|i| loop {
            let frame = imageops::flip_horizontal(&camera.last_frame());
            let packet = mediapipe.process(frame);

            if packet.landmarks.is_some() {
                println!("{}", count - i);
                break packet;
            } else {
                thread::sleep(Duration::from_millis(anthon_rs::MPF));
            }
        })
        .collect()
}

fn filter(packets: &[Packet], wma_order: usize) -> Vec<(f32, f32)> {
    let mut pointer = PointerTracker::new(wma_order).unwrap();

    packets
        .iter()
        .enumerate()
        .map(|(i, packet)| {
            let t = i as f32 / packets.len() as f32;
            pointer.track(packet).unwrap();

            (t, pointer.x())
        })
        .collect()
}

fn plot<F>(path: &str, draw: F)
where
    F: Fn(&mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>),
{
    let root = BitMapBackend::new(path, (640, 360)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .margin(5u32)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(0f32..1f32, 0.5f32..0.85f32)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    draw(&mut chart);

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();
}

fn main() {
    let packets = record(100);
    let real: Vec<(f32, f32)> = packets
        .iter()
        .enumerate()
        .map(|(i, packet)| {
            let t = i as f32 / packets.len() as f32;

            let x = if let Some(ref landmarks) = packet.landmarks {
                (landmarks[5].x + landmarks[17].x).abs() / 2f32
            } else {
                0f32
            };

            (t, x)
        })
        .collect();

    let filtered_5 = filter(&packets, 5);
    let filtered_10 = filter(&packets, 10);
    let filtered_20 = filter(&packets, 20);

    plot("plot0.png", |chart| {
        chart
            .draw_series(LineSeries::new(real.clone().into_iter(), &RED))
            .unwrap()
            .label("Real X")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    });

    plot("plot1.png", |chart| {
        chart
            .draw_series(LineSeries::new(real.clone().into_iter(), &RED))
            .unwrap()
            .label("Real X")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        chart
            .draw_series(LineSeries::new(filtered_5.clone().into_iter(), &BLUE))
            .unwrap()
            .label("Filtered X")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    });

    plot("plot2.png", |chart| {
        chart
            .draw_series(LineSeries::new(real.clone().into_iter(), &RED))
            .unwrap()
            .label("Real X")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        chart
            .draw_series(LineSeries::new(filtered_10.clone().into_iter(), &BLUE))
            .unwrap()
            .label("Filtered X")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    });

    plot("plot3.png", |chart| {
        chart
            .draw_series(LineSeries::new(real.clone().into_iter(), &RED))
            .unwrap()
            .label("Real X")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        chart
            .draw_series(LineSeries::new(filtered_20.clone().into_iter(), &BLUE))
            .unwrap()
            .label("Filtered X")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    });

    plot("plot4.png", |chart| {
        chart
            .draw_series(LineSeries::new(real.clone().into_iter(), &RED))
            .unwrap()
            .label("Real X")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        chart
            .draw_series(LineSeries::new(filtered_5.clone().into_iter(), &BLUE))
            .unwrap()
            .label("Filtered X")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
        chart
            .draw_series(LineSeries::new(filtered_10.clone().into_iter(), &BLUE))
            .unwrap()
            .label("Filtered X")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
        chart
            .draw_series(LineSeries::new(filtered_20.clone().into_iter(), &BLUE))
            .unwrap()
            .label("Filtered X")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    });
}
