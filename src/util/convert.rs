// Copyright (C) 2018  Project Tsukurou!
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Definitions for non-standard conversions `TryAsRef` and `TryAsMut`.

/// An analogue of `AsRef` that allows failure, useful when the inner
/// referenced value isn't present due to an enum mismatch.
pub trait TryAsRef<T>
    where
        T: ?Sized,
{
    /// Attempt to unwrap the inner reference.
    fn try_as_ref(&self) -> Option<&T>;
}

/// An analogue of `AsMut` that allows failure, useful when the inner
/// referenced value isn't present due to an enum mismatch.
pub trait TryAsMut<T>
    where
        T: ?Sized,
{
    /// Attempt to unwrap the inner reference.
    fn try_as_mut(&mut self) -> Option<&mut T>;
}

impl<T, U> TryAsRef<U> for T
    where
        T: AsRef<U>,
{
    fn try_as_ref(&self) -> Option<&U> {
        Some(self.as_ref())
    }
}

impl<T, U> TryAsMut<U> for T
    where
        T: AsMut<U>,
{
    fn try_as_mut(&mut self) -> Option<&mut U> {
        Some(self.as_mut())
    }
}