// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-user-feature-61: user feature 61 component model.
//!
//! Real implementation: features, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Feature {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserFeatureSixOneError {
    EmptyFeatureName,
    DuplicateFeature(String),
    EmptyEventSubject,
}

impl fmt::Display for UserFeatureSixOneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyFeatureName => write!(f, "feature name must not be empty"),
            Self::DuplicateFeature(id) => write!(f, "duplicate feature {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for UserFeatureSixOneError {}

pub struct UserFeatureSixOne {
    pub name: String,
    features: BTreeMap<String, Feature>,
    events: BTreeMap<String, Event>,
}

impl UserFeatureSixOne {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            features: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_feature(&mut self, item: Feature) -> Result<(), UserFeatureSixOneError> {
        if item.name.trim().is_empty() {
            return Err(UserFeatureSixOneError::EmptyFeatureName);
        }
        let id = item.id.clone();
        if self.features.contains_key(&id) {
            return Err(UserFeatureSixOneError::DuplicateFeature(id.clone()));
        }
        self.features.insert(id, item);
        Ok(())
    }

    pub fn feature_count(&self) -> usize {
        self.features.len()
    }

    pub fn get_feature(&self, id: &str) -> Option<&Feature> {
        self.features.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), UserFeatureSixOneError> {
        if event.subject.trim().is_empty() {
            return Err(UserFeatureSixOneError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_feature(id: &str, name: &str) -> Feature {
        Feature {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = UserFeatureSixOne::new("UserFeatureSixOne");
        assert_eq!(m.name, "UserFeatureSixOne");
        assert_eq!(m.feature_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_feature() {
        let mut m = UserFeatureSixOne::new("UserFeatureSixOne");
        m.add_feature(sample_feature("a1", "Alpha")).unwrap();
        assert_eq!(m.feature_count(), 1);
        assert!(m.get_feature("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = UserFeatureSixOne::new("UserFeatureSixOne");
        assert!(m.add_feature(sample_feature("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = UserFeatureSixOne::new("UserFeatureSixOne");
        m.add_feature(sample_feature("x", "One")).unwrap();
        assert!(m.add_feature(sample_feature("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = UserFeatureSixOne::new("UserFeatureSixOne");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
