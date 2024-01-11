mod modifier_check;
mod modifier_list;

lazy_static::lazy_static! {
/// Hashmap containing the name of the modifier as key and the Modifier (Global, Offensive, Defensive, Weapon) and the Boolean or the Function to know if the Modifier can be applied
///
/// # Attributes
///
/// FlameOfTheEast
///
/// EyeOfTheBull
///
/// OnyxCore
///
/// BannerOfTheTwiceBranded
///
/// TheirMasterSBanner
///
/// IconOfAshuruk
///
/// BlazeOfProtection
///
/// KadimBinding
///
/// BreathOfTheBrassBull
///
/// GoldenIdolOfShamut
///
/// MaskOfAges
///
/// LugarSDice
///
/// RingOfDesiccation
///
/// TabletOfVezodinezh
///
/// LegendOfTheBlackKing
///
/// BlackStandardOfZagvozd
///
/// ReaperSHarvest
///
/// TrueThirst
///
/// HypnoticPendant
///
/// EternityGem
///
/// NecromanticStaff
///
/// NightSCrown
///
/// UnholyTome
///
/// CursedMedallion
///
/// Lahmia
///
/// Strigoi
///
/// BrotherhoodOfTheDragon
///
/// Nosferatu
///
/// VonKarnstein
///
/// GhoulLord
///
/// Commandment
///
/// CrimsonRage
///
/// StormCaller
///
/// ArcaneKnowledge
///
/// SupremeLich
///
/// EternalDuellist
///
/// MonsterHunter
///
/// FlyingHorror
///
/// UnbreakableWill
///
/// BestialBulk
///
/// MesmerisingGaze
///
/// MysteriesOfTheNight
///
/// OakenMight
///
/// BannerOfDeception
///
/// PredatorPennant
///
/// BannerOfSilentMist
///
/// HunterSHonour
///
/// WatcherSWoe
///
/// BoughOfWyscan
///
/// SpiritOfTheWhirlwind
///
/// ShieldingBark
///
/// MistWalkerSMirror
///
/// HailShot
///
/// SacredSeeds
///
/// HornOfTheWildHunt
///
/// DrumsOfCenyrn
///
/// GlyphOfAmryl
///
/// Shapeshifter
///
/// WildHunter
///
/// Pathfinder
///
/// ForestGuardian
///
/// BladeDancer
///
/// ScarredBark
///
/// ToxicSpores
///
/// EntanglingVines
///
/// OakenCrown
///
/// ExecutionerSIcon
///
/// EyeOfTheGorgon
///
/// CaedhrenSPennon
///
/// CripplingFrost
///
/// MasteryOfSlaughter
///
/// PrideOfGarDaecos
///
/// Transcendence
///
/// LaceratingTouch
///
/// SealOfTheRepublic
///
/// BeastmasterSWhistle
///
/// CeinranSScales
///
/// RingOfTheObsidianThrones
///
/// SealOfThe9thFleet
///
/// MaskOfTheWarCrow
///
/// MoithirSMirror
///
/// BannerOfTheEntombed
///
/// Godslayer
///
/// ScourgeOfKings
///
/// JackalSBlessing
///
/// SunSEmbrace
///
/// CrownOfThePharaohs
///
/// SacredHourglass
///
/// BookOfTheDead
///
/// SteedsOfNephetRa
///
/// BlessedWrappings
///
/// SekhemSceptre
///
/// AnkhOfNaptesh
///
/// SandstormCloak
///
/// DeathMaskOfTeput
///
/// ScrollOfDesiccation
///
/// Oriflamme
///
/// BannerOfRoland
///
/// RelicShroud
///
/// BannerOfElan
///
/// CastellanSCrest
///
/// TristanSResolve
///
/// MortalReminder
///
/// DivineJudgement
///
/// UtherSMettle
///
/// PrayerEtched
///
/// PercivalSPanoply
///
/// FortressOfFaith
///
/// BlackKnightSTabard
///
/// SacredChalice
///
/// Quin
///
/// Cleric
///
/// Bannerman
///
/// Minstrel
///
/// Castellan
///
/// Honour
///
/// Justice
///
/// Valour
///
/// Excellence
///
/// Faith
///
/// Forbearance
///
/// Generosity
///
/// ThriceForged
///
/// GladiatorSSpirit
///
/// IconOfTheInfinite
///
/// WastelandTorch
///
/// ZealotsBanner
///
/// BurningPortent
///
/// SymbolOfSlaughter
///
/// LedgerOfSouls
///
/// ImmortalGauntlets
///
/// VeilgateOrb
///
/// LordOfTheDamned
///
/// WyrdStone
///
/// DaemonicWings
///
/// DarkPrelate
///
/// EntropicAura
///
/// LuckOfTheDarkGods
///
/// IdolOfSpite
///
/// BannerOfTheWildHerd
///
/// HawthorneCurse
///
/// AncestralCarvings
///
/// TwinHungers
///
/// FatalFolly
///
/// AaghorSAffliction
///
/// TricksterSCunning
///
/// WildForm
///
/// ObscuringFog
///
/// DarkRain
///
/// SeedOfTheDarkForest
///
/// PillagerIcon
///
/// InscribingBurin
///
/// EyeOfDominance
///
/// CrownOfHorns
///
/// DaemonSBane
///
/// StarMetalAlloy
///
/// ProtectionOfDorac
///
/// GleamingRobe
///
/// NavigatorSBanner
///
/// BannerOfBecalming
///
/// WarBannerOfRyma
///
/// NovaFlare
///
/// SliverOfTheBlazingDawn
///
/// EluSHeartwood
///
/// BookOfMeladys
///
/// RingOfThePearlThrone
///
/// DiademOfProtection
///
/// AmethystCrystal
///
/// GlitteringLacquer
///
/// AsfadScholar
///
/// OrderOfTheFieryHeart
///
/// MasterOfCanreigTower
///
/// HighWardenOfTheFlame
///
/// FleetOfficer
///
/// RoyalHuntsman
///
/// QueenSCavalier
///
/// QueenSCompanion
///
/// HouseholdStandard
///
/// BannerOfUnity
///
/// MarksmanSPennant
///
/// TheLightOfSonnstahl
///
/// DeathWarrant
///
/// HammerOfWitches
///
/// ImperialSeal
///
/// Blacksteel
///
/// WitchfireGuard
///
/// ShieldOfVolund
///
/// WinterCloak
///
/// LocketOfSunna
///
/// ExemplarSFlame
///
/// KaradonSCourser
///
/// MantleOfUllor
///
/// Trolleater
///
/// Hoardmaster
///
/// Headhunter
///
/// CultLeader
///
/// Firebrand
///
/// GutRoarer
///
/// Wildheart
///
/// Spinesplitter
///
/// Rottenjaw
///
/// BannerOfTheGyengget
///
/// PennantOfTheGreatGrassSky
///
/// SkullOfQenghet
///
/// KhagadaiSLegacy
///
/// ViperSCurse
///
/// HeartRipper
///
/// RitualBloodletter
///
/// WrestlerSBelt
///
/// MammothHideCloak
///
/// KarkadanSResilience
///
/// YetiFurs
///
/// LygurSTongue
///
/// AurochsCharm
///
/// RampagerSChain
///
/// Accurate
///
/// DevastatingCharge
///
/// LethalStrike
///
/// MagicResistance
///
/// PlateArmour
///
/// PoisonAttacks
///
/// Swiftstride
///
/// Vanguard
///
/// MikinokSTotem
///
/// GreenTide
///
/// OmenOfTheApocalypse
///
/// ShadyShanking
///
/// MazaSZappin
///
/// TuktekSGuard
///
/// CrownOfTheCavernKing
///
/// SkullFetish
///
/// TrollAleFlask
///
/// PanOfProtectionPinchin
///
/// PlagueHermitSBlessing
///
/// SacredAquila
///
/// BellOfTheDeepRoads
///
/// RodentiumBullets
///
/// StormRocket
///
/// SecretsOfTheDoomBlade
///
/// SwarmMaster
///
/// TomeOfTheRatking
///
/// TarinaSLyre
///
/// OratorSToga
///
/// CrownOfHubris
///
/// DarkstoneDetonator
///
/// OrbOfAteus
///
/// CowlOfTheApostate
///
/// RuneOfDestruction
///
/// RuneOfSmashing
///
/// RuneOfPenetration
///
/// RuneOfMight
///
/// RuneOfPrecision
///
/// RuneOfCraftsmanship
///
/// RuneOfQuickening
///
/// RuneOfFury
///
/// RuneOfLightning
///
/// RuneOfReturning
///
/// RuneOfFire
///
/// RuneOfResistance
///
/// RuneOfSteel
///
/// RuneOfRetribution
///
/// RuneOfIron
///
/// RuneOfTheForge
///
/// RuneOfDenial
///
/// RuneOfDevouring
///
/// RuneOfHarnessing
///
/// RuneOfTheCourage
///
/// RuneOfDragonSBreath
///
/// RuneOfGrounding
///
/// RuneOfReadiness
///
/// RuneOfStorms
///
/// RuneOfShielding
///
/// RuneOfMastery
///
/// RuneOfMining
///
/// RuneOfKinship
///
/// RunicStandardOfShielding
///
/// RunicStandardOfSwiftness
///
/// RunicStandardOfDismay
///
/// RunicStandardOfSteadiness
///
/// RunicStandardOfTheAnvil
///
/// RunicStandardOfWisdom
///
/// RunicStandardOfTheHold
///
/// IronHusk
///
/// MirroredScales
///
/// KaleidoscopicFlesh
///
/// MarkOfTheEternalChampion
///
/// HammerHand
///
/// UnnaturalRoots
///
/// BrimstoneSecretions
///
/// CentipedeLegs
///
/// ChitinousScales
///
/// DarkHide
///
/// LivingShield
///
/// PiercingSpike
///
/// GreenfireEyes
///
/// VenomSacs
///
/// DigestiveVomit
///
/// Broodmother
///
/// UnhingingJaw
///
/// SegmentedShell
///
/// DiviningSnout
///
/// SmotheringCoils
///
/// MesmerisingPlumage
///
/// RoamingHands
///
/// HotBlood
///
/// ChillingYawn
///
/// SorcerousAntennae
///
/// AuraOfDespair
///
/// WhipcrackTail
///
/// RedHaze
///
/// IncendiaryIchor
///
/// BronzeBackbone
///
/// StiffUpperLip
///
/// HornsOfHubris
///
/// KoruStone
///
/// ObeliskOfCollaboration
///
/// SerpentSNestCharm
///
/// GloryOfTheDawnAge
///
/// AlchemicalArrows
///
/// StarfallScales
///
/// VitalEssence
///
/// AncientPlaque
///
/// InfiltratorSDart
///
/// StampedeResonatorCrystal
///
/// TeAupouriSmokestone
///
/// CelestialAstrolabe
///
/// CarvedTablet
///
/// CarvedWisdom
///
/// EngineOfTheAncients
///
/// LodestoneShield
///
/// MagneticGreatBow
///
/// MonolithOfVitalism
///
/// SuncatcherCrystal
///
/// VenomousFortress
///
/// EideticMastery
///
/// TelepathicMastery
///
/// VeilMastery
///
/// MindShiftingMastery
///
/// EternalMastery
///
/// ForbiddenMastery
///
    static ref HASHMAP: std::collections::HashMap<modifier_list::ModifierList, (crate::modifier::Modifier, modifier_check::ModifierCheck)> = {
        let mut m: std::collections::HashMap<modifier_list::ModifierList, (crate::modifier::Modifier, modifier_check::ModifierCheck)> = std::collections::HashMap::new();
        m.insert(modifier_list::ModifierList::FlameOfTheEast, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::EyeOfTheBull, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::OnyxCore, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BannerOfTheTwiceBranded, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::TheirMasterSBanner, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::IconOfAshuruk, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BlazeOfProtection, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::KadimBinding, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BreathOfTheBrassBull, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::GoldenIdolOfShamut, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MaskOfAges, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::LugarSDice, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RingOfDesiccation, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::TabletOfVezodinezh, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::LegendOfTheBlackKing, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BlackStandardOfZagvozd, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ReaperSHarvest, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::TrueThirst, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::HypnoticPendant, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::EternityGem, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::NecromanticStaff, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::NightSCrown, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::UnholyTome, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::CursedMedallion, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Lahmia, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Strigoi, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BrotherhoodOfTheDragon, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Nosferatu, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::VonKarnstein, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::GhoulLord, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Commandment, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::CrimsonRage, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::StormCaller, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ArcaneKnowledge, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SupremeLich, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::EternalDuellist, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MonsterHunter, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::FlyingHorror, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::UnbreakableWill, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BestialBulk, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MesmerisingGaze, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MysteriesOfTheNight, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::OakenMight, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BannerOfDeception, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::PredatorPennant, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BannerOfSilentMist, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::HunterSHonour, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::WatcherSWoe, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BoughOfWyscan, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SpiritOfTheWhirlwind, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ShieldingBark, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MistWalkerSMirror, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::HailShot, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SacredSeeds, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::HornOfTheWildHunt, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::DrumsOfCenyrn, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::GlyphOfAmryl, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Shapeshifter, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::WildHunter, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Pathfinder, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ForestGuardian, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BladeDancer, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ScarredBark, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ToxicSpores, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::EntanglingVines, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::OakenCrown, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ExecutionerSIcon, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::EyeOfTheGorgon, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::CaedhrenSPennon, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::CripplingFrost, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MasteryOfSlaughter, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::PrideOfGarDaecos, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Transcendence, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::LaceratingTouch, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SealOfTheRepublic, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BeastmasterSWhistle, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::CeinranSScales, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RingOfTheObsidianThrones, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SealOfThe9thFleet, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MaskOfTheWarCrow, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MoithirSMirror, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BannerOfTheEntombed, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Godslayer, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ScourgeOfKings, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::JackalSBlessing, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SunSEmbrace, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::CrownOfThePharaohs, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SacredHourglass, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BookOfTheDead, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SteedsOfNephetRa, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BlessedWrappings, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SekhemSceptre, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::AnkhOfNaptesh, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SandstormCloak, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::DeathMaskOfTeput, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ScrollOfDesiccation, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Oriflamme, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BannerOfRoland, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RelicShroud, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BannerOfElan, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::CastellanSCrest, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::TristanSResolve, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MortalReminder, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::DivineJudgement, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::UtherSMettle, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::PrayerEtched, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::PercivalSPanoply, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::FortressOfFaith, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BlackKnightSTabard, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SacredChalice, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Quin, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Cleric, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Bannerman, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Minstrel, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Castellan, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Honour, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Justice, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Valour, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Excellence, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Faith, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Forbearance, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Generosity, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ThriceForged, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::GladiatorSSpirit, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::IconOfTheInfinite, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::WastelandTorch, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ZealotsBanner, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BurningPortent, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SymbolOfSlaughter, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::LedgerOfSouls, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ImmortalGauntlets, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::VeilgateOrb, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::LordOfTheDamned, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::WyrdStone, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::DaemonicWings, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::DarkPrelate, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::EntropicAura, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::LuckOfTheDarkGods, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::IdolOfSpite, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BannerOfTheWildHerd, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::HawthorneCurse, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::AncestralCarvings, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::TwinHungers, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::FatalFolly, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::AaghorSAffliction, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::TricksterSCunning, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::WildForm, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ObscuringFog, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::DarkRain, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SeedOfTheDarkForest, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::PillagerIcon, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::InscribingBurin, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::EyeOfDominance, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::CrownOfHorns, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::DaemonSBane, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::StarMetalAlloy, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ProtectionOfDorac, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::GleamingRobe, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::NavigatorSBanner, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BannerOfBecalming, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::WarBannerOfRyma, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::NovaFlare, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SliverOfTheBlazingDawn, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::EluSHeartwood, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BookOfMeladys, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RingOfThePearlThrone, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::DiademOfProtection, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::AmethystCrystal, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::GlitteringLacquer, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::AsfadScholar, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::OrderOfTheFieryHeart, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MasterOfCanreigTower, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::HighWardenOfTheFlame, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::FleetOfficer, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RoyalHuntsman, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::QueenSCavalier, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::QueenSCompanion, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::HouseholdStandard, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BannerOfUnity, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MarksmanSPennant, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::TheLightOfSonnstahl, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::DeathWarrant, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::HammerOfWitches, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ImperialSeal, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Blacksteel, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::WitchfireGuard, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ShieldOfVolund, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::WinterCloak, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::LocketOfSunna, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ExemplarSFlame, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::KaradonSCourser, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MantleOfUllor, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Trolleater, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Hoardmaster, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Headhunter, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::CultLeader, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Firebrand, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::GutRoarer, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Wildheart, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Spinesplitter, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Rottenjaw, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BannerOfTheGyengget, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::PennantOfTheGreatGrassSky, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SkullOfQenghet, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::KhagadaiSLegacy, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ViperSCurse, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::HeartRipper, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RitualBloodletter, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::WrestlerSBelt, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MammothHideCloak, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::KarkadanSResilience, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::YetiFurs, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::LygurSTongue, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::AurochsCharm, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RampagerSChain, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Accurate, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::DevastatingCharge, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::LethalStrike, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MagicResistance, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::PlateArmour, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::PoisonAttacks, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Swiftstride, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Vanguard, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MikinokSTotem, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::GreenTide, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::OmenOfTheApocalypse, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ShadyShanking, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MazaSZappin, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::TuktekSGuard, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::CrownOfTheCavernKing, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SkullFetish, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::TrollAleFlask, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::PanOfProtectionPinchin, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::PlagueHermitSBlessing, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SacredAquila, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BellOfTheDeepRoads, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RodentiumBullets, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::StormRocket, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SecretsOfTheDoomBlade, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SwarmMaster, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::TomeOfTheRatking, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::TarinaSLyre, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::OratorSToga, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::CrownOfHubris, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::DarkstoneDetonator, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::OrbOfAteus, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::CowlOfTheApostate, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfDestruction, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfSmashing, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfPenetration, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfMight, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfPrecision, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfCraftsmanship, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfQuickening, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfFury, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfLightning, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfReturning, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfFire, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfResistance, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfSteel, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfRetribution, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfIron, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfTheForge, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfDenial, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfDevouring, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfHarnessing, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfTheCourage, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfDragonSBreath, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfGrounding, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfReadiness, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfStorms, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfShielding, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfMastery, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfMining, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RuneOfKinship, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RunicStandardOfShielding, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RunicStandardOfSwiftness, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RunicStandardOfDismay, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RunicStandardOfSteadiness, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RunicStandardOfTheAnvil, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RunicStandardOfWisdom, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RunicStandardOfTheHold, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::IronHusk, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MirroredScales, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::KaleidoscopicFlesh, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MarkOfTheEternalChampion, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::HammerHand, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::UnnaturalRoots, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BrimstoneSecretions, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::CentipedeLegs, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ChitinousScales, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::DarkHide, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::LivingShield, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::PiercingSpike, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::GreenfireEyes, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::VenomSacs, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::DigestiveVomit, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::Broodmother, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::UnhingingJaw, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SegmentedShell, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::DiviningSnout, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SmotheringCoils, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MesmerisingPlumage, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RoamingHands, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::HotBlood, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ChillingYawn, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SorcerousAntennae, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::AuraOfDespair, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::WhipcrackTail, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::RedHaze, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::IncendiaryIchor, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::BronzeBackbone, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::StiffUpperLip, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::HornsOfHubris, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::KoruStone, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ObeliskOfCollaboration, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SerpentSNestCharm, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::GloryOfTheDawnAge, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::AlchemicalArrows, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::StarfallScales, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::VitalEssence, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::AncientPlaque, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::InfiltratorSDart, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::StampedeResonatorCrystal, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::TeAupouriSmokestone, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::CelestialAstrolabe, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::CarvedTablet, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::CarvedWisdom, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::EngineOfTheAncients, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::LodestoneShield, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MagneticGreatBow, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MonolithOfVitalism, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::SuncatcherCrystal, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::VenomousFortress, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::EideticMastery, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::TelepathicMastery, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::VeilMastery, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MindShiftingMastery, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::EternalMastery, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::ForbiddenMastery, (crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m
    };
}
