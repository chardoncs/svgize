use crate::error::Error;

use std::str::FromStr;

pub enum ReferrerPolicy {
    NoReferrer,
    NoReferrerWhenDowngrade,
    Origin,
    OriginWhenCrossOrigin,
    SameOrigin,
    StrictOrigin,
    StrictOriginWhenCrossOrigin,
    UnsafeUrl,
}

impl FromStr for ReferrerPolicy {
    type Err = Error;

    fn from_str(s:&str) -> Result<Self, Self::Err> {
        todo!();
        match s {
            _ => Err(Error::NoOptionFound),
        }
    }
}
