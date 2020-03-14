#![allow(dead_code)]

use std::fmt::{Binary, Display, Error, Formatter, Result};
use std::ops::{BitAnd, BitOr};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
#[derive(Clone, Debug)]
#[repr(u8)]
enum ProcessingOptionFlags {
    AssumeIDs = 0b00000001,
    ParseEntities = 0b00000010,
    AddNamespaces = 0b00000100,
}

#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ProcessingOptions(u8);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for ProcessingOptions {
    fn default() -> Self {
        Self(0)
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ProcessingOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "ProcessingOptions {{")?;

        write!(
            f,
            "{}",
            vec![
                if self.assume_ids() { "AssumeIDs" } else { "" },
                if self.parse_entities() {
                    "ParseEntities"
                } else {
                    ""
                },
                if self.add_namespaces() {
                    "AddNamespaces"
                } else {
                    ""
                },
            ]
            .join(", ")
        )?;

        write!(f, "}}")
    }
}

// ------------------------------------------------------------------------------------------------

impl Binary for ProcessingOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if f.alternate() {
            write!(f, "{:b}", self.0)
        } else {
            write!(f, "{:#b}", self.0)
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl BitAnd for ProcessingOptions {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

// ------------------------------------------------------------------------------------------------

impl BitOr for ProcessingOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

// ------------------------------------------------------------------------------------------------

impl ProcessingOptions {
    fn is_empty(&self) -> bool {
        self.0 == 0
    }
    fn assume_ids(&self) -> bool {
        self.0 & (ProcessingOptionFlags::AssumeIDs as u8) != 0
    }
    fn parse_entities(&self) -> bool {
        self.0 & (ProcessingOptionFlags::ParseEntities as u8) != 0
    }
    fn add_namespaces(&self) -> bool {
        self.0 & (ProcessingOptionFlags::AddNamespaces as u8) != 0
    }
    fn set_assume_ids(&mut self) {
        self.0 = self.0 | (ProcessingOptionFlags::AssumeIDs as u8)
    }
    fn set_parse_entities(&mut self) {
        self.0 = self.0 | (ProcessingOptionFlags::ParseEntities as u8)
    }
    fn set_add_namespaces(&mut self) {
        self.0 = self.0 | (ProcessingOptionFlags::AddNamespaces as u8)
    }
}
