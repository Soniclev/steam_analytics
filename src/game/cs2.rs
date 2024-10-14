use serde::Serialize;

// Enum for item categories
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum ItemCategory {
    Sticker,
    Case,
    Key,
    Pin,
    Patch,
    PatchPack,
    Package,
    AutographCapsule,
    Graffiti,
    MusicKit,
    Pass,
    Agent,
    Gift,
    Tool,
    Weapon(String), // We'll use this for specific weapons like "AK" or "M4A1"
    Unknown,        // Fallback category if no match is found
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
        "Michael Syfers  | FBI Sniper",  // thanks Valve for the double space typo
    ];
    
    

    // TODO: mb extract type from html page
    static WEAPON_NAMES: [&str; 66 ] = [
        "ak-47", 
        "m4a1-s", 
        "m4a4", 
        "awp", 
        "g3", 
        "sg 553", 
        "mag-7",
        "galil", 
        "famas", 
        "p90", 
        "xm1014", 
        "ump45", 
        "mac-10", 
        "mp5-sd",
        "mp7",
        "mp9", 
        "five-seven", 
        "p250", 
        "glock", 
        "sawed-off", 
        "m249", 
        "negev", 
        "nova", 
        "mag7", 
        "sawed-off", 
        "m24", 
        "ump", 
        "ssg 08", 
        "scar-20",
        "dual berettas",
        "desert eagle",
        "usp-s",
        "cz75-auto",
        "tec-9",
        "r8 revolver",
        "p2000",
        "pp-bizon",
        "aug",
        "zeus x27",
        "butterfly knife",
        "karambit",
        "huntsman knife",
        "skeleton knife",
        "gut knife",
        "classic knife",
        "flip knife",
        "shadow daggers",
        "bayonet",
        "bowie knife",
        "paracord knife",
        "falchion knife",
        "ursus knife",
        "survival knife",
        "stiletto knife",
        "nomad knife",
        "navaja knife",
        "talon knife",
        "kukri knife",

        "broken fang gloves",
        "moto gloves",
        "hand wraps",
        "specialist gloves",
        "sport gloves",
        "driver gloves",
        "hydra gloves",
        "bloodhound gloves",
];

    // Match patterns for each category
    if item_name_lower.contains("sticker") {
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
    }
    else if item_name_lower.contains("key") {
        return ItemCategory::Key;
    }
    else if item_name_lower.ends_with("pin") {
        return ItemCategory::Pin;
    }
    else if item_name_lower.starts_with("patch") {
        return ItemCategory::Patch;
    }
    else if item_name_lower.contains("patch pack") {
        return ItemCategory::PatchPack;
    }
    else if item_name_lower.ends_with("package") {
        return ItemCategory::Package;
    }
    else if item_name_lower.contains("autograph capsule") {
        return ItemCategory::AutographCapsule;
    }
    else if item_name_lower.starts_with("sealed graffiti") {
        return ItemCategory::Graffiti;
    }
    else if item_name_lower.contains("music kit") {
        return ItemCategory::MusicKit;
    }
    else if item_name_lower.ends_with("pass") || item_name_lower.ends_with("pass + 3 souvenir tokens"){
        return ItemCategory::Pass;
    }
    else if item_name_lower.contains("gift") 
    || item_name_lower.contains("gift pack")
    || item_name == "Pallet of Presents"
    || item_name == "Audience Participation Parcel"
    {
        return ItemCategory::Gift;
    }
    else if item_name == "Name Tag" || item_name == "StatTrak™ Swap Tool" {
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

    // Fallback if no category matches
    ItemCategory::Unknown
}


