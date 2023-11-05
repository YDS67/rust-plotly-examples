extern crate plotly;
use plotly::color::{NamedColor, Rgb};
use plotly::common::{Anchor, Font, Line, Marker, MarkerSymbol, Mode, Title};
use plotly::layout::{Axis, Legend, Shape, ShapeLine, ShapeType, Margin};
use plotly::{ImageFormat, Layout, Plot, Scatter};
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn line_and_scatter_plot(x: Vec<f64>, y: Vec<f64>, flnm: &str, ylab: &str, legend: &str, col: NamedColor) {
    let bgcol = Rgb::new(255, 255, 255);
    let linecol1 = col;
    let forecol = Rgb::new(0, 0, 0);
    let gridcol = Rgb::new(120, 120, 120);
    let transp = NamedColor::Transparent;
    let thick: usize = 4;
    let medium: usize = 3;
    let _thin: usize = 2;
    let msize: usize = 10;
    let fsz_title: usize = 35;
    let fsz_legend: usize = 35;
    let fsz_ticks: usize = 30;
    let fsz_axes: usize = 35;

    let trace1 = Scatter::new(x.clone(), y)
        .name(legend)
        .mode(Mode::LinesMarkers)
        .line(Line::new().color(linecol1).width(medium as f64))
        .marker(Marker::new().size(msize).symbol(MarkerSymbol::Circle));

    let title = Title::new("Electrons vs Quantum dots")
        .font(Font::new().size(fsz_title).family("Serif").color(forecol));

    let legend = Legend::new()
        .x(0.01)
        .x_anchor(Anchor::Left)
        .y(1.0 - 0.0133)
        .y_anchor(Anchor::Top)
        .font(Font::new().size(fsz_legend).color(forecol).family("Serif"))
        .border_width(medium)
        .border_color(forecol)
        .background_color(bgcol)
        .item_width(52);

    let axis = Axis::new()
        .position(0.0)
        .show_line(true)
        .line_color(forecol)
        .line_width(thick)
        .tick_length(6)
        .tick_width(medium)
        .tick_color(forecol)
        .tick_font(Font::new().color(forecol))
        .zero_line(false)
        .show_grid(true)
        .grid_color(gridcol);

    let axisx = axis.clone().title(
        Title::new("Electric field, kV/cm")
            .font(Font::new().size(fsz_axes).color(forecol).family("Serif")));

    let axisy = axis
        .clone()
        .title(Title::new(ylab)
            .font(Font::new().size(fsz_axes).color(forecol).family("Serif")));

    let line_top = Shape::new()
        .shape_type(ShapeType::Line)
        .x_ref("paper")
        .y_ref("paper")
        .x0(0.)
        .y0(1.)
        .x1(1.)
        .y1(1.)
        .line(ShapeLine::new().color(forecol).width(thick as f64));

    let line_right = Shape::new()
        .shape_type(ShapeType::Line)
        .x_ref("paper")
        .y_ref("paper")
        .x0(1.)
        .y0(0.)
        .x1(1.)
        .y1(1.)
        .line(ShapeLine::new().color(forecol).width(thick as f64));

    let mut layout = Layout::new()
        .width(1024)
        .height(768)
        .font(Font::new().size(fsz_ticks))
        .title(title)
        .legend(legend)
        .x_axis(axisx)
        .y_axis(axisy)
        .plot_background_color(transp)
        .paper_background_color(bgcol)
        .margin(Margin::new().left(100).bottom(100));

    layout.add_shape(line_top);
    layout.add_shape(line_right);

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.set_layout(layout);

    //plot.write_html(flnm);
    //plot.write_image(flnm, ImageFormat::SVG, 1024, 768, 1.0);
    plot.write_image(flnm, ImageFormat::PNG, 1024, 768, 1.0);
}

fn main() -> std::io::Result<()> {
    let file = File::open("IV_curve.dat").expect("Error opening file");
    let reader = BufReader::new(file);

    let mut s: Vec<String> = Vec::new();

    for line in reader.lines() {
        s.push(line.unwrap());
    }

    let length = s.len();
    let mut x = vec![0.0; length];
    let mut y1 = vec![0.0; length];
    let mut y2 = vec![0.0; length];

    for i in 0..length {
        let res: Vec<f64> = s[i]
            .split(' ')
            .map(|s| s.trim()) 
            .filter(|s| !s.is_empty()) 
            .map(|s| s.parse().unwrap()) 
            .collect();
        x[i] = 1e4 * res[0];
        y1[i] = res[1];
        y2[i] = res[2];
    }

    line_and_scatter_plot(x.clone(), y1, "Velocity vs field", "Velocity, nm/fs", " <v>  ", NamedColor::DarkBlue);
    line_and_scatter_plot(x, y2, "Energy vs field", "Energy, eV", " <E>  ", NamedColor::DarkRed);
    Ok(())
}
