use serde::Serialize;

// Enum for item categories
#[derive(Debug, Clone, PartialEq, Serialize, Eq, Hash)]
pub enum ItemCategory {
    Sticker,
    Case,
    Key,
    Pin,
    Patch,
    PatchPack,
    Package,
    StickerCapsule,
    AutographCapsule,
    Graffiti,
    MusicKit,
    Pass,
    Agent,
    Gift,
    Tool,
    Knife(String),
    Glove(String),
    Weapon(String), // We'll use this for specific weapons like "AK" or "M4A1"
    Unknown,        // Fallback category if no match is found
}

impl ToString for ItemCategory {
    fn to_string(&self) -> String {
        match self {
            ItemCategory::Sticker => "Sticker".to_string(),
            ItemCategory::Case => "Case".to_string(),
            ItemCategory::Key => "Key".to_string(),
            ItemCategory::Pin => "Pin".to_string(),
            ItemCategory::Patch => "Patch".to_string(),
            ItemCategory::PatchPack => "PatchPack".to_string(),
            ItemCategory::Package => "Package".to_string(),
            ItemCategory::StickerCapsule => "StickerCapsule".to_string(),
            ItemCategory::AutographCapsule => "AutographCapsule".to_string(),
            ItemCategory::Graffiti => "Graffiti".to_string(),
            ItemCategory::MusicKit => "MusicKit".to_string(),
            ItemCategory::Pass => "Pass".to_string(),
            ItemCategory::Agent => "Agent".to_string(),
            ItemCategory::Gift => "Gift".to_string(),
            ItemCategory::Tool => "Tool".to_string(),
            ItemCategory::Knife(knife_name) => knife_name.to_string(),
            ItemCategory::Glove(glove_name) => glove_name.to_string(),
            ItemCategory::Weapon(weapon_name) => weapon_name.to_string(),
            ItemCategory::Unknown => "Unknown".to_string(),
        }
    }
}

// Function to determine the category based on the item name
pub fn determine_item_category(item_name: &str) -> ItemCategory {
    // Convert the name to lowercase for easier matching
    let item_name_lower = item_name.to_lowercase();

    static AGENT_NAMES: [&str; 66] = [
        "Primeiro Tenente | Brazilian 1st Battalion",
        "Cmdr. Frank 'Wet Sox' Baroud | SEAL Frogman",
        "Lieutenant Rex Krikey | SEAL Frogman",
        "Cmdr. Davida 'Goggles' Fernandez | SEAL Frogman",
        "'Blueberries' Buckshot | NSWC SEAL",
        "Buckshot | NSWC SEAL",
        "Lt. Commander Ricksaw | NSWC SEAL",
        "Seal Team 6 Soldier | NSWC SEAL",
        "D Squadron Officer | NZSAS",
        "Osiris | Elite Crew",
        "Ground Rebel | Elite Crew",
        "Jungle Rebel | Elite Crew",
        "Prof. Shahmat | Elite Crew",
        "Rezan The Ready | Sabre",
        "Dragomir | Sabre",
        "Blackwolf | Sabre",
        "'The Doctor' Romanov | Sabre",
        "Maximus | Sabre",
        "Rezan the Redshirt | Sabre",
        "Dragomir | Sabre Footsoldier",
        "1st Lieutenant Farlow | SWAT",
        "Bio-Haz Specialist | SWAT",
        "Chem-Haz Specialist | SWAT",
        "Cmdr. Mae 'Dead Cold' Jamison | SWAT",
        "John 'Van Healen' Kask | SWAT",
        "Lieutenant 'Tree Hugger' Farlow | SWAT",
        "Sergeant Bombson | SWAT",
        "Operator | FBI SWAT",
        "Markus Delrow | FBI HRT",
        "Special Agent Ava | FBI",
        "Slingshot | Phoenix",
        "Enforcer | Phoenix",
        "Patch | Phoenix",
        "Sealed Graffiti | Phoenix",
        "Soldier | Phoenix",
        "Street Soldier | Phoenix",
        "Safecracker Voltzmann | The Professionals",
        "Getaway Sally | The Professionals",
        "Little Kev | The Professionals",
        "Number K | The Professionals",
        "Sir Bloody Darryl Royale | The Professionals",
        "Sir Bloody Loudmouth Darryl | The Professionals",
        "Sir Bloody Miami Darryl | The Professionals",
        "Sir Bloody Silent Darryl | The Professionals",
        "Sir Bloody Skullhead Darryl | The Professionals",
        "Bloody Darryl The Strapped | The Professionals",
        "Trapper Aggressor | Guerrilla Warfare",
        "Crasswater The Forgotten | Guerrilla Warfare",
        "Elite Trapper Solman | Guerrilla Warfare",
        "'Medium Rare' Crasswater | Guerrilla Warfare",
        "Trapper | Guerrilla Warfare",
        "Col. Mangos Dabisi | Guerrilla Warfare",
        "Arno The Overgrown | Guerrilla Warfare",
        "Vypa Sista of the Revolution | Guerrilla Warfare",
        "'Two Times' McCoy | USAF TACP",
        "Officer Jacques Beltram | Gendarmerie Nationale",
        "Sous-Lieutenant Medic | Gendarmerie Nationale",
        "Aspirant | Gendarmerie Nationale",
        "Chef d'Escadron Rouchard | Gendarmerie Nationale",
        "Chem-Haz Capitaine | Gendarmerie Nationale",
        "The Elite Mr. Muhlik | Elite Crew",
        "Ground Rebel  | Elite Crew", // thanks Valve for the double space typo
        "B Squadron Officer | SAS",
        "3rd Commando Company | KSK",
        "'Two Times' McCoy | TACP Cavalry",
        "Michael Syfers  | FBI Sniper", // thanks Valve for the double space typo
    ];

    static WEAPON_NAMES: [&str; 35] = [
        // Rifles
        "ak-47",
        "m4a1-s",
        "m4a4",
        "g3",
        "sg 553",
        "galil",
        "famas",
        "aug",
        // Sniper Rifles
        "awp",
        "ssg 08",
        "scar-20",
        // Shotguns
        "mag-7",
        "xm1014",
        "sawed-off",
        "nova",
        // Submachine Guns
        "p90",
        "ump-45",
        "mac-10",
        "mp5-sd",
        "mp7",
        "mp9",
        "pp-bizon",
        // Pistols
        "five-seven",
        "p250",
        "glock",
        "dual berettas",
        "desert eagle",
        "usp-s",
        "cz75-auto",
        "tec-9",
        "r8 revolver",
        "p2000",
        // Machine Guns
        "m249",
        "negev",
        // Equipment
        "zeus x27",
    ];

    static KNIFE_NAMES: [&str; 19] = [
        "bayonet",
        "bowie knife",
        "butterfly knife",
        "classic knife",
        "falchion knife",
        "flip knife",
        "gut knife",
        "huntsman knife",
        "karambit",
        "kukri knife",
        "navaja knife",
        "nomad knife",
        "paracord knife",
        "skeleton knife",
        "shadow daggers",
        "stiletto knife",
        "survival knife",
        "talon knife",
        "ursus knife",
    ];

    static GLOVE_NAMES: [&str; 8] = [
        "bloodhound gloves",
        "broken fang gloves",
        "driver gloves",
        "hand wraps",
        "hydra gloves",
        "moto gloves",
        "specialist gloves",
        "sport gloves",
    ];

    if item_name_lower.contains("sticker capsule") {
        return ItemCategory::StickerCapsule;
    }

    // Match patterns for each category
    if item_name_lower.starts_with("sticker") {
        return ItemCategory::Sticker;
    } else if item_name_lower.ends_with("case")
        || item_name_lower.ends_with("case 1")
        || item_name_lower.ends_with("case 2")
        || item_name_lower.ends_with("case 3")
        || item_name_lower.ends_with("(holo-foil)")
        || item_name_lower.ends_with("(holo)")
        || item_name_lower.ends_with("(foil)")
        || item_name_lower.ends_with("capsule")
        || item_name_lower.ends_with("capsule 1")
        || item_name_lower.ends_with("capsule 2")
        || item_name_lower.ends_with("capsule series 1")
        || item_name_lower.ends_with("capsule series 2")
        || item_name_lower.ends_with("capsule series 3")
        || "Community Graffiti Box 1" == item_name
        || "Community Capsule 2018" == item_name
        || item_name_lower.ends_with("box")
        || item_name_lower.ends_with("challengers")
        || item_name_lower.ends_with("contenders")
        || item_name_lower.ends_with("legends")
    {
        return ItemCategory::Case;
    } else if item_name_lower.contains("key") {
        return ItemCategory::Key;
    } else if item_name_lower.ends_with("pin") {
        return ItemCategory::Pin;
    } else if item_name_lower.starts_with("patch") {
        return ItemCategory::Patch;
    } else if item_name_lower.contains("patch pack") {
        return ItemCategory::PatchPack;
    } else if item_name_lower.ends_with("package") {
        return ItemCategory::Package;
    } else if item_name_lower.contains("autograph capsule") {
        return ItemCategory::AutographCapsule;
    } else if item_name_lower.starts_with("sealed graffiti") {
        return ItemCategory::Graffiti;
    } else if item_name_lower.contains("music kit") {
        return ItemCategory::MusicKit;
    } else if item_name_lower.ends_with("pass")
        || item_name_lower.ends_with("pass + 3 souvenir tokens")
    {
        return ItemCategory::Pass;
    } else if item_name_lower.contains("gift")
        || item_name_lower.contains("gift pack")
        || item_name == "Pallet of Presents"
        || item_name == "Audience Participation Parcel"
    {
        return ItemCategory::Gift;
    } else if item_name == "Name Tag" || item_name == "StatTrak™ Swap Tool" {
        return ItemCategory::Tool;
    }

    for agent_name in AGENT_NAMES {
        if item_name == agent_name {
            return ItemCategory::Agent;
        }
    }

    for weapon_name in WEAPON_NAMES {
        if item_name_lower.contains(weapon_name) {
            return ItemCategory::Weapon(weapon_name.to_string());
        }
    }

    for knife_name in KNIFE_NAMES {
        if item_name_lower.contains(knife_name) {
            return ItemCategory::Knife(knife_name.to_string());
        }
    }

    for glove_name in GLOVE_NAMES {
        if item_name_lower.contains(glove_name) {
            return ItemCategory::Glove(glove_name.to_string());
        }
    }

    // Fallback if no category matches
    ItemCategory::Unknown
}
