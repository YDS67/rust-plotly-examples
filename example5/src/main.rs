extern crate plotly;
use plotly::color::{NamedColor, Rgb};
use plotly::common::{Anchor, Font, Line, Marker, MarkerSymbol, Mode, Title};
use plotly::layout::{Axis, Legend, Shape, ShapeLine, ShapeType, ItemSizing, Margin};
use plotly::{ImageFormat, Layout, Plot, Scatter};

fn line_and_scatter_plot(x1: Vec<f64>, y1: Vec<f64>, x2: Vec<f64>, y2: Vec<f64>, flnm: &str, title: &str) {
    let bgcol = Rgb::new(255, 255, 255);
    let linecol1 = NamedColor::DarkBlue;
    let linecol2 = NamedColor::DarkRed;
    let forecol = Rgb::new(0, 0, 0);
    let gridcol = Rgb::new(180, 180, 180);
    let transp = NamedColor::Transparent;
    let thick: usize = 3;
    let medium: usize = 3;
    let _thin: usize = 2;
    let msize: usize = 10;
    let fsz_title: usize = 35;
    let fsz_legend: usize = 35;
    let fsz_ticks: usize = 30;
    let fsz_axes: usize = 35;

    let trace1 = Scatter::new(x1, y1)
        .name("Diffusion")
        .mode(Mode::Lines)
        .line(Line::new().color(linecol1).width(medium as f64)
        //.marker(Marker::new().size(msize).symbol(MarkerSymbol::Circle),
    );
        
    let trace2 = Scatter::new(x2, y2)
        .name("Random walk")
        .mode(Mode::Markers)
        //.line(Line::new().color(linecol2).width(medium as f64))
        .marker(Marker::new().size(msize).color(linecol2).symbol(MarkerSymbol::Circle)
    );

    let title = Title::new(title)
        .font(Font::new().size(fsz_title).family("Serif").color(forecol));

    let legend = Legend::new()
        .x(0.99)
        .x_anchor(Anchor::Right)
        .y(1.0 - 0.0133)
        .y_anchor(Anchor::Top)
        .font(Font::new().size(fsz_legend).color(forecol).family("Serif"))
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
        Title::new("x. a.u.")
            .font(Font::new().size(fsz_axes).color(forecol).family("Serif")));

    let axisy = axis
        .clone()
        .title(Title::new("Number of particles")
            .font(Font::new().size(fsz_axes).color(forecol).family("Serif")))
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
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.set_layout(layout);

    //plot.write_html(flnm);
    //plot.write_image(flnm, ImageFormat::SVG, 1280, 960, 1.0);
    plot.write_image(flnm, ImageFormat::PNG, 1280, 960, 1.0);
}

fn read_from_file(file_path: &str) -> Vec<Vec<f64>> {
    use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;

    let file = File::open(file_path).expect("Error opening file");
    let reader = BufReader::new(file);
    let mut file_contents: Vec<String> = Vec::new();

    for line in reader.lines() {
        file_contents.push(line.unwrap());
    }

    let mut data_rows: Vec<Vec<f64>> = Vec::new();
    let mut data_columns: Vec<Vec<f64>> = Vec::new();

    let mut n_columns: Vec<usize> = Vec::new();
    let mut n_rows: usize = 0;

    for line in file_contents {
        let row: Vec<f64> = line
        .split(|c| c == ' ' || c == '\t'|| c == ',')
        .map(|s| s.trim()) 
        .filter(|s| !s.is_empty()) 
        .map(|s| s.parse().unwrap()) 
        .collect();

        n_columns.push(row.len());
        n_rows = n_rows+1;
        
        data_rows.push(row)
    };

    let n1 = n_columns[1];

    for _i in 0..n1 {
        let column: Vec<f64> = Vec::new();
        data_columns.push(column)
    }

    for j in 0..n_rows {
        for i in 0..n_columns[j] {
            data_columns[i].push(data_rows[j][i])
        }
    }

    data_columns

}

fn main() {
 
    let data = read_from_file("data_file.dat");
    let x1 = data[0].clone();
    let y1 = data[1].clone();
    let x2 = data[3].clone();
    let y2 = data[4].clone();

    line_and_scatter_plot(x1, y1, x2, y2, "Data_plot", "Diffusion / random walk");
}