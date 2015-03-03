// Re-export a Function inside another Module to present External Interface
// that does not directly map to internal code organisation
// 'pub use' declaration brings Functions into scope at this level of 
// Module hierarchy so internal organisation does not define the External Interface
pub use self::greetings::guten_tag;
pub use self::farewells::auf_wiedersehen;

pub mod greetings; // greetings.rs

pub mod farewells; // farewells.rs