#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use dataflow::*;
use primitives::{
    palette, BaseLine, CanvasContext, Color, Point, TextAlign, TextStyle, TextWeight,
};
use std::{borrow::Borrow, cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::*;

/// A pie in a pie chart.
#[derive(Default, Clone)]
pub struct PieEntity {
    // Chart chart,
    color: Color,
    highlight_color: Color,
    // formatted_value: String,
    index: usize,
    old_value: Option<f64>,
    value: Option<f64>,

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
            utils::is_in_range(angle, self.start_angle, self.end_angle)
        } else {
            // Counterclockwise.
            utils::is_in_range(angle, self.end_angle, self.start_angle)
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
        let mut a1 = utils::lerp(self.old_start_angle, self.start_angle, percent);
        let mut a2 = utils::lerp(self.old_end_angle, self.end_angle, percent);
        if a1 > a2 {
            let tmp = a1;
            a1 = a2;
            a2 = tmp;
        }
        let center = &self.center;
        if highlight {
            let highlight_outer_radius = HIGHLIGHT_OUTER_RADIUS_FACTOR * self.outer_radius;
            ctx.set_fill_color(self.highlight_color);
            ctx.begin_path();
            ctx.arc(center.x, center.y, highlight_outer_radius, a1, a2, false);
            ctx.arc(center.x, center.y, self.inner_radius, a2, a1, true);
            ctx.fill();
        }

        ctx.set_fill_color(self.color);
        ctx.begin_path();
        ctx.arc(center.x, center.y, self.outer_radius, a1, a2, false);
        ctx.arc(center.x, center.y, self.inner_radius, a2, a1, true);
        ctx.fill();
        ctx.stroke();

        // && chart is PieChart
        // if !self.formatted_value.is_empty() && a2 - a1 > PI / 36.0 {
        //     // let labels = self.chart.options.channel.labels;
        //     // if labels.enabled {
        //     //     let r = 0.25 * self.inner_radius + 0.75 * self.outer_radius;
        //     //     let a = 0.5 * (a1 + a2);
        //     //     let p = utils::polar2cartesian(center, r, a);
        //     //     ctx.set_fill_color(labels.style.color);
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
            .update_channel_visible(record.index, record.removed_count, record.added_count);
        self.base.data_rows_changed(record);
        self.base.update_legend_content();
    }

    fn get_entity_group_index(&self, x: f64, y: f64) -> i64 {
        let p = Point::new(x, y);
        // let entities = channels.first.entities;
        // for (let i = entities.len(); i >= 0; i--) {
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

    fn channel_visibility_changed(&self, index: usize) {
        self.update_channel(0);
    }

    fn update_tooltip_content(&self) {
        // let pie = channels[0].entities[focused_entity_index] as Pie;
        // tooltip.style
        //   ..borderColor = pie.color
        //   ..padding = "4px 12px";
        // let label = tooltip_label_formatter(pie.name);
        // let value = tooltip_value_formatter(pie.value);
        // tooltip.innerHtml = "$label: <strong>$value</strong>";
        unimplemented!()
    }

    /// Called when [data_table] has been changed.
    fn data_table_changed(&self) {
        info!("data_table_changed");
        // self.calculate_drawing_sizes(ctx);
        let mut channels = self.base.channels.borrow_mut();
        *channels = self.create_channels(0, self.base.data_table.meta.len());
    }
}

impl<'a, C, M, D> Chart<'a, C, M, D, PieEntity> for PieChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display,
{
    fn calculate_drawing_sizes(&self, ctx: &C) {
        self.base.calculate_drawing_sizes(ctx);
        let mut props = self.props.borrow_mut();
        let rect = &self.base.props.borrow().channel_and_axes_box;
        let half_w = rect.size.width as i64 >> 1;
        let half_h = rect.size.height as i64 >> 1;

        let x = rect.origin.x + half_w as f64;
        let y = rect.origin.y + half_h as f64;
        props.center = Point::new(x, y);

        // self.outer_radius = (half_w.min(half_h) as f64) / HIGHLIGHT_OUTER_RADIUS_FACTOR;
        let mut pie_hole = self.base.options.pie_hole;

        if pie_hole > 1.0 {
            pie_hole = 0.0;
        }

        if pie_hole < 0.0 {
            pie_hole = 0.0;
        }

        props.inner_radius = pie_hole * props.outer_radius;

        let opt = &self.base.options.channel;
        let mut baseprops = self.base.props.borrow_mut();
        baseprops.entity_value_formatter = if let Some(formatter) = opt.labels.formatter {
            Some(formatter)
        } else {
            Some(default_value_formatter)
        };

        props.direction = if opt.counterclockwise {
            COUNTERCLOCKWISE
        } else {
            CLOCKWISE
        };

        props.start_angle = utils::deg2rad(opt.start_angle);
    }

    fn set_stream(&mut self, stream: DataStream<'a, M, D>) {
        self.base.data_table = stream;
    }

    fn draw(&self, ctx: &C) {
        self.base.dispose();
        // data_tableSubscriptionTracker
        //   ..add(dataTable.onCellChange.listen(data_cell_changed))
        //   ..add(dataTable.onColumnsChange.listen(dataColumnsChanged))
        //   ..add(dataTable.onRowsChange.listen(data_rows_changed));
        // self.easing_function = get_easing(self.options.animation().easing);
        self.base.initialize_legend();
        self.base.initialize_tooltip();

        self.base.draw(ctx);
        // if force_redraw {
        //     println!("BaseChart force_redraw");
        //     self.stop_animation();
        //     self.data_table_changed();
        //     self.position_legend();

        //     // This call is redundant for row and column changes but necessary for
        //     // cell changes.
        //     self.calculate_drawing_sizes(ctx);
        //     self.update_channel(0);
        // }

        // self.ctx.clearRect(0, 0, self.width, self.height);
        self.draw_axes_and_grid(ctx);
        self.base.start_animation();
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

            // Update the visibility states of all channel before the last frame.
            let mut channels = self.base.channels.borrow_mut();

            for channel in channels.iter_mut() {
                if channel.state == Visibility::Showing {
                    channel.state = Visibility::Shown;
                } else if channel.state == Visibility::Hiding {
                    channel.state = Visibility::Hidden;
                }
            }
        }

        let props = self.base.props.borrow();

        let ease = props.easing_function.unwrap();
        self.draw_channel(ctx, ease(percent));
        // ctx.drawImageScaled(ctx.canvas, 0, 0, width, height);
        // ctx.drawImageScaled(ctx.canvas, 0, 0, width, height);
        self.base.draw_title(ctx);

        if percent < 1.0 {
            // animation_frame_id = window.requestAnimationFrame(draw_frame);
        } else if time.is_some() {
            self.base.animation_end();
        }
    }

    fn draw_channel(&self, ctx: &C, percent: f64) -> bool {
        ctx.set_line_width(2.);
        ctx.set_stroke_color(palette::WHITE);
        ctx.set_text_align(TextAlign::Center);
        ctx.set_text_baseline(BaseLine::Middle);

        let channels = self.base.channels.borrow();
        let channel = channels.first().unwrap();
        let labels = &self.base.options.channel.labels.style;

        ctx.set_font(
            labels.font_family.unwrap_or(DEFAULT_FONT_FAMILY),
            labels.font_style.unwrap_or(TextStyle::Normal),
            TextWeight::Normal,
            labels.font_size.unwrap_or(12.),
        );

        let baseprops = self.base.props.borrow();
        let focused_channel_index = baseprops.focused_channel_index as usize;
        let focused_entity_index = baseprops.focused_entity_index as usize;

        for entity in channel.entities.iter() {
            if entity.is_empty() && percent == 1.0 {
                continue;
            }
            let highlight =
                entity.index == focused_channel_index || entity.index == focused_entity_index;
            entity.draw(ctx, percent, highlight);
        }

        return false;
    }

    fn update_channel(&self, index: usize) {
        // let mut sum = 0.0;
        // let props = self.props.borrow();
        // let mut start_angle = props.start_angle;

        // let pie_count = self.base.data_table.frames.len();

        // let mut channels = self.base.channels.borrow_mut();

        // let channel = channels.first_mut().unwrap();

        // let channel_states = &self.base.props.borrow().channel_states;

        // // Sum the values of all visible pies.
        // for idx in 0..pie_count {
        //     let channel_state = channel_states[idx];
        //     if channel_state == Visibility::Showing || channel_state == Visibility::Shown {
        //         sum += channel.entities[idx].value;
        //     }
        // }

        // for idx in 0..pie_count {
        //     let entity = channel.entities.get_mut(idx).unwrap();
        //     let color = self.base.get_color(idx);
        //     entity.index = idx;
        //     // TODO: deal with name
        //     //   pie.name = self.base.data_table.frames[idx][0];
        //     entity.color = color;
        //     entity.highlight_color = self.base.get_highlight_color(color);
        //     entity.center = props.center;
        //     entity.inner_radius = props.inner_radius;
        //     entity.outer_radius = props.outer_radius;

        //     let channel_state = channel_states[idx];
        //     if channel_state == Visibility::Showing || channel_state == Visibility::Shown {
        //         entity.start_angle = start_angle;
        //         entity.end_angle = start_angle + props.direction as f64 * entity.value * TAU / sum;
        //         start_angle = entity.end_angle;
        //     } else {
        //         entity.start_angle = start_angle;
        //         entity.end_angle = start_angle;
        //     }
        // }
    }

    fn create_entity(
        &self,
        channel_index: usize,
        entity_index: usize,
        value: Option<f64>,
        color: Color,
        highlight_color: Color,
    ) -> PieEntity {
        // Override the colors.
        let color = self.base.get_color(entity_index);
        let highlight_color = self.base.change_color_alpha(color, 0.5);

        let stream = &self.base.data_table;
        let frame = stream.frames.get(entity_index).unwrap();
        let name = format!("{}", frame.metric);

        let props = self.props.borrow();

        let mut start_angle = props.start_angle;

        if entity_index > 0 {
            let channels = self.base.channels.borrow();
            let channel = channels.first().unwrap();
            let prev = channel.entities.get(entity_index - 1).unwrap();
            start_angle = prev.end_angle;
        }

        // let formatted_value = if value != 0. {
        //     self.entity_value_formatter(value)
        // } else {
        //     null
        // };

        PieEntity {
            index: entity_index,
            old_value: None,
            value,
            // formatted_value,
            name,
            color,
            highlight_color,
            old_start_angle: start_angle,
            old_end_angle: start_angle,
            center: props.center,
            inner_radius: props.inner_radius,
            outer_radius: props.outer_radius,
            start_angle,
            end_angle: start_angle, // To be updated in [update_channel]
        }
    }

    fn create_channels(&self, start: usize, end: usize) -> Vec<ChartChannel<PieEntity>> {
        info!("create_channels");
        let result = Vec::new();
        // let entity_count = self.data_table.frames.len();
        // while (start < end) {
        //   let name = self.base.data_table.columns[start + 1].name;
        //   let color = get_color(start);
        //   let highlight_color = get_highlight_color(color);
        //   let entities =
        //       create_entities(start, 0, entity_count, color, highlight_color);
        //   result.add(Series(name, color, highlight_color, entities));
        //   start++;
        // }
        result
    }

    fn create_entities(
        &self,
        channel_index: usize,
        start: usize,
        end: usize,
        color: Color,
        highlight: Color,
    ) -> Vec<PieEntity> {
        info!("create_entities");
        let result = Vec::new();
        // while (start < end) {
        //   let value = self.base.data_table.rows[start][channelIndex + 1];
        //   let e = create_entity(channelIndex, start, value, color, highlight_color);
        //   e.chart = this;
        //   result.add(e);
        //   start++;
        // }
        result
    }

    fn get_tooltip_position(&self, tooltip_width: f64, tooltip_height: f64) -> Point<f64> {
        let channels = self.base.channels.borrow();
        let channel = channels.first().unwrap();

        let focused_entity_index = self.base.props.borrow().focused_entity_index as usize;

        let props = self.props.borrow();

        let pie = channel.entities.get(focused_entity_index).unwrap();
        let angle = 0.5 * (pie.start_angle + pie.end_angle);
        let radius = 0.5 * (props.inner_radius + props.outer_radius);
        let point = utils::polar2cartesian(&props.center, radius, angle);
        let x = point.x - 0.5 * tooltip_width;
        let y = point.y - tooltip_height;
        Point::new(x, y)
    }
}
