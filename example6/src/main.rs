mod file;
mod plot;

fn main() {
    let n: usize = 1000;
    let x_min = 0.0;
    let x_max = 4.0;
    let dx = (x_max-x_min)/(n as f64);
    let x: Vec<f64> = (0..n).map(|n| n as f64 * dx + x_min).collect();
    let y1: Vec<f64> = x.clone().into_iter().map(|x| x.cos()).collect();
    let y2: Vec<f64> = x.clone().into_iter().map(|x| 1.0-x.powi(2)/2.0).collect();
    let y3: Vec<f64> = x.clone().into_iter().map(|x| 1.0-x.powi(2)/2.0+x.powi(4)/24.0).collect();
    let y4: Vec<f64> = x.clone().into_iter().map(|x| 1.0-x.powi(2)/2.0+x.powi(4)/24.0-x.powi(6)/720.0).collect();

    let flnm = format!("Taylor_cos.r");
    let title = format!("Taylor series approximations");
    let plot_par = plot::PlotPar::new(
        "x, arbitrary units", 
        "f(x)", 
        &title, 
        &flnm,
        vec![
            format!("cos(x)"), 
            format!("order 2"),
            format!("order 3"),
            format!("order 4"),
        ],
    );
    plot::line_plot(&vec![x.clone(); 4], &vec![y1.clone(), y2.clone(), y3.clone(), y4.clone()], &plot_par);
    file::save_columns_to_file(&vec![x, y1, y2, y3, y4], "results", "taylor.dat");
}
