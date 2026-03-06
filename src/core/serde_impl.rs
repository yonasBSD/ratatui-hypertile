use crate::core::helpers::normalize_ratio;
use crate::core::{Node, PaneId};
use ratatui::layout::Direction;

impl serde::Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        match self {
            Node::Pane(id) => serializer.serialize_u64(id.get()),
            Node::Split {
                direction,
                ratio,
                first,
                second,
            } => {
                let direction = match direction {
                    Direction::Horizontal => "horizontal",
                    Direction::Vertical => "vertical",
                };

                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry("direction", direction)?;
                map.serialize_entry("ratio", ratio)?;
                map.serialize_entry("first", first)?;
                map.serialize_entry("second", second)?;
                map.end()
            }
        }
    }
}

impl<'de> serde::Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct NodeVisitor;

        impl<'de> serde::de::Visitor<'de> for NodeVisitor {
            type Value = Node;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a pane id integer or a split map")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Node::Pane(PaneId::new(value)))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if value < 0 {
                    return Err(E::custom("pane id must be >= 0"));
                }
                Ok(Node::Pane(PaneId::new(value as u64)))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut direction = None;
                let mut ratio = None;
                let mut first = None;
                let mut second = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "direction" => {
                            let raw: String = map.next_value()?;
                            direction = Some(match raw.as_str() {
                                "horizontal" => Direction::Horizontal,
                                "vertical" => Direction::Vertical,
                                _ => {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(raw.as_str()),
                                        &"horizontal or vertical",
                                    ));
                                }
                            });
                        }
                        "ratio" => ratio = Some(map.next_value::<f32>()?),
                        "first" => first = Some(map.next_value::<Node>()?),
                        "second" => second = Some(map.next_value::<Node>()?),
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let direction =
                    direction.ok_or_else(|| serde::de::Error::missing_field("direction"))?;
                let ratio = ratio.ok_or_else(|| serde::de::Error::missing_field("ratio"))?;
                let first = first.ok_or_else(|| serde::de::Error::missing_field("first"))?;
                let second = second.ok_or_else(|| serde::de::Error::missing_field("second"))?;

                Ok(Node::Split {
                    direction,
                    ratio: normalize_ratio(ratio),
                    first: Box::new(first),
                    second: Box::new(second),
                })
            }
        }

        deserializer.deserialize_any(NodeVisitor)
    }
}
