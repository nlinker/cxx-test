use crate::ffi;
use cxx::UniquePtr;
use std::pin::Pin;

// Mirrors its counterpart LE::Point
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

// required for ffi layer
unsafe impl cxx::ExternType for Point {
    type Id = cxx::type_id!("LE::Point");
    type Kind = cxx::kind::Trivial;
}

pub struct Game {
    pub(crate) raw: *mut ffi::Game,
}

impl Game {
    pub fn new() -> Self {
        let game: UniquePtr<ffi::Game> = ffi::_shim_createGame();
        Self { raw: game.into_raw() }
    }

    pub fn get_ints(&self) -> Vec<i32> {
        let game = unsafe { Pin::new_unchecked(&mut *self.raw) };
        ffi::_shim_getInts(game)
    }

    pub fn get_points(&self) -> Vec<Point> {
        let game = unsafe { Pin::new_unchecked(&mut *self.raw) };
        ffi::_shim_getPoints(game)
    }
}
