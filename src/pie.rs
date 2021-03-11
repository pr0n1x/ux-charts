#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};
use ux_dataflow::*;
use ux_primitives::{canvas::CanvasContext, color::Color, geom::Point, text::TextAlign};

use crate::*;

/// A pie in a pie chart.
#[derive(Default, Clone)]
pub struct PieEntity {
    // Chart chart,
    color: Color,
    highlight_color: Color,
    // formatted_value: String,
    index: usize,
    old_value: f64,
    value: f64,

    old_start_angle: f64,
    old_end_angle: f64,
    start_angle: f64,
    end_angle: f64,

    center: Point<f64>,
    inner_radius: f64,
    outer_radius: f64,

    // [Series] field.
    name: String,
}

impl PieEntity {
    pub fn is_empty(&self) -> bool {
        self.start_angle == self.end_angle
    }

    fn contains_point(&self, p: Point<f64>) -> bool {
        // p -= center;
        let mag = p.distance_to(Point::default()); //p.magnitude();
        if mag > self.outer_radius || mag < self.inner_radius {
            return false;
        }

        let mut angle = f64::atan2(p.y, p.x);

        //TODO: complete it
        // let chartStartAngle = (chart as dynamic).start_angle;

        // // Make sure [angle] is in range [chartStartAngle]..[chartStartAngle] + TAU.
        // angle = (angle - chartStartAngle) % TAU + chartStartAngle;

        // If counterclockwise, make sure [angle] is in range
        // [start] - 2*pi..[start].
        if self.start_angle > self.end_angle {
            angle -= TAU;
        }

        if self.start_angle <= self.end_angle {
            // Clockwise.
            is_in_range(angle, self.start_angle, self.end_angle)
        } else {
            // Counterclockwise.
            is_in_range(angle, self.end_angle, self.start_angle)
        }
    }
}

impl Entity for PieEntity {
    fn free(&mut self) {
        // chart = null;
    }

    fn save(&self) {
        // self.old_start_angle = self.start_angle;
        // self.old_end_angle = self.end_angle;
        // self.old_value = self.value;
    }
}

impl<C> Drawable<C> for PieEntity
where
    C: CanvasContext,
{
    fn draw(&self, ctx: &C, percent: f64, highlight: bool) {
        let mut a1 = lerp(self.old_start_angle, self.start_angle, percent);
        let mut a2 = lerp(self.old_end_angle, self.end_angle, percent);
        if a1 > a2 {
            let tmp = a1;
            a1 = a2;
            a2 = tmp;
        }
        let center = &self.center;
        if highlight {
            let highlight_outer_radius = HIGHLIGHT_OUTER_RADIUS_FACTOR * self.outer_radius;
            ctx.set_fill_style_color(self.highlight_color);
            ctx.begin_path();
            ctx.arc(center.x, center.y, highlight_outer_radius, a1, a2, false);
            ctx.arc(center.x, center.y, self.inner_radius, a2, a1, true);
            ctx.fill();
        }

        ctx.set_fill_style_color(self.color);
        ctx.begin_path();
        ctx.arc(center.x, center.y, self.outer_radius, a1, a2, false);
        ctx.arc(center.x, center.y, self.inner_radius, a2, a1, true);
        ctx.fill();
        ctx.stroke();

        // && chart is PieChart
        // if !self.formatted_value.is_empty() && a2 - a1 > PI / 36.0 {
        //     // let labels = self.chart.options.series.labels;
        //     // if labels.enabled {
        //     //     let r = 0.25 * self.inner_radius + 0.75 * self.outer_radius;
        //     //     let a = 0.5 * (a1 + a2);
        //     //     let p = utils::polar2cartesian(center, r, a);
        //     //     ctx.set_fill_style_color(labels.style.color);
        //     //     ctx.fill_text(self.formatted_value.as_str(), p.x, p.y);
        //     // }
        // }
    }
}

#[derive(Default, Clone)]
struct PieChartProperties {
    center: Point<f64>,
    outer_radius: f64,
    inner_radius: f64,

    /// The start angle in radians.
    start_angle: f64,

    /// 1 means clockwise and -1 means counterclockwise.
    direction: i64,
}

pub struct PieChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display,
{
    props: RefCell<PieChartProperties>,
    base: BaseChart<'a, C, PieEntity, M, D, PieChartOptions<'a>>,
}

impl<'a, C, M, D> PieChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display,
{
    pub fn new(options: PieChartOptions<'a>) -> Self {
        Self {
            props: Default::default(),
            base: BaseChart::new(options),
        }
    }

    fn data_rows_changed(&self, record: DataCollectionChangeRecord) {
        self.base
            .update_series_visible(record.index, record.removed_count, record.added_count);
        self.base.data_rows_changed(record);
        self.base.update_legend_content();
    }

    fn get_entity_group_index(&self, x: f64, y: f64) -> i64 {
        let p = Point::new(x, y);
        // let entities = series_list.first.entities;
        // for (let i = entities.len() - 1; i >= 0; i--) {
        //   let pie = entities[i] as Pie;
        //   if (pie.containsPoint(p)) return i;
        // }
        // return -1;
        unimplemented!()
    }

    pub fn get_legend_labels(&self) -> Vec<String> {
        //self.data_table.getColumnValues<String>(0)
        unimplemented!()
    }

    fn series_visibility_changed(&self, index: usize) {
        self.update_series(0);
    }

    fn update_tooltip_content(&self) {
        // let pie = series_list[0].entities[focused_entity_index] as Pie;
        // tooltip.style
        //   ..borderColor = pie.color
        //   ..padding = "4px 12px";
        // let label = tooltip_label_formatter(pie.name);
        // let value = tooltip_value_formatter(pie.value);
        // tooltip.innerHtml = "$label: <strong>$value</strong>";
        unimplemented!()
    }
}

impl<'a, C, M, D> Chart<'a, C, M, D, PieEntity> for PieChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display,
{
    fn calculate_drawing_sizes(&self) {
        self.base.calculate_drawing_sizes();
        let rect = &self.base.props.borrow().series_and_axes_box;
        let half_w = rect.size.width as i64 >> 1;
        let half_h = rect.size.height as i64 >> 1;

        // self.center = Point {
        //     x: (rect.left + half_w) as f64,
        //     y: (rect.top + half_h) as f64,
        // };

        // self.outer_radius = (half_w.min(half_h) as f64) / HIGHLIGHT_OUTER_RADIUS_FACTOR;
        let mut pie_hole = self.base.options.pie_hole;

        if pie_hole > 1.0 {
            pie_hole = 0.0;
        }

        if pie_hole < 0.0 {
            pie_hole = 0.0;
        }

        // self.inner_radius = pie_hole * self.outer_radius;

        let opt = &self.base.options.series;

        // FIXME: complete
        // self.base.entity_value_formatter =
        //     opt.labels.formatter ?? default_value_formatter;

        // self.direction = if opt.counterclockwise {
        //     COUNTERCLOCKWISE
        // } else {
        //     CLOCKWISE
        // };

        // self.start_angle = deg2rad(opt.start_angle);
    }

    fn set_stream(&self, stream: DataStream<'a, M, D>) {}

    fn draw(&self, ctx: &C) {
        self.base.dispose();
        // data_tableSubscriptionTracker
        //   ..add(dataTable.onCellChange.listen(data_cell_changed))
        //   ..add(dataTable.onColumnsChange.listen(dataColumnsChanged))
        //   ..add(dataTable.onRowsChange.listen(data_rows_changed));
        // self.easing_function = get_easing(self.options.animation().easing);
        self.base.initialize_legend();
        self.base.initialize_tooltip();

        // self.axes_context.clearRect(0, 0, self.width, self.height);
        self.draw_axes_and_grid(ctx);
        self.base.start_animation();
    }

    fn update(&self, ctx: &C) {
        self.base.update(ctx);
    }

    fn resize(&self, w: f64, h: f64) {
        self.base.resize(w, h);
    }

    /// Draws the axes and the grid.
    ///
    fn draw_axes_and_grid(&self, ctx: &C) {
        self.base.draw_axes_and_grid(ctx);
    }

    /// Draws the current animation frame.
    ///
    /// If [time] is `null`, draws the last frame (i.e. no animation).
    fn draw_frame(&self, ctx: &C, time: Option<i64>) {
        self.base.draw_frame(ctx, time);

        let mut percent = self.base.calculate_percent(time);

        if percent >= 1.0 {
            percent = 1.0;

            // Update the visibility states of all series before the last frame.
            // for (let i = series_states.len() - 1; i >= 0; i--) {
            //     if (series_states[i] == Visibility::showing) {
            //         series_states[i] = Visibility::shown;
            //     } else if (series_states[i] == Visibility::hiding) {
            //         series_states[i] = Visibility::hidden;
            //     }
            // }
        }

        let props = self.base.props.borrow();

        let ease = props.easing_function.unwrap();
        self.draw_series(ctx, ease(percent));
        // context.drawImageScaled(axes_context.canvas, 0, 0, width, height);
        // context.drawImageScaled(series_context.canvas, 0, 0, width, height);
        self.base.draw_title(ctx);

        if percent < 1.0 {
            // animation_frame_id = window.requestAnimationFrame(draw_frame);
        } else if time.is_some() {
            self.base.animation_end();
        }
    }

    fn draw_series(&self, ctx: &C, percent: f64) -> bool {
        // series_context
        //   ..lineWidth = 2
        //   ..strokeStyle = "#fff"
        //   ..textAlign = TextAlign::Center
        //   ..textBaseline = BaseLine::Middle;
        // let pies = series_list.first.entities;
        // let labelOptions = self.base.options.series.labels;
        // series_context.font = get_font(labelOptions["style"]);
        // for (Pie pie in pies) {
        //   if (pie.isEmpty && percent == 1.0) continue;
        //   let highlight =
        //       pie.index == focused_series_index || pie.index == focused_entity_index;
        //   pie.draw(series_context, percent, highlight);
        // }

        // return false;
        unimplemented!()
    }

    fn update_series(&self, index: usize) {
        // let sum = 0.0;
        // let start_angle = start_angle;
        // let pieCount = self.base.data_table.rows.len();
        // let entities = series_list[0].entities;

        // // Sum the values of all visible pies.
        // for (let i = 0; i < pieCount; i++) {
        //   if (series_states[i].index >= Visibility::showing.index) {
        //     sum += entities[i].value;
        //   }
        // }

        // for (let i = 0; i < pieCount; i++) {
        //   Pie pie = entities[i];
        //   let color = self.base.get_color(i);
        //   pie.index = i;
        //   pie.name = self.base.data_table.rows[i][0];
        //   pie.color = color;
        //   pie.highlight_color = get_highlight_color(color);
        //   pie.center = center;
        //   pie.inner_radius = inner_radius;
        //   pie.outer_radius = outer_radius;

        //   if (series_states[i].index >= Visibility::showing.index) {
        //     pie.start_angle = start_angle;
        //     pie.end_angle = start_angle + direction * pie.value * TAU / sum;
        //     start_angle = pie.end_angle;
        //   } else {
        //     pie.start_angle = start_angle;
        //     pie.end_angle = start_angle;
        //   }
        // }
        unimplemented!()
    }

    fn create_entity(
        &self,
        series_index: usize,
        entity_index: usize,
        value: f64,
        color: Color,
        highlight_color: Color,
    ) -> PieEntity {
        // // Override the colors.
        // color = self.base.get_color(entity_index);
        // highlight_color = change_color_alpha(color, .5);
        // let name = self.base.data_table.rows[entity_index][0];
        // let start_angle = start_angle;
        // if (entity_index > 0 && series_list != null) {
        //   let prevPie = series_list[0].entities[entity_index - 1] as Pie;
        //   start_angle = prevPie.end_angle;
        // }
        // return Pie()
        //   ..index = entity_index
        //   ..value = value
        //   ..formatted_value = value != null ? entity_value_formatter(value) : null
        //   ..name = name
        //   ..color = color
        //   ..highlight_color = highlight_color
        //   ..old_start_angle = start_angle
        //   ..old_end_angle = start_angle
        //   ..center = center
        //   ..inner_radius = inner_radius
        //   ..outer_radius = outer_radius
        //   ..start_angle = start_angle
        //   ..end_angle = start_angle; // To be updated in [update_series].
        unimplemented!()
    }

    fn get_tooltip_position(&self, tooltip_width: f64, tooltip_height: f64) -> Point<f64> {
        // let pie = series_list.first.entities[focused_entity_index] as Pie;
        // let angle = .5 * (pie.start_angle + pie.end_angle);
        // let radius = .5 * (inner_radius + outer_radius);
        // let point = utils::polar2cartesian(center, radius, angle);
        // let x = point.x - .5 * tooltip.offset_width;
        // let y = point.y - tooltip.offset_height;
        // return Point(x, y);
        unimplemented!()
    }
}
