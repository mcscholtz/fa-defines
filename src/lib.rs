use yew_common::traits::{ 
    ToClass,
    ToDynClass
};

#[derive(Clone, Copy, PartialEq)]
pub enum Icons {
    Envelope,
    Check,
    Cross,
    User,
    Loading,
}

impl ToClass for Icons {
    fn to_class(&self) -> &'static str
    {
        match &self {
            Icons::Envelope => "fa-envelope",
            Icons::Check => "fa-check",
            Icons::Cross => "fa-times",
            Icons::User => "fa-user",
            Icons::Loading => "fa-spinner"
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Style {
    Solid,
    Regular,
    Light,
    Duotone,
    Brands
}

impl ToClass for Style {
    fn to_class(&self) -> &'static str
    {
        match &self {
            Style::Solid => "fas",
            Style::Regular => "far",
            Style::Light => "fal",
            Style::Duotone => "fad",
            Style::Brands => "fab"
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Size {
    Xs,
    Sm,
    Lg,
    X2,
    X3,
    X5,
    X7,
    X10
}

impl ToClass for Size {
    fn to_class(&self) -> &'static str
    {
        match &self {
            Size::Xs => "fa-xs",
            Size::Sm => "fa-sm",
            Size::Lg => "fa-lg",
            Size::X2 => "fa-2x",
            Size::X3 => "fa-3x",
            Size::X5 => "fa-5x",
            Size::X7 => "fa-7x",
            Size::X10 => "fa-10x"
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Layout {
    Border,
    Inverse,
    PullLeft,
    PullRight,
    StackTop,
    StackBottom
}

impl ToClass for Layout {
    fn to_class(&self) -> &'static str
    {
        match &self {
            Layout::Border => "fa-border",
            Layout::Inverse => "fa-inverse",
            Layout::PullLeft => "fa-pull-left",
            Layout::PullRight => "fa-pull-right",
            Layout::StackTop => "fa-stack-2x",
            Layout::StackBottom => "fa-stack-1x",
        }
    }
}


#[derive(Clone, Copy, PartialEq)]
pub enum Animation {
    Spin,
    Pulse
}

impl ToClass for Animation {
    fn to_class(&self) -> &'static str
    {
        match &self {
            Animation::Spin => "fa-spin",
            Animation::Pulse => "fa-pulse"
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Rotation {
    Rotate90,
    Rotate180,
    Rotate270,
    FlipHorizontal,
    FlipVertical,
    FlipBoth,
}

impl ToClass for Rotation {
    fn to_class(&self) -> &'static str
    {
        match &self {
            Rotation::Rotate90 => "fa-rotate-90",
            Rotation::Rotate180 => "fa-rotate-180",
            Rotation::Rotate270 => "fa-rotate-270",
            Rotation::FlipHorizontal => "fa-flip-horizontal",
            Rotation::FlipVertical => "fa-flip-vertical",
            Rotation::FlipBoth => "fa-flip-both"
        }
    }
}

// Used liked this:
// <i class="fas fa-seedling" data-fa-transform="shrink-8" style="background:MistyRose"></i>
// Where the field is data-fa-transform
#[derive(Clone, Copy, PartialEq)]
pub enum Transform {
    Grow(u8),
    Shrink(u8),
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8),
    Rotate(i16),
    FlipH,
    FlipV
}

impl ToDynClass for Transform {
    fn to_class(&self) -> String
    {
        match self {
            Transform::Grow(u) => format!("grow-{}", u),
            Transform::Shrink(u) => format!("shrink-{}", u),
            Transform::Up(u) => format!("up-{}", u),
            Transform::Down(u) => format!("down-{}", u),
            Transform::Left(u) => format!("left-{}", u),
            Transform::Right(u) => format!("right-{}", u),
            Transform::Rotate(i) => format!("rotate-{}", i),
            Transform::FlipH => "flip-h".to_owned(),
            Transform::FlipV => "flip-v".to_owned()
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Props {
    Icon(Icons),
    Style(Style),
    Size(Size),
    Rotation(Rotation),
    Animation(Animation),
    Layout(Layout),
    FixedWidth
}

impl ToClass for Props {
    fn to_class(&self) -> &'static str {
        match self {
            Props::Icon(icon) => icon.to_class(),
            Props::Style(style) => style.to_class(),
            Props::Size(size) => size.to_class(),
            Props::Rotation(rot) => rot.to_class(),
            Props::Animation(ani) => ani.to_class(),
            Props::Layout(layout) => layout.to_class(),
            Props::FixedWidth => "fa-fw"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Transform;
    use common::traits::ToDynClass;
    #[test]
    fn test_transform_pos() {
        let val = Transform::Rotate(123);
        assert_eq!(val.to_class(), "rotate-123");
    }

    #[test]
    fn test_transform_neg() {
        let val = Transform::Rotate(-123);
        assert_eq!(val.to_class(), "rotate--123");
    }
}