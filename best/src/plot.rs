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
    pub width: usize,
    pub height: usize,
    pub xlab: String,
    pub ylab: String,
    pub log_x: bool,
    pub log_y: bool,
    pub custom_range_x: bool,
    pub custom_range_y: bool,
    pub range_x: [f64; 2],
    pub range_y: [f64; 2],
    pub title: String,
    pub flnm: String,
    pub show_legend: bool,
    pub legends: Vec<String>,
    pub legend_al: LegendAl,
    pub line_or_points: Vec<LineOrPoints>,
    pub colors: Vec<[u8; 3]>,
    pub dashes: Vec<DashType>,
    pub font_scale: f64,
    pub line_scale: f64,
    pub font_family: String,
}

impl PlotPar{
    pub fn new(width: usize, height: usize, xlab: &str, ylab: &str, title: &str, flnm: &str, legends: Vec<String>) -> PlotPar {
        PlotPar {
            width,
            height,
            xlab: format!("{}", xlab),
            ylab: format!("{}", ylab),
            log_x: false,
            log_y: false,
            custom_range_x: false,
            custom_range_y: false,
            range_x: [0.0; 2],
            range_y: [0.0; 2],
            title: format!("{}", title),
            flnm: format!("{}", flnm),
            show_legend: true,
            legends,
            legend_al: LegendAl::TopRight,
            line_or_points: vec![LineOrPoints::Line; 100],
            colors: COLORS.to_vec(),
            dashes: vec![DashType::Solid; 100],
            font_scale: 1.0,
            line_scale: 1.0,
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
    let thick: usize = (3.0 * plot_par.line_scale) as usize;
    let medium: usize = (5.0 * plot_par.line_scale) as usize;
    let msize: usize = (10.0 * plot_par.line_scale) as usize;
    let fsz_title: usize = (38.0*plot_par.font_scale) as usize;
    let fsz_legend: usize = (36.0*plot_par.font_scale) as usize;
    let fsz_ticks: usize = (32.0*plot_par.font_scale) as usize;
    let fsz_axes: usize = (38.0*plot_par.font_scale) as usize;
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
        .border_width(thick)
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
        .grid_color(gridcol).auto_margin(true);

    let mut axisx = axis.clone().title(
        Title::new(&plot_par.xlab)
            .font(Font::new().size(fsz_axes).color(forecol).family(&plot_par.font_family)));

    let mut axisy = axis
        .clone()
        .title(Title::new(&plot_par.ylab)
            .font(Font::new().size(fsz_axes).color(forecol).family(&plot_par.font_family)));

    if plot_par.log_x {
        axisx = axisx.exponent_format(plotly::common::ExponentFormat::SmallE).type_(plotly::layout::AxisType::Log)
    };

    if plot_par.log_y {
        axisy = axisy.tick_angle(0.0).exponent_format(plotly::common::ExponentFormat::SmallE).type_(plotly::layout::AxisType::Log)
    } else {
        axisy = axisy.tick_angle(270.0)
    };

    if plot_par.custom_range_x {
        axisx = axisx.fixed_range(true).range(plot_par.range_x.to_vec())
    }

    if plot_par.custom_range_y {
        axisy = axisy.fixed_range(true).range(plot_par.range_y.to_vec())
    }

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
        .width(plot_par.width)
        .height(plot_par.height)
        .font(Font::new().size(fsz_ticks))
        .title(title)
        .legend(legend)
        .show_legend(plot_par.show_legend)
        .x_axis(axisx)
        .y_axis(axisy)
        .plot_background_color(transp)
        .paper_background_color(bgcol)
        .margin(Margin::new()
            .left((100.0 * plot_par.font_scale) as usize)
            .bottom((75.0 * plot_par.font_scale) as usize)
            .top((75.0 * plot_par.font_scale) as usize)
        );

    layout.add_shape(line_top);
    layout.add_shape(line_right);

    let mut plot = Plot::new();

    for l in 0..lines_number {
        let trace = traces[l].clone();
        plot.add_trace(trace);
    }
    plot.set_layout(layout);

    //let config = plotly::Configuration::new().static_plot(true);
    //plot.set_configuration(config);

    plot.write_image(&plot_par.flnm, ImageFormat::PDF, plot_par.width, plot_par.height, 1.0);
    plot.write_image(&plot_par.flnm, ImageFormat::PNG, plot_par.width, plot_par.height, 1.0);
    //plot.write_html(&format!("{}.html", plot_par.flnm));
}

pub const COLORS: [[u8; 3]; 48] = [
    [68,119,170], // good blue
    [238,119,51], // orange
    [34,136,51], // green
    [80, 80, 80], // dark gray
    [170,51,119], // purple, cherry
    [0,153,136], // teal
    [204,51,17], // red
    [102,37,6], // brown
    [221,170,51], // good yellow
    [102,204,238], // sky blue
    [51,34,136], // indigo, dark purple
    [204,187,68], // lemon
    [187,187,187], // gray
    [0,68,136], // darker blue
    [153,153,51], // olive
    [136,34,85], // wine
    [0, 0, 0], // black
    [68,119,170], // good blue
    [238,119,51], // orange
    [34,136,51], // green
    [221,170,51], // good yellow
    [204,51,17], // red
    [170,51,119], // purple, cherry
    [0,153,136], // teal
    [102,37,6], // brown
    [102,204,238], // sky blue
    [51,34,136], // indigo, dark purple
    [204,187,68], // lemon
    [187,187,187], // gray
    [0,68,136], // darker blue
    [153,153,51], // olive
    [136,34,85], // wine
    [0, 0, 0], // black
    [68,119,170], // good blue
    [238,119,51], // orange
    [34,136,51], // green
    [221,170,51], // good yellow
    [204,51,17], // red
    [170,51,119], // purple, cherry
    [0,153,136], // teal
    [102,37,6], // brown
    [102,204,238], // sky blue
    [51,34,136], // indigo, dark purple
    [204,187,68], // lemon
    [187,187,187], // gray
    [0,68,136], // darker blue
    [153,153,51], // olive
    [136,34,85], // wine
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
