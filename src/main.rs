use plotters::prelude::*;

const P0: f64 = 0.322232431088;
const P1: f64 = 1.0;
const P2: f64 = 0.342242088547;
const P3: f64 = 0.204231210245e-1;
const P4: f64 = 0.453642210148e-4;

const Q0: f64 = 0.099348462606;
const Q1: f64 = 0.588581570495;
const Q2: f64 = 0.531103462366;
const Q3: f64 = 0.103537752850;
const Q4: f64 = 0.385607006340e-2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let m = 5.0;
    let s = 2.0;

    let mut data = Vec::new();
    for _ in 0..10_000_000 {
        let number = sputinik(m, s);
        data.push(number);
    }

    let bin_count = 1_000_000;

    let min_value = *data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_value = *data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

    let bin_width = (max_value - min_value) / bin_count as f64;
    let mut frequencies = vec![0; bin_count];
    for &value in &data {
        let bin_index = ((value - min_value) / bin_width) as usize;
        if bin_index < bin_count {
            frequencies[bin_index] += 1;
        }
    }

    let root = BitMapBackend::new("histograma.png", (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Histograma", ("sans-serif", 25).into_font())
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0..bin_count, 0..*frequencies.iter().max().unwrap())?;

    chart.configure_mesh().draw()?;

    // Desenhar as barras do histograma
    chart.draw_series(
        Histogram::vertical(&chart)
            .style(BLUE.mix(0.5).filled())
            .data(frequencies.iter().enumerate().map(|(i, &f)| (i, f))),
    )?;

    root.present()?;

    println!("Histograma salvo como 'histograma.png'");
    Ok(())
}

fn sputinik(m: f64, s: f64) -> f64 {
    let u: f64 = rand::random::<f64>();
    let t: f64;
    if u < 0.5 {
        t = (-2.0 * u.ln()).sqrt();
    } else {
        t = (-2.0 * (1.0 - u).ln()).sqrt();
    }

    let p = P0 + t * (P1 + t * (P2 + t * (P3 + t * P4)));
    let q = Q0 + t * (Q1 + t * (Q2 + t * (Q3 + t * Q4)));

    let z: f64;
    if u < 0.5 {
        z = (p / q) - t;
    } else {
        z = t - (p / q);
    }
    m + s * z
}
