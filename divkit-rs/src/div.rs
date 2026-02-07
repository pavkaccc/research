//! DivKit entity type definitions.
//!
//! These mirror the auto-generated Python types in `pydivkit.div`.
//! Each type uses the `div_entity!` macro to generate a struct with
//! `dict()`, `build()`, and `schema()` methods.

#[allow(unused_imports)]
use crate::{div_entity, div_enum};

// ============================================================
// Enums
// ============================================================

div_enum! {
    pub enum DivAlignmentVertical {
        Top => "top",
        Center => "center",
        Bottom => "bottom",
        Baseline => "baseline",
    }
}

div_enum! {
    pub enum DivAlignmentHorizontal {
        Left => "left",
        Center => "center",
        Right => "right",
        Start => "start",
        End => "end",
    }
}

div_enum! {
    pub enum DivContainerOrientation {
        Vertical => "vertical",
        Horizontal => "horizontal",
        Overlap => "overlap",
    }
}

div_enum! {
    pub enum DivVisibility {
        Visible => "visible",
        Invisible => "invisible",
        Gone => "gone",
    }
}

div_enum! {
    pub enum DivFontWeight {
        Light => "light",
        Regular => "regular",
        Medium => "medium",
        Bold => "bold",
    }
}

div_enum! {
    pub enum DivLineStyle {
        None => "none",
        Single => "single",
    }
}

div_enum! {
    pub enum DivImageScale {
        Fill => "fill",
        NoScale => "no_scale",
        Fit => "fit",
        Stretch => "stretch",
    }
}

div_enum! {
    pub enum DivSizeUnit {
        Dp => "dp",
        Sp => "sp",
        Px => "px",
    }
}

div_enum! {
    pub enum DivGalleryScrollMode {
        Paging => "paging",
        Default => "default",
    }
}

div_enum! {
    pub enum DivGalleryCrossContentAlignment {
        Start => "start",
        Center => "center",
        End => "end",
    }
}

// ============================================================
// Size types
// ============================================================

div_entity! {
    pub struct DivFixedSize {
        type_name: "fixed",
        fields: {
            #[required]
            value: i64,
            unit: String,
        }
    }
}

div_entity! {
    pub struct DivMatchParentSize {
        type_name: "match_parent",
        fields: {
            weight: f64,
        }
    }
}

div_entity! {
    pub struct DivWrapContentSize {
        type_name: "wrap_content",
        fields: {
            constrained: bool,
            max_size: DivFixedSize,
            min_size: DivFixedSize,
        }
    }
}

// ============================================================
// Basic component types
// ============================================================

div_entity! {
    pub struct DivEdgeInsets {
        fields: {
            left: i64,
            top: i64,
            right: i64,
            bottom: i64,
            start: i64,
            end: i64,
            unit: String,
        }
    }
}

div_entity! {
    pub struct DivBorder {
        fields: {
            corner_radius: i64,
            corners_radius: DivCornersRadius,
            has_shadow: bool,
            shadow: DivShadow,
            stroke: DivStroke,
        }
    }
}

div_entity! {
    pub struct DivCornersRadius {
        fields: {
            bottom_left: i64,
            bottom_right: i64,
            top_left: i64,
            top_right: i64,
        }
    }
}

div_entity! {
    pub struct DivShadow {
        fields: {
            alpha: f64,
            blur: i64,
            color: String,
            offset: DivPoint,
        }
    }
}

div_entity! {
    pub struct DivPoint {
        fields: {
            x: DivDimension,
            y: DivDimension,
        }
    }
}

div_entity! {
    pub struct DivDimension {
        fields: {
            value: f64,
            unit: String,
        }
    }
}

div_entity! {
    pub struct DivStroke {
        fields: {
            color: String,
            unit: String,
            width: i64,
        }
    }
}

div_entity! {
    pub struct DivAction {
        fields: {
            log_id: String,
            url: String,
            log_url: String,
            payload: String,
            typed: String,
        }
    }
}

// ============================================================
// Background types
// ============================================================

div_entity! {
    pub struct DivSolidBackground {
        type_name: "solid",
        fields: {
            #[required]
            color: String,
        }
    }
}

div_entity! {
    pub struct DivGradientBackground {
        type_name: "gradient",
        fields: {
            angle: i64,
            #[required]
            colors: Vec<String>,
        }
    }
}

div_entity! {
    pub struct DivImageBackground {
        type_name: "image",
        fields: {
            alpha: f64,
            content_alignment_horizontal: String,
            content_alignment_vertical: String,
            #[required]
            image_url: String,
            preload_required: bool,
            scale: String,
        }
    }
}

// ============================================================
// Shape types
// ============================================================

div_entity! {
    pub struct DivRoundedRectangleShape {
        type_name: "rounded_rectangle",
        fields: {
            corner_radius: DivFixedSize,
            item_height: DivFixedSize,
            item_width: DivFixedSize,
        }
    }
}

div_entity! {
    pub struct DivCircleShape {
        type_name: "circle",
        fields: {
            radius: DivFixedSize,
            background_color: String,
            stroke: DivStroke,
        }
    }
}

div_entity! {
    pub struct DivShapeDrawable {
        type_name: "shape_drawable",
        fields: {
            #[required]
            color: String,
            shape: DivRoundedRectangleShape,
            stroke: DivStroke,
        }
    }
}

// ============================================================
// Main Div components
// ============================================================

div_entity! {
    /// Text component.
    pub struct DivText {
        type_name: "text",
        fields: {
            text: String,
            font_size: i64,
            font_weight: String,
            font_family: String,
            text_color: String,
            text_alignment_horizontal: String,
            text_alignment_vertical: String,
            line_height: i64,
            max_lines: i64,
            letter_spacing: f64,
            strike: String,
            underline: String,
            width: DivSize,
            height: DivSize,
            alpha: f64,
            alignment_horizontal: String,
            alignment_vertical: String,
            paddings: DivEdgeInsets,
            margins: DivEdgeInsets,
            border: DivBorder,
            background: Vec<DivSolidBackground>,
            visibility: String,
            actions: Vec<DivAction>,
            action: DivAction,
            accessibility: DivAccessibility,
            id: String,
            column_span: i64,
            row_span: i64,
        }
    }
}

div_entity! {
    /// Image component.
    pub struct DivImage {
        type_name: "image",
        fields: {
            image_url: String,
            placeholder_color: String,
            scale: String,
            content_alignment_horizontal: String,
            content_alignment_vertical: String,
            preview: String,
            width: DivSize,
            height: DivSize,
            alpha: f64,
            alignment_horizontal: String,
            alignment_vertical: String,
            paddings: DivEdgeInsets,
            margins: DivEdgeInsets,
            border: DivBorder,
            background: Vec<DivSolidBackground>,
            visibility: String,
            actions: Vec<DivAction>,
            action: DivAction,
            accessibility: DivAccessibility,
            id: String,
            column_span: i64,
            row_span: i64,
        }
    }
}

div_entity! {
    /// Container component.
    pub struct DivContainer {
        type_name: "container",
        fields: {
            items: Vec<DivValue>,
            orientation: String,
            content_alignment_horizontal: String,
            content_alignment_vertical: String,
            width: DivSize,
            height: DivSize,
            alpha: f64,
            alignment_horizontal: String,
            alignment_vertical: String,
            paddings: DivEdgeInsets,
            margins: DivEdgeInsets,
            border: DivBorder,
            background: Vec<DivSolidBackground>,
            visibility: String,
            actions: Vec<DivAction>,
            action: DivAction,
            accessibility: DivAccessibility,
            id: String,
            column_span: i64,
            row_span: i64,
        }
    }
}

div_entity! {
    /// Gallery component.
    pub struct DivGallery {
        type_name: "gallery",
        fields: {
            #[required]
            items: Vec<DivValue>,
            scroll_mode: String,
            item_spacing: i64,
            cross_content_alignment: String,
            width: DivSize,
            height: DivSize,
            alpha: f64,
            paddings: DivEdgeInsets,
            margins: DivEdgeInsets,
            border: DivBorder,
            background: Vec<DivSolidBackground>,
            visibility: String,
            column_span: i64,
            row_span: i64,
        }
    }
}

div_entity! {
    /// Slider component.
    pub struct DivSlider {
        type_name: "slider",
        fields: {
            min_value: i64,
            max_value: i64,
            thumb_style: DivShapeDrawable,
            thumb_text_style: DivSliderTextStyle,
            track_active_style: DivShapeDrawable,
            track_inactive_style: DivShapeDrawable,
            thumb_secondary_style: DivShapeDrawable,
            thumb_secondary_text_style: DivSliderTextStyle,
            width: DivSize,
            height: DivSize,
            alpha: f64,
            paddings: DivEdgeInsets,
            margins: DivEdgeInsets,
            border: DivBorder,
            visibility: String,
        }
    }
}

div_entity! {
    pub struct DivSliderTextStyle {
        fields: {
            font_size: i64,
            font_weight: String,
            text_color: String,
            offset: DivPoint,
        }
    }
}

div_entity! {
    pub struct DivAccessibility {
        fields: {
            description: String,
            hint: String,
            mode: String,
            state_description: String,
            r#type: String,
        }
    }
}

div_entity! {
    /// GIF image component.
    pub struct DivGifImage {
        type_name: "gif",
        fields: {
            gif_url: String,
            width: DivSize,
            height: DivSize,
            alpha: f64,
            paddings: DivEdgeInsets,
            margins: DivEdgeInsets,
            border: DivBorder,
            accessibility: DivAccessibility,
        }
    }
}

div_entity! {
    /// Separator component.
    pub struct DivSeparator {
        type_name: "separator",
        fields: {
            delimiter_style: DivSeparatorDelimiterStyle,
            width: DivSize,
            height: DivSize,
            alpha: f64,
            paddings: DivEdgeInsets,
            margins: DivEdgeInsets,
            border: DivBorder,
            visibility: String,
        }
    }
}

div_entity! {
    pub struct DivSeparatorDelimiterStyle {
        fields: {
            color: String,
            orientation: String,
        }
    }
}

div_entity! {
    /// Input component.
    pub struct DivInput {
        type_name: "input",
        fields: {
            text_variable: String,
            font_size: i64,
            font_weight: String,
            text_color: String,
            hint_text: String,
            hint_color: String,
            highlight_color: String,
            line_height: i64,
            max_lines: i64,
            keyboard_type: String,
            width: DivSize,
            height: DivSize,
            alpha: f64,
            paddings: DivEdgeInsets,
            margins: DivEdgeInsets,
            border: DivBorder,
            visibility: String,
        }
    }
}

div_entity! {
    /// Pager component.
    pub struct DivPager {
        type_name: "pager",
        fields: {
            #[required]
            items: Vec<DivValue>,
            layout_mode: DivPagerLayoutMode,
            orientation: String,
            width: DivSize,
            height: DivSize,
            paddings: DivEdgeInsets,
            margins: DivEdgeInsets,
        }
    }
}

div_entity! {
    pub struct DivPagerLayoutMode {
        fields: {
            neighbour_page_width: DivFixedSize,
            page_width: DivPercentageSize,
        }
    }
}

div_entity! {
    pub struct DivPercentageSize {
        type_name: "percentage",
        fields: {
            #[required]
            value: f64,
        }
    }
}

div_entity! {
    /// Tabs component.
    pub struct DivTabs {
        type_name: "tabs",
        fields: {
            #[required]
            items: Vec<DivTabsItem>,
            selected_tab: i64,
            width: DivSize,
            height: DivSize,
            paddings: DivEdgeInsets,
            margins: DivEdgeInsets,
        }
    }
}

div_entity! {
    pub struct DivTabsItem {
        fields: {
            #[required]
            title: String,
            #[required]
            div: DivValue,
        }
    }
}

div_entity! {
    /// State component.
    pub struct DivState {
        type_name: "state",
        fields: {
            #[required]
            states: Vec<DivStateState>,
            div_id: String,
            default_state_id: String,
            width: DivSize,
            height: DivSize,
        }
    }
}

div_entity! {
    pub struct DivStateState {
        fields: {
            #[required]
            state_id: String,
            div: DivValue,
        }
    }
}

div_entity! {
    /// Custom component.
    pub struct DivCustom {
        type_name: "custom",
        fields: {
            #[required]
            custom_type: String,
            custom_props: String,
            width: DivSize,
            height: DivSize,
        }
    }
}

div_entity! {
    /// Indicator component.
    pub struct DivIndicator {
        type_name: "indicator",
        fields: {
            pager_id: String,
            active_item_color: String,
            inactive_item_color: String,
            space_between_centers: DivFixedSize,
            shape: DivRoundedRectangleShape,
            width: DivSize,
            height: DivSize,
            paddings: DivEdgeInsets,
            margins: DivEdgeInsets,
        }
    }
}

// ============================================================
// Animation types
// ============================================================

div_entity! {
    pub struct DivAnimation {
        type_name: "animation",
        fields: {
            #[required]
            name: String,
            duration: i64,
            start_delay: i64,
            interpolator: String,
            repeat_count: i64,
            end_value: f64,
            start_value: f64,
        }
    }
}

div_entity! {
    pub struct DivTransform {
        fields: {
            pivot_x: DivPivot,
            pivot_y: DivPivot,
            rotation: f64,
        }
    }
}

div_entity! {
    pub struct DivPivot {
        fields: {
            r#type: String,
            value: f64,
        }
    }
}

// ============================================================
// Tooltip
// ============================================================

div_entity! {
    pub struct DivTooltip {
        fields: {
            #[required]
            id: String,
            #[required]
            div: DivValue,
            #[required]
            position: String,
            duration: i64,
            offset: DivPoint,
        }
    }
}
