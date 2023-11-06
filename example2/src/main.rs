extern crate plotly;
use plotly::color::{NamedColor, Rgb};
use plotly::common::{Anchor, Font, Line, Marker, MarkerSymbol, Mode, Title};
use plotly::layout::{Axis, Legend, Shape, ShapeLine, ShapeType};
use plotly::{ImageFormat, Layout, Plot, Scatter};

fn line_and_scatter_plot(x: Vec<f64>, y1: Vec<f64>, y2: Vec<f64>) {
    let bgcol = Rgb::new(40, 40, 40);
    let linecol1 = Rgb::new(0, 146, 204);
    let linecol2 = Rgb::new(255, 51, 51);
    let forecol = Rgb::new(240, 240, 240);
    let gridcol = Rgb::new(120, 120, 120);
    let transp = NamedColor::Transparent;
    let thick: usize = 3;
    let medium: usize = 3;
    let _thin: usize = 2;
    let msize: usize = 10;
    let fsz_title: usize = 35;
    let fsz_legend: usize = 35;
    let fsz_ticks: usize = 30;
    let fsz_axes: usize = 35;

    let trace1 = Scatter::new(x.clone(), y1)
        .name("sin(x)  ")
        .mode(Mode::LinesMarkers)
        .line(Line::new().color(linecol1).width(medium as f64))
        .marker(Marker::new().size(msize).symbol(MarkerSymbol::Circle));
    let trace2 = Scatter::new(x, y2)
        .name("cos(x)  ")
        .mode(Mode::LinesMarkers)
        .line(Line::new().color(linecol2).width(medium as f64))
        .marker(Marker::new().size(msize).symbol(MarkerSymbol::DiamondDot));

    let title = Title::new("Trigonometric functions")
        .font(Font::new().size(fsz_title).family("Serif").color(forecol));

    let legend = Legend::new()
        .x(0.01)
        .x_anchor(Anchor::Left)
        .y(0.0133)
        .y_anchor(Anchor::Bottom)
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

    let axisx = axis.clone().title(Title::new("x, radians")
        .font(Font::new().size(fsz_axes).color(forecol).family("Serif")));
    let axisy = axis.clone().title(Title::new("f(x)")
        .font(Font::new().size(fsz_axes).color(forecol).family("Serif")));

    let line_top = Shape::new()
        .shape_type(ShapeType::Line)
        .x_ref("paper")
        .y_ref("paper")
        .x0(0.)
        .y0(1.)
        .x1(1.)
        .y1(1.)
        .line(ShapeLine::new()
        .color(forecol)
        .width(thick as f64));

    let line_right = Shape::new()
        .shape_type(ShapeType::Line)
        .x_ref("paper")
        .y_ref("paper")
        .x0(1.)
        .y0(0.)
        .x1(1.)
        .y1(1.)
        .line(ShapeLine::new()
        .color(forecol)
        .width(thick as f64));

    let mut layout = Layout::new()
        .width(1024)
        .height(768)
        .font(Font::new().size(fsz_ticks))
        .title(title)
        .legend(legend)
        .x_axis(axisx)
        .y_axis(axisy)
        .plot_background_color(transp)
        .paper_background_color(bgcol);

    layout.add_shape(line_top);
    layout.add_shape(line_right);

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.set_layout(layout);

    plot.write_html("./line_and_scatter_plot.html");
    plot.write_image("./line_and_scatter_plot", ImageFormat::SVG, 1024, 768, 1.0);
    plot.write_image("./line_and_scatter_plot", ImageFormat::PNG, 1024, 768, 1.0);
}

fn main() -> std::io::Result<()> {
    let n = 50;
    let xmin = 0.0;
    let xmax = 10.0;
    let mut x = vec![0.0; n];
    let mut y1 = vec![0.0; n];
    let mut y2 = vec![0.0; n];
    let mut s: f64;

    for i in 0..n {
        s = i as f64 / n as f64;
        x[i] = xmin + s * (xmax - xmin);
        y1[i] = x[i].sin();
        y2[i] = x[i].cos();
    }

    line_and_scatter_plot(x, y1, y2);
    Ok(())
}
