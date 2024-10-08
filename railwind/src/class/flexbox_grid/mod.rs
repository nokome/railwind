mod types;

pub use types::*;

use crate::class::Decl;
use crate::utils::{get_args, get_class_name, get_opt_args};
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref BASIS: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("basis.ron")).unwrap();
    pub static ref FLEX: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("flex.ron")).unwrap();
    pub static ref GROW: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grow.ron")).unwrap();
    pub static ref SHRINK: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("shrink.ron")).unwrap();
    pub static ref ORDER: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("order.ron")).unwrap();
    pub static ref GRID_TEMPLATE_COLUMNS: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grid_template_columns.ron")).unwrap();
    pub static ref GRID_COLUMN_SPAN: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grid_column_span.ron")).unwrap();
    pub static ref GRID_COLUMN_START: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grid_column_start_end.ron")).unwrap();
    pub static ref GRID_COLUMN_END: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grid_column_start_end.ron")).unwrap();
    pub static ref GRID_TEMPLATE_ROWS: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grid_template_rows.ron")).unwrap();
    pub static ref GRID_ROW_SPAN: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grid_row_span.ron")).unwrap();
    pub static ref GRID_ROW_START: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grid_row_start_end.ron")).unwrap();
    pub static ref GRID_ROW_END: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grid_row_start_end.ron")).unwrap();
    pub static ref GRID_AUTO_COLUMNS: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grid_auto_columns.ron")).unwrap();
    pub static ref GRID_AUTO_ROWS: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("grid_auto_rows.ron")).unwrap();
    pub static ref GAP: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("gap.ron")).unwrap();
    pub static ref GAP_X: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("gap.ron")).unwrap();
    pub static ref GAP_Y: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("gap.ron")).unwrap();
}

#[derive(Debug)]
pub enum FlexboxGrid<'a> {
    Basis(Basis<'a>),
    Direction(Direction),
    Wrap(Wrap),
    Flex(Flex<'a>),
    Grow(Grow<'a>),
    Shrink(Shrink<'a>),
    Order(Order<'a>),
    GridTemplateColumns(GridTemplateColumns<'a>),
    GridColumn(GridColumn<'a>),
    GridTepmlateRows(GridTepmlateRows<'a>),
    GridRow(GridRow<'a>),
    GridAutoFlow(GridAutoFlow),
    GridAutoColumns(GridAutoColumns<'a>),
    GridAutoRows(GridAutoRows<'a>),
    Gap(Gap<'a>),
    JustifyContent(JustifyContent),
    JustifyItems(JustifyItems),
    JustifySelf(JustifySelf),
    AlignContent(AlignContent),
    AlignItems(AlignItems),
    AlignSelf(AlignSelf),
    PlaceContent(PlaceContent),
    PlaceItems(PlaceItems),
    PlaceSelf(PlaceSelf),
}

impl<'a> FlexboxGrid<'a> {
    pub fn new(value: &'a str) -> Result<Option<Self>, WarningType> {
        let flexbox_grid = match get_class_name(value) {
            "basis" => FlexboxGrid::Basis(Basis(get_args(value)?)),
            "flex" => {
                let args = get_args(value)?;

                if let Some(direction) = Direction::new(args) {
                    FlexboxGrid::Direction(direction)
                } else if let Some(wrap) = Wrap::new(args) {
                    FlexboxGrid::Wrap(wrap)
                } else {
                    FlexboxGrid::Flex(Flex(args))
                }
            }
            "grow" => FlexboxGrid::Grow(Grow(get_opt_args(value))),
            "shrink" => FlexboxGrid::Shrink(Shrink(get_opt_args(value))),
            "order" | "-order" => {
                FlexboxGrid::Order(Order::new(get_class_name(value), get_args(value)?))
            }
            "grid" => {
                let args = get_args(value)?;

                match get_class_name(args) {
                    "cols" => {
                        FlexboxGrid::GridTemplateColumns(GridTemplateColumns(get_args(args)?))
                    }
                    "rows" => FlexboxGrid::GridTepmlateRows(GridTepmlateRows(get_args(args)?)),
                    "flow" => FlexboxGrid::GridAutoFlow(GridAutoFlow::new(get_args(args)?)?),
                    v => {
                        return Err(WarningType::InvalidArg(
                            v.into(),
                            "Grid Template / Auto Flow".into(),
                            vec!["cols", "rows", "flow"],
                        ))
                    }
                }
            }
            "col" => FlexboxGrid::GridColumn(GridColumn::new(get_args(value)?)?),
            "row" => FlexboxGrid::GridRow(GridRow::new(get_args(value)?)),
            "auto" => {
                let args = get_args(value)?;

                match get_class_name(args) {
                    "cols" => FlexboxGrid::GridAutoColumns(GridAutoColumns(get_args(args)?)),
                    "rows" => FlexboxGrid::GridAutoRows(GridAutoRows(get_args(args)?)),
                    v => {
                        return Err(WarningType::InvalidArg(
                            v.into(),
                            "Grid Auto Columns / Rows".into(),
                            vec!["cols", "rows"],
                        ))
                    }
                }
            }
            "gap" => FlexboxGrid::Gap(Gap(get_args(value)?)),
            "justify" => {
                let args = get_args(value)?;

                if let Some(content) = JustifyContent::new(args) {
                    FlexboxGrid::JustifyContent(content)
                } else {
                    match get_class_name(args) {
                        "items" => FlexboxGrid::JustifyItems(JustifyItems::new(get_args(args)?)?),
                        "self" => FlexboxGrid::JustifySelf(JustifySelf::new(get_args(args)?)?),
                        v => {
                            return Err(WarningType::InvalidArg(
                                v.into(),
                                "Justify Items / Self".into(),
                                vec![
                                    "start", "end", "center", "between", "around", "evenly",
                                    "items", "self",
                                ],
                            ))
                        }
                    }
                }
            }
            "content" => {
                if let Some(align_content) = AlignContent::new(get_args(value)?) {
                    FlexboxGrid::AlignContent(align_content)
                } else {
                    return Ok(None);
                }
            }
            "items" => FlexboxGrid::AlignItems(AlignItems::new(get_args(value)?)?),
            "self" => FlexboxGrid::AlignSelf(AlignSelf::new(get_args(value)?)?),
            "place" => {
                let args = get_args(value)?;

                match get_class_name(args) {
                    "content" => FlexboxGrid::PlaceContent(PlaceContent::new(get_args(args)?)?),
                    "items" => FlexboxGrid::PlaceItems(PlaceItems::new(get_args(args)?)?),
                    "self" => FlexboxGrid::PlaceSelf(PlaceSelf::new(get_args(args)?)?),
                    v => {
                        return Err(WarningType::InvalidArg(
                            v.into(),
                            "Place Content / Items / Self".into(),
                            vec!["content", "items", "self"],
                        ))
                    }
                }
            }
            _ => return Ok(None),
        };

        Ok(Some(flexbox_grid))
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        match self {
            FlexboxGrid::Basis(fg) => fg.to_decl(),
            FlexboxGrid::Direction(fg) => Ok(fg.to_decl()),
            FlexboxGrid::Wrap(fg) => Ok(fg.to_decl()),
            FlexboxGrid::Flex(fg) => fg.to_decl(),
            FlexboxGrid::Grow(fg) => fg.to_decl(),
            FlexboxGrid::Shrink(fg) => fg.to_decl(),
            FlexboxGrid::Order(fg) => fg.to_decl(),
            FlexboxGrid::GridTemplateColumns(fg) => fg.to_decl(),
            FlexboxGrid::GridColumn(fg) => fg.to_decl(),
            FlexboxGrid::GridTepmlateRows(fg) => fg.to_decl(),
            FlexboxGrid::GridRow(fg) => fg.to_decl(),
            FlexboxGrid::GridAutoFlow(fg) => Ok(fg.to_decl()),
            FlexboxGrid::GridAutoColumns(fg) => fg.to_decl(),
            FlexboxGrid::GridAutoRows(fg) => fg.to_decl(),
            FlexboxGrid::Gap(fg) => fg.to_decl(),
            FlexboxGrid::JustifyContent(fg) => Ok(fg.to_decl()),
            FlexboxGrid::JustifyItems(fg) => Ok(fg.to_decl()),
            FlexboxGrid::JustifySelf(fg) => Ok(fg.to_decl()),
            FlexboxGrid::AlignContent(fg) => Ok(fg.to_decl()),
            FlexboxGrid::AlignItems(fg) => Ok(fg.to_decl()),
            FlexboxGrid::AlignSelf(fg) => Ok(fg.to_decl()),
            FlexboxGrid::PlaceContent(fg) => Ok(fg.to_decl()),
            FlexboxGrid::PlaceItems(fg) => Ok(fg.to_decl()),
            FlexboxGrid::PlaceSelf(fg) => Ok(fg.to_decl()),
        }
    }
}
