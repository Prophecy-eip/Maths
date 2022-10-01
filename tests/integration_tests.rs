use maths::fight;
mod initialize_units;

#[test]
fn test_warriors_against_heavy_infantry() {
    assert_eq!(
        fight::resolve_fight(
            initialize_units::initialize_warriors(),
            initialize_units::initialize_heavy_infantry()
        ),
        8
    );
}

#[test]
fn test_warriors_againt_wildhorn_herd() {
    assert_eq!(
        fight::resolve_fight(
            initialize_units::initialize_warriors(),
            initialize_units::initialize_wildhorn_herd()
        ),
        4
    );
}

#[test]
fn test_imps_againt_wildhorn_herd() {
    assert_eq!(
        fight::resolve_fight(
            initialize_units::initialize_imps(),
            initialize_units::initialize_wildhorn_herd()
        ),
        0
    );
}

#[test]
fn test_imps_againt_heavy_infantry() {
    assert_eq!(
        fight::resolve_fight(
            initialize_units::initialize_imps(),
            initialize_units::initialize_heavy_infantry()
        ),
        1
    );
}

#[test]
fn test_warriors_againt_warriors() {
    assert_eq!(
        fight::resolve_fight(
            initialize_units::initialize_warriors(),
            initialize_units::initialize_warriors()
        ),
        4
    );
}

#[test]
fn test_imps_againt_imps() {
    assert_eq!(
        fight::resolve_fight(
            initialize_units::initialize_imps(),
            initialize_units::initialize_imps()
        ),
        1
    );
}

#[test]
fn test_heavy_infantry_againt_heavy_infantry() {
    assert_eq!(
        fight::resolve_fight(
            initialize_units::initialize_heavy_infantry(),
            initialize_units::initialize_heavy_infantry()
        ),
        1
    );
}

#[test]
fn test_silexian_spears_against_imps() {
    assert_eq!(
        fight::resolve_fight(
            initialize_units::initialize_silexian_spears(),
            initialize_units::initialize_imps()
        ),
        2
    );
}

#[test]
fn test_silexian_spears_against_warriors() {
    assert_eq!(
        fight::resolve_fight(
            initialize_units::initialize_silexian_spears(),
            initialize_units::initialize_warriors()
        ),
        1
    );
}

#[test]
fn test_clan_warriors_against_citizen_spears() {
    assert_eq!(
        fight::resolve_fight(
            initialize_units::initialize_clan_warriors(),
            initialize_units::initialize_citizen_spears()
        ),
        1
    );
}

#[test]
fn test_clan_warriors_against_silexian_spears() {
    assert_eq!(
        fight::resolve_fight(
            initialize_units::initialize_clan_warriors(),
            initialize_units::initialize_silexian_spears()
        ),
        1
    );
}

#[test]
fn test_infernal_warriors_against_clan_warriors() {
    assert_eq!(
        fight::resolve_fight(
            initialize_units::initialize_infernal_warriors(),
            initialize_units::initialize_clan_warriors()
        ),
        1
    );
}

#[test]
fn test_zombies_against_imps() {
    assert_eq!(
        fight::resolve_fight(
            initialize_units::initialize_zombies(),
            initialize_units::initialize_imps()
        ),
        1
    );
}
