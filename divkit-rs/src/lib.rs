//! # divkit-json-builder
//!
//! A Rust port of the DivKit Python json-builder.
//!
//! Provides entity types with `build()`/`dict()` and `schema()` methods
//! for constructing DivKit JSON layouts programmatically.
//!
//! ## Quick Start
//!
//! ```rust
//! use divkit_json_builder::div::*;
//! use divkit_json_builder::entity::Entity;
//! use divkit_json_builder::value::DivValue;
//!
//! let text = DivText::new().text("Hello from Rust!");
//! let json = text.dict();
//! // => {"type": "text", "text": "Hello from Rust!"}
//!
//! let text2 = DivText::new().text("Hello from Rust!");
//! let container = DivContainer::new()
//!     .items(vec![DivValue::from(text2)]);
//! let json = container.dict();
//! // => {"type": "container", "items": [{"type": "text", "text": "Hello from Rust!"}]}
//! ```

#[macro_use]
pub mod macros;

pub mod div;
pub mod entity;
pub mod expr;
pub mod field;
pub mod schema;
pub mod value;

// Python bindings
pub mod py_entity;
pub mod py_value;
pub mod python;

// Re-export commonly used items
pub use entity::{make_card, make_div, DivData, DivDataState, Entity, Template};
pub use expr::Expr;
pub use field::{Constraints, FieldBuilder, FieldDescriptor};
pub use value::DivValue;

#[cfg(test)]
mod tests {
    use super::div::*;
    use super::entity::Entity;
    use super::expr::Expr;
    use super::value::DivValue;
    use serde_json::json;

    #[test]
    fn test_div_text_basic() {
        let text = DivText::new().text("Hello from pydivkit");
        let result = text.dict();
        assert_eq!(
            result,
            json!({"type": "text", "text": "Hello from pydivkit"})
        );
    }

    #[test]
    fn test_div_text_build() {
        let text = DivText::new().text("Hello");
        let result = text.build();
        assert_eq!(result, json!({"type": "text", "text": "Hello"}));
    }

    #[test]
    fn test_div_fixed_size() {
        let size = DivFixedSize::new().value(32);
        assert_eq!(size.dict(), json!({"type": "fixed", "value": 32}));
    }

    #[test]
    fn test_div_wrap_content_size() {
        let size = DivWrapContentSize::new();
        assert_eq!(size.dict(), json!({"type": "wrap_content"}));
    }

    #[test]
    fn test_div_match_parent_size() {
        let size = DivMatchParentSize::new();
        assert_eq!(size.dict(), json!({"type": "match_parent"}));
    }

    #[test]
    fn test_div_edge_insets() {
        let insets = DivEdgeInsets::new()
            .left(12)
            .right(12)
            .top(10)
            .bottom(10);
        assert_eq!(
            insets.dict(),
            json!({"left": 12, "right": 12, "top": 10, "bottom": 10})
        );
    }

    #[test]
    fn test_div_border() {
        let border = DivBorder::new().corner_radius(12);
        assert_eq!(border.dict(), json!({"corner_radius": 12}));
    }

    #[test]
    fn test_div_solid_background() {
        let bg = DivSolidBackground::new().color("#f0f0f0");
        assert_eq!(bg.dict(), json!({"type": "solid", "color": "#f0f0f0"}));
    }

    #[test]
    fn test_div_container() {
        let text = DivText::new().text("Hello from pydivkit");
        let gallery = DivGallery::new().items(vec![DivValue::from(text)]);
        let container = DivContainer::new().items(vec![DivValue::from(gallery)]);

        let result = container.dict();
        assert_eq!(
            result,
            json!({
                "type": "container",
                "items": [{
                    "type": "gallery",
                    "items": [{
                        "type": "text",
                        "text": "Hello from pydivkit"
                    }]
                }]
            })
        );
    }

    #[test]
    fn test_div_image() {
        let img = DivImage::new()
            .image_url("https://example.com/image.png")
            .width(DivFixedSize::new().value(100))
            .height(DivFixedSize::new().value(100));
        let result = img.dict();
        assert_eq!(
            result,
            json!({
                "type": "image",
                "image_url": "https://example.com/image.png",
                "width": {"type": "fixed", "value": 100},
                "height": {"type": "fixed", "value": 100}
            })
        );
    }

    #[test]
    fn test_slider_example() {
        let slider = DivSlider::new()
            .width(DivMatchParentSize::new())
            .max_value(10)
            .min_value(1)
            .thumb_style(
                DivShapeDrawable::new()
                    .color("#00b300")
                    .stroke(DivStroke::new().color("#ffffff").width(3))
                    .shape(
                        DivRoundedRectangleShape::new()
                            .item_width(DivFixedSize::new().value(32))
                            .item_height(DivFixedSize::new().value(32))
                            .corner_radius(DivFixedSize::new().value(100)),
                    ),
            )
            .track_active_style(
                DivShapeDrawable::new()
                    .color("#00b300")
                    .shape(
                        DivRoundedRectangleShape::new()
                            .item_height(DivFixedSize::new().value(6)),
                    ),
            )
            .track_inactive_style(
                DivShapeDrawable::new()
                    .color("#20000000")
                    .shape(
                        DivRoundedRectangleShape::new()
                            .item_height(DivFixedSize::new().value(6)),
                    ),
            );

        let result = slider.dict();

        assert_eq!(result["type"], "slider");
        assert_eq!(result["max_value"], 10);
        assert_eq!(result["min_value"], 1);
        assert_eq!(result["width"]["type"], "match_parent");
        assert_eq!(result["thumb_style"]["type"], "shape_drawable");
        assert_eq!(result["thumb_style"]["color"], "#00b300");
        assert_eq!(result["thumb_style"]["stroke"]["color"], "#ffffff");
        assert_eq!(result["thumb_style"]["stroke"]["width"], 3);
        assert_eq!(
            result["thumb_style"]["shape"]["type"],
            "rounded_rectangle"
        );
        assert_eq!(result["thumb_style"]["shape"]["corner_radius"]["value"], 100);
        assert_eq!(result["thumb_style"]["shape"]["item_width"]["value"], 32);
        assert_eq!(result["thumb_style"]["shape"]["item_height"]["value"], 32);
        assert_eq!(result["track_active_style"]["color"], "#00b300");
        assert_eq!(result["track_inactive_style"]["color"], "#20000000");
    }

    #[test]
    fn test_schema_basic() {
        let text = DivText::new();
        let schema = text.schema(None);
        assert_eq!(schema["type"], "object");

        let props = schema["properties"].as_object().unwrap();
        assert!(props.contains_key("type"));
        assert!(props.contains_key("text"));
        assert!(props.contains_key("font_size"));
    }

    #[test]
    fn test_schema_exclude_fields() {
        let text = DivText::new();
        let schema = text.schema(Some(&["accessibility", "actions"]));

        let props = schema["properties"].as_object().unwrap();
        assert!(!props.contains_key("accessibility"));
        assert!(!props.contains_key("actions"));
        assert!(props.contains_key("text"));
    }

    #[test]
    fn test_expr() {
        assert!(Expr::new("@{value}").is_ok());
        assert!(Expr::new("@{some_expr + 1}").is_ok());
        assert!(Expr::new("not_an_expr").is_err());
        assert!(Expr::new("@{before").is_err());
        assert!(Expr::new("{before").is_err());
        assert!(Expr::new("after}").is_err());

        let e = Expr::new("@{my_var}").unwrap();
        assert_eq!(e.to_string(), "@{my_var}");
    }

    #[test]
    fn test_div_enum_values() {
        assert_eq!(DivAlignmentVertical::Top.value(), "top");
        assert_eq!(DivAlignmentVertical::Center.value(), "center");
        assert_eq!(DivAlignmentVertical::Bottom.value(), "bottom");
        assert_eq!(DivAlignmentVertical::Baseline.value(), "baseline");

        assert_eq!(DivContainerOrientation::Horizontal.value(), "horizontal");
        assert_eq!(DivContainerOrientation::Vertical.value(), "vertical");
    }

    #[test]
    fn test_container_with_enum() {
        let container = DivContainer::new()
            .orientation(DivContainerOrientation::Horizontal)
            .content_alignment_vertical(DivAlignmentVertical::Center)
            .items(vec![DivValue::from(DivText::new().text("Hi"))]);

        let result = container.dict();
        assert_eq!(result["orientation"], "horizontal");
        assert_eq!(result["content_alignment_vertical"], "center");
    }

    #[test]
    fn test_nested_entity_composition() {
        let container = DivContainer::new()
            .width(DivWrapContentSize::new())
            .background(vec![DivValue::from(DivSolidBackground::new().color("#f0f0f0"))])
            .border(DivBorder::new().corner_radius(12))
            .paddings(
                DivEdgeInsets::new()
                    .left(12)
                    .right(12)
                    .top(10)
                    .bottom(10),
            )
            .items(vec![
                DivValue::from(DivImage::new()
                    .width(DivFixedSize::new().value(20))
                    .height(DivFixedSize::new().value(20))
                    .margins(DivEdgeInsets::new().right(6))
                    .image_url("https://example.com/icon.png")),
                DivValue::from(DivText::new()
                    .width(DivWrapContentSize::new())
                    .max_lines(1)
                    .text("Category")),
            ]);

        let result = container.dict();
        assert_eq!(result["type"], "container");
        assert_eq!(result["width"]["type"], "wrap_content");
        assert_eq!(result["background"][0]["type"], "solid");
        assert_eq!(result["background"][0]["color"], "#f0f0f0");
        assert_eq!(result["border"]["corner_radius"], 12);
        assert_eq!(result["paddings"]["left"], 12);
        assert_eq!(result["items"].as_array().unwrap().len(), 2);
        assert_eq!(result["items"][0]["type"], "image");
        assert_eq!(result["items"][1]["type"], "text");
        assert_eq!(result["items"][1]["text"], "Category");
    }

    #[test]
    fn test_div_data() {
        let text = DivText::new().text("Hello");
        let data = super::DivData {
            log_id: "sample_card".to_string(),
            states: vec![super::DivDataState {
                state_id: 0,
                div: Box::new(text),
            }],
        };

        let result = data.dict();
        assert_eq!(result["log_id"], "sample_card");
        assert_eq!(result["states"][0]["state_id"], 0);
        assert_eq!(result["states"][0]["div"]["type"], "text");
        assert_eq!(result["states"][0]["div"]["text"], "Hello");
    }

    #[test]
    fn test_field_builder() {
        let fb = super::FieldBuilder::new()
            .description("A test field")
            .gt(0.0)
            .le(100.0)
            .min_length(1);
        let desc = fb.build("test_field");
        assert_eq!(desc.name, "test_field");
        assert_eq!(desc.description.as_deref(), Some("A test field"));
        assert_eq!(desc.constraints.exclusive_minimum, Some(0.0));
        assert_eq!(desc.constraints.maximum, Some(100.0));
        assert_eq!(desc.constraints.min_length, Some(1));
    }

    #[test]
    fn test_gallery_with_items() {
        let gallery = DivGallery::new().items(vec![
            DivValue::from(DivText::new().text("Item 1")),
            DivValue::from(DivText::new().text("Item 2")),
            DivValue::from(DivText::new().text("Item 3")),
        ]);

        let result = gallery.dict();
        assert_eq!(result["type"], "gallery");
        let items = result["items"].as_array().unwrap();
        assert_eq!(items.len(), 3);
        assert_eq!(items[0]["text"], "Item 1");
        assert_eq!(items[1]["text"], "Item 2");
        assert_eq!(items[2]["text"], "Item 3");
    }
}
