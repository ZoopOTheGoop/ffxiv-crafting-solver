#![allow(clippy::assertions_on_constants)]

use super::{tables::NORMAL_CONDITIONS, *};
use rand::{prelude::StdRng, SeedableRng};
use std::collections::HashMap;

mod no_qa;
mod qa;
mod relic_conds;

const RAND_ANALYSIS_STEPS: usize = 10_000_000;
// 0.1% error allowed
const DISTRIBUTION_ANALYSIS_ALLOWED_ERROR: f64 = 0.1 / 100.;
