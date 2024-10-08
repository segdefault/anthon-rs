use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use strum::EnumCount;
use strum::IntoEnumIterator;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

use crate::common::{Circle, Point2F, Vec2F};

use super::BitString;

#[allow(dead_code)]
enum FeatureState {
    Exists(bool),
    Irrelevant,
}

#[derive(Debug, Copy, Clone, Eq, Derivative, EnumCountMacro, EnumIter)]
#[derivative(PartialEq, Hash)]
pub enum Feature {
    HandRotated,
    IndexClosed,
    MiddleClosed,
    RingClosed,
    PinkyClosed,
    ThumbIndexSpread,
    IndexMiddleSpread,
    MiddleRingSpread,
    RingPinkySpread,
}

impl Display for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let feature = match self {
            Feature::HandRotated => "Rotated",
            Feature::IndexClosed => "Index",
            Feature::MiddleClosed => "Middle",
            Feature::RingClosed => "Ring",
            Feature::PinkyClosed => "Pinky",
            Feature::ThumbIndexSpread => "Thmb Indx",
            Feature::IndexMiddleSpread => "Indx Mdl",
            Feature::MiddleRingSpread => "Mdl Rng",
            Feature::RingPinkySpread => "Rng Pnk",
        };

        write!(f, "{}", feature)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Sign {
    required_attributes: BitString,
    irrelevant_attributes: BitString,
}

impl Sign {
    pub fn required_attributes(&self) -> &BitString {
        &self.required_attributes
    }

    pub fn irrelevant_attributes(&self) -> &BitString {
        &self.irrelevant_attributes
    }

    pub fn set_feature(&mut self, feature_index: usize, irrelevant: bool, required: bool) {
        if irrelevant {
            self.irrelevant_attributes.set(feature_index);
        } else {
            self.irrelevant_attributes.unset(feature_index);
            if required {
                self.required_attributes.set(feature_index);
            } else {
                self.required_attributes.unset(feature_index);
            }
        }
    }
}

impl Default for Sign {
    fn default() -> Self {
        Self {
            required_attributes: BitString::new(0, Feature::COUNT),
            irrelevant_attributes: BitString::new(usize::MAX, Feature::COUNT),
        }
    }
}

impl cmp::PartialEq<Sign> for Sign {
    fn eq(&self, other: &Sign) -> bool {
        let irrelevant = self.irrelevant_attributes.bits() | other.irrelevant_attributes.bits();

        let a = irrelevant | self.required_attributes.bits();
        let b = irrelevant | other.required_attributes.bits();

        a ^ b == 0
    }
}

impl fmt::Debug for Sign {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, feature) in Feature::iter().enumerate() {
            let value = self.required_attributes.get(i) | self.irrelevant_attributes.get(i);
            writeln!(f, "{}: {}", feature, value)?;
        }

        Ok(())
    }
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "Sign (relevant, irrelevant): ({:#b}, {:#b})",
            self.required_attributes.bits(),
            self.irrelevant_attributes.bits(),
        )
    }
}

impl From<&[Point2F]> for Sign {
    fn from(landmarks: &[Point2F]) -> Self {
        let mut features = HashMap::new();
        let palm_circle = Circle::from(vec![landmarks[0], landmarks[5], landmarks[17]].as_ref());

        let hand_angle = Vec2F::from((landmarks[0], landmarks[17]))
            .angle(&Vec2F { x: 0f32, y: 1f32 })
            .to_degrees();

        let thumb_index_angle = Vec2F::from((landmarks[0], landmarks[4]))
            .angle(&Vec2F::from((landmarks[0], landmarks[5])))
            .to_degrees();
        let index_middle_angle = Vec2F::from((landmarks[5], landmarks[8]))
            .angle(&Vec2F::from((landmarks[9], landmarks[12])))
            .to_degrees();
        let middle_ring_angle = Vec2F::from((landmarks[9], landmarks[12]))
            .angle(&Vec2F::from((landmarks[13], landmarks[16])))
            .to_degrees();
        let ring_pinky_angle = Vec2F::from((landmarks[13], landmarks[16]))
            .angle(&Vec2F::from((landmarks[17], landmarks[20])))
            .to_degrees();

        features.insert(
            Feature::IndexClosed,
            FeatureState::Exists(palm_circle.contains(&landmarks[8])),
        );
        features.insert(
            Feature::MiddleClosed,
            FeatureState::Exists(palm_circle.contains(&landmarks[12])),
        );
        features.insert(
            Feature::RingClosed,
            FeatureState::Exists(palm_circle.contains(&landmarks[16])),
        );
        features.insert(
            Feature::PinkyClosed,
            FeatureState::Exists(palm_circle.contains(&landmarks[20])),
        );
        features.insert(
            Feature::HandRotated,
            FeatureState::Exists(hand_angle < 140f32),
        );
        features.insert(
            Feature::ThumbIndexSpread,
            FeatureState::Exists(thumb_index_angle > 10f32),
        );
        features.insert(
            Feature::IndexMiddleSpread,
            FeatureState::Exists(index_middle_angle > 10f32),
        );
        features.insert(
            Feature::MiddleRingSpread,
            FeatureState::Exists(middle_ring_angle > 10f32),
        );
        features.insert(
            Feature::RingPinkySpread,
            FeatureState::Exists(ring_pinky_angle > 10f32),
        );

        features.into()
    }
}

impl From<HashMap<Feature, FeatureState>> for Sign {
    fn from(attributes: HashMap<Feature, FeatureState>) -> Self {
        (&attributes).into()
    }
}

impl From<&HashMap<Feature, FeatureState>> for Sign {
    fn from(attributes: &HashMap<Feature, FeatureState>) -> Self {
        let mut sign = Sign::default();

        attributes.iter().for_each(|(attr, state)| {
            if let FeatureState::Exists(exists) = state {
                let index = *attr as usize;

                sign.irrelevant_attributes.unset(index);
                if *exists {
                    sign.required_attributes.set(index);
                }
            }
        });

        sign
    }
}
