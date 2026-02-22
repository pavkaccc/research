use pyo3::prelude::*;

use super::py_entity::{register_entity_class, PyDivData, PyDivDataState, PyDivEntity};
use super::py_value::json_to_py;
use crate::entity::{self, Entity};

/// Register all entity types on the module.
fn register_all_entities(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Size types
    register_entity_class(py, m, "DivFixedSize", Some("fixed"),
        &["value", "unit"], &["value"])?;
    register_entity_class(py, m, "DivMatchParentSize", Some("match_parent"),
        &["weight"], &[])?;
    register_entity_class(py, m, "DivWrapContentSize", Some("wrap_content"),
        &["constrained", "max_size", "min_size"], &[])?;

    // Basic types
    register_entity_class(py, m, "DivEdgeInsets", None,
        &["left", "top", "right", "bottom", "start", "end", "unit"], &[])?;
    register_entity_class(py, m, "DivBorder", None,
        &["corner_radius", "corners_radius", "has_shadow", "shadow", "stroke"], &[])?;
    register_entity_class(py, m, "DivCornersRadius", None,
        &["bottom_left", "bottom_right", "top_left", "top_right"], &[])?;
    register_entity_class(py, m, "DivShadow", None,
        &["alpha", "blur", "color", "offset"], &[])?;
    register_entity_class(py, m, "DivPoint", None,
        &["x", "y"], &[])?;
    register_entity_class(py, m, "DivDimension", None,
        &["value", "unit"], &[])?;
    register_entity_class(py, m, "DivStroke", None,
        &["color", "unit", "width"], &[])?;
    register_entity_class(py, m, "DivAction", None,
        &["log_id", "url", "log_url", "payload", "typed"], &[])?;

    // Background types
    register_entity_class(py, m, "DivSolidBackground", Some("solid"),
        &["color"], &["color"])?;
    register_entity_class(py, m, "DivGradientBackground", Some("gradient"),
        &["angle", "colors"], &["colors"])?;
    register_entity_class(py, m, "DivImageBackground", Some("image"),
        &["alpha", "content_alignment_horizontal", "content_alignment_vertical",
          "image_url", "preload_required", "scale"],
        &["image_url"])?;

    // Shape types
    register_entity_class(py, m, "DivRoundedRectangleShape", Some("rounded_rectangle"),
        &["corner_radius", "item_height", "item_width"], &[])?;
    register_entity_class(py, m, "DivCircleShape", Some("circle"),
        &["radius", "background_color", "stroke"], &[])?;
    register_entity_class(py, m, "DivShapeDrawable", Some("shape_drawable"),
        &["color", "shape", "stroke"], &["color"])?;

    // Main Div components
    register_entity_class(py, m, "DivText", Some("text"),
        &["text", "font_size", "font_weight", "font_family", "text_color",
          "text_alignment_horizontal", "text_alignment_vertical",
          "line_height", "max_lines", "letter_spacing", "strike", "underline",
          "width", "height", "alpha", "alignment_horizontal", "alignment_vertical",
          "paddings", "margins", "border", "background", "visibility",
          "actions", "action", "accessibility", "id", "column_span", "row_span"],
        &[])?;

    register_entity_class(py, m, "DivImage", Some("image"),
        &["image_url", "placeholder_color", "scale",
          "content_alignment_horizontal", "content_alignment_vertical", "preview",
          "width", "height", "alpha", "alignment_horizontal", "alignment_vertical",
          "paddings", "margins", "border", "background", "visibility",
          "actions", "action", "accessibility", "id", "column_span", "row_span"],
        &[])?;

    register_entity_class(py, m, "DivContainer", Some("container"),
        &["items", "orientation", "content_alignment_horizontal",
          "content_alignment_vertical", "width", "height", "alpha",
          "alignment_horizontal", "alignment_vertical",
          "paddings", "margins", "border", "background", "visibility",
          "actions", "action", "accessibility", "id", "column_span", "row_span"],
        &[])?;

    register_entity_class(py, m, "DivGallery", Some("gallery"),
        &["items", "scroll_mode", "item_spacing", "cross_content_alignment",
          "width", "height", "alpha", "paddings", "margins", "border",
          "background", "visibility", "column_span", "row_span"],
        &["items"])?;

    register_entity_class(py, m, "DivSlider", Some("slider"),
        &["min_value", "max_value", "thumb_style", "thumb_text_style",
          "track_active_style", "track_inactive_style",
          "thumb_secondary_style", "thumb_secondary_text_style",
          "width", "height", "alpha", "paddings", "margins", "border", "visibility"],
        &[])?;

    register_entity_class(py, m, "DivSliderTextStyle", None,
        &["font_size", "font_weight", "text_color", "offset"], &[])?;

    register_entity_class(py, m, "DivAccessibility", None,
        &["description", "hint", "mode", "state_description", "type"], &[])?;

    register_entity_class(py, m, "DivGifImage", Some("gif"),
        &["gif_url", "width", "height", "alpha", "paddings", "margins",
          "border", "accessibility"],
        &[])?;

    register_entity_class(py, m, "DivSeparator", Some("separator"),
        &["delimiter_style", "width", "height", "alpha", "paddings", "margins",
          "border", "visibility"],
        &[])?;

    register_entity_class(py, m, "DivSeparatorDelimiterStyle", None,
        &["color", "orientation"], &[])?;

    register_entity_class(py, m, "DivInput", Some("input"),
        &["text_variable", "font_size", "font_weight", "text_color",
          "hint_text", "hint_color", "highlight_color",
          "line_height", "max_lines", "keyboard_type",
          "width", "height", "alpha", "paddings", "margins", "border", "visibility"],
        &[])?;

    register_entity_class(py, m, "DivPager", Some("pager"),
        &["items", "layout_mode", "orientation", "width", "height",
          "paddings", "margins"],
        &["items"])?;

    register_entity_class(py, m, "DivPagerLayoutMode", None,
        &["neighbour_page_width", "page_width"], &[])?;

    register_entity_class(py, m, "DivPercentageSize", Some("percentage"),
        &["value"], &["value"])?;

    register_entity_class(py, m, "DivTabs", Some("tabs"),
        &["items", "selected_tab", "width", "height", "paddings", "margins"],
        &["items"])?;

    register_entity_class(py, m, "DivTabsItem", None,
        &["title", "div"], &["title", "div"])?;

    register_entity_class(py, m, "DivState", Some("state"),
        &["states", "div_id", "default_state_id", "width", "height"],
        &["states"])?;

    register_entity_class(py, m, "DivStateState", None,
        &["state_id", "div"], &["state_id"])?;

    register_entity_class(py, m, "DivCustom", Some("custom"),
        &["custom_type", "custom_props", "width", "height"],
        &["custom_type"])?;

    register_entity_class(py, m, "DivIndicator", Some("indicator"),
        &["pager_id", "active_item_color", "inactive_item_color",
          "space_between_centers", "shape", "width", "height",
          "paddings", "margins"],
        &[])?;

    register_entity_class(py, m, "DivAnimation", Some("animation"),
        &["name", "duration", "start_delay", "interpolator",
          "repeat_count", "end_value", "start_value"],
        &["name"])?;

    register_entity_class(py, m, "DivTransform", None,
        &["pivot_x", "pivot_y", "rotation"], &[])?;

    register_entity_class(py, m, "DivPivot", None,
        &["type", "value"], &[])?;

    register_entity_class(py, m, "DivTooltip", None,
        &["id", "div", "position", "duration", "offset"],
        &["id", "div", "position"])?;

    Ok(())
}

/// Register enum-like namespaces.
fn register_enums(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let enums: &[(&str, &[(&str, &str)])] = &[
        ("DivAlignmentVertical", &[
            ("TOP", "top"), ("CENTER", "center"), ("BOTTOM", "bottom"), ("BASELINE", "baseline"),
        ]),
        ("DivAlignmentHorizontal", &[
            ("LEFT", "left"), ("CENTER", "center"), ("RIGHT", "right"),
            ("START", "start"), ("END", "end"),
        ]),
        ("DivContainerOrientation", &[
            ("VERTICAL", "vertical"), ("HORIZONTAL", "horizontal"), ("OVERLAP", "overlap"),
        ]),
        ("DivVisibility", &[
            ("VISIBLE", "visible"), ("INVISIBLE", "invisible"), ("GONE", "gone"),
        ]),
        ("DivFontWeight", &[
            ("LIGHT", "light"), ("REGULAR", "regular"), ("MEDIUM", "medium"), ("BOLD", "bold"),
        ]),
        ("DivLineStyle", &[
            ("NONE", "none"), ("SINGLE", "single"),
        ]),
        ("DivImageScale", &[
            ("FILL", "fill"), ("NO_SCALE", "no_scale"), ("FIT", "fit"), ("STRETCH", "stretch"),
        ]),
        ("DivSizeUnit", &[
            ("DP", "dp"), ("SP", "sp"), ("PX", "px"),
        ]),
        ("DivGalleryScrollMode", &[
            ("PAGING", "paging"), ("DEFAULT", "default"),
        ]),
        ("DivGalleryCrossContentAlignment", &[
            ("START", "start"), ("CENTER", "center"), ("END", "end"),
        ]),
    ];

    for (enum_name, variants) in enums {
        let attrs = variants
            .iter()
            .map(|(name, val)| format!("'{}': '{}'", name, val))
            .collect::<Vec<_>>()
            .join(", ");
        let code = format!("type('{}', (), {{{}}})", enum_name, attrs);
        let code_cstr = std::ffi::CString::new(code).unwrap();
        let cls = py.eval(&code_cstr, None, None)?;
        m.setattr(*enum_name, cls)?;
    }

    Ok(())
}

#[pyfunction]
fn make_div(py: Python<'_>, div: PyRef<'_, PyDivEntity>) -> PyResult<Py<PyAny>> {
    let rust_entity = div.to_rust_entity();
    let json_val = entity::make_div(rust_entity.as_ref());
    json_to_py(py, &json_val)
}

#[pyfunction]
fn make_card(
    py: Python<'_>,
    log_id: &str,
    div: PyRef<'_, PyDivEntity>,
) -> PyResult<Py<PyAny>> {
    let rust_entity = div.to_rust_entity();
    let data = entity::make_card(log_id, rust_entity.as_ref());
    let json_val = <entity::DivData as Entity>::dict(&data);
    json_to_py(py, &json_val)
}

/// The native Python module `divkit_rs._native`.
#[pymodule]
pub fn _native(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDivEntity>()?;
    m.add_class::<PyDivData>()?;
    m.add_class::<PyDivDataState>()?;

    register_all_entities(py, m)?;
    register_enums(py, m)?;

    m.add_function(wrap_pyfunction!(make_div, m)?)?;
    m.add_function(wrap_pyfunction!(make_card, m)?)?;

    Ok(())
}
