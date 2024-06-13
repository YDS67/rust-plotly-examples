mod file;
mod plot;

fn main() {
    let n: usize = 50;
    let x_min = 0.0;
    let x_max = 1.5;
    let dx = (x_max-x_min)/(n as f64);
    let x: Vec<f64> = (0..n).map(|n| n as f64 * dx + x_min).collect();
    let y1: Vec<f64> = x.clone().into_iter().map(|x| (x*x).cos()).collect();
    let y2: Vec<f64> = x.clone().into_iter().map(|x| 1.0-x.powi(4)/2.0).collect();
    let y3: Vec<f64> = x.clone().into_iter().map(|x| 1.0-x.powi(4)/2.0+x.powi(8)/24.0).collect();
    let y4: Vec<f64> = x.clone().into_iter().map(|x| 1.0-x.powi(4)/2.0+x.powi(8)/24.0-x.powi(12)/720.0).collect();

    let flnm = format!("Taylor_cos2");
    let title = format!(r"cos(x<sup>2</sup>)");
    //set parameters
    let mut plot_par = plot::PlotPar::new(
        "x, arbitrary units", 
        "f(x)", 
        &title, 
        &flnm,
        vec![
            format!("f(x)"), 
            format!("T<sub>2</sup>(x)"),
            format!("T<sub>3</sup>(x)"),
            format!("T<sub>4</sup>(x)"),
        ],
    );
    // change legend alignments or plotting mode
    use plot::{LegendAl, LineOrPoints};
    plot_par.legend_al = LegendAl::BottomLeft;
    plot_par.line_or_points = vec![LineOrPoints::Line, LineOrPoints::LineAndPoints, LineOrPoints::Points, LineOrPoints::Points];
    //plot
    plot::line_plot(&vec![x.clone(); 4], &vec![y1.clone(), y2.clone(), y3.clone(), y4.clone()], &plot_par);
    file::save_columns_to_file(&vec![x, y1, y2, y3, y4], "results", "taylor.dat");

}
