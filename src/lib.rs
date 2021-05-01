
pub mod game;

#[allow(non_snake_case)]
#[cxx::bridge]
mod ffi {
    impl Vec<Point> {}

    unsafe extern "C++" {
        include!("cxx-test/src/lib.h");

        #[namespace = "LE"]
        pub type Game;

        #[namespace = "LE"]
        type Point = crate::game::Point;

        pub fn _shim_createGame() -> UniquePtr<Game>;
        pub fn _shim_getInts(game: Pin<&mut Game>) -> Vec<i32>;
        pub fn _shim_getPoints(game: Pin<&mut Game>) -> Vec<Point>;

        pub fn debug(self: &Game);
    }
}