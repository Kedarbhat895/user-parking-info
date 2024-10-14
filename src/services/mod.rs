pub mod user_service;
pub mod user_service_impl;
pub mod user_controller;
pub mod lot_service;
pub mod lot_service_impl;
pub mod lot_controller;

pub use user_service::UserService;
pub use user_service_impl::UserServiceImpl;
pub use lot_service::LotService;
pub use lot_service_impl::LotServiceImpl;
pub use user_controller::{register, login, register_vehicle};
pub use lot_controller::get_available_slots;
