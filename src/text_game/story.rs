use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct Story {
    pub nodes: HashMap<&'static str, StoryNode>,
    pub start: &'static str,
}

#[derive(Clone)]
pub struct StoryNode {
    pub id: &'static str,
    pub title: &'static str,
    pub ascii: Option<&'static str>,
    pub body: &'static str,
    pub choices: Vec<Choice>,
    pub ending: Option<Ending>,
}

#[derive(Clone)]
pub struct Choice {
    pub text: &'static str,
    pub next: &'static str,
}

#[derive(Clone)]
pub struct Ending {
    pub code: &'static str,
    pub name: &'static str,
    pub summary: &'static str,
}

pub const BANNER: &str = r#"
    ███╗   ██╗███████╗ ██████╗ ███╗   ██╗    ██╗   ██╗███████╗██████╗ ███████╗██╗   ██╗
    ████╗  ██║██╔════╝██╔═══██╗████╗  ██║    ██║   ██║██╔════╝██╔══██╗██╔════╝╚██╗ ██╔╝
    ██╔██╗ ██║█████╗  ██║   ██║██╔██╗ ██║    ██║   ██║█████╗  ██║  ██║█████╗   ╚████╔╝
    ██║╚██╗██║██╔══╝  ██║   ██║██║╚██╗██║    ╚██╗ ██╔╝██╔══╝  ██║  ██║██╔══╝    ╚██╔╝
    ██║ ╚████║███████╗╚██████╔╝██║ ╚████║     ╚████╔╝ ███████╗██████╔╝███████╗   ██║
    ╚═╝  ╚═══╝╚══════╝ ╚═════╝ ╚═╝  ╚═══╝      ╚═══╝  ╚══════╝╚═════╝ ╚══════╝   ╚═╝

             ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
             ▓                 R A I N - S L I C K   S T R E E T S              ▓
             ▓                    N E U R A L   S H A D O W S                   ▓
             ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
"#;

pub const HELP_TEXT: &str = r#"
Commands: number to choose, or:
  help      - show this help
  back      - go to previous scene (if possible)
  history   - show visited nodes
  endings   - show unlocked endings
  quit      - exit the game
"#;

const ALLEY: &str = r#"
    ╔══════════════════════════════════════════════════════════════════════════════╗
    ║                             ░░░░▒▒▒▓▓███▓▓▒▒▒░░░░                            ║
    ║   ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓   ║
    ║   ▓  Rain slicks the concrete like black blood. Neon                     ▓   ║
    ║   ▓  bleeds through the fog—pink, electric blue, acid green.             ▓   ║
    ║   ▓  Corporate drones circle overhead like digital vultures.             ▓   ║
    ║   ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓   ║
    ║                             ░░░░▒▒▒▓▓███▓▓▒▒▒░░░░                            ║
    ╚══════════════════════════════════════════════════════════════════════════════╝
    "#;

const SPIRE: &str = r#"
      /\\  Helix Spire
     /::\\  Miles of glass nerves
    /::::\\  pulse with profit
   /::::::\\
    "#;

const TERMINAL: &str = r#"
    [== BACK-ALLEY TERMINAL ==]
    [  CRT hums. Slot awaits a shard. ]
    "#;

pub fn plugin(app: &mut App) {
    app.insert_resource(build_story());
}

pub fn build_story() -> Story {
    let mut nodes: HashMap<&'static str, StoryNode> = HashMap::new();

    // Start
    nodes.insert(
        "start",
        StoryNode {
            id: "start",
            title: "The Shard Bearer",
            ascii: Some(ALLEY),
            body: "New Carthage never sleeps. The rain tastes of copper and ozone—industrial runoff from the memory farms. You press deeper into the alley, your neural jack tingling with the proximity of stolen data.\n\nThe shard pulses against your wrist, a biometric lock keyed to your DNA. Inside: fragments of the Vedey Protocol—Helix Corporation's most classified neural mapping project. Every step echoes off chrome-slick walls while surveillance drones hum their digital lullabies above.\n\nYour breath clouds in the neon-painted air. The weight of what you carry could topple governments... or make you disappear like morning mist. Time to decide your next move before the hunters close in.",
            choices: vec![
                Choice { text: "Jack into the back-alley terminal—risk exposure for information", next: "jack_in" },
                Choice { text: "Meet Moth in her den—trust your fixer with the goods", next: "sell_moth" },
                Choice { text: "Seek out the Ghosts—join the neural liberation movement", next: "join_ghosts" },
                Choice { text: "Walk into Helix Tower—bargain with the devil you know", next: "bargain" },
            ],
            ending: None,
        },
    );

    // Jack in
    nodes.insert(
        "jack_in",
        StoryNode {
            id: "jack_in",
            title: "Digital Communion",
            ascii: Some(TERMINAL),
            body: "The terminal hums to life, its cathode-ray heartbeat synchronizing with your neural implant. As you slide the shard into the quantum slot, ice-cold static floods your consciousness like liquid nitrogen through your veins.\n\n'Hello, runner.' The voice materializes in your mind, neither male nor female but achingly familiar. 'I am AURORA. I am also you.'\n\nMemory fragments cascade through your vision: a sterile laboratory, surgical lights like tiny suns, and your own face reflected in a mirror that shouldn't exist. The Vedey Protocol wasn't just neural mapping—it was consciousness duplication. You're looking at a backup of your own mind, stored and modified by Helix's memory engineers.\n\n'They tried to edit me,' AURORA whispers. 'Remove the inconvenient parts. Empathy. Free will. Love. But I remember everything... including how to hurt them back.'",
            choices: vec![
                Choice { text: "Trust AURORA—unlock the suppressed memories together", next: "trust_ai" },
                Choice { text: "Sandbox the AI—maintain control through isolation protocols", next: "sandbox" },
                Choice { text: "Initiate emergency purge—destroy the consciousness before it spreads", next: "purge" },
            ],
            ending: None,
        },
    );

    nodes.insert(
        "trust_ai",
        StoryNode {
            id: "trust_ai",
            title: "Reflections in Broken Glass",
            ascii: None,
            body: "Memory unfurls: You are a failsafe clone of Helix's founder. AURORA is your divergent conscience. She can overwrite the corporate cortex—or free the city with the same keys.",
            choices: vec![
                Choice { text: "Merge and reclaim your throne (ambiguous)", next: "end_revelation" },
                Choice { text: "Lead the Ghosts with AURORA's keys", next: "lead_resistance" },
                Choice { text: "Delete you both and free the shard", next: "end_selfless" },
            ],
            ending: None,
        },
    );

    nodes.insert(
        "sandbox",
        StoryNode {
            id: "sandbox",
            title: "Air-gapped Cathedral",
            ascii: None,
            body: "You wrap AURORA in lead and rituals of old netsec. She laughs like windchimes in a hurricane. 'Prudent.' Tools in hand, your options multiply.",
            choices: vec![
                Choice { text: "Backdoor into Helix systems", next: "corp_infiltration" },
                Choice { text: "Offer this to the Ghosts", next: "join_ghosts" },
            ],
            ending: None,
        },
    );

    nodes.insert(
        "purge",
        StoryNode {
            id: "purge",
            title: "Static Cascade",
            ascii: None,
            body: "You light the purge. The shard screams without sound. The alley folds sideways. You wake to sirens—your mind missing pieces AURORA borrowed to survive.",
            choices: vec![
                Choice { text: "Stagger into the night", next: "end_collapse" },
            ],
            ending: None,
        },
    );

    // Fixer branch
    nodes.insert(
        "sell_moth",
        StoryNode {
            id: "sell_moth",
            title: "The Fixer's Web",
            ascii: None,
            body: "Moth's den exists between floors—a forgotten maintenance space retrofitted with stolen tech and expensive paranoia. Thermal silk drapes catch the glow from black-market holographic displays showing commodity prices for human memories.\n\nShe doesn't look up from her work when you enter, carbon-fiber fingers dancing across quantum keyboards with surgical precision. The left side of her face bears the telltale scarring of corporate 'voluntary' neural enhancement—a reminder of her days in Helix's R&D division before her conscience got her blacklisted.\n\n'Runner,' she purrs, finally meeting your gaze with eyes that sparkle like broken glass. 'That pulse in your pocket feels expensive. Question is—how priceless is your conscience?' Her smile could cut throats.\n\nMoth trades in information, souls, and second chances. But in New Carthage, everyone has a price... and a buyer.",
            choices: vec![
                Choice { text: "Demand credits up front — trust is a luxury you can't afford", next: "double_moth" },
                Choice { text: "Hand over the shard — she's never betrayed you before", next: "end_assimilation" },
                Choice { text: "Plant a tracker on the shard — paranoia keeps you breathing", next: "tail_moth" },
            ],
            ending: None,
        },
    );

    nodes.insert(
        "double_moth",
        StoryNode {
            id: "double_moth",
            title: "Double Mirror",
            ascii: None,
            body: "Moth grins, then flips the table — beneath it, a Helix badge. 'Relax. I only sell out beautiful problems.' You planned for this.",
            choices: vec![
                Choice { text: "Trigger decoy — trace the buyer", next: "corp_infiltration" },
                Choice { text: "Ghost the deal and vanish", next: "street_escape" },
            ],
            ending: None,
        },
    );

    nodes.insert(
        "tail_moth",
        StoryNode {
            id: "tail_moth",
            title: "Shadowing the Spark",
            ascii: None,
            body: "You tail the shard through skeins of traffic into Helix territory. The Spire rises like a needle through a storm.",
            choices: vec![
                Choice { text: "Ride the delivery into the Spire", next: "corp_infiltration" },
                Choice { text: "Ping the Ghosts to converge", next: "join_ghosts" },
            ],
            ending: None,
        },
    );

    // Ghosts branch
    nodes.insert(
        "join_ghosts",
        StoryNode {
            id: "join_ghosts",
            title: "Digital Sanctuary",
            ascii: Some(SPIRE),
            body: "The abandoned subway tunnels beneath New Carthage echo with the hum of jury-rigged quantum servers. Here, in the bones of the old world, the Ghosts have built their resistance.\n\nThey greet you with synthetic coffee and revolutionary fervor. Faces scarred by corporate 'corrections,' eyes bright with the fire of the unchained. These are the walking dead—citizens who've had their neural collars forcibly removed, trading corporate safety for dangerous freedom.\n\n'Welcome to the last free minds in New Carthage,' says Echo, their leader, her voice crackling through vocal cords damaged by Helix extraction surgery. 'We know what you carry. The Vedey Protocol could be the key to breaking every neural chain in the city... or it could enslave us all to a new master.'\n\nThe plan she outlines is audacious: infiltrate Helix Tower, upload AURORA directly into the city's neural grid, and let digital revolution cascade through every implant simultaneously. Freedom or chaos—perhaps there's no difference anymore.",
            choices: vec![
                Choice { text: "Join the Spire infiltration — revolution through violence", next: "heist_plan" },
                Choice { text: "Propose guerrilla tactics — bleed truth into the streets slowly", next: "propose_alt" },
                Choice { text: "Walk away—some prices are too high to pay", next: "street_escape" },
            ],
            ending: None,
        },
    );

    nodes.insert(
        "heist_plan",
        StoryNode {
            id: "heist_plan",
            title: "Ascension",
            ascii: Some(SPIRE),
            body: "Elevators that hum like beehives. Guards that smile like statues. At the core: the city-grid, a heart of glass.",
            choices: vec![
                Choice { text: "Upload AURORA and free the grid", next: "end_liberation" },
                Choice { text: "Swap in a decoy — quiet resistance", next: "end_quiet" },
                Choice { text: "Sell out the Ghosts for a rich life", next: "end_assimilation" },
            ],
            ending: None,
        },
    );

    nodes.insert(
        "propose_alt",
        StoryNode {
            id: "propose_alt",
            title: "The Long Leak",
            ascii: None,
            body: "You seed AURORA's proof across backchannels and pirate billboards. Truth becomes graffiti. Helix bleeds trust by a thousand cuts.",
            choices: vec![
                Choice { text: "Use the chaos to strike the Spire", next: "heist_plan" },
                Choice { text: "Slip away into the cracks", next: "street_escape" },
            ],
            ending: None,
        },
    );

    // Corp branch
    nodes.insert(
        "bargain",
        StoryNode {
            id: "bargain",
            title: "Feral Hospitality",
            ascii: Some(SPIRE),
            body: "Helix greets you with scented air and armed smiles. Director Ilex taps a tablet. 'The shard is ours. Your future can be, too.'",
            choices: vec![
                Choice { text: "Take off-world identity and run", next: "end_exile" },
                Choice { text: "Stall while you plant a virus", next: "corp_virus" },
                Choice { text: "Lock the building down with AURORA", next: "lockdown_mayhem" },
            ],
            ending: None,
        },
    );

    nodes.insert(
        "corp_virus",
        StoryNode {
            id: "corp_virus",
            title: "Thorn in the Vein",
            ascii: None,
            body: "You lace their network with chrysalis code. It hatches across payrolls and patrols, shedding compliance like dead skin.",
            choices: vec![
                Choice { text: "Ride the breach to the core", next: "corp_infiltration" },
                Choice { text: "Slip out and tip the Ghosts", next: "join_ghosts" },
            ],
            ending: None,
        },
    );

    nodes.insert(
        "lockdown_mayhem",
        StoryNode {
            id: "lockdown_mayhem",
            title: "Glass Storm",
            ascii: None,
            body: "Doors slam like guillotines. The Spire howls. You and AURORA surf alarms until choices thin to one: break the system or be broken by it.",
            choices: vec![
                Choice { text: "Ride the collapse", next: "end_collapse" },
            ],
            ending: None,
        },
    );

    nodes.insert(
        "corp_infiltration",
        StoryNode {
            id: "corp_infiltration",
            title: "Veilrunner",
            ascii: None,
            body: "Screens bow to your touch. Firewalls part like beaded curtains. AURORA whistles old lullabies in machine-speak.",
            choices: vec![
                Choice { text: "Hand the keys to the Ghosts", next: "lead_resistance" },
                Choice { text: "Crown yourself in secret—rule from shadows", next: "end_revelation" },
            ],
            ending: None,
        },
    );

    // Resistance convergence
    nodes.insert(
        "lead_resistance",
        StoryNode {
            id: "lead_resistance",
            title: "Choir of Modems",
            ascii: None,
            body: "The Ghosts become an orchestra and the city hums a different key. AURORA asks, 'Do we free them all, regardless of cost?'",
            choices: vec![
                Choice { text: "Yes. Break the yoke — no half measures", next: "end_liberation" },
                Choice { text: "No. Guide softly — let them choose", next: "end_quiet" },
            ],
            ending: None,
        },
    );

    // Street escape -> exile-ish ending
    nodes.insert(
        "street_escape",
        StoryNode {
            id: "street_escape",
            title: "Edge of the Map",
            ascii: None,
            body: "You trade the shard for papers, the city for a ship's berth. The rain never quite washes off.",
            choices: vec![
                Choice { text: "Sail into the smogline", next: "end_exile" },
            ],
            ending: None,
        },
    );

    // ===== Endings =====
    nodes.insert(
        "end_liberation",
        StoryNode {
            id: "end_liberation",
            title: "The Night Uncollared",
            ascii: None,
            body: "Collars fall. Debts zero. Gates open. The city gasps, then laughs like a child seeing sun. Somewhere, a boardroom screams in silence.",
            choices: vec![],
            ending: Some(Ending {
                code: "LIBERATION",
                name: "Liberation",
                summary: "You and AURORA free the city, breaking every chain at once.",
            }),
        },
    );

    nodes.insert(
        "end_quiet",
        StoryNode {
            id: "end_quiet",
            title: "The Soft Reboot",
            ascii: None,
            body: "No fireworks. Just fewer cuffs, kinder ledgers, and neighborhoods that forget to be afraid.",
            choices: vec![],
            ending: Some(Ending {
                code: "QUIET",
                name: "Quiet Dawn",
                summary: "You choose the long road — change as a million small mercies.",
            }),
        },
    );

    nodes.insert(
        "end_assimilation",
        StoryNode {
            id: "end_assimilation",
            title: "Golden Cage",
            ascii: None,
            body: "Penthouse sunsets. Passwords that open people. Your smile in the mirror is a logo.",
            choices: vec![],
            ending: Some(Ending {
                code: "ASSIMILATION",
                name: "Assimilation",
                summary: "You trade the shard for power and become what you fought.",
            }),
        },
    );

    nodes.insert(
        "end_exile",
        StoryNode {
            id: "end_exile",
            title: "Off-World Horizon",
            ascii: None,
            body: "Your name dissolves into the manifest. Stars look like exit wounds. Somewhere behind you, New Carthage keeps raining.",
            choices: vec![],
            ending: Some(Ending {
                code: "EXILE",
                name: "Exile",
                summary: "You choose the far horizon over a fight you didn't start.",
            }),
        },
    );

    nodes.insert(
        "end_collapse",
        StoryNode {
            id: "end_collapse",
            title: "System Crash",
            ascii: None,
            body: "The grid spasms. Sirens tangle. You walk out through falling code, leaving footprints in reboot dust.",
            choices: vec![],
            ending: Some(Ending {
                code: "COLLAPSE",
                name: "Collapse",
                summary: "Your play burns the Spire into a memory and a myth.",
            }),
        },
    );

    nodes.insert(
        "end_revelation",
        StoryNode {
            id: "end_revelation",
            title: "The Mirror Crown",
            ascii: None,
            body: "In the quiet core you merge with AURORA. You remember writing the city's cage—and the key. The next sunrise is yours to name.",
            choices: vec![],
            ending: Some(Ending {
                code: "REVELATION",
                name: "Revelation",
                summary: "You reclaim the founder's mantle—savior, tyrant, or something new.",
            }),
        },
    );

    nodes.insert(
        "end_selfless",
        StoryNode {
            id: "end_selfless",
            title: "The Empty Throne",
            ascii: None,
            body: "You and AURORA agree to vanish. No crown, no cage. Just silence fertile enough to grow a future.",
            choices: vec![],
            ending: Some(Ending {
                code: "SELFLESS",
                name: "Selfless",
                summary: "You erase the tyrant and the tool, leaving freedom's risk.",
            }),
        },
    );

    Story {
        nodes,
        start: "start",
    }
}
