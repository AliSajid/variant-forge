// SPDX-FileCopyrightText: 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#[allow(dead_code)]
pub trait DistanceMetric {
    type Output: Copy + PartialOrd + std::fmt::Debug;
    fn name(&self) -> &'static str;

    fn is_symmetric(&self) -> bool;

    fn get(&self, a: char, b: char) -> Option<Self::Output>;
}
