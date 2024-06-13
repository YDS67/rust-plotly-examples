use plotly::color::{NamedColor, Rgb};
use plotly::common::{Anchor, DashType, Font, Line, Marker, MarkerSymbol, Mode, Title};
use plotly::layout::{Axis, Legend, Shape, ShapeLine, ShapeType, ItemSizing, Margin};
use plotly::{ImageFormat, Layout, Plot, Scatter};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LegendAl{
    BottomRight,
    TopRight,
    BottomLeft,
    TopLeft,
    CenterRight,
    CenterLeft,
    BottomCenter,
    TopCenter,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineOrPoints{
    Line,
    Points,
    LineAndPoints,
}

pub struct PlotPar{
    pub xlab: String,
    pub ylab: String,
    pub title: String,
    pub flnm: String,
    pub legends: Vec<String>,
    pub legend_al: LegendAl,
    pub line_or_points: Vec<LineOrPoints>,
    pub colors: Vec<[u8; 3]>,
    pub dashes: Vec<DashType>,
    pub font_scale: f64,
    pub font_family: String,
}

impl PlotPar{
    pub fn new(xlab: &str, ylab: &str, title: &str, flnm: &str, legends: Vec<String>) -> PlotPar {
        PlotPar {
            xlab: format!("{}", xlab),
            ylab: format!("{}", ylab),
            title: format!("{}", title),
            flnm: format!("{}", flnm),
            legends,
            legend_al: LegendAl::TopRight,
            line_or_points: vec![LineOrPoints::Line; 100],
            colors: COLORS.to_vec(),
            dashes: vec![DashType::Solid; 100],
            font_scale: 2.0,
            font_family: format!("Serif"),
        }
    }
}

pub fn line_plot(x: &Vec<Vec<f64>>, y: &Vec<Vec<f64>>, plot_par: &PlotPar) {
    let lines_number = x.len();
    let bgcol = Rgb::new(255, 255, 255);
    let forecol = Rgb::new(0, 0, 0);
    let gridcol = Rgb::new(220, 220, 220);
    let transp = NamedColor::Transparent;
    let thick: usize = 3;
    let medium: usize = 4;
    let _thin: usize = 2;
    let msize: usize = 10;
    let fsz_title: usize = (19.0*plot_par.font_scale) as usize;
    let fsz_legend: usize = (17.0*plot_par.font_scale) as usize;
    let fsz_ticks: usize = (16.0*plot_par.font_scale) as usize;
    let fsz_axes: usize = (17.0*plot_par.font_scale) as usize;
    let dashes: Vec<DashType> = plot_par.dashes.clone();

    let mut traces = Vec::new();

    for l in 0..lines_number {
        match plot_par.line_or_points[l] {
            LineOrPoints::Line => {
                traces.push(
                    Scatter::new(x[l].clone(), y[l].clone())
                        .name(&plot_par.legends[l])
                        .mode(Mode::Lines)
                        .line(Line::new()
                            .color(Rgb::new(plot_par.colors[l][0], plot_par.colors[l][1], plot_par.colors[l][2]))
                            .width(medium as f64).dash(dashes[l].clone())
                        ),
                )
            },
            LineOrPoints::Points => {
                traces.push(
                    Scatter::new(x[l].clone(), y[l].clone())
                        .name(&plot_par.legends[l])
                        .mode(Mode::Markers)
                        .marker(Marker::new().size(msize)
                        .color(Rgb::new(plot_par.colors[l][0], plot_par.colors[l][1], plot_par.colors[l][2]))
                        .symbol(MarkerSymbol::Circle)
                    ),
                )
            },
            LineOrPoints::LineAndPoints => {
                traces.push(
                    Scatter::new(x[l].clone(), y[l].clone())
                        .name(&plot_par.legends[l])
                        .mode(Mode::LinesMarkers)
                        .line(Line::new()
                            .color(Rgb::new(plot_par.colors[l][0], plot_par.colors[l][1], plot_par.colors[l][2]))
                            .width(medium as f64).dash(dashes[l].clone())
                        )
                        .marker(Marker::new().size(msize).symbol(MarkerSymbol::Circle)),
                )
            },
        }
    }

    let title = Title::new(&plot_par.title)
        .font(Font::new().size(fsz_title).family(&plot_par.font_family).color(forecol));

    let legend_bottom_right = Legend::new()
        .x(0.99)
        .x_anchor(Anchor::Right)
        .y(0.01)
        .y_anchor(Anchor::Bottom);

    let legend_top_right = Legend::new()
        .x(0.99)
        .x_anchor(Anchor::Right)
        .y(0.99)
        .y_anchor(Anchor::Top);

    let legend_bottom_left = Legend::new()
        .x(0.01)
        .x_anchor(Anchor::Left)
        .y(0.01)
        .y_anchor(Anchor::Bottom);

    let legend_top_left = Legend::new()
        .x(0.01)
        .x_anchor(Anchor::Left)
        .y(0.99)
        .y_anchor(Anchor::Top);

    let legend_top_center = Legend::new()
        .x(0.5)
        .x_anchor(Anchor::Center)
        .y(0.99)
        .y_anchor(Anchor::Top);

    let legend_bottom_center = Legend::new()
        .x(0.5)
        .x_anchor(Anchor::Center)
        .y(0.01)
        .y_anchor(Anchor::Bottom);

    let legend_center_right = Legend::new()
        .x(1.01)
        .x_anchor(Anchor::Left)
        .y(0.5)
        .y_anchor(Anchor::Center);

    let legend_center_left = Legend::new()
        .x(0.01)
        .x_anchor(Anchor::Left)
        .y(0.5)
        .y_anchor(Anchor::Center);

    let legend = match plot_par.legend_al {
        LegendAl::BottomLeft => legend_bottom_left,
        LegendAl::BottomRight => legend_bottom_right,
        LegendAl::TopLeft => legend_top_left,
        LegendAl::TopRight => legend_top_right,
        LegendAl::BottomCenter => legend_bottom_center,
        LegendAl::CenterLeft => legend_center_left,
        LegendAl::CenterRight => legend_center_right,
        LegendAl::TopCenter => legend_top_center,
    }.font(Font::new().size(fsz_legend).color(forecol).family(&plot_par.font_family))
        .border_width(medium)
        .border_color(forecol)
        .background_color(bgcol)
        .item_width(52)
        .item_sizing(ItemSizing::Trace);

    let axis = Axis::new()
        .position(0.0)
        .show_line(true)
        .line_color(forecol)
        .line_width(thick)
        .tick_length(9)
        .tick_width(medium)
        .tick_color(forecol)
        .tick_font(Font::new().color(forecol))
        .zero_line(false)
        .show_grid(true)
        .grid_color(gridcol);

    let axisx = axis.clone().title(
        Title::new(&plot_par.xlab)
            .font(Font::new().size(fsz_axes).color(forecol).family(&plot_par.font_family)));

    let axisy = axis
        .clone()
        .title(Title::new(&plot_par.ylab)
            .font(Font::new().size(fsz_axes).color(forecol).family(&plot_par.font_family)))
        .tick_angle(270.0);

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
        .show_legend(true)
        .x_axis(axisx)
        .y_axis(axisy)
        .plot_background_color(transp)
        .paper_background_color(bgcol)
        .margin(Margin::new().left(105).bottom(105));

    layout.add_shape(line_top);
    layout.add_shape(line_right);

    let mut plot = Plot::new();

    for l in 0..lines_number {
        let trace = traces[l].clone();
        plot.add_trace(trace);
    }
    plot.set_layout(layout);

    // let config = plotly::Configuration::new().typeset_math(true);
    // plot.set_configuration(config);

    plot.write_image(&plot_par.flnm, ImageFormat::PDF, 1280, 960, 1.0);
    //plot.write_html(&format!("{}.html",plot_par.flnm));
    //plot.write_image(&plot_par.flnm, ImageFormat::PNG, 1280, 960, 1.0);
}

pub const COLORS: [[u8; 3]; 45] = [
    [0, 0, 0],
    [68,119,170],
    [238,119,51],
    [34,136,51],
    [170,51,119],
    [204,187,68],
    [0,153,136],
    [102,204,238],
    [51,34,136],
    [221,170,51],
    [204,51,17],
    [187,187,187],
    [0,68,136],
    [153,153,51],
    [136,34,85],
    [0, 0, 0],
    [68,119,170],
    [238,119,51],
    [34,136,51],
    [170,51,119],
    [204,187,68],
    [0,153,136],
    [102,204,238],
    [51,34,136],
    [221,170,51],
    [204,51,17],
    [187,187,187],
    [0,68,136],
    [153,153,51],
    [136,34,85],
    [0, 0, 0],
    [68,119,170],
    [238,119,51],
    [34,136,51],
    [170,51,119],
    [204,187,68],
    [0,153,136],
    [102,204,238],
    [51,34,136],
    [221,170,51],
    [204,51,17],
    [187,187,187],
    [0,68,136],
    [153,153,51],
    [136,34,85],
];

pub const DASHTYPES: [DashType; 36] = [
    DashType::Solid,
    DashType::Dash,
    DashType::Dot,
    DashType::DashDot,
    DashType::LongDash,
    DashType::LongDashDot,
    DashType::Solid,
    DashType::Dash,
    DashType::Dot,
    DashType::DashDot,
    DashType::LongDash,
    DashType::LongDashDot,
    DashType::Solid,
    DashType::Dash,
    DashType::Dot,
    DashType::DashDot,
    DashType::LongDash,
    DashType::LongDashDot,
    DashType::Solid,
    DashType::Dash,
    DashType::Dot,
    DashType::DashDot,
    DashType::LongDash,
    DashType::LongDashDot,
    DashType::Solid,
    DashType::Dash,
    DashType::Dot,
    DashType::DashDot,
    DashType::LongDash,
    DashType::LongDashDot,
    DashType::Solid,
    DashType::Dash,
    DashType::Dot,
    DashType::DashDot,
    DashType::LongDash,
    DashType::LongDashDot,
];