extern crate plotly;
use plotly::color::{NamedColor, Rgba};
use plotly::common::{Anchor, Font, Mode, Title};
use plotly::layout::{Axis, Legend};
use plotly::{color::Rgb, ImageFormat, Layout, Plot, Scatter};

fn line_and_scatter_plot(x: Vec<f64>, y1: Vec<f64>, y2: Vec<f64>) {
    let trace1 = Scatter::new(x.clone(), y1)
        .name("sin(x)")
        .mode(Mode::LinesMarkers);
    let trace2 = Scatter::new(x, y2).name("cos(x)").mode(Mode::LinesMarkers);

    let title = Title::new("Trigonometric functions").font(
        Font::new()
            .size(35)
            .family("Serif")
            .color(Rgb::new(155, 57, 4)),
    );

    let legend = Legend::new()
        .x(0.01)
        .x_anchor(Anchor::Left)
        .y(0.0133)
        .y_anchor(Anchor::Bottom)
        .font(Font::new().size(30).color(NamedColor::Black))
        .border_width(2)
        .border_color(NamedColor::DarkBlue);

    let axis = Axis::new()
        .position(0.0)
        .show_line(true)
        .line_color(NamedColor::DarkBlue)
        .line_width(4)
        .tick_length(6)
        .tick_width(3)
        .tick_color(NamedColor::DarkBlue)
        .tick_font(Font::new().color(NamedColor::Black))
        .zero_line(false);

    let layout = Layout::new()
        .width(1024)
        .height(768)
        .font(Font::new().size(24))
        .title(title)
        .legend(legend)
        .x_axis(axis.clone())
        .y_axis(axis)
        .plot_background_color(Rgba::new(0, 0, 155, 0.1));

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.set_layout(layout);

    //plot.write_html("./line_and_scatter_plot.html");
    plot.write_image("./line_and_scatter_plot", ImageFormat::SVG, 1024, 768, 1.0);
    plot.write_image("./line_and_scatter_plot", ImageFormat::PNG, 1024, 768, 1.0);
}

fn main() -> std::io::Result<()> {
    let n = 100;
    let mut x = vec![0.0; n];
    let mut y1 = vec![0.0; n];
    let mut y2 = vec![0.0; n];

    for i in 0..n {
        x[i] = i as f64 / 10.0;
        y1[i] = x[i].sin();
        y2[i] = x[i].cos();
    }

    line_and_scatter_plot(x, y1, y2);
    Ok(())
}
