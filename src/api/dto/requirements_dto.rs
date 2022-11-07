use lazy_static::lazy_static;
use std::collections::hash_map;

pub type RequirementsDto = u16;

lazy_static! {
    pub static ref REQUIREMENTS: hash_map::HashMap<String, RequirementsDto> =
        hash_map::HashMap::from([
            ("WeaknessToFire".to_owned(), 0),
            ("WeaknessToMagic".to_owned(), 1),
            ("WeaknessToIce".to_owned(), 2),
        ]);
    pub static ref REQUIREMENTS_REVERSE: hash_map::HashMap<RequirementsDto, String> =
        hash_map::HashMap::from([
            (0, "WeaknessToFire".to_owned()),
            (1, "WeaknessToMagic".to_owned()),
            (2, "WeaknessToIce".to_owned()),
        ]);
}

pub trait RequirementsDtoMut {
    fn dehydrate(requirements: String) -> RequirementsDto {
        REQUIREMENTS[&requirements]
    }

    fn hydrate(requirements: &RequirementsDto) -> String {
        REQUIREMENTS_REVERSE[requirements].to_owned()
    }
}

impl RequirementsDtoMut for RequirementsDto {}
