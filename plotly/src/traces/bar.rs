//! Bar trace

use serde::Serialize;

use crate::{
    common::{
        Calendar, ConstrainText, Dim, ErrorData, Font, HoverInfo, Label, Marker, Orientation,
        PlotType, TextAnchor, TextPosition, Visible,
    },
    private, Trace,
};

/// Construct a bar trace.
///
/// # Examples
///
/// ```
/// use plotly::Bar;
///
/// let x = vec![0, 1, 2, 3, 4, 5];
/// let y = vec![0, 2, 4, 6, 8, 10];
///
/// let trace = Bar::new(x, y).show_legend(true).opacity(0.5);
///
/// let expected = serde_json::json!({
///     "type": "bar",
///     "x": [0, 1, 2, 3, 4, 5],
///     "y": [0, 2, 4, 6, 8, 10],
///     "showlegend": true,
///     "opacity": 0.5
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone)]
pub struct Bar<X, Y>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
{
    r#type: PlotType,
    x: Option<Vec<X>>,
    y: Option<Vec<Y>>,
    name: Option<String>,
    visible: Option<Visible>,
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(rename = "legendgroup")]
    legend_group: Option<String>,
    opacity: Option<f64>,
    ids: Option<Vec<String>>,
    width: Option<usize>,
    offset: Option<Dim<usize>>,
    text: Option<Dim<String>>,
    #[serde(rename = "textposition")]
    text_position: Option<Dim<TextPosition>>,
    #[serde(rename = "texttemplate")]
    text_template: Option<Dim<String>>,
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(rename = "xaxis")]
    x_axis: Option<String>,
    #[serde(rename = "yaxis")]
    y_axis: Option<String>,
    orientation: Option<Orientation>,
    #[serde(rename = "alignmentgroup")]
    alignment_group: Option<String>,
    #[serde(rename = "offsetgroup")]
    offset_group: Option<String>,
    marker: Option<Marker>,
    #[serde(rename = "textangle")]
    text_angle: Option<f64>,
    #[serde(rename = "textfont")]
    text_font: Option<Font>,
    error_x: Option<ErrorData>,
    error_y: Option<ErrorData>,
    #[serde(rename = "cliponaxis")]
    clip_on_axis: Option<bool>,
    #[serde(rename = "constraintext")]
    constrain_text: Option<ConstrainText>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(rename = "insidetextanchor")]
    inside_text_anchor: Option<TextAnchor>,
    #[serde(rename = "insidetextfont")]
    inside_text_font: Option<Font>,
    #[serde(rename = "outsidetextfont")]
    outside_text_font: Option<Font>,
    #[serde(rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    #[serde(rename = "ycalendar")]
    y_calendar: Option<Calendar>,
}

impl<X, Y> Default for Bar<X, Y>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
{
    fn default() -> Self {
        Self {
            r#type: PlotType::Bar,
            x: None,
            y: None,
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            opacity: None,
            ids: None,
            width: None,
            offset: None,
            text: None,
            text_position: None,
            text_template: None,
            hover_text: None,
            hover_info: None,
            hover_template: None,
            x_axis: None,
            y_axis: None,
            orientation: None,
            alignment_group: None,
            offset_group: None,
            marker: None,
            text_angle: None,
            text_font: None,
            error_x: None,
            error_y: None,
            clip_on_axis: None,
            constrain_text: None,
            hover_label: None,
            inside_text_anchor: None,
            inside_text_font: None,
            outside_text_font: None,
            x_calendar: None,
            y_calendar: None,
        }
    }
}

impl<X, Y> Bar<X, Y>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
{
    pub fn new(x: Vec<X>, y: Vec<Y>) -> Box<Self> {
        Box::new(Bar {
            x: Some(x),
            y: Some(y),
            ..Default::default()
        })
    }

    pub fn alignment_group(mut self, alignment_group: &str) -> Box<Self> {
        self.alignment_group = Some(alignment_group.to_owned());
        Box::new(self)
    }

    pub fn clip_on_axis(mut self, clip_on_axis: bool) -> Box<Self> {
        self.clip_on_axis = Some(clip_on_axis);
        Box::new(self)
    }

    pub fn constrain_text(mut self, constrain_text: ConstrainText) -> Box<Self> {
        self.constrain_text = Some(constrain_text);
        Box::new(self)
    }

    pub fn error_x(mut self, error_x: ErrorData) -> Box<Self> {
        self.error_x = Some(error_x);
        Box::new(self)
    }

    pub fn error_y(mut self, error_y: ErrorData) -> Box<Self> {
        self.error_y = Some(error_y);
        Box::new(self)
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Box<Self> {
        self.hover_info = Some(hover_info);
        Box::new(self)
    }

    pub fn hover_label(mut self, hover_label: Label) -> Box<Self> {
        self.hover_label = Some(hover_label);
        Box::new(self)
    }

    pub fn hover_template(mut self, hover_template: &str) -> Box<Self> {
        self.hover_template = Some(Dim::Scalar(hover_template.to_owned()));
        Box::new(self)
    }

    pub fn hover_template_array<S: AsRef<str>>(mut self, hover_template: Vec<S>) -> Box<Self> {
        let hover_template = private::owned_string_vector(hover_template);
        self.hover_template = Some(Dim::Vector(hover_template));
        Box::new(self)
    }

    pub fn hover_text(mut self, hover_text: &str) -> Box<Self> {
        self.hover_text = Some(Dim::Scalar(hover_text.to_owned()));
        Box::new(self)
    }

    pub fn hover_text_array<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> Box<Self> {
        let hover_text = private::owned_string_vector(hover_text);
        self.hover_text = Some(Dim::Vector(hover_text));
        Box::new(self)
    }

    pub fn ids<S: AsRef<str>>(mut self, ids: Vec<S>) -> Box<Self> {
        let ids = private::owned_string_vector(ids);
        self.ids = Some(ids);
        Box::new(self)
    }

    pub fn inside_text_anchor(mut self, inside_text_anchor: TextAnchor) -> Box<Self> {
        self.inside_text_anchor = Some(inside_text_anchor);
        Box::new(self)
    }

    pub fn inside_text_font(mut self, inside_text_font: Font) -> Box<Self> {
        self.inside_text_font = Some(inside_text_font);
        Box::new(self)
    }

    pub fn legend_group(mut self, legend_group: &str) -> Box<Self> {
        self.legend_group = Some(legend_group.to_owned());
        Box::new(self)
    }

    pub fn marker(mut self, marker: Marker) -> Box<Self> {
        self.marker = Some(marker);
        Box::new(self)
    }

    pub fn name(mut self, name: &str) -> Box<Self> {
        self.name = Some(name.to_owned());
        Box::new(self)
    }
    pub fn show_legend(mut self, show_legend: bool) -> Box<Self> {
        self.show_legend = Some(show_legend);
        Box::new(self)
    }

    pub fn text(mut self, text: &str) -> Box<Self> {
        self.text = Some(Dim::Scalar(text.to_owned()));
        Box::new(self)
    }

    pub fn text_array<S: AsRef<str>>(mut self, text: Vec<S>) -> Box<Self> {
        let text = private::owned_string_vector(text);
        self.text = Some(Dim::Vector(text));
        Box::new(self)
    }

    pub fn text_angle(mut self, text_angle: f64) -> Box<Self> {
        self.text_angle = Some(text_angle);
        Box::new(self)
    }

    pub fn text_position(mut self, text_position: TextPosition) -> Box<Self> {
        self.text_position = Some(Dim::Scalar(text_position));
        Box::new(self)
    }

    pub fn text_position_array(mut self, text_position: Vec<TextPosition>) -> Box<Self> {
        self.text_position = Some(Dim::Vector(text_position));
        Box::new(self)
    }

    pub fn text_template(mut self, text_template: &str) -> Box<Self> {
        self.text_template = Some(Dim::Scalar(text_template.to_owned()));
        Box::new(self)
    }

    pub fn text_template_array<S: AsRef<str>>(mut self, text_template: Vec<S>) -> Box<Self> {
        let text_template = private::owned_string_vector(text_template);
        self.text_template = Some(Dim::Vector(text_template));
        Box::new(self)
    }

    pub fn offset(mut self, offset: usize) -> Box<Self> {
        self.offset = Some(Dim::Scalar(offset));
        Box::new(self)
    }

    pub fn offset_array(mut self, offset: Vec<usize>) -> Box<Self> {
        self.offset = Some(Dim::Vector(offset));
        Box::new(self)
    }

    pub fn offset_group(mut self, offset_group: &str) -> Box<Self> {
        self.offset_group = Some(offset_group.to_owned());
        Box::new(self)
    }

    pub fn opacity(mut self, opacity: f64) -> Box<Self> {
        self.opacity = Some(opacity);
        Box::new(self)
    }

    pub fn orientation(mut self, orientation: Orientation) -> Box<Self> {
        self.orientation = Some(orientation);
        Box::new(self)
    }

    pub fn outside_text_font(mut self, outside_text_font: Font) -> Box<Self> {
        self.outside_text_font = Some(outside_text_font);
        Box::new(self)
    }

    pub fn text_font(mut self, text_font: Font) -> Box<Self> {
        self.text_font = Some(text_font);
        Box::new(self)
    }

    pub fn visible(mut self, visible: Visible) -> Box<Self> {
        self.visible = Some(visible);
        Box::new(self)
    }

    pub fn width(mut self, width: usize) -> Box<Self> {
        self.width = Some(width);
        Box::new(self)
    }

    pub fn x_axis(mut self, axis: &str) -> Box<Self> {
        self.x_axis = Some(axis.to_owned());
        Box::new(self)
    }

    pub fn x_calendar(mut self, x_calendar: Calendar) -> Box<Self> {
        self.x_calendar = Some(x_calendar);
        Box::new(self)
    }
    pub fn y_axis(mut self, axis: &str) -> Box<Self> {
        self.y_axis = Some(axis.to_owned());
        Box::new(self)
    }

    pub fn y_calendar(mut self, y_calendar: Calendar) -> Box<Self> {
        self.y_calendar = Some(y_calendar);
        Box::new(self)
    }
}

impl<X, Y> Trace for Bar<X, Y>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
{
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;
    use crate::common::ErrorType;

    #[test]
    fn test_default_bar() {
        let trace: Bar<i32, i32> = Bar::default();
        let expected = json!({"type": "bar"}).to_string();

        assert_eq!(trace.to_json(), expected);
    }

    #[test]
    fn test_serialize_bar() {
        let bar = Bar::new(vec![1, 2], vec![3, 4])
            .alignment_group("alignment_group")
            .clip_on_axis(true)
            .constrain_text(ConstrainText::Both)
            .error_x(ErrorData::new(ErrorType::Constant))
            .error_y(ErrorData::new(ErrorType::Percent))
            .hover_info(HoverInfo::All)
            .hover_label(Label::new())
            .hover_template("tmpl")
            .hover_template_array(vec!["tmpl1", "tmpl2"])
            .hover_text("hover_text")
            .hover_text_array(vec!["hover_text"])
            .ids(vec!["1"])
            .inside_text_anchor(TextAnchor::End)
            .inside_text_font(Font::new())
            .legend_group("legend-group")
            .marker(Marker::new())
            .name("Bar")
            .offset(5)
            .offset_array(vec![5, 5])
            .offset_group("offset_group")
            .opacity(0.5)
            .orientation(Orientation::Vertical)
            .outside_text_font(Font::new())
            .show_legend(false)
            .text("text")
            .text_angle(0.05)
            .text_array(vec!["text"])
            .text_font(Font::new())
            .text_position(TextPosition::None)
            .text_position_array(vec![TextPosition::None])
            .text_template("text_template")
            .text_template_array(vec!["text_template"])
            .visible(Visible::LegendOnly)
            .width(999)
            .x_axis("xaxis")
            .x_calendar(Calendar::Nanakshahi)
            .y_axis("yaxis")
            .y_calendar(Calendar::Ummalqura);

        let expected = json!({
            "type": "bar",
            "hoverinfo": "all",
            "hovertemplate": ["tmpl1", "tmpl2"],
            "x": [1, 2],
            "y": [3, 4],
            "name": "Bar",
            "visible": "legendonly",
            "showlegend": false,
            "legendgroup": "legend-group",
            "opacity": 0.5,
            "ids": ["1"],
            "width": 999,
            "offset": [5, 5],
            "text": ["text"],
            "textposition": ["none"],
            "texttemplate": ["text_template"],
            "hovertext": ["hover_text"],
            "xaxis": "xaxis",
            "yaxis": "yaxis",
            "orientation": "v",
            "alignmentgroup": "alignment_group",
            "offsetgroup": "offset_group",
            "marker": {},
            "textangle": 0.05,
            "textfont": {},
            "error_x": {"type": "constant"},
            "error_y": {"type": "percent"},
            "cliponaxis": true,
            "constraintext": "both",
            "hoverlabel": {},
            "insidetextanchor": "end",
            "insidetextfont": {},
            "outsidetextfont": {},
            "xcalendar": "nanakshahi",
            "ycalendar": "ummalqura",
        });

        assert_eq!(to_value(bar).unwrap(), expected);
    }
}
