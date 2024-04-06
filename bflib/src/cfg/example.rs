/*
Copyright 2024 Eric Stokes.

This file is part of bflib.

bflib is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your
option) any later version.

bflib is distributed in the hope that it will be useful, but WITHOUT
ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero Public License
for more details.
*/

use super::*;
use dcso3::coalition::Side;
use fxhash::FxHashMap;

fn default_red_troops() -> Vec<Troop> {
    vec![
        Troop {
            name: "Standard".into(),
            template: "RSTANDARDTROOP".into(),
            persist: PersistTyp::Forever,
            can_capture: true,
            jtac: Some(DeployableJtac {
                range: 8000,
                nolos: false,
            }),
            limit: 10,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            cost: 0,
            weight: 800,
        },
        Troop {
            name: "Anti Tank".into(),
            template: "RATTROOP".into(),
            persist: PersistTyp::Forever,
            can_capture: true,
            jtac: Some(DeployableJtac {
                range: 8000,
                nolos: false,
            }),
            limit: 10,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            cost: 1,
            weight: 1000,
        },
        Troop {
            name: "Mortar".into(),
            template: "RMORTARTROOP".into(),
            persist: PersistTyp::Forever,
            can_capture: true,
            jtac: Some(DeployableJtac {
                range: 8000,
                nolos: false,
            }),
            limit: 10,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            cost: 5,
            weight: 1200,
        },
        Troop {
            name: "Igla".into(),
            template: "RIGLATROOP".into(),
            persist: PersistTyp::Forever,
            can_capture: false,
            jtac: None,
            limit: 10,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            cost: 5,
            weight: 500,
        },
    ]
}

fn default_blue_troops() -> Vec<Troop> {
    vec![
        Troop {
            name: "Standard".into(),
            template: "BSTANDARDTROOP".into(),
            persist: PersistTyp::Forever,
            can_capture: true,
            jtac: Some(DeployableJtac {
                range: 8000,
                nolos: false,
            }),
            limit: 10,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            cost: 0,
            weight: 800,
        },
        Troop {
            name: "Anti Tank".into(),
            template: "BATTROOP".into(),
            persist: PersistTyp::Forever,
            can_capture: true,
            jtac: Some(DeployableJtac {
                range: 8000,
                nolos: false,
            }),
            limit: 10,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            cost: 1,
            weight: 1000,
        },
        Troop {
            name: "Mortar".into(),
            template: "BMORTARTROOP".into(),
            persist: PersistTyp::Forever,
            can_capture: true,
            jtac: Some(DeployableJtac {
                range: 8000,
                nolos: false,
            }),
            limit: 10,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            cost: 5,
            weight: 1200,
        },
        Troop {
            name: "Stinger".into(),
            template: "BSTINGERTROOP".into(),
            persist: PersistTyp::Forever,
            can_capture: false,
            jtac: Some(DeployableJtac {
                range: 8000,
                nolos: false,
            }),
            limit: 10,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            cost: 5,
            weight: 500,
        },
    ]
}

fn default_red_deployables() -> Vec<Deployable> {
    vec![
        Deployable {
            path: vec!["Radar SAMs".into(), "SA 6 Kub".into()],
            template: "DEPSA6".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            cost: 25,
            crates: vec![
                Crate {
                    name: "Kub Launcher".into(),
                    weight: 2000,
                    required: 2,
                    pos_unit: Some("Kub 2P25 ln".into()),
                    max_drop_height_agl: 10,
                    max_drop_speed: 13,
                },
                Crate {
                    name: "Kub Radar".into(),
                    weight: 2000,
                    required: 1,
                    pos_unit: Some("Kub 1S91 str".into()),
                    max_drop_height_agl: 10,
                    max_drop_speed: 13,
                },
            ],
            repair_crate: Some(Crate {
                name: "Kub Repair".into(),
                weight: 2000,
                required: 1,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }),
            logistics: None,
            ewr: Some(DeployableEwr { range: 30000 }),
            jtac: None,
        },
        Deployable {
            path: vec!["Radar SAMs".into(), "SA 11 Buk".into()],
            template: "DEPSA11".into(),
            persist: PersistTyp::Forever,
            limit: 2,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            cost: 50,
            crates: vec![
                Crate {
                    name: "SA11 Launcher".into(),
                    weight: 2000,
                    required: 2,
                    pos_unit: Some("SA-11 Buk LN 9A310M1".into()),
                    max_drop_height_agl: 10,
                    max_drop_speed: 13,
                },
                Crate {
                    name: "SA11 Search Radar".into(),
                    weight: 2000,
                    required: 1,
                    pos_unit: Some("SA-11 Buk SR 9S18M1".into()),
                    max_drop_height_agl: 10,
                    max_drop_speed: 13,
                },
                Crate {
                    name: "SA11 CC".into(),
                    weight: 2000,
                    required: 1,
                    pos_unit: Some("SA-11 Buk CC 9S470M1".into()),
                    max_drop_height_agl: 10,
                    max_drop_speed: 13,
                },
            ],
            repair_crate: Some(Crate {
                name: "Buk Repair".into(),
                weight: 2000,
                required: 1,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }),
            logistics: None,
            ewr: Some(DeployableEwr { range: 60000 }),
            jtac: None,
        },
        Deployable {
            path: vec!["Radar SAMs".into(), "SA15 Tor".into()],
            template: "DEPSA15".into(),
            persist: PersistTyp::Forever,
            limit: 2,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            cost: 50,
            crates: vec![Crate {
                name: "SA15 Tor".into(),
                weight: 2000,
                required: 4,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: None,
            ewr: Some(DeployableEwr { range: 20000 }),
            jtac: None,
        },
        Deployable {
            path: vec!["Radar SAMs".into(), "SA8 Osa".into()],
            template: "DEPSA8".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            cost: 15,
            crates: vec![Crate {
                name: "SA8 Osa".into(),
                weight: 2000,
                required: 2,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: None,
            ewr: None,
            jtac: None,
        },
        Deployable {
            path: vec!["AAA".into(), "ZU23 Emplacement".into()],
            template: "DEPZU23".into(),
            persist: PersistTyp::Forever,
            limit: 10,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            cost: 5,
            crates: vec![Crate {
                name: "ZU23 Emplacement".into(),
                weight: 1000,
                required: 1,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: None,
            ewr: None,
            jtac: None,
        },
        Deployable {
            path: vec!["AAA".into(), "Shilka".into()],
            template: "DEPSHILKA".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            cost: 10,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            crates: vec![Crate {
                name: "Shilka Crate".into(),
                weight: 2000,
                required: 2,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: None,
            ewr: None,
            jtac: None,
        },
        Deployable {
            path: vec!["AAA".into(), "Tunguska".into()],
            template: "DEPTUNGUSKA".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            cost: 15,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            crates: vec![Crate {
                name: "Tunguska Crate".into(),
                weight: 2000,
                required: 2,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: None,
            ewr: None,
            jtac: None,
        },
        Deployable {
            path: vec!["IR SAMs".into(), "SA13 Strela".into()],
            template: "DEPSA13".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            cost: 15,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            crates: vec![Crate {
                name: "SA13 Strela Crate".into(),
                weight: 2000,
                required: 2,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: None,
            ewr: None,
            jtac: None,
        },
        Deployable {
            path: vec!["Ground Units".into(), "SPH 2S19 Msta 152MM".into()],
            template: "DEPMSTA".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            cost: 15,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            crates: vec![Crate {
                name: "MSTA Crate".into(),
                weight: 2000,
                required: 2,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: None,
            ewr: None,
            jtac: None,
        },
        Deployable {
            path: vec!["Ground Units".into(), "T72".into()],
            template: "DEPT72".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            cost: 50,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            crates: vec![Crate {
                name: "T72 Crate".into(),
                weight: 2000,
                required: 4,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: None,
            ewr: None,
            jtac: Some(DeployableJtac {
                range: 8000,
                nolos: false,
            }),
        },
        Deployable {
            path: vec!["Ground Units".into(), "BMP3".into()],
            template: "DEPBMP3".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            cost: 25,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            crates: vec![Crate {
                name: "BMP3 Crate".into(),
                weight: 2000,
                required: 4,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: None,
            ewr: None,
            jtac: Some(DeployableJtac {
                range: 8000,
                nolos: false,
            }),
        },
        Deployable {
            path: vec!["Ground Units".into(), "Ammo Truck".into()],
            template: "DEPRAMMO".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            cost: 5,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            crates: vec![Crate {
                name: "Ammo Truck Crate".into(),
                weight: 2000,
                required: 2,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: None,
            ewr: None,
            jtac: None,
        },
        Deployable {
            path: vec!["EWRs".into(), "1L13".into()],
            template: "DEP1L13".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            cost: 5,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            crates: vec![Crate {
                name: "1L13 Crate".into(),
                weight: 2000,
                required: 2,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: None,
            ewr: Some(DeployableEwr { range: 500000 }),
            jtac: None,
        },
        Deployable {
            path: vec!["FARP".into()],
            template: "RDEPFARP".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            cost: 50,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            crates: vec![Crate {
                name: "FARP Crate".into(),
                weight: 2000,
                required: 4,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: Some(DeployableLogistics {
                pad_templates: vec![
                    "DEPRFARPPAD0".into(),
                    "DEPRFARPPAD1".into(),
                    "DEPRFARPPAD2".into(),
                    "DEPRFARPPAD3".into(),
                ],
                ammo_template: "DEPRFARPAMMO".into(),
                fuel_template: "DEPRFARPFUEL".into(),
                barracks_template: "DEPRFARPTENT".into(),
            }),
            ewr: None,
            jtac: None,
        },
    ]
}

fn default_blue_deployables() -> Vec<Deployable> {
    vec![
        Deployable {
            path: vec!["Radar SAMs".into(), "Roland ADS".into()],
            template: "DEPROLAND".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            cost: 35,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            crates: vec![Crate {
                name: "Roland".into(),
                weight: 1000,
                required: 4,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: None,
            ewr: Some(DeployableEwr { range: 20000 }),
            jtac: None,
        },
        Deployable {
            path: vec!["Radar SAMs".into(), "Hawk System".into()],
            template: "DEPHAWK".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            cost: 35,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            crates: vec![
                Crate {
                    name: "Hawk Launcher".into(),
                    weight: 1000,
                    required: 1,
                    pos_unit: Some("Hawk ln".into()),
                    max_drop_height_agl: 10,
                    max_drop_speed: 13,
                },
                Crate {
                    name: "Hawk Search Radar".into(),
                    weight: 1000,
                    required: 1,
                    pos_unit: Some("Hawk sr".into()),
                    max_drop_height_agl: 10,
                    max_drop_speed: 13,
                },
                Crate {
                    name: "Hawk Track Radar".into(),
                    weight: 1000,
                    required: 1,
                    pos_unit: Some("Hawk tr".into()),
                    max_drop_height_agl: 10,
                    max_drop_speed: 13,
                },
                Crate {
                    name: "Hawk CC".into(),
                    weight: 1000,
                    required: 1,
                    pos_unit: Some("Hawk pcp".into()),
                    max_drop_height_agl: 10,
                    max_drop_speed: 13,
                },
            ],
            repair_crate: Some(Crate {
                name: "Hawk Repair".into(),
                weight: 1000,
                required: 1,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }),
            logistics: None,
            ewr: Some(DeployableEwr { range: 60000 }),
            jtac: None,
        },
        Deployable {
            path: vec!["IR SAMs".into(), "Avenger".into()],
            template: "DEPAVENGER".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            cost: 15,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            crates: vec![Crate {
                name: "Avenger Crate".into(),
                weight: 1000,
                required: 2,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: None,
            ewr: None,
            jtac: None,
        },
        Deployable {
            path: vec!["IR SAMs".into(), "Linebacker".into()],
            template: "DEPLINEBACKER".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            cost: 25,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            crates: vec![Crate {
                name: "Linebacker Crate".into(),
                weight: 1000,
                required: 2,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: None,
            ewr: None,
            jtac: None,
        },
        Deployable {
            path: vec!["AAA".into(), "Flakpanzergepard".into()],
            template: "DEPGEPARD".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            cost: 25,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            crates: vec![Crate {
                name: "Flakpanzergepard Crate".into(),
                weight: 1000,
                required: 4,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: None,
            ewr: None,
            jtac: None,
        },
        Deployable {
            path: vec!["AAA".into(), "Vulkan".into()],
            template: "DEPVULKAN".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            cost: 15,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            crates: vec![Crate {
                name: "Vulkan Crate".into(),
                weight: 1000,
                required: 2,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: None,
            ewr: None,
            jtac: None,
        },
        Deployable {
            path: vec!["Ground Units".into(), "Firtina 155MM".into()],
            template: "DEPFIRTINA".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            cost: 15,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            crates: vec![Crate {
                name: "Firtina Crate".into(),
                weight: 1000,
                required: 2,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: None,
            ewr: None,
            jtac: None,
        },
        Deployable {
            path: vec!["Ground Units".into(), "M2A2 Bradley".into()],
            template: "DEPBRADLEY".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            cost: 25,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            crates: vec![Crate {
                name: "Bradley Crate".into(),
                weight: 1000,
                required: 4,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: None,
            ewr: None,
            jtac: Some(DeployableJtac {
                range: 8000,
                nolos: false,
            }),
        },
        Deployable {
            path: vec!["Ground Units".into(), "2A6M Leopard".into()],
            template: "DEPLEOPARD".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            cost: 50,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            crates: vec![Crate {
                name: "Leopard Crate".into(),
                weight: 1000,
                required: 4,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: None,
            ewr: None,
            jtac: Some(DeployableJtac {
                range: 8000,
                nolos: false,
            }),
        },
        Deployable {
            path: vec!["Ground Units".into(), "Ammo Truck".into()],
            template: "DEPBAMMO".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            cost: 5,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            crates: vec![Crate {
                name: "Ammo Truck Crate".into(),
                weight: 2000,
                required: 2,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: None,
            ewr: None,
            jtac: None,
        },
        Deployable {
            path: vec!["EWRs".into(), "AN/FPS-117".into()],
            template: "DEPFPS117".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            cost: 5,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            crates: vec![Crate {
                name: "FPS-117 Crate".into(),
                weight: 1000,
                required: 2,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: None,
            ewr: Some(DeployableEwr { range: 500000 }),
            jtac: None,
        },
        Deployable {
            path: vec!["FARP".into()],
            template: "BDEPFARP".into(),
            persist: PersistTyp::Forever,
            limit: 4,
            cost: 50,
            limit_enforce: LimitEnforceTyp::DeleteOldest,
            crates: vec![Crate {
                name: "FARP Crate".into(),
                weight: 1000,
                required: 4,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            }],
            repair_crate: None,
            logistics: Some(DeployableLogistics {
                pad_templates: vec![
                    "DEPBFARPPAD0".into(),
                    "DEPBFARPPAD1".into(),
                    "DEPBFARPPAD2".into(),
                    "DEPBFARPPAD3".into(),
                ],
                ammo_template: "DEPBFARPAMMO".into(),
                fuel_template: "DEPBFARPFUEL".into(),
                barracks_template: "DEPBFARPTENT".into(),
            }),
            ewr: None,
            jtac: None,
        },
    ]
}

fn default_life_types() -> FxHashMap<Vehicle, LifeType> {
    FxHashMap::from_iter([
        ("FA-18C_hornet".into(), LifeType::Standard),
        ("F-14A-135-GR".into(), LifeType::Standard),
        ("F-14B".into(), LifeType::Standard),
        ("F-15C".into(), LifeType::Standard),
        ("F-15ESE".into(), LifeType::Standard),
        ("MiG-29S".into(), LifeType::Standard),
        ("M-2000C".into(), LifeType::Standard),
        ("F-16C_50".into(), LifeType::Standard),
        ("MiG-29A".into(), LifeType::Standard),
        ("Su-27".into(), LifeType::Standard),
        ("AH-64D_BLK_II".into(), LifeType::Attack),
        ("Mi-24P".into(), LifeType::Attack),
        ("Ka-50_3".into(), LifeType::Attack),
        ("A-10C".into(), LifeType::Attack),
        ("A-10A".into(), LifeType::Attack),
        ("Su-25".into(), LifeType::Attack),
        ("Su-25T".into(), LifeType::Attack),
        ("AJS37".into(), LifeType::Attack),
        ("Ka-50".into(), LifeType::Attack),
        ("AV8BNA".into(), LifeType::Attack),
        ("A-10C_2".into(), LifeType::Attack),
        ("JF-17".into(), LifeType::Attack),
        ("SA342L".into(), LifeType::Logistics),
        ("UH-1H".into(), LifeType::Logistics),
        ("Mi-8MT".into(), LifeType::Logistics),
        ("SA342M".into(), LifeType::Logistics),
        ("L-39C".into(), LifeType::Recon),
        ("L-39ZA".into(), LifeType::Recon),
        ("TF-51D".into(), LifeType::Recon),
        ("Yak-52".into(), LifeType::Recon),
        ("C-101CC".into(), LifeType::Recon),
        ("MB-339A".into(), LifeType::Recon),
        ("F-5E-3".into(), LifeType::Intercept),
        ("MiG-21Bis".into(), LifeType::Intercept),
        ("MiG-19P".into(), LifeType::Intercept),
        ("Mirage-F1EE".into(), LifeType::Intercept),
        ("Mirage-F1CE".into(), LifeType::Intercept),
    ])
}

fn default_cargo() -> FxHashMap<Vehicle, CargoConfig> {
    FxHashMap::from_iter([
        (
            "UH-1H".into(),
            CargoConfig {
                troop_slots: 1,
                crate_slots: 1,
                total_slots: 2,
            },
        ),
        (
            "Mi-8MT".into(),
            CargoConfig {
                troop_slots: 2,
                crate_slots: 1,
                total_slots: 2,
            },
        ),
        (
            "SA342L".into(),
            CargoConfig {
                troop_slots: 1,
                crate_slots: 1,
                total_slots: 1,
            },
        ),
        (
            "SA342M".into(),
            CargoConfig {
                troop_slots: 1,
                crate_slots: 1,
                total_slots: 1,
            },
        ),
        (
            "Mi-24P".into(),
            CargoConfig {
                troop_slots: 1,
                crate_slots: 1,
                total_slots: 1,
            },
        ),
    ])
}

fn default_threatened_distance() -> FxHashMap<Vehicle, u32> {
    FxHashMap::from_iter([
        ("FA-18C_hornet".into(), 36000),
        ("F-14A-135-GR".into(), 21600),
        ("F-14B".into(), 21600),
        ("F-15C".into(), 36000),
        ("F-15ESE".into(), 36000),
        ("MiG-29S".into(), 21600),
        ("M-2000C".into(), 21600),
        ("F-16C_50".into(), 36000),
        ("MiG-29A".into(), 21600),
        ("Su-27".into(), 21600),
        ("AH-64D_BLK_II".into(), 14400),
        ("Mi-24P".into(), 14400),
        ("Ka-50_3".into(), 14400),
        ("A-10C".into(), 21600),
        ("A-10A".into(), 21600),
        ("Su-25".into(), 21600),
        ("Su-25T".into(), 21600),
        ("AJS37".into(), 36000),
        ("Ka-50".into(), 14400),
        ("AV8BNA".into(), 36000),
        ("A-10C_2".into(), 14400),
        ("JF-17".into(), 36000),
        ("SA342L".into(), 9000),
        ("UH-1H".into(), 9000),
        ("Mi-8MT".into(), 9000),
        ("SA342M".into(), 9000),
        ("L-39C".into(), 9000),
        ("L-39ZA".into(), 9000),
        ("TF-51D".into(), 0),
        ("Yak-52".into(), 0),
        ("C-101CC".into(), 9000),
        ("MB-339A".into(), 9000),
        ("F-5E-3".into(), 14400),
        ("MiG-21Bis".into(), 14400),
        ("MiG-19P".into(), 9000),
        ("Mirage-F1EE".into(), 14400),
        ("Mirage-F1CE".into(), 14400),
        ("A-50".into(), 0),
        ("IL-78".into(), 0),
        ("IL-78M".into(), 0),
        ("MQ-9 Reaper".into(), 0),
        ("Tu-22M3".into(), 36000),
        ("KC-135".into(), 0),
        ("KC130".into(), 0),
        ("B-1B".into(), 36000),
        ("E-3A".into(), 0),
    ])
}

fn default_unit_classification() -> FxHashMap<Vehicle, UnitTags> {
    use UnitTag::*;
    FxHashMap::from_iter(
        [
            (
                "M6 Linebacker".into(),
                SAM | SR | IRGuided | Launcher | APC | LightCannon | Driveable,
            ),
            (
                "M1097 Avenger".into(),
                SAM | SR | IRGuided | Launcher | SmallArms | Driveable,
            ),
            ("Hawk cwar".into(), SAM | LR | RadarGuided | AuxRadarUnit),
            ("Hawk pcp".into(), SAM | LR | RadarGuided | ControlUnit),
            ("Hawk sr".into(), SAM | LR | RadarGuided | SearchRadar),
            ("Hawk tr".into(), SAM | LR | RadarGuided | TrackRadar),
            ("Hawk ln".into(), SAM | LR | RadarGuided | Launcher),
            (
                "M1134 Stryker ATGM".into(),
                APC | MR | ATGM | SmallArms | Driveable,
            ),
            (
                "M-2 Bradley".into(),
                APC | MR | ATGM | LightCannon | Driveable,
            ),
            (
                "M-1 Abrams".into(),
                Armor | MR | HeavyCannon | SmallArms | Driveable,
            ),
            ("outpost".into(), Logistics | SR | SmallArms),
            ("bofors40".into(), AAA | LR),
            ("M 818".into(), Logistics | Unarmed),
            ("M978 HEMTT Tanker".into(), Logistics | Unarmed),
            ("Soldier M249".into(), Infantry | SR | SmallArms),
            ("HL_ZU-23".into(), AAA | SR),
            (
                "Roland ADS".into(),
                SAM | MR
                    | RadarGuided
                    | EngagesWeapons
                    | Launcher
                    | SearchRadar
                    | TrackRadar
                    | Driveable,
            ),
            ("Vulcan".into(), AAA | MR | RadarGuided | Driveable),
            ("Gepard".into(), AAA | LR | RadarGuided | Driveable),
            ("Soldier RPG".into(), Infantry | MR | RPG),
            ("Soldier M4".into(), Infantry | SR | SmallArms),
            ("2B11 mortar".into(), Infantry | LR | Artillery),
            (
                "Soldier stinger".into(),
                SAM | Infantry | SR | IRGuided | Launcher | Driveable,
            ),
            (
                "Stinger comm".into(),
                SAM | Infantry | ControlUnit | Unarmed,
            ),
            (
                "T155_Firtina".into(),
                Armor | LR | Artillery | SmallArms | Driveable,
            ),
            (
                "Leopard-2".into(),
                Armor | MR | HeavyCannon | SmallArms | Driveable,
            ),
            ("ZSU-23-4 Shilka".into(), AAA | MR | RadarGuided | Driveable),
            ("ZSU_57_2".into(), AAA | LR | Driveable),
            (
                "Strela-10M3".into(),
                SAM | SR | IRGuided | Launcher | Driveable,
            ),
            (
                "Strela-1 9P31".into(),
                SAM | SR | IRGuided | Launcher | Driveable,
            ),
            (
                "SA-11 Buk CC 9S470M1".into(),
                SAM | LR | RadarGuided | ControlUnit,
            ),
            (
                "SA-11 Buk SR 9S18M1".into(),
                SAM | LR | RadarGuided | SearchRadar,
            ),
            (
                "SA-11 Buk LN 9A310M1".into(),
                SAM | LR | RadarGuided | TrackRadar | Launcher,
            ),
            ("BMD-1".into(), APC | MR | ATGM | LightCannon | Driveable),
            ("BMP-1".into(), APC | MR | ATGM | LightCannon | Driveable),
            ("BMP-3".into(), APC | MR | ATGM | LightCannon | Driveable),
            (
                "T-80UD".into(),
                Armor | MR | ATGM | HeavyCannon | SmallArms | Driveable,
            ),
            (
                "T-72B".into(),
                Armor | MR | HeavyCannon | SmallArms | Driveable,
            ),
            (
                "T-55".into(),
                Armor | MR | HeavyCannon | SmallArms | Driveable,
            ),
            ("S-60_Type59_Artillery".into(), AAA | LR),
            ("ZU-23 Emplacement Closed".into(), AAA | SR),
            ("ATZ-10".into(), Logistics | Unarmed),
            ("Ural-375".into(), Logistics | Unarmed | Driveable),
            ("Infantry AK ver3".into(), Infantry | SR | SmallArms),
            ("Infantry AK ver2".into(), Infantry | SR | SmallArms),
            ("Paratrooper RPG-16".into(), Infantry | MR | RPG),
            (
                "Kub 1S91 str".into(),
                SAM | MR | RadarGuided | SearchRadar | TrackRadar,
            ),
            ("Kub 2P25 ln".into(), SAM | MR | RadarGuided | Launcher),
            (
                "Tor 9A331".into(),
                SAM | MR
                    | RadarGuided
                    | EngagesWeapons
                    | SearchRadar
                    | TrackRadar
                    | Launcher
                    | Driveable,
            ),
            (
                "Osa 9A33 ln".into(),
                SAM | SR | RadarGuided | SearchRadar | TrackRadar | Launcher | Driveable,
            ),
            ("ZU-23 Emplacement".into(), AAA | SR),
            (
                "2S6 Tunguska".into(),
                SAM | AAA | SR | OpticallyGuided | Launcher,
            ),
            ("Cow".into(), Logistics | Unarmed),
            ("FARP Ammo Dump Coating".into(), Logistics | Unarmed),
            ("FARP Fuel Depot".into(), Logistics | Unarmed),
            ("FARP Tent".into(), Logistics | Unarmed),
            ("Invisible FARP".into(), Logistics | Unarmed | Invincible),
            ("M-109".into(), Armor | Artillery | Driveable),
            ("SAU Msta".into(), Armor | Artillery | SmallArms | Driveable),
            ("1L13 EWR".into(), EWR | Unarmed),
            ("FPS-117".into(), EWR | Unarmed),
            ("FPS-117 ECS".into(), EWR | Unarmed),
            ("p-19 s-125 sr".into(), SAM | LR | RadarGuided | SearchRadar),
            ("SNR_75V".into(), SAM | LR | RadarGuided | TrackRadar),
            ("S_75M_Volhov".into(), SAM | LR | RadarGuided | Launcher),
            ("RD_75".into(), SAM | LR | RadarGuided | AuxRadarUnit),
            ("MiG-29A".into(), Aircraft | Driveable),
            ("Su-27".into(), Aircraft | Driveable),
            ("SA342L".into(), Helicopter.into()),
            ("Su-25T".into(), Aircraft | Driveable),
            ("MiG-21Bis".into(), Aircraft.into()),
            ("AH-64D_BLK_II".into(), Helicopter.into()),
            ("F-16C_50".into(), Aircraft.into()),
            ("UH-1H".into(), Helicopter.into()),
            ("M-2000C".into(), Aircraft.into()),
            ("Mi-8MT".into(), Helicopter.into()),
            (
                "SA-18 Igla-S manpad".into(),
                SAM | SR | IRGuided | Launcher | Driveable,
            ),
            ("SA-18 Igla comm".into(), SAM | SR | IRGuided | ControlUnit),
            (
                "SA-18 Igla-S comm".into(),
                SAM | SR | IRGuided | ControlUnit,
            ),
            ("Ka-50_3".into(), Helicopter.into()),
            ("Ka-50".into(), Helicopter.into()),
            ("Infantry AK".into(), Infantry | SR | SmallArms),
            ("Mi-24P".into(), Helicopter.into()),
            ("F-15ESE".into(), Aircraft.into()),
            ("Mirage-F1EE".into(), Aircraft.into()),
            ("E-3A".into(), Aircraft | AWACS | Link16),
            ("A-50".into(), Aircraft.into()),
            ("MQ-9 Reaper".into(), Aircraft.into()),
            ("KC-135".into(), Aircraft.into()),
            ("KC130".into(), Aircraft.into()),
            ("F-15C".into(), Aircraft.into()),
            ("B-1B".into(), Aircraft.into()),
            ("IL-78M".into(), Aircraft.into()),
            ("Tu-22M3".into(), Aircraft.into()),
            (".Ammunition depot".into(), Logistics | Unarmed),
        ]
        .into_iter()
        .map(|(k, f)| (k, UnitTags::from(f))),
    )
}

fn default_jtac_priority() -> Vec<UnitTags> {
    use UnitTag::*;
    [
        SAM | LR | RadarGuided | TrackRadar,
        SAM | MR | RadarGuided | TrackRadar,
        SAM | OpticallyGuided,
        SAM | IRGuided,
        AAA | LR,
        APC | ATGM,
        AAA.into(),
        Logistics.into(),
        Infantry.into(),
        Armor.into(),
        Artillery.into(),
    ]
    .into_iter()
    .map(UnitTags::from)
    .collect()
}

fn default_repair_crate() -> FxHashMap<Side, Crate> {
    FxHashMap::from_iter([
        (
            Side::Blue,
            Crate {
                name: "Logistics Repair".into(),
                weight: 1500,
                required: 1,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            },
        ),
        (
            Side::Red,
            Crate {
                name: "Logistics Repair".into(),
                weight: 2000,
                required: 1,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            },
        ),
    ])
}

fn default_supply_transfer_crate() -> FxHashMap<Side, Crate> {
    FxHashMap::from_iter([
        (
            Side::Blue,
            Crate {
                name: "Supply Transfer".into(),
                weight: 1500,
                required: 1,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            },
        ),
        (
            Side::Red,
            Crate {
                name: "Supply Transfer".into(),
                weight: 2000,
                required: 1,
                pos_unit: None,
                max_drop_height_agl: 10,
                max_drop_speed: 13,
            },
        ),
    ])
}

fn default_red_actions() -> IndexMap<String, Action, FxBuildHasher> {
    IndexMap::from_iter([
        (
            "awacs".into(),
            Action {
                cost: 100,
                penalty: Some(100),
                limit: None,
                kind: ActionKind::Awacs(AwacsCfg {
                    ewr: DeployableEwr { range: 400000 },
                    plane: AiPlaneCfg {
                        kind: AiPlaneKind::FixedWing,
                        duration: Some(8),
                        template: "RAWACS".into(),
                        altitude: 11000.,
                        altitude_typ: AltType::BARO,
                        speed: 200.,
                    },
                }),
            },
        ),
        (
            "awacs-waypoint".into(),
            Action {
                cost: 10,
                penalty: None,
                limit: None,
                kind: ActionKind::AwacsWaypoint,
            },
        ),
        (
            "tanker".into(),
            Action {
                cost: 50,
                penalty: Some(50),
                limit: None,
                kind: ActionKind::Tanker(AiPlaneCfg {
                    kind: AiPlaneKind::FixedWing,
                    duration: Some(8),
                    template: "RTANKER".into(),
                    altitude: 10000.,
                    altitude_typ: AltType::BARO,
                    speed: 180.,
                }),
            },
        ),
        (
            "tanker-waypoint".into(),
            Action {
                cost: 10,
                penalty: None,
                limit: None,
                kind: ActionKind::TankerWaypoint,
            },
        ),
        (
            "drone".into(),
            Action {
                cost: 25,
                penalty: Some(25),
                limit: None,
                kind: ActionKind::Drone(DroneCfg {
                    plane: AiPlaneCfg {
                        kind: AiPlaneKind::FixedWing,
                        duration: Some(12),
                        template: "RDRONE".into(),
                        altitude: 7000.,
                        altitude_typ: AltType::BARO,
                        speed: 90.,
                    },
                    jtac: DeployableJtac {
                        range: 16000,
                        nolos: true,
                    },
                }),
            },
        ),
        (
            "drone-waypoint".into(),
            Action {
                cost: 5,
                penalty: None,
                limit: None,
                kind: ActionKind::DroneWaypoint,
            },
        ),
        (
            "bomber".into(),
            Action {
                cost: 100,
                penalty: Some(100),
                limit: None,
                kind: ActionKind::Bomber(BomberCfg {
                    targets: 15,
                    power: 1000,
                    accuracy: 15,
                    plane: AiPlaneCfg {
                        kind: AiPlaneKind::FixedWing,
                        template: "RBOMBER".into(),
                        altitude: 12000.,
                        altitude_typ: AltType::BARO,
                        duration: None,
                        speed: 500.,
                    },
                }),
            },
        ),
        (
            "fighters".into(),
            Action {
                cost: 100,
                penalty: Some(100),
                limit: None,
                kind: ActionKind::Fighters(AiPlaneCfg {
                    kind: AiPlaneKind::FixedWing,
                    duration: Some(2),
                    template: "RFIGHTERS".into(),
                    altitude: 10000.,
                    altitude_typ: AltType::BARO,
                    speed: 250.,
                }),
            },
        ),
        (
            "fighters-waypoint".into(),
            Action {
                cost: 5,
                penalty: None,
                limit: None,
                kind: ActionKind::FighersWaypoint,
            },
        ),
        (
            "attack-helicopters".into(),
            Action {
                cost: 100,
                penalty: Some(100),
                limit: None,
                kind: ActionKind::Attackers(AiPlaneCfg {
                    kind: AiPlaneKind::Helicopter,
                    duration: Some(2),
                    template: "RATTACKHELI".into(),
                    altitude: 500.,
                    altitude_typ: AltType::RADIO,
                    speed: 80.,
                }),
            },
        ),
        (
            "attack-waypoint".into(),
            Action {
                cost: 5,
                penalty: None,
                limit: None,
                kind: ActionKind::AttackersWaypoint,
            },
        ),
        (
            "paratroops".into(),
            Action {
                cost: 50,
                penalty: Some(100),
                limit: None,
                kind: ActionKind::Paratrooper(DeployableCfg {
                    name: "Standard".into(),
                    plane: AiPlaneCfg {
                        kind: AiPlaneKind::Helicopter,
                        template: "RTROOPCARRIER".into(),
                        altitude: 500.,
                        altitude_typ: AltType::RADIO,
                        speed: 70.,
                        duration: None,
                    },
                }),
            },
        ),
        (
            "deploy-ewr".into(),
            Action {
                cost: 50,
                penalty: Some(100),
                limit: None,
                kind: ActionKind::Deployable(DeployableCfg {
                    name: "1L13".into(),
                    plane: AiPlaneCfg {
                        kind: AiPlaneKind::Helicopter,
                        template: "RTROOPCARRIER".into(),
                        altitude: 500.,
                        altitude_typ: AltType::RADIO,
                        duration: None,
                        speed: 70.,
                    },
                }),
            },
        ),
        (
            "repair".into(),
            Action {
                cost: 100,
                penalty: Some(100),
                limit: None,
                kind: ActionKind::LogisticsRepair(AiPlaneCfg {
                    kind: AiPlaneKind::Helicopter,
                    template: "RCARGOCARRIER".into(),
                    altitude: 500.,
                    altitude_typ: AltType::RADIO,
                    duration: None,
                    speed: 70.,
                }),
            },
        ),
        (
            "transfer".into(),
            Action {
                cost: 100,
                penalty: Some(100),
                limit: None,
                kind: ActionKind::LogisticsTransfer(AiPlaneCfg {
                    kind: AiPlaneKind::Helicopter,
                    template: "RCARGOCARRIER".into(),
                    altitude: 500.,
                    altitude_typ: AltType::RADIO,
                    duration: None,
                    speed: 70.,
                }),
            },
        ),
        (
            "nuke".into(),
            Action {
                cost: 5000,
                penalty: None,
                limit: None,
                kind: ActionKind::Nuke(NukeCfg {
                    cost_scale: 5,
                    power: 1000,
                }),
            },
        ),
        (
            "move".into(),
            Action {
                cost: 10,
                penalty: Some(25),
                limit: None,
                kind: ActionKind::Move(MoveCfg {
                    troop: 1000,
                    deployable: 1000,
                }),
            },
        ),
    ])
}

fn default_blue_actions() -> IndexMap<String, Action, FxBuildHasher> {
    IndexMap::from_iter([
        (
            "awacs".into(),
            Action {
                cost: 100,
                penalty: Some(100),
                limit: None,
                kind: ActionKind::Awacs(AwacsCfg {
                    plane: AiPlaneCfg {
                        kind: AiPlaneKind::FixedWing,
                        duration: Some(8),
                        template: "BAWACS".into(),
                        altitude: 11000.,
                        altitude_typ: AltType::BARO,
                        speed: 200.,
                    },
                    ewr: DeployableEwr { range: 400000 },
                }),
            },
        ),
        (
            "awacs-waypoint".into(),
            Action {
                cost: 10,
                penalty: None,
                limit: None,
                kind: ActionKind::AwacsWaypoint,
            },
        ),
        (
            "basket-tanker".into(),
            Action {
                cost: 50,
                penalty: Some(50),
                limit: None,
                kind: ActionKind::Tanker(AiPlaneCfg {
                    kind: AiPlaneKind::FixedWing,
                    duration: Some(8),
                    template: "BBASKETTANKER".into(),
                    altitude: 10000.,
                    altitude_typ: AltType::BARO,
                    speed: 180.,
                }),
            },
        ),
        (
            "boom-tanker".into(),
            Action {
                cost: 50,
                penalty: Some(50),
                limit: None,
                kind: ActionKind::Tanker(AiPlaneCfg {
                    kind: AiPlaneKind::FixedWing,
                    duration: Some(8),
                    template: "BBOOMTANKER".into(),
                    altitude: 10000.,
                    altitude_typ: AltType::BARO,
                    speed: 180.,
                }),
            },
        ),
        (
            "tanker-waypoint".into(),
            Action {
                cost: 10,
                penalty: None,
                limit: None,
                kind: ActionKind::TankerWaypoint,
            },
        ),
        (
            "drone".into(),
            Action {
                cost: 25,
                penalty: Some(25),
                limit: None,
                kind: ActionKind::Drone(DroneCfg {
                    plane: AiPlaneCfg {
                        kind: AiPlaneKind::FixedWing,
                        duration: Some(12),
                        template: "BDRONE".into(),
                        altitude: 7000.,
                        altitude_typ: AltType::BARO,
                        speed: 90.,
                    },
                    jtac: DeployableJtac {
                        range: 16000,
                        nolos: true,
                    },
                }),
            },
        ),
        (
            "drone-waypoint".into(),
            Action {
                cost: 5,
                penalty: None,
                limit: None,
                kind: ActionKind::DroneWaypoint,
            },
        ),
        (
            "bomber".into(),
            Action {
                cost: 100,
                penalty: Some(100),
                limit: None,
                kind: ActionKind::Bomber(BomberCfg {
                    targets: 15,
                    power: 1000,
                    accuracy: 15,
                    plane: AiPlaneCfg {
                        kind: AiPlaneKind::FixedWing,
                        template: "BBOMBER".into(),
                        duration: None,
                        altitude: 12000.,
                        altitude_typ: AltType::BARO,
                        speed: 300.,
                    },
                }),
            },
        ),
        (
            "fighters".into(),
            Action {
                cost: 100,
                penalty: Some(100),
                limit: None,
                kind: ActionKind::Fighters(AiPlaneCfg {
                    kind: AiPlaneKind::FixedWing,
                    duration: Some(2),
                    template: "BFIGHTERS".into(),
                    altitude: 10000.,
                    altitude_typ: AltType::BARO,
                    speed: 250.,
                }),
            },
        ),
        (
            "fighters-waypoint".into(),
            Action {
                cost: 5,
                penalty: None,
                limit: None,
                kind: ActionKind::FighersWaypoint,
            },
        ),
        (
            "attack-helicopters".into(),
            Action {
                cost: 100,
                penalty: Some(100),
                limit: None,
                kind: ActionKind::Attackers(AiPlaneCfg {
                    kind: AiPlaneKind::Helicopter,
                    duration: Some(2),
                    template: "BATTACKHELI".into(),
                    altitude: 500.,
                    altitude_typ: AltType::RADIO,
                    speed: 60.,
                }),
            },
        ),
        (
            "attack-waypoint".into(),
            Action {
                cost: 5,
                penalty: None,
                limit: None,
                kind: ActionKind::AttackersWaypoint,
            },
        ),
        (
            "paratroops".into(),
            Action {
                cost: 50,
                penalty: Some(100),
                limit: None,
                kind: ActionKind::Paratrooper(DeployableCfg {
                    name: "Standard".into(),
                    plane: AiPlaneCfg {
                        kind: AiPlaneKind::Helicopter,
                        template: "BTROOPCARRIER".into(),
                        altitude: 500.,
                        altitude_typ: AltType::RADIO,
                        duration: None,
                        speed: 60.,
                    },
                }),
            },
        ),
        (
            "deploy-ewr".into(),
            Action {
                cost: 50,
                penalty: Some(100),
                limit: None,
                kind: ActionKind::Deployable(DeployableCfg {
                    name: "AN/FPS-117".into(),
                    plane: AiPlaneCfg {
                        kind: AiPlaneKind::Helicopter,
                        template: "BTROOPCARRIER".into(),
                        altitude: 500.,
                        altitude_typ: AltType::RADIO,
                        duration: None,
                        speed: 60.,
                    },
                }),
            },
        ),
        (
            "repair".into(),
            Action {
                cost: 100,
                penalty: Some(100),
                limit: None,
                kind: ActionKind::LogisticsRepair(AiPlaneCfg {
                    kind: AiPlaneKind::Helicopter,
                    template: "BCARGOCARRIER".into(),
                    altitude: 500.,
                    altitude_typ: AltType::RADIO,
                    duration: None,
                    speed: 60.,
                }),
            },
        ),
        (
            "transfer".into(),
            Action {
                cost: 100,
                penalty: Some(100),
                limit: None,
                kind: ActionKind::LogisticsTransfer(AiPlaneCfg {
                    kind: AiPlaneKind::Helicopter,
                    template: "BCARGOCARRIER".into(),
                    altitude: 500.,
                    altitude_typ: AltType::RADIO,
                    duration: None,
                    speed: 60.,
                }),
            },
        ),
        (
            "nuke".into(),
            Action {
                cost: 5000,
                penalty: None,
                limit: None,
                kind: ActionKind::Nuke(NukeCfg {
                    cost_scale: 5,
                    power: 1000,
                }),
            },
        ),
        (
            "move".into(),
            Action {
                cost: 10,
                penalty: Some(25),
                limit: None,
                kind: ActionKind::Move(MoveCfg {
                    troop: 1000,
                    deployable: 1000,
                }),
            },
        ),
    ])
}

impl Default for Cfg {
    fn default() -> Self {
        Self {
            admins: FxHashMap::from_iter([(
                "f279deb7a6b62c96a78eca3ddb2bd8d0".parse().unwrap(),
                "REAPER 32 | EvilKipper".into(),
            )]),
            banned: FxHashMap::default(),
            max_msgs_per_second: 3,
            repair_time: 1800,
            repair_crate: default_repair_crate(),
            shutdown: Some(10),
            rules: Rules {
                actions: Rule::AlwaysAllowed,
                cargo: Rule::AlwaysAllowed,
                troops: Rule::AlwaysAllowed,
                jtac: Rule::AlwaysAllowed,
                ca: Rule::AlwaysAllowed,
            },
            points: Some(PointsCfg {
                new_player_join: 25,
                air_kill: 25,
                ground_kill: 2,
                lr_sam_bonus: 5,
                logistics_repair: 25,
                logistics_transfer: 15,
                capture: 15,
            }),
            warehouse: Some(WarehouseConfig {
                hub_max: 25,
                airbase_max: 5,
                tick: 10,
                ticks_per_delivery: 6,
                supply_transfer_crate: default_supply_transfer_crate(),
                supply_transfer_size: 25,
                supply_source: FxHashMap::from_iter([
                    (Side::Blue, "BINVENTORY".into()),
                    (Side::Red, "RINVENTORY".into()),
                ]),
            }),
            logistics_exclusion: 10000,
            unit_cull_distance: 37040, // 20 nm
            ground_vehicle_cull_distance: 10000,
            cull_after: 1800,
            slow_timed_events_freq: 10,
            threatened_distance: default_threatened_distance(),
            threatened_cooldown: 300,
            crate_load_distance: 50,
            crate_spread: 250,
            artillery_mission_range: 15000,
            side_switches: Some(1),
            max_crates: Some(4),
            default_lives: FxHashMap::from_iter([
                (LifeType::Standard, (3, 21600)),
                (LifeType::Intercept, (4, 21600)),
                (LifeType::Attack, (4, 21600)),
                (LifeType::Logistics, (6, 21600)),
                (LifeType::Recon, (6, 21600)),
            ]),
            life_types: default_life_types(),
            actions: FxHashMap::from_iter([
                (Side::Red, default_red_actions()),
                (Side::Blue, default_blue_actions()),
            ]),
            cargo: default_cargo(),
            crate_template: FxHashMap::from_iter([
                (Side::Red, "RCRATE".into()),
                (Side::Blue, "BCRATE".into()),
            ]),
            deployables: FxHashMap::from_iter([
                (Side::Red, default_red_deployables()),
                (Side::Blue, default_blue_deployables()),
            ]),
            troops: FxHashMap::from_iter([
                (Side::Red, default_red_troops()),
                (Side::Blue, default_blue_troops()),
            ]),
            unit_classification: default_unit_classification(),
            airborne_jtacs: FxHashMap::from_iter([
                (
                    "L-39ZA".into(),
                    DeployableJtac {
                        range: 16000,
                        nolos: true,
                    },
                ),
                (
                    "MB-339A".into(),
                    DeployableJtac {
                        range: 16000,
                        nolos: true,
                    },
                ),
            ]),
            jtac_priority: default_jtac_priority(),
            extra_fixed_wing_objectives: FxHashSet::default(),
        }
    }
}
