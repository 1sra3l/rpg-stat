/*!
# Legendary Creatures

A huge set of creatures from around the world.  This can be iterated through, and will hopefully 

*/
extern crate num;
use num::NumCast;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::stats::{ Builder, Basic, Normal, Advanced };

use std::ops::{Add, AddAssign,  Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
/*
This is public information from [Wikipedia](https://en.wikipedia.org/wiki/Lists_of_legendary_creatures)

The following is a list of lists of legendary creatures, beings and entities from the folklore record. Entries consist of legendary and unique creatures, not of particularly unique individuals of a commonly known species.

*/
#[derive(Clone, PartialEq, Copy, Debug, EnumIter)]//, Serialize, Deserialize)]
pub enum Legendary {
    /// [Á Bao A Qu](https://en.wikipedia.org/wiki/A_Bao_A_Qu)(Malay) – Entity that lives in the Tower of Victory in Chitor.
    ABaoAQu,
    /// [Aatxe](https://en.wikipedia.org/wiki/Aatxe)(Basque) – Bull spirit.
    Aatxe,
    /// [Abaasy](https://en.wikipedia.org/wiki/Abasy)(Yakuts) – Iron-toothed demons.
    Abaasy,
    /// [Äbädä](https://en.wikipedia.org/wiki/%C3%84b%C3%A4d%C3%A4)(Tatar) – Forest spirit.
    Abada,
    /// [Abaia](https://en.wikipedia.org/wiki/Abaia)(Tatar) – Forest spirit. (Melanesia) – Huge magical eel.
    Abaia,
    /// [Abarimon](https://en.wikipedia.org/wiki/Abarimon)(Medieval Bestiaries) – Savage humanoid with backward feet.
    Abarimon,
    /// [Abath](https://en.wikipedia.org/wiki/Abath)(Malay) – One-horned animal.
    Abath,
    /// [Abura-sumashi](https://en.wikipedia.org/wiki/Abura-sumashi)(Japanese) – Creature from a mountain pass in Kumamoto Prefecture.
    AburaSumashi,
    /// [Acephali](https://en.wikipedia.org/wiki/Headless_men)(Greek) – Headless humanoids.
    Acephali,
    /// [](https://en.wikipedia.org/wiki/)(Mitologia Hindu) – Disease-bringing ghost.
    Acheri,
    /// [](https://en.wikipedia.org/wiki/)(Roman) – Curious elk.
    Achlis,
    /// [Adar Llwch Gwin](https://en.wikipedia.org/wiki/)(Welsh) – Giant birds that understand human languages.
    AdarLlwchGwin,
    /// [](https://en.wikipedia.org/wiki/)(Solomon Islands) – Malevolent merfolk.
    Adaro,
    /// [](https://en.wikipedia.org/wiki/)(Manx) – Nature spirit.
    Adhene,
    /// [](https://en.wikipedia.org/wiki/)(Inuit) – Vampiric dog-human hybrid
    Adlet,
    /// [](https://en.wikipedia.org/wiki/)(Lugbara) – Nature spirit.
    Adroanzi,
    /// [](https://en.wikipedia.org/wiki/)(Ewe people) – African vampiric-forest being.
    Adze,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Disease demon.
    Aerico,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Norse deities.
    AEsir,
    /// [](https://en.wikipedia.org/wiki/)(Welsh) – Lake monster (exact lake varies by story).
    Afanc,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – God of fire and sacrifices.
    Agni,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Spirit of vinefields and grainfields.
    Agathodaemon,
    /// [](https://en.wikipedia.org/wiki/)(Inuit) – Ice spirit that aids hunters and fishermen.
    Agloolik,
    /// [](https://en.wikipedia.org/wiki/)(East Africa) – Small, ape-like humanoid.
    Agogwe,
    /// [](https://en.wikipedia.org/wiki/)(Inuit) – Animated skeleton that causes shipwrecks.
    Ahkiyyini,
    /// [](https://en.wikipedia.org/wiki/)(Aztec) – Anthropophagous dog-monkey hybrid.
    Ahuizotl,
    /// [](https://en.wikipedia.org/wiki/)(Zoroastrianism) – Zoroastrian spirits.
    Ahura,
    /// [](https://en.wikipedia.org/wiki/)(Khoikhoi) – Anthropophagous humanoid with eyes in its instep.
    Aigamuxa,
    /// [](https://en.wikipedia.org/wiki/)(Etruscan) – Fish-tailed goat.
    Aigikampoi,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Divine elephant.
    Airavata,
    /// [](https://en.wikipedia.org/wiki/)(Polynesian) – Malevolent spirits or demons.
    Aitu,
    /// [](https://en.wikipedia.org/wiki/)(Lithuanian) – Household spirit.
    Aitvaras,
    /// [](https://en.wikipedia.org/wiki/)(Finnish) – Dragon/snake female spirit, is said to spread diseases
    Ajatar,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Tree-dwelling monster.
    Akateko,
    /// [](https://en.wikipedia.org/wiki/)(Inuit) – Orca-wolf shapeshifter.
    Akhlut,
    /// [](https://en.wikipedia.org/wiki/)(Finnish) – Female spirits or minor goddesses.
    Akka,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Large, grotesque humanoid.
    Akki,
    /// [](https://en.wikipedia.org/wiki/)(Ainu) – Sea monster.
    Akkorokamui,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Evil spirit or devil
    Akuma,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Giant turtle that supports the world.
    Akupara,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Ghostly flame which causes disease.
    AkurojinNoHi,
    /// [](https://en.wikipedia.org/wiki/)(Armenian and Persian) – Spirit that steals unborn babies and livers from pregnant women.
    Al,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Bad weather demon.
    Ala,
    /// [](https://en.wikipedia.org/wiki/)(Chaldean) – Queen of the full moon.
    Alal,
    /// [](https://en.wikipedia.org/wiki/)(Philippine) – Winged humanoid that steals reproductive waste to make children.
    Alan,
    /// [](https://en.wikipedia.org/wiki/)(Heraldic) – Wingless griffin.
    Alce,
    /// [](https://en.wikipedia.org/wiki/)(Bengali) – Spirit of a dead fisherman.
    Aleya,
    /// [](https://en.wikipedia.org/wiki/)(Chilean) – Bird that eats gold and silver.
    Alicanto,
    /// [](https://en.wikipedia.org/wiki/)(Bestiario medieval) – Winged unicorn.
    Alicorn,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Angelic bird with human head and breasts.
    Alkonost,
    /// [](https://en.wikipedia.org/wiki/)(Heraldic) – Ass-camel hybrid.
    Allocamelus,
    /// [](https://en.wikipedia.org/wiki/)(Mongolian) – Savage humanoid.
    Almas,
    /// [](https://en.wikipedia.org/wiki/)(Islamic) – One-horned rabbit.
    AlMiRaj,
    /// [](https://en.wikipedia.org/wiki/)(Catalan) – Female water spirit.
    Aloja,
    /// [](https://en.wikipedia.org/wiki/)(Abenaki) – Little people and tricksters.
    AlomBagWinnosis,
    /// [](https://en.wikipedia.org/wiki/)(German) – Male night-demon.
    Alp,
    /// [](https://en.wikipedia.org/wiki/)(Heraldic) – Lion-like creature, sometimes with dragon or goat forelegs.
    Alphyn,
    /// [](https://en.wikipedia.org/wiki/)(Irish) – Parasitic fairy.
    AlpLuachra,
    /// [](https://en.wikipedia.org/wiki/)(Islamic) – Guard dog of the Seven Sleepers.
    AlRakim,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Grove nymph.
    Alseid,
    /// [](https://en.wikipedia.org/wiki/)(Assyrian) – Leprous demon.
    Alu,
    /// [](https://en.wikipedia.org/wiki/)(Mayan) – Little people.
    Alux,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Ritual disciplinary demon from Shikoku.
    Amaburakosagi,
    /// [](https://en.wikipedia.org/wiki/)(Tsimshian) – Giant who holds up the world.
    Amala,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Ritual disciplinary demon from Hokuriku.
    Amamehagi,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Small demon.
    Amanojaku,
    /// [](https://en.wikipedia.org/wiki/)(Inuit) – Giant wolf.
    Amarok,
    /// [](https://en.wikipedia.org/wiki/)(Quechua) – Water boa spirit.
    Amarum,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Disease-causing hag.
    AmazakeBabaa,
    /// [](https://en.wikipedia.org/wiki/)(Ainu) – Lake monster.
    Amemasu,
    /// [](https://en.wikipedia.org/wiki/)(Ancient Egyptian) – Female demon who was part lion, hippopotamus and crocodile and devoured the souls of the wicked.
    Ammit,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Tennyo from the island of Amami Ōshima.
    Amoronagu,
    /// [](https://en.wikipedia.org/wiki/)(Heraldic) – Winged serpent.
    Amphiptere,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Serpent with a head at each end.
    Amphisbaena,
    /// [](https://en.wikipedia.org/wiki/)(Jewish) – Giant.
    Anak,
    /// [](https://en.wikipedia.org/wiki/)(Ancient Egyptian) – Human-headed sphinx.
    Androsphinx,
    /// [](https://en.wikipedia.org/wiki/)(mainly Christian, Jewish, Islamic traditions) – Divine beings of Heaven who act as mediators between God and humans; the counterparts of Demons.
    Angel,
    /// [](https://en.wikipedia.org/wiki/)(Arabian) – Legendary Huge Satanic Eagle with Human Face. sometimes can resurrect herself like phoenix did.
    Anqa,
    /// [](https://en.wikipedia.org/wiki/)(Cherokee) – Lightning spirit.
    AniHyuntikwalaski,
    /// [](https://en.wikipedia.org/wiki/)(French) – Skeletal grave watcher with a lantern and scythe.
    Ankou,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Ritual disciplinary demon from Iwate Prefecture.
    Anmo,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Giant who was extremely strong as long as he remained in contact with the ground.
    Antaeus,
    /// [](https://en.wikipedia.org/wiki/)(Ancient Egyptian) – God of the Underworld
    Anubis,
    /// [](https://en.wikipedia.org/wiki/)(Finnish) – Subterranean giant.
    AnteroVipunen,
    /// [](https://en.wikipedia.org/wiki/)(Sumerian) – Divine storm bird
    Anzu,
    /// [](https://en.wikipedia.org/wiki/)(Guaraní) – Anthropophagous peccary or sheep.
    AoAo,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Blue monk who kidnaps children.
    Aobozu,
    /// [](https://en.wikipedia.org/wiki/)(Sumerian) – Fish-human hybrid that attends the god Enki.
    Apkallu,
    /// [](https://en.wikipedia.org/wiki/)(Buddhist and Hindu) – Female cloud spirit.
    Apsaras,
    /// [](https://en.wikipedia.org/wiki/)(Akkadian) – Human-scorpion hybrid.
    Aqrabuamelu,
    /// [](https://en.wikipedia.org/wiki/)(Akkadian) – Disease demon.
    ArdatLili,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Hundred-eyed giant.
    ArgusPanoptes,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Old woman with magical powers.
    ArikuraNoBaba,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – One-eyed humanoid.
    Arimaspi,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Swift green-maned talking horse.
    Arion,
    /// [](https://en.wikipedia.org/wiki/)(Manx) – Fairy hedgehog.
    ArkanSonney,
    /// [](https://en.wikipedia.org/wiki/)(Sumerian) – Hideous rock demon.
    Asag,
    /// [](https://en.wikipedia.org/wiki/)(Sumerian) – Demon.
    Asakku,
    /// [](https://en.wikipedia.org/wiki/)(West Africa) – Iron-toothed vampire.
    Asanbosam,
    /// [](https://en.wikipedia.org/wiki/)(Turkic) – Blue-maned wolf.
    Asena,
    /// [](https://en.wikipedia.org/wiki/)(Abenaki) – Stone giant.
    ASeneeKiWakw,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Invisible tendril that impedes movement.
    AshiMagari,
    /// [](https://en.wikipedia.org/wiki/)(Dahomey) – Vampiric possession spirit.
    Asiman,
    /// [](https://en.wikipedia.org/wiki/)(Germanic) – Female tree spirit.
    Askefrue,
    /// [](https://en.wikipedia.org/wiki/)(Abenaki) – Fire elemental and spectral fire.
    AskWeeDaEed,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Spectral fire from Kōchi Prefecture.
    Asobibi,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Island-sized whale or sea turtle.
    Aspidochelone,
    /// [](https://en.wikipedia.org/wiki/)(English) – Water spirit.
    Asrai,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Humanoid sustained by pleasant smells instead of food.
    Astomi,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Hindu malevolent divinities.
    Asura,
    /// [](https://en.wikipedia.org/wiki/)(Philippine) – Carrion-eating humanoid.
    Aswang,
    /// [](https://en.wikipedia.org/wiki/)(English) – Surprisingly small creature.
    Atomy,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Invisible spirit that follows people.
    AtoOiKozo,
    /// [](https://en.wikipedia.org/wiki/)(Inuit) – Anthropophagous spirit.
    Atshen,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Pasture nymph.
    Auloniad,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiary) – King of the birds.
    Avalerion,
    /// [](https://en.wikipedia.org/wiki/)(Abenaki) – Insect spirit.
    AwaHonDo,
    /// [](https://en.wikipedia.org/wiki/)(Ancient Egyptian) – Falcon-lion hybrid.
    Axex,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Sea serpent that travels over boats in an arc while dripping oil.
    Ayakashi,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Spectral fire from Ishikawa Prefecture.
    AyakashiNoAyashibi,
    /// [](https://en.wikipedia.org/wiki/)(Dahomey) – Little people that help hunters.
    Aziza,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Spirit that washes azuki beans along riversides.
    Azukiarai,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Spirit that washes azuki beans along riversides.
    Azukitogi,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Bean-grinding hag who devours people.
    Azukibabaa,
    /// [](https://en.wikipedia.org/wiki/)(Egyptian) – Soul of the deceased, depicted as a bird or a human-headed bird
    Ba,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Forest spirit and hag
    BabaYaga,
    /// [](https://en.wikipedia.org/wiki/)(Guyanese/Surinamese) – Malevolent little people
    Baccoo,
    /// [](https://en.wikipedia.org/wiki/)(Italian) – Goat-like creature from the southern central Alps
    Badalisc,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Malevolent water spirit
    Bagiennik,
    /// [](https://en.wikipedia.org/wiki/)(Arabian) – Giant fish
    Bahamut,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Talking beast which handed down knowledge on harmful spirits
    BaiZe,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Banana tree spirit
    BaJiaoGui,
    /// [](https://en.wikipedia.org/wiki/)(Indian) - Assamese shape-shifting aqueous creature
    Bak,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Ghostly whale skeleton that drifts along the coastline of Shimane Prefecture
    BakeKujira,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Magical cat
    Bakeneko,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Animated straw sandal
    Bakezori,
    /// [](https://en.wikipedia.org/wiki/)(Iranian) – Night demon
    Bakhtak,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Dream-devouring, tapir-like creature
    Baku,
    /// [](https://en.wikipedia.org/wiki/)(Philippine) – Sea serpent that causes eclipses
    Bakunawa,
    /// [](https://en.wikipedia.org/wiki/)(Romanian) – Multi-headed dragon
    Balaur,
    /// [](https://en.wikipedia.org/wiki/)(Albanian) – Sea monster
    Baloz,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Bathhouse spirit
    Bannik,
    /// [](https://en.wikipedia.org/wiki/)(Irish) – Screaming death spirit
    Banshee,
    /// [](https://en.wikipedia.org/wiki/)(Celtic Mythology) – Beautiful vampiric seductresses who prey on young travelers
    BaobhanSith,
    /// [](https://en.wikipedia.org/wiki/)(Swiss) – Dwarf with giant, snowshoe-like feet
    Barbegazi,
    /// [](https://en.wikipedia.org/wiki/)(Albanian) – Mountain spirit
    Bardha,
    /// [](https://en.wikipedia.org/wiki/)(Trabzon) – Shapechanging death spirit
    Bardi,
    /// Yorkshire black dog
    Barghest,
    /// [](https://en.wikipedia.org/wiki/)(Jewish) – Gigantic bird
    BarJuchne,
    /// [](https://en.wikipedia.org/wiki/)(Medieval folklore) – Geese which hatch from barnacles
    BarnacleGeese,
    /// [](https://en.wikipedia.org/wiki/)(Balinese) – Tutelary spirit
    Barong,
    /// [](https://en.wikipedia.org/wiki/)(Basque) – Ancestral, megalith-building race
    Basajaun,
    /// [](https://en.wikipedia.org/wiki/)(Serbian) – Powerful, evil winged man whose soul is not held by his body and can be subdued only by causing him to suffer dehydration
    BasCelik,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Elephant-swallowing serpent
    Bashe,
    /// [](https://en.wikipedia.org/wiki/)(Chilota) – Chicken-serpent hybrid
    BasiliscoChilote,
    /// [](https://en.wikipedia.org/wiki/)(Italian) – Multi-limbed, venomous lizard
    Basilisk,
    /// [](https://en.wikipedia.org/wiki/)(Philippine) – Primordial god of creation
    Bathala,
    /// [](https://en.wikipedia.org/wiki/)(Philippine) – Female night-demon
    Batibat,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Drought spirit
    Batsu,
    /// [](https://en.wikipedia.org/wiki/)(Lithuanian) – Malevolent spirit
    Baubas,
    /// [](https://en.wikipedia.org/wiki/)(Ojibwa) – Flying skeleton
    Baykok,
    /// [](https://en.wikipedia.org/wiki/)(American Folklore) – Werewolf
    BeastOfBrayRoad,
    /// [](https://en.wikipedia.org/wiki/)(Irish) – Death spirit; a type of Banshee/Bean Sídhe)
    BeanNighe,
    /// [](https://en.wikipedia.org/wiki/)(Jewish) – Massive beast, possibly like a dinosaur
    Behemoth,
    /// [](https://en.wikipedia.org/wiki/)(Welsh) – Giant king
    Bendigeidfran,
    /// [](https://en.wikipedia.org/wiki/)(Egyptian) – Heron-like, regenerative bird, equivalent to (or inspiration for) the Phoenix
    Bennu,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Water spirit
    Berehynia,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Mountain giants who live alongside the Hrimthursar (lit. 'Rime-Giants') in Jotunheim
    Bergrisar,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Mountain spirit
    Bergsra,
    /// [](https://en.wikipedia.org/wiki/)(Brazilian) – Centauroid specter
    BestialBeast,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Invisible spirit which follows people at night, making the sound of footsteps
    BetobetoSan,
    /// [](https://en.wikipedia.org/wiki/)(Buddhist and Hindu) – Ghost of someone killed by execution or suicide
    Bhuta,
    /// [](https://en.wikipedia.org/wiki/)(Khoikhoi) – Female, cannibalistic, partially invisible monster
    BiBlouk,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Demon
    Bies,
    /// [](https://en.wikipedia.org/wiki/)(American Folklore) – Forest-dwelling hominid cryptid.
    Bigfoot,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Spirit of poverty
    Binbogami,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Fish-like humanoid
    BishopFish,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Animated biwa
    BiwaBokuboku,
    /// [](https://en.wikipedia.org/wiki/)(English) – Blue-faced hag
    BlackAnnis,
    /// [](https://en.wikipedia.org/wiki/)(British) – Canine death spirit
    BlackDog,
    /// Norfolk, Essex, and Suffolk black dog
    BlackShuck,
    /// Imaginary creature from the early United States of America
    Blafard,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiary) – Headless humanoid with face in torso
    Blemmyae,
    /// [](https://en.wikipedia.org/wiki/)(Irish) – Water bogeyman
    BloodyBones,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Mischievous gnome
    Bludnik,
    /// [](https://en.wikipedia.org/wiki/)(Brazilian) – Giant amazonian bird
    BlueCrow,
    /// [](https://en.wikipedia.org/wiki/)(English) – Mine-dwelling fairy
    Bluecap,
    /// [](https://en.wikipedia.org/wiki/)(Scottish) – Malevolent spirit
    Bodach,
    /// [](https://en.wikipedia.org/wiki/)(English) – Malevolent spirit
    Bogeyman,
    /// [](https://en.wikipedia.org/wiki/)(English) – Malevolent household spirit
    Boggart,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Nature spirit
    Boginki,
    /// [](https://en.wikipedia.org/wiki/)(Scottish) – Malevolent spirit
    Bogle,
    /// [](https://en.wikipedia.org/wiki/)(Brazilian) – Giant snake
    BoiTata,
    /// [](https://en.wikipedia.org/wiki/)(Albanian) – Dragon
    Bolla,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Bull-horse hybrid with flaming dung
    Bonnacon,
    /// [](https://en.wikipedia.org/wiki/)(American Folklore) – Vampire-like creature that steals energy from sleeping victims
    BooHag,
    /// [](https://en.wikipedia.org/wiki/)(Scottish) – Roaring water bird
    Boobrie,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Death spirit
    Bozaloshtsh,
    /// [](https://en.wikipedia.org/wiki/)(English) – Malevolent water horse
    Brag,
    /// [](https://en.wikipedia.org/wiki/)(English and Scottish) – Benevolent household spirit
    Brownie,
    /// [](https://en.wikipedia.org/wiki/)(Jewish) – Nocturnal bird that drains goats of their milk
    Broxa,
    /// [](https://en.wikipedia.org/wiki/)(Cornish) – Male sea-spirit, a merman, that inhabited mines and coastal communities as a hobgoblin during storms
    Bucca,
    /// [](https://en.wikipedia.org/wiki/)(Dutch) – Ghosts/devils riding flying goats; co-opted by bandits to instil fear during raids
    Bokkenrijders,
    /// [](https://en.wikipedia.org/wiki/)(English) – Bearlike goblin
    Bugbear,
    /// [](https://en.wikipedia.org/wiki/)(Manx) – Ogre-like humanoid
    Buggane,
    /// [](https://en.wikipedia.org/wiki/)(Celtic) – Extremely ugly, but kind, forest spirit
    BugulNoz,
    /// [](https://en.wikipedia.org/wiki/)(Serbia) – Six-legged lake monster
    Bukavac,
    /// [](https://en.wikipedia.org/wiki/)(Australian Aboriginal) – Horse-walrus hybrid lake monster
    Bunyip,
    /// [](https://en.wikipedia.org/wiki/)(American Folklore) West Virginia Urban Legend – Spirit/Maniac that wears a bunny costume and wields an axe
    BunnyMan,
    /// [](https://en.wikipedia.org/wiki/)(Guyanese) – Spirit that seduces and kills men
    BushDaiDai,
    /// [](https://en.wikipedia.org/wiki/)(Bengali) – Fortune-telling birds
    Byangoma,
    /// [](https://en.wikipedia.org/wiki/)(Scandinavian) – Diminutive forest spirit
    Bysen,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Smith and wine spirit
    Cabeiri,
    /// [](https://en.wikipedia.org/wiki/)(Roman) – Fire-breathing giant
    Cacus,
    /// [](https://en.wikipedia.org/wiki/)(Central America) – Cow-sized dog-goat hybrid
    Cadejo,
    /// [](https://en.wikipedia.org/wiki/)(Scottish) – Divine creator and weather deity hag
    Cailleach,
    /// [](https://en.wikipedia.org/wiki/)(Tupi) – Fox-human hybrid and nature spirit
    Caipora,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiary) – White bird that can foretell if a sick person will recover or die
    Caladrius,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiary) – Humanoid with an eight-year lifespan
    Calingi,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiary) – Apes who always bear twins, one the mother loves, the other it hates
    Callitrix,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Giant, chthonic boar
    CalydonianBoar,
    /// [](https://en.wikipedia.org/wiki/)(Heraldic) – Wildcat-deer/antelope-eagle-ox-lion hybrid :>
    Calygreyhound,
    /// [](https://en.wikipedia.org/wiki/)(Chilota) – One-horned calf
    Camahueto,
    /// [](https://en.wikipedia.org/wiki/)(Medieval folklore) – Offspring of a human and an incubus or succubus
    Cambion,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Dragon-human-scorpion hybrid
    Campe,
    /// [](https://en.wikipedia.org/wiki/)(Mayan) – Bird that ate the heads of the first men
    Camulatz,
    /// [](https://en.wikipedia.org/wiki/)(Colombian) – Spectral, fiery hag
    Candileja,
    /// [](https://en.wikipedia.org/wiki/)(Guyanese) – Were-jaguar
    Canaima,
    /// [](https://en.wikipedia.org/wiki/)(Lakota) – Little people and tree spirits
    Canotila,
    /// [](https://en.wikipedia.org/wiki/)(Scottish) – Death spirit (a particular type of Banshee/Bean Sídhe)
    Caoineag,
    /// [](https://en.wikipedia.org/wiki/)(Lakota) – Beaver spirit
    Chapa,
    ///(Manipuri)-Semi-hornbill, semi-human creature
    Chareng,
    /// [](https://en.wikipedia.org/wiki/)(Romanian) – Large, monstrous humanoid
    Capcaun,
    /// [](https://en.wikipedia.org/wiki/)(Latin America) – Small creature with a jewel on its head
    Carbuncle,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiary) – Scaled buffalo-hog hybrid
    Catoblepas,
    /// [](https://en.wikipedia.org/wiki/)(Scottish) – Fairy cat
    CatSidhe,
    /// [](https://en.wikipedia.org/wiki/)(Scottish) — Benevolent Scottish mermaids
    Ceasg,
    /// [](https://en.wikipedia.org/wiki/)(Welsh) – Malevolent water horse
    CeffylDwr,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Human-horse hybrid
    Centaur,
    /// [](https://en.wikipedia.org/wiki/)(Indian) – Horse-Antelope-Lion-Bear hybrid
    Centicore,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Extremely flexible, horned snake
    Cerastes,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Three-headed dog that guards the entrance to the underworld
    Cerberus,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Mischievous forest spirit
    Cercopes,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiary) – Apes who always bear twins, one the mother loves, the other it hates
    Cericopithicus,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Hind with golden antlers and bronze or brass hooves
    CeryneianHind,
    /// [](https://en.wikipedia.org/wiki/)(Lakota) – Hawk spirit
    Cetan,
    /// [](https://en.wikipedia.org/wiki/)(Greek) The Cetus was variously described as a sea monster or sea serpent. Other versions describe Cetus as a monster with the head of a boar or a greyhound and the body of a whale or dolphin, and a divided, fan-like tail. Cetus was said to be a colossal beast the size of a ship, its skull alone measuring 40 feet (12.2 meters) in length, its spines being a cubit in thickness, and its skeleton taller at the shoulder than an elephant.
    Cetus,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Lunar bird
    Chakora,
    /// [](https://en.wikipedia.org/wiki/)(Apocryphal writings) – Angelic birds
    Chalkydri,
    /// [](https://en.wikipedia.org/wiki/)(Persian) – Dog-bird hybrid
    Chamrosh,
    /// [](https://en.wikipedia.org/wiki/)(Aztec) – Little people and nature spirits
    Chaneque,
    /// [](https://en.wikipedia.org/wiki/)(European) – Humanoid child (fairy, elf, troll, etc.) substituted for a kidnapped human child
    Changeling,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Sea monster in the form of a giant mouth
    Charybdis,
    /// [](https://en.wikipedia.org/wiki/)(Mi'kmaq/Algonquian) – Giant, human-eating ice monsters; former humans who either committed terrible crime(s) or were possessed by evil spirits, turning their hearts to ice
    Chenoo,
    /// [](https://en.wikipedia.org/wiki/)(Narragansett) – Ancestral spirit that instructs tribe members
    Chepi,
    /// [](https://en.wikipedia.org/wiki/)(Mapuche) – Volcano-dwelling monster
    Cherufe,
    /// [](https://en.wikipedia.org/wiki/)(French) – Evil horse who runs away with travelers
    ChevalMallet,
    /// [](https://en.wikipedia.org/wiki/)(French) – Evil horse who drowns riders, similar to kelpie
    ChevalGauvin,
    /// [](https://en.wikipedia.org/wiki/)(Abenaki) – Ghost of an improperly buried person
    Chibaiskweda,
    /// Human-faced cow that feeds on good women
    Chichevache,
    /// [](https://en.wikipedia.org/wiki/)(Bahamian) – Bird-mammal hybrid
    Chickcharney,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Lion-goat-snake hybrid
    Chimaera,
    /// [](https://en.wikipedia.org/wiki/)(Navajo) – Vengeful ghost that causes dust devils
    Chindi,
    /// [](https://en.wikipedia.org/wiki/)(Burmese) – Temple-guarding feline, similar to Chinese Shi and Japanese Shisa
    Chinthe,
    /// [](https://en.wikipedia.org/wiki/)(Zulu) – Human-lizard hybrid
    Chitauli,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Animated paper lantern
    Chochinobake,
    /// [](https://en.wikipedia.org/wiki/)(Biblical mythology) – Regenerative bird
    Chol,
    /// [](https://en.wikipedia.org/wiki/)(Korean) – Supernaturally fast horse
    Chollima,
    /// [](https://en.wikipedia.org/wiki/)(Mapuche) – Disembodied, flying head
    Chonchon,
    /// [](https://en.wikipedia.org/wiki/)(Guyanese) – Ghost of a woman that died in childbirth
    Choorile,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiary) – Hairy savage with dog teeth
    Chromandi,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – The giant son of the gorgon Medusa.
    Chrysaor,
    /// [](https://en.wikipedia.org/wiki/)(Greek mythology) – Golden winged ram
    Chrysomallus,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Giant turtle that supports the world
    Chukwa,
    /// [](https://en.wikipedia.org/wiki/)(Latin America) – Cryptid beast named for its habit of sucking the blood of livestock
    Chupacabra,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Vampiric, female ghost
    Churel,
    /// [](https://en.wikipedia.org/wiki/)(Dominican Republic) – Malevolent seductress
    Ciguapa,
    /// [](https://en.wikipedia.org/wiki/)(Aztec) – Ghost of women that died in childbirth
    Cihuateteo,
    /// [](https://en.wikipedia.org/wiki/)(Serbian) – Bird that serves its owner
    Cikavac,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Giant bird that makes its nest out of cinnamon
    CinnamonBird,
    /// [](https://en.wikipedia.org/wiki/)(Aztec) – Sea monster, crocodile-fish hybrid
    Cipactli,
    /// [](https://en.wikipedia.org/wiki/)(Scottish) – Sea serpent
    CireinCroin,
    /// [](https://en.wikipedia.org/wiki/)(Welsh) – Little people and mine spirits
    Coblynau,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Chicken-lizard hybrid
    Cockatrice,
    /// [](https://en.wikipedia.org/wiki/)(English) – Cove god
    Cofgod,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Bronze-hoofed bulls
    ColchisBull,
    /// [](https://en.wikipedia.org/wiki/)(Mapuche) – Rat-bird hybrid that can shapeshift into a serpent
    ColoColo,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Nymph of the Corycian Cave
    CorycianNymphs,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Monstrous bull
    CretanBull,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Fountain nymph
    Crinaeae,
    /// [](https://en.wikipedia.org/wiki/)(Ancient Egypt) – Ram-headed sphinx
    Criosphinx,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Monstrous dog-wolf
    Crocotta,
    /// [](https://en.wikipedia.org/wiki/)(Mexican) – El Pájaro Cu; a bird.
    TheCuBird,
    /// [](https://en.wikipedia.org/wiki/)(Latin America) – Bogeyman
    Cuco,
    /// [](https://en.wikipedia.org/wiki/)(Latin America) – Malevolent spirit
    Cucuy,
    /// [](https://en.wikipedia.org/wiki/)(Cantabrian) – Monstrous, three-armed humanoid
    Cuegle,
    /// [](https://en.wikipedia.org/wiki/)(Asturian and Cantabrian) – Dragon
    Cuelebre,
    /// [](https://en.wikipedia.org/wiki/)(Tupi) – Nature spirit
    Curupira,
    /// [](https://en.wikipedia.org/wiki/)(Scottish) – Gigantic fairy dog
    CuSith,
    /// [](https://en.wikipedia.org/wiki/)(Welsh) – Underworld hunting dog
    CwnAnnwn,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – One-eyed giant
    Cyclops,
    /// [](https://en.wikipedia.org/wiki/)(Welsh) – Death spirit
    Cyhyraeth,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Dog-headed humanoid
    Cynocephalus,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Little people and smith and healing spirits
    Dactyl,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Incorporeal spirit
    Daemon,
    /// [](https://en.wikipedia.org/wiki/)(France, Switzerland and the north of Italy) – Similar to a deer or ibex; legs on one side of its body are shorter than on the other side
    Dahu,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Giant responsible for creating many geographical features in Japan
    Daidarabotchi,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Most powerful class of tengu, each of whom lives on a separate mountain
    Daitengu,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Giant
    Daitya,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Water demon
    Danava,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Laurel tree nymph
    Daphnaie,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Old woman who steals clothes from the souls of the dead
    DatsueBa,
    /// [](https://en.wikipedia.org/wiki/)(Islamic) – Human tribe turned into apes for ignoring Moses' message
    DeadSeaApes,
    /// [](https://en.wikipedia.org/wiki/)(Russia) – A winter spirit who delivers gifts to children on New Year's Eve
    DedMoroz,
    /// [](https://en.wikipedia.org/wiki/)(Native American) – Human-deer hybrid
    DeerWoman,
    /// [](https://en.wikipedia.org/wiki/)(Global) – Preternatural or supernatural possibly immortal being
    Deity,
    /// [](https://en.wikipedia.org/wiki/)(Global) – Half human, half god
    Demigod,
    /// [](https://en.wikipedia.org/wiki/)(Balkans) – Human/vampire hybrid
    Dhampir,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Hanged ghost
    DiaoSiGui,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Earth dragon
    Dilong,
    /// [](https://en.wikipedia.org/wiki/)(Catalan) – Demonic and vampiric dog
    Dip,
    /// [](https://en.wikipedia.org/wiki/)(Roman) – House spirit
    DiPenates,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Extremely venomous snake
    Dipsa,
    /// [](https://en.wikipedia.org/wiki/)(Australian Aboriginal) – Goanna spirit
    Dirawong,
    /// [](https://en.wikipedia.org/wiki/)(Gotland) – Little people and nature spirits
    DiSmaUndarJordi,
    /// [](https://en.wikipedia.org/wiki/)(Philippine) – Tree spirit
    Diwata,
    /// [](https://en.wikipedia.org/wiki/)(Albanian) – Devil
    Djall,
    /// [](https://en.wikipedia.org/wiki/)(Irish) – King otter
    DobharChu,
    /// [](https://en.wikipedia.org/wiki/)(Abenaki) – Little people
    DoGakwHoWad,
    /// [](https://en.wikipedia.org/wiki/)(Korean) – Grotesque, horned humanoids
    Dokkaebi,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Male ancestral spirits; the Dark Elves
    Dokkalfar,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Tutelary and fate spirit
    Dola,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – House spirit
    Domovoi,
    /// [](https://en.wikipedia.org/wiki/)(German) – Ghostly double
    Doppelganger,
    /// [](https://en.wikipedia.org/wiki/)(Catalan) – Lion or bull-faced dragon[]
    /// [](https://en.wikipedia.org/wiki/)(French) – Winged sea serpent
    Drac,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Greek dragons
    Drakon,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Dragons depicted with female characteristics
    Drakaina,
    /// [](https://en.wikipedia.org/wiki/)(Many cultures worldwide) – Fire-breathing and,/// [](https://en.wikipedia.org/wiki/)(normally) winged reptiles
    Dragon,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Giant turtle with dragon-like head
    DragonTurtle,
    /// [](https://en.wikipedia.org/wiki/)(Albanian) – Semi-human winged warriors
    Drangue,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Undead
    Draugr,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Restless ghost of an unbaptised child
    Drekavac,
    /// [](https://en.wikipedia.org/wiki/)(Australian) – Large carnivorous koala that hunts by dropping on its prey from trees
    DropBear,
    /// [](https://en.wikipedia.org/wiki/)(Scottish) – Cavern spirit
    Drow,
    /// [](https://en.wikipedia.org/wiki/)(German) – Possessing demon
    Drude,
    /// [](https://en.wikipedia.org/wiki/)(Bhutanese) – Dragon
    Druk,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Tree nymph
    Dryad,
    /// [](https://en.wikipedia.org/wiki/)(Spanish and Portuguese) – Little people and forest spirits
    Duende,
    /// [](https://en.wikipedia.org/wiki/)(English) – Malevolent little people
    Duergar,
    /// [](https://en.wikipedia.org/wiki/)(Irish) – Headless death spirit
    Dullahan,
    /// [](https://en.wikipedia.org/wiki/)(Philippine) – Little people, some are house spirits, others nature spirits
    Duwende,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Subterranean little people smiths
    Dvergr,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Courtyard spirit
    Dvorovoi,
    /// [](https://en.wikipedia.org/wiki/)(Germanic) – Little people nature spirits
    Dwarf,
    /// [](https://en.wikipedia.org/wiki/)(Jewish) – Spirit,/// [](https://en.wikipedia.org/wiki/)(sometimes the soul of a wicked deceased) that possesses the living
    Dybbuk,
    /// [](https://en.wikipedia.org/wiki/)(Abenaki) – Hideous monster
    DzeeDzeeBonDa,
    /// [](https://en.wikipedia.org/wiki/)(Kwakwaka'wakw) – Child-eating hag
    Dzunukwa,
    /// [Easter Bunny](https://en.wikipedia.org/wiki/Easter_Bunny)(Christianity) – Anthropomorphic lagomorph.
    EasterBunny,
    /// [Easter Bilby](https://en.wikipedia.org/wiki/Easter_Bilby)(Australian) – Anthropomorphic bilby.
    EasterBilby,
    /// [Each-uisge](https://en.wikipedia.org/wiki/Each-uisge)(Scottish) – Malevolent water horse
    EachUisge,
    /// [](https://en.wikipedia.org/wiki/)(Many cultures worldwide) – Leadership or guidance totem
    EagleSpirit,
    /// [](https://en.wikipedia.org/wiki/)(Flores) – Diminutive humanoids, possibly inspired by Homo floresiensis
    EbuGogo,
    /// [](https://en.wikipedia.org/wiki/)(Greek)
    Echidna,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Remora, said to attach to ships to slow them down
    Echeneis,
    /// [](https://en.wikipedia.org/wiki/)(Sumerian) – Ghosts of those not buried properly
    Edimmu,
    /// [](https://en.wikipedia.org/wiki/)(Yoruba) – Humanoid that carries a magical mat
    Egbere,
    /// [](https://en.wikipedia.org/wiki/)(Norse)
    Eikthyrnir,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Spirits of brave warriors
    Einherjar,
    /// [](https://en.wikipedia.org/wiki/)(Philippine) – Flesh-eating, winged humanoids
    Ekek,
    /// [](https://en.wikipedia.org/wiki/)(Ojibwa) – Hags with awls in their elbows
    ElbowWitch,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Fire Giants who reside in Muspelheim, with Surtr as their leader
    Eldjotnar,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Marsh nymph
    Eleionomae,
    /// [](https://en.wikipedia.org/wiki/)(Alchemy) – Personification of one of the Classical elements
    Elemental,
    /// [](https://en.wikipedia.org/wiki/)(Hawaiian) – Monarch flycatcher spirit that guides canoe-builders to the proper trees
    Elepaio,
    /// [](https://en.wikipedia.org/wiki/)(Germanic) – Nature and fertility spirit
    Elf,
    /// [](https://en.wikipedia.org/wiki/)(Central Africa) – Little people and malevolent nature spirits
    Eloko,
    /// [](https://en.wikipedia.org/wiki/)(Yoruba) – Child that can move back and forth between the material world and the afterlife at will
    Emere,
    /// [](https://en.wikipedia.org/wiki/)(Jewish) – Giant
    Emim,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Female demon that waylays travelers and seduces and kills men
    Empusa,
    /// [](https://en.wikipedia.org/wiki/)(Brazilian) – Dolphin-human shapeshifter
    Encantado,
    /// [](https://en.wikipedia.org/wiki/)(Portuguese) – Enchanted princesses
    EnchantedMoor,
    /// [](https://en.wikipedia.org/wiki/)(Heraldic) – Fox-greyhound-lion-wolf-eagle hybrid
    Enfield,
    /// [](https://en.wikipedia.org/wiki/)(Philippine) – Neutral nature spirit
    Engkanto,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Kappa of Shikoku and western Honshū
    Enko,
    /// [](https://en.wikipedia.org/wiki/)(worldwide/fantasy) -Living tree that is said to live for years
    Ent,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Apple tree nymph
    Epimeliad,
    /// [](https://en.wikipedia.org/wiki/)(Sardinia) – Ox-human, wereox
    Erchitu,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Hungry ghost
    ErGui,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Winged spirits of vengeance or justice, also known as Furies
    Erinyes,
    /// [](https://en.wikipedia.org/wiki/)(German) – Death spirit
    Erlking,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Giant boar
    ErymanthianBoar,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Horned, winged horse
    EthiopianPegasus,
    /// [](https://en.wikipedia.org/wiki/)(Finnish mythology) – Spirit being of a living person
    Etiainen,
    /// [](https://en.wikipedia.org/wiki/)(English) – Three-headed giant
    Ettin,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Blue-black, carrion-eater in the underworld
    Eurynomos,
    /// [](https://en.wikipedia.org/wiki/)(Cherokee) – Human-cougar hybrid
    Ewah,
    /// [](https://en.wikipedia.org/wiki/)(Lithuanian) – Lake spirit
    Eerinis,
    /// [](https://en.wikipedia.org/wiki/)(Irish and Scottish) – Monster with half a body
    Fachen,
    /// [](https://en.wikipedia.org/wiki/)(Germanic mythology) – Dwarf who was cursed and turned into a dragon. He was later slain by Sigurd in the Saga of Nibelung.
    Fafnir,
    /// [](https://en.wikipedia.org/wiki/)(many cultures worldwide, esp. Germanic mythology/folklore) – Nature spirits
    Fairy,
    /// [](https://en.wikipedia.org/wiki/)(English) – Animal servant
    Familiar,
    /// [](https://en.wikipedia.org/wiki/)(Irish) – Little people that constantly play pranks
    FarDarrig,
    /// [](https://en.wikipedia.org/wiki/)(French) – Small,/// [](https://en.wikipedia.org/wiki/)(some half-meter tall), wrinkled, and brown-skinned helpful sprites.
    Farfadet,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Three time-controlling sisters
    Fates,
    /// [](https://en.wikipedia.org/wiki/)(Roman) – Human-goat hybrid nature spirit
    Faun,
    /// [](https://en.wikipedia.org/wiki/)(Irish) – Hunger ghost
    FearGorta,
    /// Mesoamerican dragon
    FeatheredSerpent,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Chinese wind god
    FeiLian,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Chinese Phoenix, female in marriage symbol
    Fenghuang,
    /// [](https://en.wikipedia.org/wiki/)(Manx) – House spirit
    Fenodyree,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Gigantic, ravenous wolf
    Fenrir,
    /// [](https://en.wikipedia.org/wiki/)(Irish) – Double or doppelgänger
    Fetch,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Undead
    Fext,
    /// [](https://en.wikipedia.org/wiki/)(Orkney) – Fish-human hybrid that kidnaps humans for servants
    Finfolk,
    /// [](https://en.wikipedia.org/wiki/)(Irish) – Ancestral race
    FirBolg,
    /// [](https://en.wikipedia.org/wiki/)(Many cultures worldwide) – Regenerative solar bird
    FireBird,
    /// [](https://en.wikipedia.org/wiki/)(Germanic) – Dragon
    Firedrake,
    /// [](https://en.wikipedia.org/wiki/)(Cantabrian) – Amphibious, scaled humanoid
    FishMan,
    /// [](https://en.wikipedia.org/wiki/)(American Folklore),/// [](https://en.wikipedia.org/wiki/)(West Virginia) – Alien, humanoid
    FlatwoodsMonster,
    /// [](https://en.wikipedia.org/wiki/)(Irish) – Goat-headed giant
    Fomorian,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Giant horned red cattle
    ForestBull,
    // Norfolk black dog
    Freybug,
    /// [](https://en.wikipedia.org/wiki/)(Celtic) – Malevolent water spirit
    Fuath,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Underworld dragon
    Fucanglong,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Ghosts of people who drowned at sea
    Funayurei,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Animated jar
    FuruUtsubo,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Woman with a second mouth on the back of her head
    FutakuchiOnna,
    /// [](https://en.wikipedia.org/wiki/)(Scandinavian) – Animal familiar
    Fylgja,
    /// [](https://en.wikipedia.org/wiki/)(Seneca) – Dragon
    Gaasyendietha,
    /// [](https://en.wikipedia.org/wiki/)(Russian) – Iron-beaked bird with copper talons
    Gagana,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Ghosts of especially greedy people
    Gaki,
    /// [](https://en.wikipedia.org/wiki/)(Mesopotamian) – Underworld demons
    Gallu,
    /// [](https://en.wikipedia.org/wiki/)(Basque) – Small demonic servants
    Galtzagorriak,
    /// [](https://en.wikipedia.org/wiki/)(Russian) – Prophetic human-headed bird
    Gamayun,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Attendants of Shiva
    Gana,
    /// [](https://en.wikipedia.org/wiki/)(Irish) – Male fairy that seduces human women
    Gancanagh,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Double-headed bird
    Gandabherunda,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Male nature spirits, often depicted as part human, part animal
    Gandharva,
    /// [](https://en.wikipedia.org/wiki/)(French) – Water dragon
    Gargouille,
    /// [](https://en.wikipedia.org/wiki/)(Australian Aboriginal) – A flying humanoid who envelops his victims
    Garkain,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Giant, ravenous hound
    Garmr,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Human-eagle hybrid
    Garuda,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Giant malevolent skeletons
    Gashadokuro,
    /// [](https://en.wikipedia.org/wiki/)(Basque) – Wolf capable of walking upright
    Gaueko,
    /// [](https://en.wikipedia.org/wiki/)(Egyptian) – God of the Earth, married to Nut
    Geb,
    /// [](https://en.wikipedia.org/wiki/)(Heraldic) – The fish pike
    Ged,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Six-armed giant
    Gegenees,
    /// [](https://en.wikipedia.org/wiki/)(Roman) – Spirit that protects a specific place
    GeniusLoci,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Male spirit associated with bringing rain and hail
    German,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Three-headed six-armed giant with three torsos and (in some sources) six legs
    Geryon,
    /// [](https://en.wikipedia.org/wiki/)(Scottish) – Tree guardian
    GhillieDhu,
    /// Disembodied spirits of those that have died
    Ghost,
    /// [](https://en.wikipedia.org/wiki/)(Arabian) – Cannibalistic shapeshifting desert genie often classified as undead.
    Ghoul,
    /// [](https://en.wikipedia.org/wiki/)(Worldwide) – Immensely large and strong humanoids
    Giant,
    /// [](https://en.wikipedia.org/wiki/)(Worldwide) – Unusually large beasts
    GiantAnimal,
    /// [](https://en.wikipedia.org/wiki/)(Ojibwa) – Bison-snake-bird-cougar hybrid water spirit
    GichiAnamiEBizhiw,
    /// [](https://en.wikipedia.org/wiki/)(Sumerian) – Ghost
    Gidim,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Race of giants that fought the Olympian gods, sometimes depicted with snake-legs
    Gigantes,
    /// [](https://en.wikipedia.org/wiki/)(Scottish) – Smallest animal
    Gigelorum,
    /// [](https://en.wikipedia.org/wiki/)(Akkadian) – Human-scorpion hybrid
    Girtablilu,
    /// [](https://en.wikipedia.org/wiki/)(Scandinavian) – Corporeal ghost
    Gjenganger,
    /// [](https://en.wikipedia.org/wiki/)(Scottish) – Human-goat hybrid
    Glaistig,
    /// [](https://en.wikipedia.org/wiki/)(Manx) – Malevolent water horse
    Glashtyn,
    /// [](https://en.wikipedia.org/wiki/)(Alchemy) – Diminutive Earth elemental
    Gnome,
    /// [](https://en.wikipedia.org/wiki/)(Medieval) – Grotesque, mischievous little people
    Goblin,
    /// [](https://en.wikipedia.org/wiki/)(English) – Giant protector of London
    Gog,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Dog-sized ant that digs for gold in sandy areas
    GoldDiggingAnt,
    /// [](https://en.wikipedia.org/wiki/)(Jewish) – Animated construct
    Golem,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiary) – Hairy humanoid
    Gorgades,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Fanged, snake-haired humanoids that turn anyone who sees them into stone
    Gorgon,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Vengeful ghosts, usually of martyrs
    Goryo,
    /// [](https://en.wikipedia.org/wiki/)(Ohio, USA) – Ape-like cryptid
    Grassman,
    /// [](https://en.wikipedia.org/wiki/)(Folklore) – Creatures that sabotage airplanes
    Gremlin,
    /// [](https://en.wikipedia.org/wiki/)(Heraldic) – Lion-eagle hybrid
    Griffin,
    /// [](https://en.wikipedia.org/wiki/)(Christian, Jewish, and Islamic mythology) – Fallen angels, father of Nephilim
    Grigori,
    /// [](https://en.wikipedia.org/wiki/)(English and Scandinavian) – Tutelary spirits of churches
    Grim,
    /// [](https://en.wikipedia.org/wiki/)(Worldwide) – Death angel often thought to be God's/Satan's assistant
    GrimReaper,
    /// [](https://en.wikipedia.org/wiki/)(English) – Malevolent water spirit
    Grindylow,
    /// [](https://en.wikipedia.org/wiki/)(Mapuche) – Malevolent spirit
    Gualichu,
    /// [](https://en.wikipedia.org/wiki/)(Christian, Jewish, and Islamic belief) – Subclassification of angels that guard and protect a specific person or living being
    GuardianAngel,
    /// [](https://en.wikipedia.org/wiki/)(Akkadian) – Human-bull hybrid
    GudElim,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Anthropomorphic bird
    Guhin,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Ghost that manifests as an old woman
    GuiPo,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Ghostly tree that confuses travelers by moving
    GuiShu,
    /// [](https://en.wikipedia.org/wiki/)(Germanic) – Gluttonous dog-cat-fox hybrid
    Gulon,
    /// [](https://en.wikipedia.org/wiki/)(Korean mythology) – Demonic fox with thousands of tails believed to possess an army of spirits and magic in its tails
    Gumiho,
    /// [](https://en.wikipedia.org/wiki/)(Australian Aboriginal) - An enormous reptile-fish whose movements carved out the landscape south of the Blue Mountains
    Gurangatch,
    /// [](https://en.wikipedia.org/wiki/)(Nepalese) – Child-eating demon
    Gurumapa,
    /// [](https://en.wikipedia.org/wiki/)(Welsh) – Black dog
    Gwyllgi,
    /// [](https://en.wikipedia.org/wiki/)(Welsh) – Malevolent spirit
    Gwyllion,
    /// [](https://en.wikipedia.org/wiki/)(American folklore) – Four-legged herbivore
    Gyascutus,
    /// [](https://en.wikipedia.org/wiki/)(Lincolnshire and Yorkshire) – Black dog
    Gytrash,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Bull-headed monster
    Gyuki,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – listed as the 'best' hawk
    Habrok,
    /// [](https://en.wikipedia.org/wiki/)(Persian) – gigantic land animal
    Hadhayosh,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Ruler of the Underworld
    Hades,
    /// [](https://en.wikipedia.org/wiki/)(Korean) – dog-lion hybrid
    Haetae,
    /// [](https://en.wikipedia.org/wiki/)(Many cultures worldwide) – wise old woman who is usually a malevolent spirit or a disguised goddess
    Hag,
    /// [](https://en.wikipedia.org/wiki/)(Nuu-chah-nulth) – water serpent
    Haietlik,
    /// [](https://en.wikipedia.org/wiki/)(Khoikhoi) – male cannibalistic partially invisible monster
    HaiUri,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – talking beast which handed down knowledge on harmful spirits
    Hakutaku,
    /// [](https://en.wikipedia.org/wiki/)(Māori) – nature guardian
    Hakuturi,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – human-elf hybrid
    HalfElf,
    /// [](https://en.wikipedia.org/wiki/)(Finnish) – spirit that protects a specific place
    Haltija,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – oak tree nymph
    Hamadryad,
    /// [](https://en.wikipedia.org/wiki/)(Scandinavian) – personal protection spirit
    Hamingja,
    /// [](https://en.wikipedia.org/wiki/)(Buddhist, Hindu and Jainism) – mystic bird
    Hamsa,
    /// [](https://en.wikipedia.org/wiki/)(Rapa Nui) – long-eared humanoid
    HanauEpe,
    /// [](https://en.wikipedia.org/wiki/)(Malay) – shapeshifting water spirit
    HantuAir,
    /// [](https://en.wikipedia.org/wiki/)(Philippine) – demon
    HantuDemon,
    /// [](https://en.wikipedia.org/wiki/)(Malay) – demonic servant
    HantuRaya,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – humanoid female with barbed, prehensile hair
    Harionago,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – birdlike human-headed death spirit
    Harpy,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – undead being who cannot leave its burial mound
    Haugbui,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – saltwater spirit
    Havsrå,
    /// [](https://en.wikipedia.org/wiki/)(Manipuri mythology) – celestial maidens, daughters of the Sky God Soraren
    Helloi,
    /// [](https://en.wikipedia.org/wiki/)(European) – humanoid spirit who haunts or kills
    HeadlessHorseman,
    /// [](https://en.wikipedia.org/wiki/)(Brazilian) – fire-spewing, headless, spectral mule
    HeadlessMule,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – primordial giants with 100 hands and fifty heads
    Hecatonchires,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – crabs with human-faced shells, the spirits of warriors killed in the Battle of Dan-no-ura
    Heikegani,
    /// [](https://en.wikipedia.org/wiki/)(German) – household spirit
    Heinzelmannchen,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – fen nymph
    Helead,
    /// [](https://en.wikipedia.org/wiki/)(Many cultures worldwide) – underworld dog
    Hellhound,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – gatekeeper of Olympus
    Heracles,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – glowing bird
    Hercinia,
    /// [](https://en.wikipedia.org/wiki/)(Basque) – dragon
    Herensuge,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – nymph daughters of Atlas
    Hesperides,
    /// [](https://en.wikipedia.org/wiki/)(United States) – nocturnal forest creature
    Hidebehind,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – drought spirit
    Hiderigami,
    /// [](https://en.wikipedia.org/wiki/)(Ancient Egypt) – falcon-headed sphinx
    Hieracosphinx,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – baboon monster
    Hihi,
    /// [](https://en.wikipedia.org/wiki/)(Finnish) – nature guardian
    Hiisi,
    /// [](https://en.wikipedia.org/wiki/)(Greek)
    Hippalectryon,
    /// [](https://en.wikipedia.org/wiki/)(Etruscan, Greek and Phoenician) – horse-fish hybrid
    Hippocamp,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – hybrid of a griffin and horse; a lion-eagle-horse hybrid
    Hippogriff,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiary) – horse-hoofed humanoid
    Hippopodes,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiary) – deer-goat hybrid
    Hircocervus,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – ghosts of the newly dead, which take the form of fireballs
    Hitodama,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – one-eyed childlike spirit
    HitotsumeKozo,
    /// [](https://en.wikipedia.org/wiki/)(English) – house spirit
    Hob,
    /// [](https://en.wikipedia.org/wiki/)(English) – malevolent spirit
    Hobbididance,
    /// [](https://en.wikipedia.org/wiki/)(Medieval) – friendly or amusing goblin
    Hobgoblin,
    /// [](https://en.wikipedia.org/wiki/)(Native American) – frog-mammoth-lizard hybrid
    Hodag,
    /// [](https://en.wikipedia.org/wiki/)(Kwakiutl) – bird
    Hokhokw,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – dog-like Chinese tree spirit
    Hoko,
    /// [](https://en.wikipedia.org/wiki/)(Persian) – eagle-lion hybrid, similar to a griffin
    Homa,
    /// [](https://en.wikipedia.org/wiki/)(Colombian) – human-alligator hybrid
    HombreCaiman,
    /// [](https://en.wikipedia.org/wiki/)(Latin America) – human-cat hybrid
    HombreGato,
    /// [](https://en.wikipedia.org/wiki/)(Alchemy) – small animated construct
    Homunculus,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – rooster-swallow-fowl-snake-goose-tortoise-stag-fish hybrid
    Hoo,
    /// near passerine bird common to Africa and Eurasia that features in many mythologies in those continents
    Hoopoe,
    /// snake which rolls by taking its tail in its mouth
    HoopSnake,
    /// [](https://en.wikipedia.org/wiki/)(Native American) – serpentine rain spirit
    HornedSerpent,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – deceased person
    Hotoke,
    /// [](https://en.wikipedia.org/wiki/)(Islamic) – heavenly beings
    Houri,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – giant, who in eagle form, creates the wind by beating his wings
    Hraesvelg,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – frost giants who are the main inhabitants of either Jotunheim or Niflheim
    Hrímþursar,
    /// [](https://en.wikipedia.org/wiki/)(Mayan) – human-deer hybrid
    Huaychivo,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – pair of ravens associated with the Norse god Odin whose names mean Thought and Memory.
    HuginnAndMuninn,
    /// [](https://en.wikipedia.org/wiki/)(Icelandic/Faroese) – secret mound/rock dwelling elves
    Huldufolk,
    /// [](https://en.wikipedia.org/wiki/)(Scandinavian) – forest spirit
    Hulder,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – nine-tailed fox spirit
    HuliJing,
    /// [](https://en.wikipedia.org/wiki/)(Persian) – regenerative fire bird
    Huma,
    /// [](https://en.wikipedia.org/wiki/)(Akkadian) – lion-faced giant
    Humbaba,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – chaos spirit
    Hundun,
    /// [](https://en.wikipedia.org/wiki/)(Taíno) – nocturnal ghost
    Hupia,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – hundred-eyes creature
    Hyakume,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – multi-headed water serpent/dragon
    Hydra,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiary) – snake whose poison causes the victim to swell up
    Hydros,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiary) – snake from the Nile River that would kill crocodiles from the inside
    Hydrus,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – hair-covered kappa
    Hyosube,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiary) – snake that kills its victims in their sleep
    Hypnalis,
    /// [](https://en.wikipedia.org/wiki/)(mythology) – Hoopoe
    Hudhud,
    /// [](https://en.wikipedia.org/wiki/)(Inuit) – Little people
    Ishigaq,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Savage human-goat hybrid from a remote island chain
    IslandSatyr,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Shark-like sea monster
    Isonade,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Ghostly aerial phenomenon that attacks people
    IttanMomen,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Char which appeared as a Buddhist monk
    IwanaBozu,
    /// [](https://en.wikipedia.org/wiki/)(American) – Rabbit with antlers
    Jackalope,
    /// [](https://en.wikipedia.org/wiki/)(English) – Malevolent giant
    JackInIrons,
    /// [](https://en.wikipedia.org/wiki/)(Medieval folklore) – Vegetal lantern
    JackOLantern,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Winged serpent or small dragon
    Jaculus,
    /// [](https://en.wikipedia.org/wiki/)(Medieval folklore) – Island-sized fish
    Jasconius,
    /// [](https://en.wikipedia.org/wiki/)(Guaraní) – Nature guardian and bogeyman
    JasyJaterei,
    /// [](https://en.wikipedia.org/wiki/)(Hindu mythology) – Vulture demigod
    Jatayu,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Vampirised premature baby
    Jaud,
    /// [](https://en.wikipedia.org/wiki/)(Java) – Vampiric little people
    Jenglot,
    /// [](https://en.wikipedia.org/wiki/)(Sawa) – Water spirit
    Jengu,
    /// [](https://en.wikipedia.org/wiki/)(Basque) – Megalith-building giant
    Jentil,
    /// [](https://en.wikipedia.org/wiki/)(Mi'kmaq) – Anthropophagous giant
    Jenu,
    /// [](https://en.wikipedia.org/wiki/)(Swedish) – Gluttonous dog-cat-fox hybrid
    Jerff,
    /// [](https://en.wikipedia.org/wiki/)(American) – Demonic dragon or flying demon who was given birth to by an American living in New Jersey
    JerseyDevil,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – One-eyed, one-winged bird who requires a mate for survival
    Jian,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Life-draining, reanimated corpse
    Jiangshi,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Dragon
    Jiaolong,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Spirit that protects a specific place
    Jibakurei,
    /// [](https://en.wikipedia.org/wiki/)(Lithuanian) – House spirit
    Jievaras,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Corpse-eating ghost
    Jikininki,
    /// [](https://en.wikipedia.org/wiki/)(Arabian, Islamic) – Spiritual creatures; genii
    Jinn,
    /// [](https://en.wikipedia.org/wiki/)(Mi'kmaq) – Underwater horned snake; lives in lakes and eats humans
    JipijkaM,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Nine-headed bird worshiped by ancient natives in Hubei Province.
    Jiufeng,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Nine-headed, demonic bird
    JiuTouNiao,
    /// [](https://en.wikipedia.org/wiki/)(Iroquois) – Little people nature spirit
    Jogah,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Sea serpent
    Jormungandr,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Spider woman
    Jorogumo,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Animated folding screen cloth
    Jotai,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Gigantic nature spirits
    Jotunn,
    /// [](https://en.wikipedia.org/wiki/)(Korean) – Bird
    Jujak,
    /// [](https://en.wikipedia.org/wiki/)(Guyanese) – Malevolent spirit
    Jumbee,
    /// [](https://en.wikipedia.org/wiki/)(Dutch) – Little people that live underground, in mushrooms, or as house spirits
    Kabouter,
    /// [](https://en.wikipedia.org/wiki/)(Hopi and Puebloan) – Nature spirit
    Kachina,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Little people and water spirits
    Kahaku,
    /// [](https://en.wikipedia.org/wiki/)(Scandinavian) – Wind spirit
    Kajsa,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Descendants of Kala
    Kalakeyas,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Grotesque, malevolent spirit
    Kallikantzaroi,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Wind spirit
    Kamaitachi,
    /// [](https://en.wikipedia.org/wiki/)(Philippine) – Philippine counterpart of Death
    Kamatayan,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Nature spirit
    Kami,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Hair-cutting spirit
    Kamikiri,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Bathroom spirit
    KanbariNyudo,
    /// [](https://en.wikipedia.org/wiki/)(Manipuri mythology) – Great Dragon in the Kangla Palace
    KanglaSha,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Drought spirit
    Kanbo,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Money spirit
    Kanedama,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Little people and water spirit
    Kappa,
    /// [](https://en.wikipedia.org/wiki/)(Philippine) – Malevolent tree spirit
    Kapre,
    /// [](https://en.wikipedia.org/wiki/)(Bulgarian and Turkish), also in Bosnia and Herzegovina and Serbia known as Karanđoloz – Troublesome spirit
    Karakoncolos,
    /// [](https://en.wikipedia.org/wiki/)(Turkish) – Male night-demon
    Karakura,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Tengu with a bird's bill
    KarasuTengu,
    /// [](https://en.wikipedia.org/wiki/)(Persian) – One-horned giant animal
    Karkadann,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Giant crab
    Karkinos,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Eagle-human hybrid
    Karura,
    /// [](https://en.wikipedia.org/wiki/)(Polish) – Little people and mine spirits
    Karzelek,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Animated parasol
    KasaObake,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Cat-like demon which descends from the sky and carries away corpses
    Kasha,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Kappa who climb into the mountains for the winter
    Kashanbo,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Woman riding on a flaming wheel
    KatawaGuruma,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Handsome man from the moon
    KatsuraOtoko,
    /// [](https://en.wikipedia.org/wiki/)(Albanian) – Man-eating giant
    Katallan,
    /// [](https://en.wikipedia.org/wiki/)(Lithuanian) – Nature spirit
    Kaukas,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Supernatural river otter
    KawaUso,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Smelly, cowardly water spirit
    KawaZaru,
    /// [](https://en.wikipedia.org/wiki/)(Chukchi mythology) – Ogre or evil spirit
    KeLets,
    /// [](https://en.wikipedia.org/wiki/)(Inuit) – Hairless dog
    Keelut,
    /// [](https://en.wikipedia.org/wiki/)(Abenaki) – Half-human half-animal cannibalistic giant
    KeeWakw,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Amorphous afterbirth spirit
    Kekkai,
    /// [](https://en.wikipedia.org/wiki/)(Irish and Scottish) – Malevolent water horse
    Kelpie,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Female death spirit
    Ker,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Mysterious, white, fluffy creature
    KesaranPasaran,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Disease spirit
    Keukegen,
    /// [](https://en.wikipedia.org/wiki/)(Heraldic) – Wingless griffin
    Keythong,
    /// [](https://en.wikipedia.org/wiki/)(Nepalese) – Fat, hairy ape-like creature
    Khyah,
    /// [](https://en.wikipedia.org/wiki/)(Inuit) – Night-demon
    Kigatilik,
    /// [](https://en.wikipedia.org/wiki/)(Sotho) – Gluttonous monster that was one of the first beasts of creation
    Kholomodumo,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Tree sprite from Okinawa
    Kijimunaa,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – She-devil
    Kijo,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Female house spirit
    Kikimora,
    /// [](https://en.wikipedia.org/wiki/)(English and Scottish) – Ugly, mischievous mill spirit
    Killmoulis,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Human-bird hybrid
    Kinnara,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Bird
    KinU,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Japanese Unicorn
    Kirin,
    /// [](https://en.wikipedia.org/wiki/)(Angola) – Malevolent, two-faced seducer
    Kishi,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Fox spirit
    Kitsune,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Person possessed by a fox spirit
    KitsuneTsuki,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Woman who transformed into a serpentine demon out of the rage of unrequited love
    Kiyohime,
    /// [](https://en.wikipedia.org/wiki/)(German) – Ship spirit
    Klabautermann,
    /// [](https://en.wikipedia.org/wiki/)(folklore),/// [](https://en.wikipedia.org/wiki/)(Cornish and Welsh) – Little people and mine spirits
    Knocker,
    /// [](https://en.wikipedia.org/wiki/)(English) – Water dragon
    Knucker,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Goblin like thieves and tricksters
    Kobalos,
    /// [](https://en.wikipedia.org/wiki/)(German) – Little people and mine or house spirits
    Kobold,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Tree spirit
    Kodama,
    /// [](https://en.wikipedia.org/wiki/)(Germanic) – House spirit
    Kofewalt,
    /// [](https://en.wikipedia.org/wiki/)(Abenaki) – Hideous monster
    KoGok,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Ubume bird
    Kokakucho,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Protective animal
    Komainu,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Infant that cries until it is picked up, then increases its weight and crushes its victim
    KonakiJiji,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Bird-like creature
    KonohaTengu,
    /// [](https://en.wikipedia.org/wiki/)(Ainu) – Little people
    KoroPokGuru,
    /// [](https://en.wikipedia.org/wiki/)(Breton) – Little people and nature spirits
    Korrigan,
    /// [](https://en.wikipedia.org/wiki/)(Scandinavian) – Sea monster
    Kraken,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Little people nature spirits
    Krasnoludek,
    /// [](https://en.wikipedia.org/wiki/)(Southeast Asian) – Vampiric, floating head
    Krasue,
    /// [](https://en.wikipedia.org/wiki/)(Germany) – Christmas Devil who punishes badly-behaved children
    Krampus,
    /// [](https://en.wikipedia.org/wiki/)(Guaraní) – Forest spirit
    KuarahyJara,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Female corpse-chewing graveyard spirit
    Kubikajiri,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Vengeful ghost of a woman mutilated by her husband
    KuchisakeOnna,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Miniature fox spirit
    KudaGitsune,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Human-faced calf which predicts a calamity before dying
    Kudan,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – One-legged monster
    Kui,
    /// [](https://en.wikipedia.org/wiki/)(Albanian) – Female demon who spreads sickness
    Kukudhi,
    /// [](https://en.wikipedia.org/wiki/)(Mi'kmaq) – Large, hairy, greedy, human-eating bipedal monsters whose scream can kill
    Kukwes,
    /// [](https://en.wikipedia.org/wiki/)(Albanian) – Drought-causing dragon
    Kulshedra,
    /// [](https://en.wikipedia.org/wiki/)(Philippine) – Death spirits
    Kumakatok,
    /// [](https://en.wikipedia.org/wiki/)(Korean) – Fox spirit
    Kumiho,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Giant fish
    Kun,
    /// [](https://en.wikipedia.org/wiki/)(Hawaiian) – Shapeshifting tricksters
    Kupua,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Guardian spirit of a warehouse
    Kurabokko,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Jellyfish which floats through the air as a fireball
    KurageNoHinotama,
    /// [](https://en.wikipedia.org/wiki/)(Hindu mythology) – Second avatar of Vishnu in the form of a Turtle
    Kurma,
    /// [](https://en.wikipedia.org/wiki/)(Guaraní) – Wild man and fertility spirit
    Kurupi,
    /// [](https://en.wikipedia.org/wiki/)(Tlingit) – Shapeshifting 'land otter man'
    Kushtaka,
    /// [](https://en.wikipedia.org/wiki/)(Korean) – Chicken-lizard hybrid
    KyeRyong,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Animated scroll or paper
    Kyourinrin,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Nine-tailed fox
    KyubiNoKitsune,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Vampire
    Kyuketsuki,
    /// [](https://en.wikipedia.org/wiki/)(Assyrian) – Disease demon
    LaBarTu,
    /// [](https://en.wikipedia.org/wiki/)(Akkadian) – Sea snake
    LabbMu,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Sunstroke spirit
    Ladyidday,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Dragon guarding the golden apples of the Hesperides
    Ladon,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Enchanted dog that always caught his prey
    Laelaps,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Anthropophagic giants
    Laestrygonians,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Field spirit
    Lakanica,
    /// [](https://en.wikipedia.org/wiki/)(Worldwide) – Gigantic animals reported to inhabit various lakes around the world
    LakeMonster,
    /// [](https://en.wikipedia.org/wiki/)(Nepalese) – Demon with fangs
    Lakhey,
    /// [](https://en.wikipedia.org/wiki/)(Latin America) – Death spirit associated with drowning
    LaLlorona,
    /// [](https://en.wikipedia.org/wiki/)(Akkadian and Sumerian) – Protective spirit with the form of a winged bull or human-headed lion
    Lamassu,
    /// [](https://en.wikipedia.org/wiki/)(English) – Giant worm
    LambtonWorm,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Child-devouring monster
    Lamia,
    /// [](https://en.wikipedia.org/wiki/)(Basque) – Water spirit with duck-like feet
    Lamiak,
    /// [](https://en.wikipedia.org/wiki/)(Colombian) – Shapeshifting, female water spirit
    LaMojana,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Underworld nymph
    Lampades,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Nature spirits
    Landvaettir,
    /// [](https://en.wikipedia.org/wiki/)(Manipuri mythology) – Semi human, semi hornbill creature
    Langmeidong,
    /// [](https://en.wikipedia.org/wiki/)(Roman) – House spirit
    Lares,
    /// [](https://en.wikipedia.org/wiki/)(Venezuela) – Female ghost that punishes unfaithful husbands
    LaSayona,
    /// [](https://en.wikipedia.org/wiki/)(Colombian) – Nature spirit that seduces and kills men
    LaTunda,
    /// Miniature bear thought to inhabit the lava beds of south central Oregon
    LavaBear,
    /// [](https://en.wikipedia.org/wiki/)(Lithuanian) – Field spirit
    LaukuDvasios,
    /// [](https://en.wikipedia.org/wiki/)(Baltic) – Sky spirit
    Lauma,
    /// [](https://en.wikipedia.org/wiki/)(Scottish) – Gigantic water rat
    Lavellan,
    /// [](https://en.wikipedia.org/wiki/)(Celtic) – Fairy lover
    LeananSidhe,
    /// [](https://en.wikipedia.org/wiki/)(Irish) – Possessing spirit or vampire
    Leanashe,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Meadow nymph
    Leimakids,
    /// [](https://en.wikipedia.org/wiki/)(Etruscan) – Fish-tailed lion
    Leokampoi,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiary) – Tiny animal poisonous to lions
    Leontophone,
    /// [](https://en.wikipedia.org/wiki/)(Irish) – Cobbler spirit
    Leprechaun,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Tree spirit
    Leszi,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – White poplar tree nymph
    Leuce,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiary) – Crocotta-lion hybrid
    Leucrota,
    /// [](https://en.wikipedia.org/wiki/)(Jewish) – Sea monster seen in Job 41
    Leviathan,
    /// [](https://en.wikipedia.org/wiki/)(Balinese) – Anthropophagous flying head with entrails
    Leyak,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Human-horse hybrid
    LibyanAegipanes,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Human-goat hybrid
    LibyanSatyr,
    /// [](https://en.wikipedia.org/wiki/)(Hungary) – Magical chicken that transforms into a humanoid
    Liderc,
    /// [](https://en.wikipedia.org/wiki/)(Southern Africa) – Magical bird found at sites of lightning strikes
    LightningBird,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – One-eyed hag or goblin
    Likho,
    /// [](https://en.wikipedia.org/wiki/)(Jewish) – Night-demoness
    Lilin,
    /// [](https://en.wikipedia.org/wiki/)(Assyrian) – Winged demon
    Lilitu,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Lake nymph
    Limnades,
    /// [](https://en.wikipedia.org/wiki/)(Germanic) – Dragon
    Lindworm,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Sunlight spirits; the Light Elves
    Ljosalfar,
    /// [](https://en.wikipedia.org/wiki/)(Albanian)- Demoness
    Ljubi,
    /// [](https://en.wikipedia.org/wiki/)(Welsh) – Frog-bat-lizard hybrid
    LlamhigynYDwr,
    /// [](https://en.wikipedia.org/wiki/)(Scottish) – Serpentine sea monster
    LochNessMonster,
    /// [](https://en.wikipedia.org/wiki/)(Norse mythology) – God of night
    Loki,
    /// [](https://en.wikipedia.org/wiki/)(Abenaki) – Hideous monster
    LoLol,
    /// Chinese dragon
    Long,
    /// [](https://en.wikipedia.org/wiki/)(Italian) – Female human-goat hybrid and water spirit
    Longana,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Dragon-horse hybrid
    LongMa,
    /// [](https://en.wikipedia.org/wiki/)(French America) – Shapeshifting, female vampire
    Loogaroo,
    /// [](https://en.wikipedia.org/wiki/)(French) – Snake-mollusk hybrid
    LouCarcolh,
    /// [](https://en.wikipedia.org/wiki/)(French) – Werewolf
    LoupGarou,
    /// [](https://en.wikipedia.org/wiki/)(American Folklore),/// [](https://en.wikipedia.org/wiki/)(Ohio) – Cryptid, Humanoid Frog
    LovelandFrog,
    /// [](https://en.wikipedia.org/wiki/)(English) – House spirit
    LubberFiend,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Truth-detecting animal
    Luduan,
    /// [](https://en.wikipedia.org/wiki/)(Albanian) – Vampire
    Lugat,
    /// [](https://en.wikipedia.org/wiki/)(Guaraní) – Werewolf | Cadaver-eating dog
    Luison,
    /// Sea Monster
    Lusca,
    /// [](https://en.wikipedia.org/wiki/)(French) – Amusing goblin
    Lutin,
    /// [](https://en.wikipedia.org/wiki/)(Icelandic) Whale-like sea monster
    Lyngbakr,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Feline guide spirit
    Lynx,
    /// [](https://en.wikipedia.org/wiki/)(Estonian mythology) – Subterranean spirit
    MaaAlused,
    /// [](https://en.wikipedia.org/wiki/)(Medieval bestiaries) – Hermaphroditic humanoid
    Machlyes,
    /// [](https://en.wikipedia.org/wiki/)(Medieval bestiaries) – Giant-headed humanoid
    Macrocephali,
    /// [](https://en.wikipedia.org/wiki/)(West African Mythology ) – Female ghost
    MadamKoiKoi,
    /// [](https://en.wikipedia.org/wiki/)(Colombian folklore) – Nature guardian
    Madremonte,
    /// [](https://en.wikipedia.org/wiki/)(Māori) – Savage, arboreal humanoids
    Maero,
    /// [](https://en.wikipedia.org/wiki/)(English folklore) – Giant protector of London
    Magog,
    /// [](https://en.wikipedia.org/wiki/)(Hindu mythology) – Giant elephant that holds up the world
    MahaPudma,
    /// [](https://en.wikipedia.org/wiki/)(Basque mythology) – Megalith-building giant
    Mairu,
    /// [](https://en.wikipedia.org/wiki/)(Latvian mythology) – Benevolent house spirit
    MajasGari,
    // in Swahili mythology, shape-shifting spirits that can pass as humans
    Majitu,
    /// [](https://en.wikipedia.org/wiki/)(Indian mythology) – Aquatic beings
    Makara,
    /// [](https://en.wikipedia.org/wiki/)(Japanese mythology) – Pillow-moving spirit
    MakuraGaeshi,
    /// [](https://en.wikipedia.org/wiki/)(Welsh mythology) – Spirit of the hunt
    MalltYNos,
    /// [](https://en.wikipedia.org/wiki/)(Africa and the African diaspora) – Supernaturally beautiful water spirits
    MamiWata,
    /// [](https://en.wikipedia.org/wiki/)(Philippine mythology) – Vampires that sever their torsos from their legs to fly around
    Manananggal,
    /// [](https://en.wikipedia.org/wiki/)(Medieval bestiaries) – Humanoid with a forty-year lifespan
    Mandi,
    /// [](https://en.wikipedia.org/wiki/)(Medieval folklore) – Diminutive, animated construct
    Mandrake,
    /// [](https://en.wikipedia.org/wiki/)(Roman mythology) – Ancestral spirits
    Manes,
    /// [](https://en.wikipedia.org/wiki/)(Cree) – Little people with six fingers and no noses
    Mannegishi,
    /// [](https://en.wikipedia.org/wiki/)(Persian mythology) – Lion-human-scorpion hybrid
    Manticore,
    /// [](https://en.wikipedia.org/wiki/)(Brazilian mythology) – Giant sloth
    Mapinguari,
    /// [](https://en.wikipedia.org/wiki/)(Scandinavian folklore) – Female night-demon
    Mara,
    /// [](https://en.wikipedia.org/wiki/)(Italian folklore) – Malevolent water spirit
    Marabbecca,
    /// [](https://en.wikipedia.org/wiki/)(Tuamotu) – Attendant of Kiho-tumu, the supreme god
    Mareikura,
    /// [](https://en.wikipedia.org/wiki/)(Greek mythology) – Man-eating horses
    MaresOfDiomedes,
    /// [](https://en.wikipedia.org/wiki/)(Arabian mythology) – Jinn associated fortune tellers
    Marid,
    /// [](https://en.wikipedia.org/wiki/)(Norse mythology) – Mermen with prophetic abilities
    Marmennill,
    /// [](https://en.wikipedia.org/wiki/)(Lithuanian mythology) – Disease spirits
    MaroDeives,
    /// [](https://en.wikipedia.org/wiki/)(Abenaki mythology) – Shapeshifting toad spirit
    MaskiMonGweZoOs,
    /// [](https://en.wikipedia.org/wiki/)(French mythology) – Spirit that takes animal form; usually that of a black cat
    Matagot,
    /// [](https://en.wikipedia.org/wiki/)(Hindu mythology) – First Avatar of Vishnu in the form of a half-fish and half-man
    Matsya,
    /// [](https://en.wikipedia.org/wiki/)(Hindu mythology) – Peacock spirit
    Mayura,
    /// [](https://en.wikipedia.org/wiki/)(Jewish mythology) – Invisible, malevolent spirit
    Mazzikin,
    /// [](https://en.wikipedia.org/wiki/)(Guaraní mythology) – Snake-parrot hybrid
    MboiTuI,
    /// [](https://en.wikipedia.org/wiki/)(Central Africa) – Possessing demon
    Mbwiri,
    /// [](https://en.wikipedia.org/wiki/)(Greek mythology) – Serpent-female hybrid,/// [](https://en.wikipedia.org/wiki/)(Gorgon) with numerous snake heads
    Medusa,
    // biblical bird
    MelekTaus,
    /// [](https://en.wikipedia.org/wiki/)(Greek mythology) – Ash tree nymph
    Meliae,
    /// [](https://en.wikipedia.org/wiki/)(Medieval folklore) – Female water spirit, with the form of a winged mermaid or serpent
    Melusine,
    /// [](https://en.wikipedia.org/wiki/)(Hawaiian mythology) – Little people and craftsmen
    Menehune,
    /// [](https://en.wikipedia.org/wiki/)(Finnish mythology) – Little people and nature spirits
    Menninkainen,
    /// [](https://en.wikipedia.org/wiki/)(Singapore) – Combination of a lion and a fish, the symbol of Singapore
    Merlion,
    /// [](https://en.wikipedia.org/wiki/)(multiple cultures) – Human-fish hybrid
    Mermaid,
    /// [](https://en.wikipedia.org/wiki/)(multiple cultures) – Human-fish hybrid
    Merman,
    /// [](https://en.wikipedia.org/wiki/)(English mythology) – Elderly wizard
    Merlin,
    /// [](https://en.wikipedia.org/wiki/)(Irish mythology and Scottish) – Human-fish hybrid
    Merrow,
    /// [](https://en.wikipedia.org/wiki/)(Abenaki mythology) – Ice-hearted wizards
    MeteeKolenOl,
    /// [](https://en.wikipedia.org/wiki/)(Australian Aboriginal mythology) – Extremely elongated humanoid that has to live in rock crevasses to avoid blowing away
    Mimi,
    /// [](https://en.wikipedia.org/wiki/)(Australian Aboriginal mythology) – Death spirit
    MinkaBird,
    /// [](https://en.wikipedia.org/wiki/)(Philippine) – Giant swallow
    Minokawa,
    /// [](https://en.wikipedia.org/wiki/)(Greek mythology) – Human-bull hybrid
    Minotaur,
    /// [](https://en.wikipedia.org/wiki/)(Ojibwa) – Feline water spirit
    Mishibizhiw,
    /// [](https://en.wikipedia.org/wiki/)(Ojibwa) – Serpentine rain spirit
    MisiGinebig,
    /// [](https://en.wikipedia.org/wiki/)(Cree) – Serpentine rain spirit
    MisiKinepikw,
    /// [](https://en.wikipedia.org/wiki/)(Japanese mythology) – Water dragon
    Mizuchi,
    /// [](https://en.wikipedia.org/wiki/)(Chinese mythology) – Vengeful ghost or demon
    Mogwai,
    /// [](https://en.wikipedia.org/wiki/)(Latin American folklore) – Nature spirit
    Mohan,
    /// [](https://en.wikipedia.org/wiki/)(Congo) – Water-dwelling creature
    MokeleMbembe,
    /// [](https://en.wikipedia.org/wiki/)(Australian Aboriginal mythology) – Malevolent spirit that kills sorcerers
    Mokoi,
    /// [](https://en.wikipedia.org/wiki/)(Polynesian mythology) – Amphibious humanoid living in the spirit world,/// [](https://en.wikipedia.org/wiki/)(underground world)
    Mokorea,
    /// [](https://en.wikipedia.org/wiki/)(Guaraní mythology) – Giant snake with antennae
    Monai,
    /// [](https://en.wikipedia.org/wiki/)(Medieval bestiaries) – One-horned stag-horse-elephant-boar hybrid, sometimes treated as distinct from the unicorn
    Monocerus,
    /// [](https://en.wikipedia.org/wiki/)(South America) – Giant monkey
    MonoGrande,
    /// [](https://en.wikipedia.org/wiki/)(Medieval bestiaries) – Dwarf with one giant foot
    Monopod,
    /// [](https://en.wikipedia.org/wiki/)(Manx folklore) – Nature spirit
    MooinjerVeggey,
    /// [](https://en.wikipedia.org/wiki/)(Slavic mythology) – Disembodied spirit
    Mora,
    /// [](https://en.wikipedia.org/wiki/)(Breton and Welsh mythology) – Water spirits
    Morgens,
    /// [](https://en.wikipedia.org/wiki/)(Japanese mythology) – Animated tea kettle
    MorinjiNoOkama,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Underworld spirit
    Mormolykeia,
    /// [](https://en.wikipedia.org/wiki/)(Romanian) – Vampiric ghost
    Moroi,
    /// [](https://en.wikipedia.org/wiki/)(Continental Germanic mythology) – Little people and tree spirits
    MossPeople,
    /// [](https://en.wikipedia.org/wiki/)(American folklore) – Large grey winged humanoid with glowing red eyes
    Mothman,
    /// [](https://en.wikipedia.org/wiki/)(Canadian folklore) – Fish-like lake monster
    Mugwump,
    /// [](https://en.wikipedia.org/wiki/)(Japanese mythology) – Shapeshifting badger spirit
    Mujina,
    /// [](https://en.wikipedia.org/wiki/)(Australian Aboriginal mythology) – Water monster
    Muldjewangk,
    /// [](https://en.wikipedia.org/wiki/)(Philippine mythology) – Spirit of a deceased person seeking justice or has unfinished business
    Multo,
    /// [](https://en.wikipedia.org/wiki/)(Egyptian) – Undead creature who revives
    Mummy,
    /// [](https://en.wikipedia.org/wiki/)(Romanian folklore) – Forest-dwelling hag
    MumaPadurii,
    /// [](https://en.wikipedia.org/wiki/)(Australian Aboriginal) – Giant goanna
    MungoonGali,
    /// [](https://en.wikipedia.org/wiki/)(Medieval bestiaries) – Hare-squirrel-boar hybrid that has an intense body heat
    Muscaliet,
    /// [](https://en.wikipedia.org/wiki/)(Greek mythology) – Spirits that inspire artists
    Muse,
    /// [](https://en.wikipedia.org/wiki/)(Mesopotamian mythology)
    Mushusshu,
    /// [](https://en.wikipedia.org/wiki/)(Heraldic) – Sheep-goat hybrid
    Musimon,
    /// [](https://en.wikipedia.org/wiki/)(Scandinavian folklore) – Ghosts of unbaptized children
    Myling,
    /// [](https://en.wikipedia.org/wiki/)(Medieval bestiaries) – Ant-lion hybrid
    Myrmecoleon,
    /// [](https://en.wikipedia.org/wiki/)(German) – Anthropophagous undead
    Nachzehrer,
    /// [](https://en.wikipedia.org/wiki/)(Buddhist and Hindu) – Nature and water spirits, serpentine or human-serpent hybrids
    Naga,
    /// [](https://en.wikipedia.org/wiki/)(Thai) – Spectral fire
    NagaFireballs,
    /// [](https://en.wikipedia.org/wiki/)(Mesoamerica) – Human-animal shapeshifter
    Nagual,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Freshwater nymph
    Naiad,
    /// [](https://en.wikipedia.org/wiki/)(Finnish) – Water spirit
    Nakki,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Ritual disciplinary demon from the Oga Peninsula
    Namahage,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Giant catfish whose thrashing causing earthquakes
    Namazu,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Old woman who hides under the floor in abandoned storerooms
    NandoBaba,
    /// [](https://en.wikipedia.org/wiki/)(Thai) – Tree spirit
    NangTakian,
    /// [](https://en.wikipedia.org/wiki/)(Abenaki) – Earthquake spirit
    NanomKeeaPoDa,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Grotto nymph
    Napaeae,
    /// [](https://en.wikipedia.org/wiki/)(Hindu mythology) – Avatar of Vishnu in the form of half-man/half-lion
    Narasimha,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Fate spirit
    Narecnitsi,
    /// [](https://en.wikipedia.org/wiki/)(Thai) – Pod people
    Nariphon,
    /// [](https://en.wikipedia.org/wiki/)(Gunai) – Water monster
    Nargun,
    /// [](https://en.wikipedia.org/wiki/)(Arabian) – Half-human, half-demon creature with half a body
    Nasnas,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Ghost
    Nav,
    /// [](https://en.wikipedia.org/wiki/)(Hawaiian) – Savage humanoid
    Nawao,
    /// [](https://en.wikipedia.org/wiki/)(Abenaki) – Fish-human hybrid
    NDamKenoWet,
    /// [](https://en.wikipedia.org/wiki/)(Roman mythology) – God of freshwater and sea
    Neptune,
    /// [](https://en.wikipedia.org/wiki/)(Germanic mythology) – Female water spirit
    Neck,
    /// [](https://en.wikipedia.org/wiki/)(Catalan) – Little people that turn into coins
    Negret,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Split-tailed magical cat
    Nekomata,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Cat in the form of a girl
    Nekomusume,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Lion with impenetrable skin
    NemeanLion,
    /// [](https://en.wikipedia.org/wiki/)(Abrahamic mythology) – Gigantic sons of Grigori and human women
    Nephilim,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Nymph daughters of Nereus
    Nereid,
    /// [](https://en.wikipedia.org/wiki/)(Mapuche) – Nature spirit
    Ngen,
    /// [](https://en.wikipedia.org/wiki/)(Mapuche) – Fox-like water snake
    Nguruvilu,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Predatory animal
    Nian,
    /// [](https://en.wikipedia.org/wiki/)(Hawaiian) – Warrior ghosts
    Nightmarchers,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Monster which appears as a young woman and sucks all of the flesh off of its victim's body
    Nikusui,
    /// [](https://en.wikipedia.org/wiki/)(Shoshone) – Aggressive little people
    Nimerigar,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Monkey-fish hybrid
    Ningyo,
    /// [](https://en.wikipedia.org/wiki/)(Western Africa) – Large reptile, possibly a dragon
    NinkiNanka,
    /// [](https://en.wikipedia.org/wiki/)(Scandinavian) – House spirit
    Nisse,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Dragon
    Niohoggr,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Ocean demon
    Nivatakavachas,
    /// [](https://en.wikipedia.org/wiki/)(Germanic) – Female water spirit
    Nix,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Supernatural wall, also a monstrous flying squirrel
    Nobusuma,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Nightmare spirit
    Nocnitsa,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Faceless ghost
    NopperaBo,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Small sea serpent
    Nozuchi,
    /// [](https://en.wikipedia.org/wiki/)(Scottish) – Malevolent human-horse-fish hybrid
    Nuckelavee,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Monkey-raccoon dog-tiger-snake hybrid
    Nue,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Vengeful female ghost
    NuGui,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Disembodied, flying head that attacks people
    Nukekubi,
    /// [](https://en.wikipedia.org/wiki/)(Māori) – Forest spirit
    NukuMaiTore,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiary) – Humanoid with backwards, eight-toed feet
    Nuli,
    /// [](https://en.wikipedia.org/wiki/)(Roman) – Tutelary spirit
    Numen,
    /// [](https://en.wikipedia.org/wiki/)(Philippine) – Malevolent little people
    Nuno,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Animated chunk of dead flesh
    Nuppeppo,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Head-sized ball-like creature that floats in the sea and teases sailors
    Nurarihyon,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Female monster who appears on the beach
    NureOnna,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Spirit that manifests as an impassable, invisible wall
    Nurikabe,
    /// [](https://en.wikipedia.org/wiki/)(Tonga,/// [](https://en.wikipedia.org/wiki/)(Zimbabwean) mythology) – Snake-spirit of the Zambezi River
    NyamiNyami,
    /// [](https://en.wikipedia.org/wiki/)(Lithuanian) – Cavern spirit
    Nykstukas,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Nature spirit
    Nymph,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Shapeshifting spirits
    Obake,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Spook which rides piggyback on a human victim and becomes unbearably heavy
    Obariyon,
    /// [](https://en.wikipedia.org/wiki/)(Ashanti) – Vampiric possession spirit
    Obayifo,
    /// [](https://en.wikipedia.org/wiki/)(West Africa) – Gigantic animal that serves witches
    Obia,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Nymph daughters of Oceanus
    Oceanid,
    /// [](https://en.wikipedia.org/wiki/)(Basque) – Storm spirit
    Odei,
    /// [](https://en.wikipedia.org/wiki/)(Norse mythology) – King of Asgard
    Odin,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Changeling
    Odmience,
    /// [](https://en.wikipedia.org/wiki/)(Jewish) – Giant king of the Amorites
    Og,
    /// [](https://en.wikipedia.org/wiki/)(Canadian) Canadian Lake Monster
    Ogopogo,
    /// [](https://en.wikipedia.org/wiki/)(Nigeria) – Iron god for the Yoruba people,/// [](https://en.wikipedia.org/wiki/)(South Western Nigeria)
    Ogun,
    /// [](https://en.wikipedia.org/wiki/)(Medieval folklore) – Large, grotesque humanoid
    Ogre,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Ghost of a woman with a distorted face who was murdered by her husband
    Oiwa,
    /// [](https://en.wikipedia.org/wiki/)(Cantabrian) – Giant cyclops who embodies evil.
    Ojancanu,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Spirit of a plate-counting servant girl, associated with the 'Okiku-Mushi' worm
    Okiku,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Death spirit
    Okubi,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Dog or wolf that follows travelers at night, similar to the Black dog of English folklore
    OkuriInu,
    /// [](https://en.wikipedia.org/wiki/)(Guyanese) – Vampiric hag who takes the form of a fireball at night
    OleHigue,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Giant, human-eating centipede that lives in the mountains
    Omukade,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Large, grotesque humanoid demon, usually having red skin and horns
    Oni,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Spectral fire
    Onibi,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Bird-demon created from the spirits of freshly dead corpses
    Onmoraki,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Human-donkey hybrid
    Onocentaur,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Shapeshifting demon
    Onoskelis,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Vengeful ghost that manifests in a physical rather than a spectral form
    Onryo,
    /// [](https://en.wikipedia.org/wiki/)(Aztec and Latin American folklore) – Wild cat, possibly a subspecies of cougar
    Onza,
    /// [](https://en.wikipedia.org/wiki/)(Unknown origin) – Bird that flies backwards
    OozlumBird,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Bull-serpent hybrid
    Ophiotaurus,
    /// [](https://en.wikipedia.org/wiki/)(Heraldic) – Lion-eagle hybrid, similar to a griffin, but with leonine forelimbs
    Opinicus,
    /// [](https://en.wikipedia.org/wiki/)(Malay) – Forest spirit
    OrangBunian,
    /// [](https://en.wikipedia.org/wiki/)(Malay) – Spectral rapist
    OrangMinyak,
    /// [](https://en.wikipedia.org/wiki/)(Hungarian) – Shapeshifting demon
    Ordog,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Mountain nymph
    Oread,
    /// [](https://en.wikipedia.org/wiki/)(Tyrolean) – Little people and house spirits
    Ork,
    /// [](https://en.wikipedia.org/wiki/)(European) – Horse-headed, honest oracle classed as a demon
    Orobas,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Peacock-eagle-swan-crane hybrid
    OrphanBird,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Two-headed dog
    Orthrus,
    /// [](https://en.wikipedia.org/wiki/)(Hellenized) – God of the dead and the judge of the underworld
    Osiris,
    /// [](https://en.wikipedia.org/wiki/)(Nigeria) – God of love and fertility
    Oshun,
    /// [](https://en.wikipedia.org/wiki/)(Finnish) – Bear spirit
    Otso,
    /// [](https://en.wikipedia.org/wiki/)(Worldwide) – Mystic serpent/dragon that eats its own tail
    Ouroboros,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Malevolent threshing house spirit
    Ovinnik,
    /// [](https://en.wikipedia.org/wiki/)(Cornish) – Owl-like humanoid
    Owlman,
    /// [](https://en.wikipedia.org/wiki/)(Finnish) – Spectral fire
    PaasselkaDevils,
    /// [](https://en.wikipedia.org/wiki/)(Abenaki) – Weather spirit
    Pamola,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Human-goat hybrids descended from the god Pan
    Panes,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiary) – White-haired humanoid with giant ears and eight fingers and toes
    Pandi,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Demons with herds of stolen cows
    Panis,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Water dragon
    Panlong,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Humanoid with gigantic ears
    Panotti,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Feline with sweet breath
    Panther,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Shapeshifting animal whose natural form was a large ruminant
    Parandrus,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Fast, spotted feline believed to mate with lions to produce leopards
    Pard,
    /// [](https://en.wikipedia.org/wiki/)(Etruscan) – Fish-tailed leopard
    Pardalokampoi,
    /// [](https://en.wikipedia.org/wiki/)(Medieval folklore) – Giant race reputed to live in the area of Patagonia
    Patagon,
    /// [](https://en.wikipedia.org/wiki/)(Latin America) – Anthropophagous, one-legged humanoid
    Patasola,
    /// [](https://en.wikipedia.org/wiki/)(Māori) – White-skinned nature spirits
    Patupairehe,
    /// [](https://en.wikipedia.org/wiki/)(Scottish) – Strong little people
    Pech,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Spring nymph
    Pegaeae,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Winged horse
    Pegasus,
    /// Pegasus-unicorn hybrid
    Pegacorn,
    /// [](https://en.wikipedia.org/wiki/)(Malay) – Servant spirit
    Pelesit,
    /// [](https://en.wikipedia.org/wiki/)(French) – Dragon
    Peluda,
    /// [](https://en.wikipedia.org/wiki/)(Malay) – Vampires that sever their heads from their bodies to fly around, usually with their intestines or other internal organs trailing behind
    Penanggalan,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Giant bird
    Peng,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Tree spirit
    Penghou,
    /// [](https://en.wikipedia.org/wiki/)(Persian) – Winged humanoid
    Peri,
    /// [](https://en.wikipedia.org/wiki/)(Allegedly Medieval folklore) – Deer-bird hybrid
    Peryton,
    /// [](https://en.wikipedia.org/wiki/)(Catalan) – Nightmare demon in the form of a cat or dog
    Pesanta,
    /// [](https://en.wikipedia.org/wiki/)(Chilota and Mapuche) – Vampiric, flying, shapeshifting serpent
    Peuchen,
    /// [](https://en.wikipedia.org/wiki/)(Thai) – Ghost of a person who has died suddenly of a violent or cruel death
    PhiTaiHong,
    /// [](https://en.wikipedia.org/wiki/)(Phoenician) – Regenerative bird reborn from its own ashes
    Phoenix,
    /// [](https://en.wikipedia.org/wiki/)(Native American mythology) – Winged, antlered feline-like dragon
    Piasa,
    /// [](https://en.wikipedia.org/wiki/)(Armenian) – Large land animal
    Piatek,
    /// [](https://en.wikipedia.org/wiki/)(Pictish stones) – Stylistic animal, possibly a dragon
    PictishBeast,
    /// [](https://en.wikipedia.org/wiki/)(Mapuche) – Nature spirit
    Pillan,
    /// [](https://en.wikipedia.org/wiki/)([Japanese spirit])
    Plagg,
    /// [](https://en.wikipedia.org/wiki/)(Abenaki) – Water spirit
    PimSkwaWagenOwad,
    /// [](https://en.wikipedia.org/wiki/)(Finnish) – Minor demon
    Piru,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Carrion-eating demon
    Pishacha,
    /// [](https://en.wikipedia.org/wiki/)(Peru) – Monster man that steals its victim's body fat for cannibalistic purposes
    Pishtaco,
    /// [](https://en.wikipedia.org/wiki/)(Abenaki) – Serpentine rain spirit
    PitaSkog,
    /// [](https://en.wikipedia.org/wiki/)(Cornish) – Little people and nature spirits
    Pixie,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Winged lion
    Pixiu,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Horned, dragon-lion hybrid
    PiYao,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Vampire created when a mother strangles her child
    Plakavac,
    /// [](https://en.wikipedia.org/wiki/)(Abenaki) – Tree spirit
    PokWejeeMen,
    /// [](https://en.wikipedia.org/wiki/)(Polish) – Little people and field spirits
    Polevik,
    /// [](https://en.wikipedia.org/wiki/)(Colombian) – Man-eating chicken spirit
    PolloMaligno,
    /// [](https://en.wikipedia.org/wiki/)(Malay) – Invisible servant spirit
    Polong,
    /// [](https://en.wikipedia.org/wiki/)(German) – Ghost that moves objects
    Poltergeist,
    /// [](https://en.wikipedia.org/wiki/)(Guaraní) – Wild man and nature spirit
    Pombero,
    /// [](https://en.wikipedia.org/wiki/)(Māori) – Grotesque, malevolent humanoid
    Ponaturi,
    /// [](https://en.wikipedia.org/wiki/)(Malay) – Undead, vampiric women who died in childbirth
    Pontianak,
    /// [](https://en.wikipedia.org/wiki/)(American Folklore) Kentucky Urban Legend – Cryptid, a murderous creature that is part man, sheep, and goat
    PopeLickMonster,
    /// [](https://en.wikipedia.org/wiki/)(Māori) – Giant bird
    Poukai,
    /// [](https://en.wikipedia.org/wiki/)(Buddhist, Hindu, and Jain) – Ghosts of especially greedy people
    Preta,
    /// [](https://en.wikipedia.org/wiki/)(Romanian – Roman) – Undead wolf
    Pricolici,
    /// [](https://en.wikipedia.org/wiki/)(Serbia) – Dog-headed monster
    Psoglav,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Mischievous spirit
    Psotnik,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Butterfly-winged nymphs, daughters of Psyche
    Psychai,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Creatures, spirits, angels, or deities in many religions who escort newly deceased souls from Earth to the afterlife
    Psychopomp,
    /// [](https://en.wikipedia.org/wiki/)(Welsh) – Shapeshifting animal spirit
    Puca,
    /// [](https://en.wikipedia.org/wiki/)(Icelandic) – Malevolent little person
    Puki,
    /// [](https://en.wikipedia.org/wiki/)(English) – House spirit
    Puck,
    /// [](https://en.wikipedia.org/wiki/)(German) – House spirit
    Putz,
    /// [](https://en.wikipedia.org/wiki/)(Philippine) – Headless humanoid
    Pugot,
    /// [](https://en.wikipedia.org/wiki/)(Frisian) – House spirit
    Puk,
    /// [](https://en.wikipedia.org/wiki/)(Latvian) – Dragon
    Pukis,
    /// [](https://en.wikipedia.org/wiki/)(Native American mythology) – Troll-like gray-skinned being
    Puckwudgie,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Little people
    Pygmy,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Insect-dragon hybrid
    Pyrausta,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Serpentine dragon
    Python,
    /// [](https://en.wikipedia.org/wiki/)(Inuit mythology) – Aquatic human abductor
    Qalupalik,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Dragon-ox-deer hybrid
    Qilin,
    /// [](https://en.wikipedia.org/wiki/)(Inuit) – Large, bald dog spirit
    Qiqirn,
    /// [](https://en.wikipedia.org/wiki/)(Jewish) – Evil spirits
    Qliphoth,
    /// [](https://en.wikipedia.org/wiki/)(Arthurian legend) – Serpent-leopard-lion-hart hybrid
    QuestingBeast,
    /// [](https://en.wikipedia.org/wiki/)(Aztec) – Important Aztec god whose name means 'feathered serpent'; he is not to be confused with the quetzal, a type of bird
    Quetzalcoatl,
    /// [](https://en.wikipedia.org/wiki/)(Frankish) – Five-horned bull
    Quinotaur,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Spirit that protects a specific place
    Ra,
    /// [](https://en.wikipedia.org/wiki/)(Akkadian) – Vampiric spirit that ambushes people
    Rabisu,
    /// [](https://en.wikipedia.org/wiki/)(Swedish) – Tree spirit
    Radande,
    /// [](https://en.wikipedia.org/wiki/)(Lithuanian) – Malevolent witch
    Ragana,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Lightning spirit
    Raiju,
    /// [](https://en.wikipedia.org/wiki/)(Native American) – Rain spirit
    RainBird,
    /// [](https://en.wikipedia.org/wiki/)(Lenape) – Crow spirit
    RainbowCrow,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Whale-sized, multi-colored fish
    RainbowFish,
    /// [](https://en.wikipedia.org/wiki/)(Australian Aboriginal) – Snake
    RainbowSerpent,
    /// [](https://en.wikipedia.org/wiki/)(Buddhist and Hindu) – Shapeshifting demon
    Rakshasa,
    /// [](https://en.wikipedia.org/wiki/)(Cantabrian) – Extremely long, weasel-like animal
    Ramidreju,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Whirlwind spirit
    Rarog,
    /// [](https://en.wikipedia.org/wiki/)(Cherokee) – Life-draining spirit
    RavenMocker,
    /// [](https://en.wikipedia.org/wiki/)(Native American, Norse, and Siberian) – Trickster spirit
    RavenSpirit,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Squirrel spirit
    Ratatoskr,
    /// [](https://en.wikipedia.org/wiki/)(American Folklore) – Possible plesiosaur or serpent
    RaystownRay,
    /// [](https://en.wikipedia.org/wiki/)(English) – Evil, ugly humanoid
    Redcap,
    /// [](https://en.wikipedia.org/wiki/)(Jewish) – Gigantic land animal
    ReEm,
    /// [](https://en.wikipedia.org/wiki/)(Heraldic) – Eagle, sometimes depicted with two heads
    Reichsadler,
    /// [](https://en.wikipedia.org/wiki/)(Jewish) – Giant
    Rephaite,
    /// [](https://en.wikipedia.org/wiki/)(Global) – Human-lizard hybrid
    ReptilianHumanoid,
    /// [](https://en.wikipedia.org/wiki/)(Medieval folklore) – Reanimated dead
    Revenant,
    /// [](https://en.wikipedia.org/wiki/)(Arabian and Persian) – Gigantic bird
    Roc,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Long-necked, humanoid trickster
    Rokurokubi,
    /// [](https://en.wikipedia.org/wiki/)(Africa and India) – Skeletal creature with elements of a rabbit, badger, and bear
    Rompo,
    /// [](https://en.wikipedia.org/wiki/)(Vietnamese) dragon
    Rong,
    /// [](https://en.wikipedia.org/wiki/)(French America) – Human-wolf shapeshifter
    Rougarou,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Female water spirit
    Rusalka,
    /// Japanese dragon
    Ryu,
    /// [](https://en.wikipedia.org/wiki/)(Brazilian) – One-legged nature spirit
    Saci,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Horse head that dangles from trees on Kyūshū
    Sagari,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Haunted pillar, installed upside-down
    Sakabashira,
    /// [](https://en.wikipedia.org/wiki/)(Alchemy) – Fire elemental
    Salamander,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Shark-man servant of the dragon king of the sea
    Samebito,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Nature spirit
    Samodiva,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – The demigod Jatayu's brother
    Sampati,
    /// [](https://en.wikipedia.org/wiki/)(Northern Europe) – Nursery spirit that induces sleep in children
    Sandman,
    /// [](https://en.wikipedia.org/wiki/)(South Western Nigeria) – Yoruba king of arts, music, dance and entertainment
    Sango,
    /// [](https://en.wikipedia.org/wiki/)(Philippine) – Spirits in the form of fireballs that roam around the forest
    Santelmo,
    /// [Santa Claus](https://en.wikipedia.org/wiki/Santa_Claus)(North Pole-European folklore) – Elderly man who delivers gifts to well-behaved children on the night of Christmas Eve
    SantaClaus,
    /// [](https://en.wikipedia.org/wiki/)(Romanian) – Nature spirit
    Sanziana,
    /// [](https://en.wikipedia.org/wiki/)(Philippine) – Bird of good fortune
    Sarimanok,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Bird spirit
    Sarngika,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Wicked monkey spirit who was defeated by a dog
    Sarugami,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Mind-reading humanoid
    Satori,
    /// [](https://en.wikipedia.org/wiki/)(Heaven--Abrahamic mythology) – Ruler of Hell
    Satan,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Human-goat hybrid and fertility spirit
    Satyr,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiary) – Apes who always bear twins, one the mother loves, the other it hates
    Satyrus,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Shapeshifting turban snail spirit
    SazaeOni,
    /// [](https://en.wikipedia.org/wiki/)(English) – Shapeshifting undead
    Sceadugenga,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Snake which mesmerizes its prey
    Scitalis,
    /// [](https://en.wikipedia.org/wiki/)(Sumerian) – Human-scorpion hybrid
    ScorpionMan,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Human-snake hybrid with a snake's tail, twelve legs, and six long-necked snake heads
    Scylla,
    /// [](https://en.wikipedia.org/wiki/)(Heraldic) – Fish-tailed bee
    SeaBee,
    /// [](https://en.wikipedia.org/wiki/)(Heraldic) a legendary creature that has the head and upper body of a lion, but with webbed forelimbs and a fish tail.
    SeaLion,
    /// [](https://en.wikipedia.org/wiki/)(Medieval folklore) – Fish-like humanoid
    SeaMonk,
    /// [](https://en.wikipedia.org/wiki/)(Worldwide) – Giant, marine animals
    SeaMonster,
    /// [](https://en.wikipedia.org/wiki/)(Worldwide) – Serpentine sea monster
    SeaSerpent,
    /// [](https://en.wikipedia.org/wiki/)(Heraldic) – Fish-tailed wyvern
    SeaWyvern,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Water spirit which can be heard making merry at night
    Seko,
    /// [](https://en.wikipedia.org/wiki/)(Faroese, Icelandic, Irish, and Scottish) – Human-seal shapeshifter
    Selkie,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Human-faced frog which guides newly deceased souls to the graveyard
    SenpokuKanpoku,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Snake with corrosive venom
    Seps,
    /// [](https://en.wikipedia.org/wiki/)(Worldwide) – Snake spirit
    Serpent,
    /// [](https://en.wikipedia.org/wiki/)(Ancient Egypt) – Serpent-leopard hybrid
    Serpopard,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Tiger-carp hybrid
    Shachihoko,
    /// [](https://en.wikipedia.org/wiki/)(Worldwide) – Spiritual imprint
    Shade,
    /// [](https://en.wikipedia.org/wiki/)(American) – Malevolent ghost
    ShadowPeople,
    /// [](https://en.wikipedia.org/wiki/)(Persian) – Giant eagle or hawk
    Shahbaz,
    /// [](https://en.wikipedia.org/wiki/)(Islam) – Islamic version of the Devil (Satan) from the Bible
    Shaitan,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Rain bird
    ShangYang,
    /// [](https://en.wikipedia.org/wiki/)(Jewish) – Chicken-legged demon
    Shedim,
    /// [](https://en.wikipedia.org/wiki/)(Akkadian and Sumerian) – Protective spirit who takes the form of a winged bull or human-headed lion
    Shedu,
    /// [](https://en.wikipedia.org/wiki/)(English, Scottish and German, as schellenrocc) – Water spirit
    Shellycoat,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Shapeshifing sea monster
    Shen,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Weather dragon
    Shenlong,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Water spirit from Shikoku
    Shibaten,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Servant spirit
    Shikigami,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Child-sized servant spirit
    ShikiOji,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Underworld hag
    Shikome,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – 'Death god'
    Shinigami,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – White, faceless spirit
    ShiroBozu,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Animated mosquito netting or dust cloth
    Shirouneri,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Spirit of a dead person
    Shiryo,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Lion-dog hybrid
    Shisa,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Protective animal
    Shishi,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Red-haired sea-sprites who love alcohol
    Shojo,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Creature that peers in through skylights
    Shokera,
    /// [](https://en.wikipedia.org/wiki/)(Albanian) – Vampire witch that feeds on children
    Shtriga,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Drowned ghost
    ShuiGui,
    /// [](https://en.wikipedia.org/wiki/)(English) – Dog/monkey
    ShugMonkey,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Red-faced ghoul
    Shunoban,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Ruler of the Oni
    ShutenDoji,
    /// [](https://en.wikipedia.org/wiki/)(Irish and Scottish) – Ancestral or nature spirit
    Sídhe,
    /// [](https://en.wikipedia.org/wiki/)(Philippine) – Goat-like vampire
    Sigbin,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Bald, fat, thick-lipped, and flat-nosed followers of Dionysus
    Sileni,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Winged dog
    Simargl,
    /// [](https://en.wikipedia.org/wiki/)(Persian) – Dog-lion-peacock hybrid
    Simurgh,
    /// [](https://en.wikipedia.org/wiki/)(Batak) – Feline animal
    Singa,
    /// [](https://en.wikipedia.org/wiki/)(Choctaw) – Serpentine rain spirit
    SintHolo,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Human-bird hybrid
    Siren,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Demonic human-headed bird
    Sirin,
    /// [](https://en.wikipedia.org/wiki/)(Akkadian) – Dragon with aquiline hind legs and feline forelegs
    Sirrush,
    /// [](https://en.wikipedia.org/wiki/)(American Indian) – Two-headed sea serpent
    Sisiutl,
    /// [](https://en.wikipedia.org/wiki/)(Paiute) – Red-haired giants
    SiTeCah,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Freshwater spirit
    Sjora,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Sea spirit
    Sjovaettir,
    /// [](https://en.wikipedia.org/wiki/)(American Indian) – Animal-human shapeshifter
    SkinWalker,
    /// [](https://en.wikipedia.org/wiki/)(Scandinavian) – Forest spirit
    Skogsra,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Wolf that chases the Sun
    Skoll,
    /// [](https://en.wikipedia.org/wiki/)(Chinook Jargon) – Hairy giant
    Skookum,
    /// [](https://en.wikipedia.org/wiki/)(Medieval folklore) – Living skeletons
    Skeleton,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Flying imp
    Skrzak,
    /// [](https://en.wikipedia.org/wiki/)(Polish) – Weather spirit
    SkyWomen,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Eight-legged horse
    Sleipnir,
    /// [](https://en.wikipedia.org/wiki/)(Irish and Scottish) – Restless ghost
    Sluagh,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Invisible spirit which pulls on sleeves
    SodehikiKozo,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Fiery ghost of an oil-stealing monk
    Sogenbi,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Ritual disciplinary demon
    Soragami,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Sound of trees being cut down, when later none seem to have been cut
    SorakiGaeshi,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Ghost with an abacus
    Sorobanbozu,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Fox spirit from Kyoto
    Sotangitsune,
    /// [](https://en.wikipedia.org/wiki/)(Trinidad and Tobago) – Vampiric hag who takes the form of a fireball at night
    Soucouyant,
    /// [](https://en.wikipedia.org/wiki/)(Cherokee) – Sharp-fingered hag
    Spearfinger,
    /// [](https://en.wikipedia.org/wiki/)(Worldwide) – Terrifying ghost
    Spectre,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Winged woman-headed lion
    Sphinx,
    /// [](https://en.wikipedia.org/wiki/)(Romanian) – Little people
    Spiridus,
    /// Ghosts
    Spirit,
    /// [](https://en.wikipedia.org/wiki/)(Cornish) – Guardians of graveyards and ruins
    Spriggan,
    /// [](https://en.wikipedia.org/wiki/)(Medieval folklore) – little people, ghosts or elves
    Sprite,
    /// [](https://en.wikipedia.org/wiki/)(American) – Ugly and lonely creature capable of evading capture by dissolving itself into a pool of tears
    Squonk,
    /// [](https://en.wikipedia.org/wiki/)(Albanian) – Demonic dragon who guards a treasure
    Stihi,
    /// [](https://en.wikipedia.org/wiki/)(Romanian) – Vampire
    Strigoi,
    /// [](https://en.wikipedia.org/wiki/)(Roman) – Vampiric bird
    Strix,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Humanoid whose males have enormous feet, and females have tiny feet
    Struthopodes,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Vampiric undead
    Strzyga,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Malevolent mountain spirit
    Stuhac,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Metallic bird
    StymphalianBird,
    /// [](https://en.wikipedia.org/wiki/)(New Guinea) – Cannibalistic sorcerer
    Suangi,
    /// [](https://en.wikipedia.org/wiki/)(Medieval folklore) – Female night-demon
    Succubus,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Fortune spirit
    Sudice,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Sand-throwing hag
    SunakakeBaba,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Small dog- or cat-like creature that rubs against a person's legs at night
    Sunekosuri,
    /// [](https://en.wikipedia.org/wiki/)(Finnish) – Hellhound
    Surma,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Japanese version of the Chinese Vermillion Bird
    Suzaku,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Unnatural strong horse, father of Sleipnir
    Svaoilfari,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Cavern spirits; the Black Elves
    Svartalfar,
    /// [](https://en.wikipedia.org/wiki/)(Ancient Egyptian) – Crocodile-leopard-hippopotamus hybrid
    Swallower,
    /// [](https://en.wikipedia.org/wiki/)(Worldwide) – Swan-human shapeshifter
    SwanMaiden,
    /// [](https://en.wikipedia.org/wiki/)(Alchemy) – Air elemental
    Sylph,
    /// [](https://en.wikipedia.org/wiki/)(Medieval folklore) – Forest spirit
    Sylvan,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – African giant
    Syrbotae,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Reptilian humanoid
    Syrictae,
    /// [](https://en.wikipedia.org/wiki/)(Jewish) – Large land animal
    Tachash,
    /// [](https://en.wikipedia.org/wiki/)(American Folklore),/// [](https://en.wikipedia.org/wiki/)(Appalachia) – Powerful animal, that takes revenge on those who steal its tail
    Tailypo,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Tengu surrounded in demonic fire
    Taimatsumaru,
    /// [](https://en.wikipedia.org/wiki/)(Persian) – Nature spirit
    Takam,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Female spirit which can stretch itself to peer into the second story of a building
    TakaOnna,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Giant made of bronze
    Talos,
    /// [](https://en.wikipedia.org/wiki/)(Scottish) – Shapeshifting water spirit
    Tangie,
    /// [](https://en.wikipedia.org/wiki/)(Māori) – Water spirit
    Taniwha,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Unharvested persimmon which becomes a monster
    Tantankororin,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Shapeshifting raccoon dog
    Tanuki,
    /// [](https://en.wikipedia.org/wiki/)(Mariana Islands) – Ancestral spirits
    TaotaoMona,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Greed spirit
    Taotie,
    /// [](https://en.wikipedia.org/wiki/)(Mangaia) – Nature spirit
    Tapairu,
    /// [](https://en.wikipedia.org/wiki/)(French) – Dragon with leonine, turtle, bear, and human attributes
    Tarasque,
    /// [](https://en.wikipedia.org/wiki/)(Basque) – One-eyed giant
    Tartalo,
    /// [](https://en.wikipedia.org/wiki/)(Christian) – Demonic punisher
    Tartaruchi,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Poltergeist that hits the tatami mats at night
    TatamiTataki,
    /// [](https://en.wikipedia.org/wiki/)(Alpine Folklore) lizard-like creature, often described as having the face of a cat, with a serpent-like body which may be slender or stubby, with four short legs or two forelegs
    Tatzelwurm,
    /// Japanese dragon
    Tatsu,
    /// [](https://en.wikipedia.org/wiki/)(Etruscan) – Fish-tailed bull
    Taurokampoi,
    /// [](https://en.wikipedia.org/wiki/)(Trabzon) – Night-demon
    Tavara,
    /// [](https://en.wikipedia.org/wiki/)(Guaraní) – Lizard with seven dog heads
    TejuJagua,
    /// [](https://en.wikipedia.org/wiki/)(Mayan) – Bird
    Tecumbalam,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Anthropomorphic bird
    Tengu,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Angelic humanoid
    Tennin,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Ghost of a blind man, with his eyes on his hands
    TeNoMe,
    /// [](https://en.wikipedia.org/wiki/)(Azerbaijani) – Azerbaijani mythical creature similar to the cyclops Polyphemus
    Tepegoz,
    /// [](https://en.wikipedia.org/wiki/)(Jewish) – Lion-eagle-scorpion hybrid made from the blood of murder victims
    TerribleMonster,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Gigantic fox
    TeumessianFox,
    /// [](https://en.wikipedia.org/wiki/)(Medieval folklore) – Animal-headed humanoid
    Theriocephalus,
    /// [](https://en.wikipedia.org/wiki/)(Asia and Africa) – Solar bird
    ThreeLeggedBird,
    /// [](https://en.wikipedia.org/wiki/)(Native American) – Avian lightning bird spirit
    Thunderbird,
    /// [](https://en.wikipedia.org/wiki/)(Norse mythology) – God of thunder and storm
    Thor,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Meteoric dog
    Tiangou,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Celestial dragon
    Tianlong,
    /// [](https://en.wikipedia.org/wiki/)(Canarian) – Evil Dog
    Tibicena,
    /// [](https://en.wikipedia.org/wiki/)(English) – Bog spirit
    TiddyMun,
    /// [](https://en.wikipedia.org/wiki/)(Philippine) – Asian fairy bluebird
    Tigmamanukan,
    /// [](https://en.wikipedia.org/wiki/)(Jewish) – Giant lion
    Tigris,
    /// [](https://en.wikipedia.org/wiki/)(Philippine) – Anthropomorphic horse
    Tikbalang,
    /// [](https://en.wikipedia.org/wiki/)(Zulu) – Little people and water spirit
    Tikoloshe,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Sea monster
    Timingila,
    /// [](https://en.wikipedia.org/wiki/)(Māori) – Spirit that protects a specific place
    Tipua,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Primeval god
    Titan,
    /// [](https://en.wikipedia.org/wiki/)(Philippine) – Demons that are souls of dead unbaptized babies
    Tiyanak,
    /// [](https://en.wikipedia.org/wiki/)(Inuit) – Sea serpent
    Tizheruk,
    /// [](https://en.wikipedia.org/wiki/)(Tlaxcalan) – Shapeshifting vampire
    Tlahuelpuchi,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Spirit child carrying a block of tofu
    TofuKozo,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Ghost who lurks in grade school restroom stalls
    ToireNoHanakosan,
    /// [](https://en.wikipedia.org/wiki/)(Scandinavian) – House spirit
    Tomte,
    /// [Tooth fairy](https://en.wikipedia.org/wiki/Tooth_fairy) a mythical creature who gives out money in exchange for teeth.
    ToothFairy,
    
    The tradition of leaving a tooth under a pillow for the Tooth Fairy or another fantasy figure to collect is practiced in various countries.,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Water spirit
    Topielec,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Greed spirit
    Totetsu,
    /// [](https://en.wikipedia.org/wiki/)(Malay) – Servant spirit
    Toyol,
    /// [](https://en.wikipedia.org/wiki/)(Spanish and Portuguese) – Grotesque, mischievous little people
    Trasgo,
    /// [](https://en.wikipedia.org/wiki/)(Chilota) – Fertility spirit
    Trauco,
    /// [](https://en.wikipedia.org/wiki/)(Cantabrian) – Diminutive demon
    Trenti,
    /// Character in a story which exhibits a great degree of intellect or secret knowledge, and uses it to play tricks or otherwise disobey normal rules and conventional behaviour
    Trickster,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Demonic inhabitants of Tripura
    Tripurasura,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Male human-fish hybrid
    Tritons,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Nature spirit
    Troll,
    /// [](https://en.wikipedia.org/wiki/)(Orkney and Shetland) – Little people and nature spirits
    Trow,
    /// [](https://en.wikipedia.org/wiki/)(Abenaki) – Vampiric demon
    TsiNoo,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Shapeshifting, giant spider
    Tsuchigumo,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Plump snake-like creature
    Tsuchinoko,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Inanimate object that becomes animated after existing for 100 years
    Tsukumogami,
    /// [](https://en.wikipedia.org/wiki/)(Cherokee) – Giant nature spirit
    TsulKalu,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Icicle woman
    TsuraraOnna,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Monster which drops or lowers a bucket from the top of a tree to catch people
    TsurubeOtoshi,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Evil shapeshifter
    TugarinZmeyevich,
    /// [](https://en.wikipedia.org/wiki/)(Welsh) – Nature spirit
    TylwythTeg,
    /// [](https://en.wikipedia.org/wiki/)(Inuit) – Animated construct
    Tupilaq,
    /// [](https://en.wikipedia.org/wiki/)(Māori) – Pale spirit
    Turehu,
    /// [](https://en.wikipedia.org/wiki/)(Swiss) – legendary figure who turns people into dogs
    Turst,
    /// [](https://en.wikipedia.org/wiki/)(Hungarian) – Giant falcon that helped shape the origins of the Magyars
    Turul,
    /// [](https://en.wikipedia.org/wiki/)(Heraldry) – Like a real tiger, but lacks stripes. It has the tufted tail of a lion and a thick mane along the neck like a horse
    Tyger,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Winged, snake-legged giant
    Typhon,
    /// [](https://en.wikipedia.org/wiki/)(Aztec) – Skeletal star spirit
    Tzitzimitl,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Ghosts of women who died in childbirth
    Ubume,
    ///(Manipuri mythology) – Semi human, semi hornbill creature
    UchekLangmeidong,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Horse's leg which dangles from a tree and kicks passersby
    UmaNoAshi,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Ghost of drowned priest
    Umibozu,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Female sea monster who steals fish
    UmiNyobo,
    /// [](https://en.wikipedia.org/wiki/)(Worldwide) – Dead that behave as if alive
    Undead,
    /// [](https://en.wikipedia.org/wiki/)(Native American) – Feline water spirit
    UnderwaterPanther,
    /// [](https://en.wikipedia.org/wiki/)(Alchemy) – Water elemental
    Undine,
    /// [](https://en.wikipedia.org/wiki/)(Lakota) – Dragon
    Unhcegila,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Horse-like creature with the legs of an antelope, the tail of a lion and a single magical healing horn. Also called an Abada (African) – Unicorn that inhabits the African Congo.
    Unicorn,
    /// [](https://en.wikipedia.org/wiki/)(Lakota) – Serpentine rain spirit
    Unktehi,
    /// [](https://en.wikipedia.org/wiki/)(Lakota) – Reptilian water monster
    Unktehila,
    /// [](https://en.wikipedia.org/wiki/)(Lithuanian) – River spirit
    Upinis,
    /// [](https://en.wikipedia.org/wiki/)(Native American) – Hairy giant
    Urayuli,
    /// [](https://en.wikipedia.org/wiki/)(Romanian) – Giant
    Urias,
    /// [](https://en.wikipedia.org/wiki/)(Mesopotamian) – Lion-human hybrid guardian spirit
    Urmahlullu,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Bull-headed monster
    UshiOni,
    /// [](https://en.wikipedia.org/wiki/)(Akkadian) – ″Underworld messenger spirit″
    Utukku,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Spirit that shouts to surprise people
    Uwan,
    /// [](https://en.wikipedia.org/wiki/)(Latvian) – Spirit that misleads people
    Vadatajs,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Divine mounts
    Vahana,
    /// [](https://en.wikipedia.org/wiki/)(Indian) – Deadly snake
    Vaibhavi,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Female spirit that leads souls of dead warriors to Valhalla
    Valkyrie,
    /// [](https://en.wikipedia.org/wiki/)(Romanian) – Female nature spirit
    Valva,
    /// [](https://en.wikipedia.org/wiki/)(Danish) – Supernatural raven
    Valravn,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Reanimated corpse that feeds on blood
    Vampire,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Human-ape hybrid
    Vanara,
    /// [](https://en.wikipedia.org/wiki/)(Romanian) – Female weather spirit
    Vantoase,
    /// [](https://en.wikipedia.org/wiki/)(Hindu mythology) – Third Avatar of Vishnu in the form of a boar
    Varaha,
    /// [](https://en.wikipedia.org/wiki/)(Romanian) – Vampire or werewolf
    Varcolac,
    /// [](https://en.wikipedia.org/wiki/)(Scandinavian) – Ghostly double
    Vardoger,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Hawk sitting between the eyes of an eagle in the crown of the World Tree Yggdrasil
    Vedrfolnir,
    /// [](https://en.wikipedia.org/wiki/)(Latvian) – Ghost, shade, formed after a death of a human
    Veli,
    /// Chuvash dragon
    VeriSelen,
    /// [](https://en.wikipedia.org/wiki/)(Hindu) – Corpses possessed by vampiric spirits
    Vetala,
    /// [](https://en.wikipedia.org/wiki/)(Catalan) – Dragon with breasts and an eagle's beak
    Víbria,
    /// [](https://en.wikipedia.org/wiki/)(German) – Gluttonous dog-cat-fox hybrid
    Vielfras,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Weather spirit
    Vila,
    /// [](https://en.wikipedia.org/wiki/)(Latvian) – Animalistic, werewolf-like monster
    Vilkacis,
    /// [](https://en.wikipedia.org/wiki/)(Colombian) – Handsome demon
    Virunas,
    /// [](https://en.wikipedia.org/wiki/)(Mayan) – Mystical dragon
    VisionSerpent,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Rooster that sits atop the tree
    Vídopnir,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Male water spirit
    Vodyanoy,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – Undead wolf-human hybrid
    Vrykolakas,
    /// [](https://en.wikipedia.org/wiki/)(Norse) – Nature spirit
    Vaettir,
    /// [](https://en.wikipedia.org/wiki/)(German) – Forest spirit
    Waldgeist,
    /// [](https://en.wikipedia.org/wiki/)(Abenaki) – Water spirits
    WanaGamesAk,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Crocodilian water monster
    Wani,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Demon in the form of a burning human-headed ox cart
    Wanyudo,
    /// [](https://en.wikipedia.org/wiki/)(Indonesian Muslim) – Egg-laying bird
    WarakNgendog,
    /// [](https://en.wikipedia.org/wiki/)(English and Scandinavian O.N. vargr) – Giant, demonic wolf
    Warg,
    /// [](https://en.wikipedia.org/wiki/)(Worldwide) – Male witch
    Warlock,
    /// [](https://en.wikipedia.org/wiki/)(Abenaki) – Aurora spirits
    WassanMonGaneehlaAk,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Water spirit
    WaterMonkey,
    /// [](https://en.wikipedia.org/wiki/)(Alchemy) – Water elemental
    WaterSprite,
    /// [](https://en.wikipedia.org/wiki/)(Australia Aboriginal) – Goanna spirits
    WatiKutjara,
    /// [](https://en.wikipedia.org/wiki/)(Abenaki) – Shapeshifting snail spirit
    WaWonDeeAMegw,
    /// [](https://en.wikipedia.org/wiki/)(German) – Female spirit
    WeisseFrauen,
    /// [](https://en.wikipedia.org/wiki/)(Mapuche) – Demon
    Wekufe,
    /// [](https://en.wikipedia.org/wiki/)(Algonquian) – Anthropophagous spirit
    Wendigo,
    /// [](https://en.wikipedia.org/wiki/)(Inuit) – Water spirit
    Wentshukumishiteu,
    /// [Werecat](https://en.wikipedia.org/wiki/Werecat)(Worldwide) – Feline-human shapeshifter
    Werecat,
    /// [Werehyena](https://en.wikipedia.org/wiki/Werehyena)(Africa) – Hyena-human shapeshifter
    Werehyena,
    /// [Werewolf](https://en.wikipedia.org/wiki/Werewolf)(Worldwide) – Wolf-human shapeshifter
    Werewolf,
    /// [](https://en.wikipedia.org/wiki/)(Worldwide) – Ghost of a murdered or mistreated woman
    WhiteLady,
    /// [](https://en.wikipedia.org/wiki/)(Australian Aboriginal) – Giant frog-headed goanna with six legs
    Whowie,
    /// [](https://en.wikipedia.org/wiki/)(European) – Hairy, bipedal, man-like creature
    WildMan,
    /// [](https://en.wikipedia.org/wiki/)(Worldwide) – Spectral fire
    WillOTheWisp,
    /// [](https://en.wikipedia.org/wiki/)(Scottish) – Malevolent spirit
    WirryCow,
    /// [](https://en.wikipedia.org/wiki/)(Worldwide) – Person who practices magic
    Witch,
    /// [](https://en.wikipedia.org/wiki/)(Dutch) – Female, ancestral spirit
    WitteWieven,
    /// [](https://en.wikipedia.org/wiki/)(German) – Forest animal comprised from various animal parts,/// [](https://en.wikipedia.org/wiki/)(similar to a Chimera)
    Wolpertinger,
    /// [](https://en.wikipedia.org/wiki/)(Australia Aboriginal) – Weather spirit
    Wondjina,
    /// [](https://en.wikipedia.org/wiki/)(Scottish) – Water spirit or ghostly apparition
    Wraith,
    /// [](https://en.wikipedia.org/wiki/)(Scottish) – Wolf-headed humanoid spirit
    Wulver,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Beheaded ghost
    WuTouGui,
    /// English dragon
    Wyrm,
    /// [](https://en.wikipedia.org/wiki/)(Germanic Heraldic) – Flying reptile, usually with two legs and two wings
    Wyvern,
    /// [](https://en.wikipedia.org/wiki/)(Asturian) – Female water spirit
    Xana,
    /// [](https://en.wikipedia.org/wiki/)(Greek)
    Xanthus,
    /// [](https://en.wikipedia.org/wiki/)(Mayan) – Bird
    Xecotcovach,
    /// [](https://en.wikipedia.org/wiki/)(Aztec) – Giant
    Xelhua,
    /// [](https://en.wikipedia.org/wiki/)(mythology), (Chinese) – Ape or four-winged bird
    Xiao,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Headless giant
    XingTian,
    /// [](https://en.wikipedia.org/wiki/)(Aztec) – Drought spirit
    Xiuhcoatl,
    /// [](https://en.wikipedia.org/wiki/)(Albanian) – Elves
    Xhindi,
    /// [](https://en.wikipedia.org/wiki/)(South America) – Sea monster
    Yacumama,
    /// [](https://en.wikipedia.org/wiki/)(Indigenous people of the Amazon) – Mythical water people, with backwards heads and feet
    Yacuruna,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Malevolent, nocturnal spirit
    Yadokai,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Demon who rides through the night on a headless horse
    YagyoSan,
    /// [](https://en.wikipedia.org/wiki/)(Buddhist, Hindu, and Jainism) – Male nature spirit
    Yaksha,
    /// [](https://en.wikipedia.org/wiki/)(Keralite) – Vampire
    Yakshi,
    /// [](https://en.wikipedia.org/wiki/)(Buddhist, Hindu, and Jainism) – Female nature spirit
    Yakshini,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Disease and misfortune spirit
    YakubyoGami,
    /// [](https://en.wikipedia.org/wiki/)(Medieval Bestiaries) – Antelope- or goat-like animal with swiveling horns
    Yale,
    /// [](https://en.wikipedia.org/wiki/)(Tamil) – Lion-like beast
    Yazhi,
    /// [](https://en.wikipedia.org/wiki/)(English) – Nature spirit
    YalleryBrown,
    /// [](https://en.wikipedia.org/wiki/)(Yama,/// [](https://en.wikipedia.org/wiki/)(East Asia)) – Wrathful god
    Yama,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Echo spirit
    YamaBiko,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Savage, mountain-dwelling humanoid
    YamaBito,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Monkey-like mountain spirit
    YamaChichi,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Dog-like mountain spirit
    YamaInu,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Mountain giant
    YamaOtoko,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Gigantic, eight-headed serpent
    YamataNoOrochi,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Malevolent, mountain-dwelling hag
    YamaUba,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Hairy, one-eyed spirit
    YamaWaro,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Spirit which causes strange noises
    Yanari,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Animalistic demon or fallen gods
    Yaoguai,
    /// [](https://en.wikipedia.org/wiki/)(Australian Aboriginal) – Diminutive, sucker-fingered vampire
    YaraMaYhaWho,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Three-legged crow of Amaterasu
    Yatagarasu,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Serpent spirits
    YatoNoKami,
    /// [](https://en.wikipedia.org/wiki/)(English) – Headless dog
    YethHound,
    /// [](https://en.wikipedia.org/wiki/)(Himalayan) – Mountain bigfoot
    Yeti,
    /// [](https://en.wikipedia.org/wiki/)(Turkic) – Either a dragon or a giant
    Yilbegan,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Mountain dwelling spirit
    Yobuko,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Supernatural monster
    Yokai,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Underworld hag
    YomotsuShikome,
    /// Korean dragon
    Yong,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Fairy
    Yosei,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Mysterious bird that sings at night, sometimes indicating that the okuri-inu is near
    Yosuzume,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Wandering ghost
    YouHunYeGui,
    /// [](https://en.wikipedia.org/wiki/)(Australian Aboriginal) – Nocturnal human-ape hybrid, also Yahoo
    Yowie,
    /// [](https://en.wikipedia.org/wiki/)(Heraldic) – Boar-camel-ox-serpent hybrid
    Ypotryll,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Distressed ghost
    YuanGui,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Childlike snow spirit
    Yukinko,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Female snow spirit
    YukiOnna,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Ghost
    Yurei,
    /// [](https://en.wikipedia.org/wiki/)(Tatar) – 100-year-old snake that transforms into a beautiful human
    Yuxa,
    /// [](https://en.wikipedia.org/wiki/)(Persian) – Dragon
    Zahhak,
    /// [](https://en.wikipedia.org/wiki/)(Baltic) – Serpentine fertility spirit
    Zaltys,
    /// [](https://en.wikipedia.org/wiki/)(Jewish) – Giant
    Zamzummim,
    /// [](https://en.wikipedia.org/wiki/)(Albanian) – Mountain fairy who bless warriors
    ZanaEMalit,
    /// [](https://en.wikipedia.org/wiki/)(Romanian) – Nature spirit
    Zână,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – House spirit
    ZashikiWarashi,
    /// [](https://en.wikipedia.org/wiki/)(Romanian) – Wolf-headed dragon
    Zburator,
    /// [](https://en.wikipedia.org/wiki/)(Slavic mythology) – Disembodied, heroic spirit
    Zduhac,
    /// [](https://en.wikipedia.org/wiki/)(Greek) – God of lightning and storms
    Zeus,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Rain-making dragon
    ZennyoRyuo,
    /// [](https://en.wikipedia.org/wiki/)(Slavic) – Glowing bird
    ZharPtitsa,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Pig-headed dragon
    Zhulong,
    /// [](https://en.wikipedia.org/wiki/)(Chinese) – Fire elemental bird
    ZhuQue,
    /// [](https://en.wikipedia.org/wiki/)(Lithuanian) – Forest spirit in the form of a glowing skeleton
    Ziburinis,
    /// [](https://en.wikipedia.org/wiki/)(Tatar) – Flying chicken-legged reptile
    Zilant,
    /// [](https://en.wikipedia.org/wiki/)(West Africa) – Water spirits
    Zin,
    /// [](https://en.wikipedia.org/wiki/)(Jewish) – Giant bird
    Ziz,
    /// [](https://en.wikipedia.org/wiki/)(Slovenia) – White golden-horned deer
    Zlatorog,
    /// [](https://en.wikipedia.org/wiki/)(Romanian folklore) – Giant with a habit of kidnapping young girls
    Zmeu,
    /// Slavic dragon
    Zmiy,
    /// [](https://en.wikipedia.org/wiki/)(Vodou/Worldwide) – Re-animated corpse
    Zombie,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Animated clock
    Zorigami,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Tutelary spirit
    Zuijin,
    /// [](https://en.wikipedia.org/wiki/)(Japanese) – Faceless ghost
    ZunberaBo,
}
impl fmt::Display for Legendary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Legendary::ABaoAQu =>  v = String::from("ABaoAQu"),
            Legendary::Aatxe =>  v = String::from("Aatxe"),
            Legendary::Abaasy =>  v = String::from("Abaasy"),
            Legendary::Abada =>  v = String::from("Abada"),
            Legendary::Abaia =>  v = String::from("Abaia"),
            Legendary::Abarimon =>  v = String::from("Abarimon"),
            Legendary::Abath =>  v = String::from("Abath"),
            Legendary::AburaSumashi =>  v = String::from("AburaSumashi"),
            Legendary::Acephali =>  v = String::from("Acephali"),
            Legendary::Acheri =>  v = String::from("Acheri"),
            Legendary::Achlis =>  v = String::from("Achlis"),
            Legendary::AdarLlwchGwin =>  v = String::from("AdarLlwchGwin"),
            Legendary::Adaro =>  v = String::from("Adaro"),
            Legendary::Adhene =>  v = String::from("Adhene"),
            Legendary::Adlet =>  v = String::from("Adlet"),
            Legendary::Adroanzi =>  v = String::from("Adroanzi"),
            Legendary::Adze =>  v = String::from("Adze"),
            Legendary::Aerico =>  v = String::from("Aerico"),
            Legendary::AEsir =>  v = String::from("AEsir"),
            Legendary::Afanc =>  v = String::from("Afanc"),
            Legendary::Agni =>  v = String::from("Agni"),
            Legendary::Agathodaemon =>  v = String::from("Agathodaemon"),
            Legendary::Agloolik =>  v = String::from("Agloolik"),
            Legendary::Agogwe =>  v = String::from("Agogwe"),
            Legendary::Ahkiyyini =>  v = String::from("Ahkiyyini"),
            Legendary::Ahuizotl =>  v = String::from("Ahuizotl"),
            Legendary::Ahura =>  v = String::from("Ahura"),
            Legendary::Aigamuxa =>  v = String::from("Aigamuxa"),
            Legendary::Aigikampoi =>  v = String::from("Aigikampoi"),
            Legendary::Airavata =>  v = String::from("Airavata"),
            Legendary::Aitu =>  v = String::from("Aitu"),
            Legendary::Aitvaras =>  v = String::from("Aitvaras"),
            Legendary::Ajatar =>  v = String::from("Ajatar"),
            Legendary::Akateko =>  v = String::from("Akateko"),
            Legendary::Akhlut =>  v = String::from("Akhlut"),
            Legendary::Akka =>  v = String::from("Akka"),
            Legendary::Akki =>  v = String::from("Akki"),
            Legendary::Akkorokamui =>  v = String::from("Akkorokamui"),
            Legendary::Akuma =>  v = String::from("Akuma"),
            Legendary::Akupara =>  v = String::from("Akupara"),
            Legendary::AkurojinNoHi =>  v = String::from("AkurojinNoHi"),
            Legendary::Al =>  v = String::from("Al"),
            Legendary::Ala =>  v = String::from("Ala"),
            Legendary::Alal =>  v = String::from("Alal"),
            Legendary::Alan =>  v = String::from("Alan"),
            Legendary::Alce =>  v = String::from("Alce"),
            Legendary::Aleya =>  v = String::from("Aleya"),
            Legendary::Alicanto =>  v = String::from("Alicanto"),
            Legendary::Alicorn =>  v = String::from("Alicorn"),
            Legendary::Alkonost =>  v = String::from("Alkonost"),
            Legendary::Allocamelus =>  v = String::from("Allocamelus"),
            Legendary::Almas =>  v = String::from("Almas"),
            Legendary::AlMiRaj =>  v = String::from("AlMiRaj"),
            Legendary::Aloja =>  v = String::from("Aloja"),
            Legendary::AlomBagWinnosis =>  v = String::from("AlomBagWinnosis"),
            Legendary::Alp =>  v = String::from("Alp"),
            Legendary::Alphyn =>  v = String::from("Alphyn"),
            Legendary::AlpLuachra =>  v = String::from("AlpLuachra"),
            Legendary::AlRakim =>  v = String::from("AlRakim"),
            Legendary::Alseid =>  v = String::from("Alseid"),
            Legendary::Alu =>  v = String::from("Alu"),
            Legendary::Alux =>  v = String::from("Alux"),
            Legendary::Amaburakosagi =>  v = String::from("Amaburakosagi"),
            Legendary::Amala =>  v = String::from("Amala"),
            Legendary::Amamehagi =>  v = String::from("Amamehagi"),
            Legendary::Amanojaku =>  v = String::from("Amanojaku"),
            Legendary::Amarok =>  v = String::from("Amarok"),
            Legendary::Amarum =>  v = String::from("Amarum"),
            Legendary::AmazakeBabaa =>  v = String::from("AmazakeBabaa"),
            Legendary::Amemasu =>  v = String::from("Amemasu"),
            Legendary::Ammit =>  v = String::from("Ammit"),
            Legendary::Amoronagu =>  v = String::from("Amoronagu"),
            Legendary::Amphiptere =>  v = String::from("Amphiptere"),
            Legendary::Amphisbaena =>  v = String::from("Amphisbaena"),
            Legendary::Anak =>  v = String::from("Anak"),
            Legendary::Androsphinx =>  v = String::from("Androsphinx"),
            Legendary::Angel =>  v = String::from("Angel"),
            Legendary::Anqa =>  v = String::from("Anqa"),
            Legendary::AniHyuntikwalaski =>  v = String::from("AniHyuntikwalaski"),
            Legendary::Ankou =>  v = String::from("Ankou"),
            Legendary::Anmo =>  v = String::from("Anmo"),
            Legendary::Antaeus =>  v = String::from("Antaeus"),
            Legendary::Anubis =>  v = String::from("Anubis"),
            Legendary::AnteroVipunen =>  v = String::from("AnteroVipunen"),
            Legendary::Anzu =>  v = String::from("Anzu"),
            Legendary::AoAo =>  v = String::from("AoAo"),
            Legendary::Aobozu =>  v = String::from("Aobozu"),
            Legendary::Apkallu =>  v = String::from("Apkallu"),
            Legendary::Apsaras =>  v = String::from("Apsaras"),
            Legendary::Aqrabuamelu =>  v = String::from("Aqrabuamelu"),
            Legendary::ArdatLili =>  v = String::from("ArdatLili"),
            Legendary::ArgusPanoptes =>  v = String::from("ArgusPanoptes"),
            Legendary::ArikuraNoBaba =>  v = String::from("ArikuraNoBaba"),
            Legendary::Arimaspi =>  v = String::from("Arimaspi"),
            Legendary::Arion =>  v = String::from("Arion"),
            Legendary::ArkanSonney =>  v = String::from("ArkanSonney"),
            Legendary::Asag =>  v = String::from("Asag"),
            Legendary::Asakku =>  v = String::from("Asakku"),
            Legendary::Asanbosam =>  v = String::from("Asanbosam"),
            Legendary::Asena =>  v = String::from("Asena"),
            Legendary::ASeneeKiWakw =>  v = String::from("ASeneeKiWakw"),
            Legendary::AshiMagari =>  v = String::from("AshiMagari"),
            Legendary::Asiman =>  v = String::from("Asiman"),
            Legendary::Askefrue =>  v = String::from("Askefrue"),
            Legendary::AskWeeDaEed =>  v = String::from("AskWeeDaEed"),
            Legendary::Asobibi =>  v = String::from("Asobibi"),
            Legendary::Aspidochelone =>  v = String::from("Aspidochelone"),
            Legendary::Asrai =>  v = String::from("Asrai"),
            Legendary::Astomi =>  v = String::from("Astomi"),
            Legendary::Asura =>  v = String::from("Asura"),
            Legendary::Aswang =>  v = String::from("Aswang"),
            Legendary::Atomy =>  v = String::from("Atomy"),
            Legendary::AtoOiKozo =>  v = String::from("AtoOiKozo"),
            Legendary::Atshen =>  v = String::from("Atshen"),
            Legendary::Auloniad =>  v = String::from("Auloniad"),
            Legendary::Avalerion =>  v = String::from("Avalerion"),
            Legendary::AwaHonDo =>  v = String::from("AwaHonDo"),
            Legendary::Axex =>  v = String::from("Axex"),
            Legendary::Ayakashi =>  v = String::from("Ayakashi"),
            Legendary::AyakashiNoAyashibi =>  v = String::from("AyakashiNoAyashibi"),
            Legendary::Aziza =>  v = String::from("Aziza"),
            Legendary::Azukiarai =>  v = String::from("Azukiarai"),
            Legendary::Azukitogi =>  v = String::from("Azukitogi"),
            Legendary::Azukibabaa =>  v = String::from("Azukibabaa"),
            Legendary::Ba =>  v = String::from("Ba"),
            Legendary::BabaYaga =>  v = String::from("BabaYaga"),
            Legendary::Baccoo =>  v = String::from("Baccoo"),
            Legendary::Badalisc =>  v = String::from("Badalisc"),
            Legendary::Bagiennik =>  v = String::from("Bagiennik"),
            Legendary::Bahamut =>  v = String::from("Bahamut"),
            Legendary::BaiZe =>  v = String::from("BaiZe"),
            Legendary::BaJiaoGui =>  v = String::from("BaJiaoGui"),
            Legendary::Bak =>  v = String::from("Bak"),
            Legendary::BakeKujira =>  v = String::from("BakeKujira"),
            Legendary::Bakeneko =>  v = String::from("Bakeneko"),
            Legendary::Bakezori =>  v = String::from("Bakezori"),
            Legendary::Bakhtak =>  v = String::from("Bakhtak"),
            Legendary::Baku =>  v = String::from("Baku"),
            Legendary::Bakunawa =>  v = String::from("Bakunawa"),
            Legendary::Balaur =>  v = String::from("Balaur"),
            Legendary::Baloz =>  v = String::from("Baloz"),
            Legendary::Bannik =>  v = String::from("Bannik"),
            Legendary::Banshee =>  v = String::from("Banshee"),
            Legendary::BaobhanSith =>  v = String::from("BaobhanSith"),
            Legendary::Barbegazi =>  v = String::from("Barbegazi"),
            Legendary::Bardha =>  v = String::from("Bardha"),
            Legendary::Bardi =>  v = String::from("Bardi"),
            Legendary::Barghest =>  v = String::from("Barghest"),
            Legendary::BarJuchne =>  v = String::from("BarJuchne"),
            Legendary::BarnacleGeese =>  v = String::from("BarnacleGeese"),
            Legendary::Barong =>  v = String::from("Barong"),
            Legendary::Basajaun =>  v = String::from("Basajaun"),
            Legendary::BasCelik =>  v = String::from("BasCelik"),
            Legendary::Bashe =>  v = String::from("Bashe"),
            Legendary::BasiliscoChilote =>  v = String::from("BasiliscoChilote"),
            Legendary::Basilisk =>  v = String::from("Basilisk"),
            Legendary::Bathala =>  v = String::from("Bathala"),
            Legendary::Batibat =>  v = String::from("Batibat"),
            Legendary::Batsu =>  v = String::from("Batsu"),
            Legendary::Baubas =>  v = String::from("Baubas"),
            Legendary::Baykok =>  v = String::from("Baykok"),
            Legendary::BeastOfBrayRoad =>  v = String::from("BeastOfBrayRoad"),
            Legendary::BeanNighe =>  v = String::from("BeanNighe"),
            Legendary::Behemoth =>  v = String::from("Behemoth"),
            Legendary::Bendigeidfran =>  v = String::from("Bendigeidfran"),
            Legendary::Bennu =>  v = String::from("Bennu"),
            Legendary::Berehynia =>  v = String::from("Berehynia"),
            Legendary::Bergrisar =>  v = String::from("Bergrisar"),
            Legendary::Bergsra =>  v = String::from("Bergsra"),
            Legendary::BestialBeast =>  v = String::from("BestialBeast"),
            Legendary::BetobetoSan =>  v = String::from("BetobetoSan"),
            Legendary::Bhuta =>  v = String::from("Bhuta"),
            Legendary::BiBlouk =>  v = String::from("BiBlouk"),
            Legendary::Bies =>  v = String::from("Bies"),
            Legendary::Bigfoot =>  v = String::from("Bigfoot"),
            Legendary::Binbogami =>  v = String::from("Binbogami"),
            Legendary::BishopFish =>  v = String::from("BishopFish"),
            Legendary::BiwaBokuboku =>  v = String::from("BiwaBokuboku"),
            Legendary::BlackAnnis =>  v = String::from("BlackAnnis"),
            Legendary::BlackDog =>  v = String::from("BlackDog"),
            Legendary::BlackShuck =>  v = String::from("BlackShuck"),
            Legendary::Blafard =>  v = String::from("Blafard"),
            Legendary::Blemmyae =>  v = String::from("Blemmyae"),
            Legendary::BloodyBones =>  v = String::from("BloodyBones"),
            Legendary::Bludnik =>  v = String::from("Bludnik"),
            Legendary::BlueCrow =>  v = String::from("BlueCrow"),
            Legendary::Bluecap =>  v = String::from("Bluecap"),
            Legendary::Bodach =>  v = String::from("Bodach"),
            Legendary::Bogeyman =>  v = String::from("Bogeyman"),
            Legendary::Boggart =>  v = String::from("Boggart"),
            Legendary::Boginki =>  v = String::from("Boginki"),
            Legendary::Bogle =>  v = String::from("Bogle"),
            Legendary::BoiTata =>  v = String::from("BoiTata"),
            Legendary::Bolla =>  v = String::from("Bolla"),
            Legendary::Bonnacon =>  v = String::from("Bonnacon"),
            Legendary::BooHag =>  v = String::from("BooHag"),
            Legendary::Boobrie =>  v = String::from("Boobrie"),
            Legendary::Bozaloshtsh =>  v = String::from("Bozaloshtsh"),
            Legendary::Brag =>  v = String::from("Brag"),
            Legendary::Brownie =>  v = String::from("Brownie"),
            Legendary::Broxa =>  v = String::from("Broxa"),
            Legendary::Bucca =>  v = String::from("Bucca"),
            Legendary::Bokkenrijders =>  v = String::from("Bokkenrijders"),
            Legendary::Bugbear =>  v = String::from("Bugbear"),
            Legendary::Buggane =>  v = String::from("Buggane"),
            Legendary::BugulNoz =>  v = String::from("BugulNoz"),
            Legendary::Bukavac =>  v = String::from("Bukavac"),
            Legendary::Bunyip =>  v = String::from("Bunyip"),
            Legendary::BunnyMan =>  v = String::from("BunnyMan"),
            Legendary::BushDaiDai =>  v = String::from("BushDaiDai"),
            Legendary::Byangoma =>  v = String::from("Byangoma"),
            Legendary::Bysen =>  v = String::from("Bysen"),
            Legendary::Cabeiri =>  v = String::from("Cabeiri"),
            Legendary::Cacus =>  v = String::from("Cacus"),
            Legendary::Cadejo =>  v = String::from("Cadejo"),
            Legendary::Cailleach =>  v = String::from("Cailleach"),
            Legendary::Caipora =>  v = String::from("Caipora"),
            Legendary::Caladrius =>  v = String::from("Caladrius"),
            Legendary::Calingi =>  v = String::from("Calingi"),
            Legendary::Callitrix =>  v = String::from("Callitrix"),
            Legendary::CalydonianBoar =>  v = String::from("CalydonianBoar"),
            Legendary::Calygreyhound =>  v = String::from("Calygreyhound"),
            Legendary::Camahueto =>  v = String::from("Camahueto"),
            Legendary::Cambion =>  v = String::from("Cambion"),
            Legendary::Campe =>  v = String::from("Campe"),
            Legendary::Camulatz =>  v = String::from("Camulatz"),
            Legendary::Candileja =>  v = String::from("Candileja"),
            Legendary::Canaima =>  v = String::from("Canaima"),
            Legendary::Canotila =>  v = String::from("Canotila"),
            Legendary::Caoineag =>  v = String::from("Caoineag"),
            Legendary::Chapa =>  v = String::from("Chapa"),
            Legendary::Chareng =>  v = String::from("Chareng"),
            Legendary::Capcaun =>  v = String::from("Capcaun"),
            Legendary::Carbuncle =>  v = String::from("Carbuncle"),
            Legendary::Catoblepas =>  v = String::from("Catoblepas"),
            Legendary::CatSidhe =>  v = String::from("CatSidhe"),
            Legendary::Ceasg =>  v = String::from("Ceasg"),
            Legendary::CeffylDwr =>  v = String::from("CeffylDwr"),
            Legendary::Centaur =>  v = String::from("Centaur"),
            Legendary::Centicore =>  v = String::from("Centicore"),
            Legendary::Cerastes =>  v = String::from("Cerastes"),
            Legendary::Cerberus =>  v = String::from("Cerberus"),
            Legendary::Cercopes =>  v = String::from("Cercopes"),
            Legendary::Cericopithicus =>  v = String::from("Cericopithicus"),
            Legendary::CeryneianHind =>  v = String::from("CeryneianHind"),
            Legendary::Cetan =>  v = String::from("Cetan"),
            Legendary::Cetus =>  v = String::from("Cetus"),
            Legendary::Chakora =>  v = String::from("Chakora"),
            Legendary::Chalkydri =>  v = String::from("Chalkydri"),
            Legendary::Chamrosh =>  v = String::from("Chamrosh"),
            Legendary::Chaneque =>  v = String::from("Chaneque"),
            Legendary::Changeling =>  v = String::from("Changeling"),
            Legendary::Charybdis =>  v = String::from("Charybdis"),
            Legendary::Chenoo =>  v = String::from("Chenoo"),
            Legendary::Chepi =>  v = String::from("Chepi"),
            Legendary::Cherufe =>  v = String::from("Cherufe"),
            Legendary::ChevalMallet =>  v = String::from("ChevalMallet"),
            Legendary::ChevalGauvin =>  v = String::from("ChevalGauvin"),
            Legendary::Chibaiskweda =>  v = String::from("Chibaiskweda"),
            Legendary::Chichevache =>  v = String::from("Chichevache"),
            Legendary::Chickcharney =>  v = String::from("Chickcharney"),
            Legendary::Chimaera =>  v = String::from("Chimaera"),
            Legendary::Chindi =>  v = String::from("Chindi"),
            Legendary::Chinthe =>  v = String::from("Chinthe"),
            Legendary::Chitauli =>  v = String::from("Chitauli"),
            Legendary::Chochinobake =>  v = String::from("Chochinobake"),
            Legendary::Chol =>  v = String::from("Chol"),
            Legendary::Chollima =>  v = String::from("Chollima"),
            Legendary::Chonchon =>  v = String::from("Chonchon"),
            Legendary::Choorile =>  v = String::from("Choorile"),
            Legendary::Chromandi =>  v = String::from("Chromandi"),
            Legendary::Chrysaor =>  v = String::from("Chrysaor"),
            Legendary::Chrysomallus =>  v = String::from("Chrysomallus"),
            Legendary::Chukwa =>  v = String::from("Chukwa"),
            Legendary::Chupacabra =>  v = String::from("Chupacabra"),
            Legendary::Churel =>  v = String::from("Churel"),
            Legendary::Ciguapa =>  v = String::from("Ciguapa"),
            Legendary::Cihuateteo =>  v = String::from("Cihuateteo"),
            Legendary::Cikavac =>  v = String::from("Cikavac"),
            Legendary::CinnamonBird =>  v = String::from("CinnamonBird"),
            Legendary::Cipactli =>  v = String::from("Cipactli"),
            Legendary::CireinCroin =>  v = String::from("CireinCroin"),
            Legendary::Coblynau =>  v = String::from("Coblynau"),
            Legendary::Cockatrice =>  v = String::from("Cockatrice"),
            Legendary::Cofgod =>  v = String::from("Cofgod"),
            Legendary::ColchisBull =>  v = String::from("ColchisBull"),
            Legendary::ColoColo =>  v = String::from("ColoColo"),
            Legendary::CorycianNymphs =>  v = String::from("CorycianNymphs"),
            Legendary::CretanBull =>  v = String::from("CretanBull"),
            Legendary::Crinaeae =>  v = String::from("Crinaeae"),
            Legendary::Criosphinx =>  v = String::from("Criosphinx"),
            Legendary::Crocotta =>  v = String::from("Crocotta"),
            Legendary::TheCuBird =>  v = String::from("TheCuBird"),
            Legendary::Cuco =>  v = String::from("Cuco"),
            Legendary::Cucuy =>  v = String::from("Cucuy"),
            Legendary::Cuegle =>  v = String::from("Cuegle"),
            Legendary::Cuelebre =>  v = String::from("Cuelebre"),
            Legendary::Curupira =>  v = String::from("Curupira"),
            Legendary::CuSith =>  v = String::from("CuSith"),
            Legendary::CwnAnnwn =>  v = String::from("CwnAnnwn"),
            Legendary::Cyclops =>  v = String::from("Cyclops"),
            Legendary::Cyhyraeth =>  v = String::from("Cyhyraeth"),
            Legendary::Cynocephalus =>  v = String::from("Cynocephalus"),
            Legendary::Dactyl =>  v = String::from("Dactyl"),
            Legendary::Daemon =>  v = String::from("Daemon"),
            Legendary::Dahu =>  v = String::from("Dahu"),
            Legendary::Daidarabotchi =>  v = String::from("Daidarabotchi"),
            Legendary::Daitengu =>  v = String::from("Daitengu"),
            Legendary::Daitya =>  v = String::from("Daitya"),
            Legendary::Danava =>  v = String::from("Danava"),
            Legendary::Daphnaie =>  v = String::from("Daphnaie"),
            Legendary::DatsueBa =>  v = String::from("DatsueBa"),
            Legendary::DeadSeaApes =>  v = String::from("DeadSeaApes"),
            Legendary::DedMoroz =>  v = String::from("DedMoroz"),
            Legendary::DeerWoman =>  v = String::from("DeerWoman"),
            Legendary::Deity =>  v = String::from("Deity"),
            Legendary::Demigod =>  v = String::from("Demigod"),
            Legendary::Dhampir =>  v = String::from("Dhampir"),
            Legendary::DiaoSiGui =>  v = String::from("DiaoSiGui"),
            Legendary::Dilong =>  v = String::from("Dilong"),
            Legendary::Dip =>  v = String::from("Dip"),
            Legendary::DiPenates =>  v = String::from("DiPenates"),
            Legendary::Dipsa =>  v = String::from("Dipsa"),
            Legendary::Dirawong =>  v = String::from("Dirawong"),
            Legendary::DiSmaUndarJordi =>  v = String::from("DiSmaUndarJordi"),
            Legendary::Diwata =>  v = String::from("Diwata"),
            Legendary::Djall =>  v = String::from("Djall"),
            Legendary::DobharChu =>  v = String::from("DobharChu"),
            Legendary::DoGakwHoWad =>  v = String::from("DoGakwHoWad"),
            Legendary::Dokkaebi =>  v = String::from("Dokkaebi"),
            Legendary::Dokkalfar =>  v = String::from("Dokkalfar"),
            Legendary::Dola =>  v = String::from("Dola"),
            Legendary::Domovoi =>  v = String::from("Domovoi"),
            Legendary::Doppelganger =>  v = String::from("Doppelganger"),
            Legendary::Drac =>  v = String::from("Drac"),
            Legendary::Drakon =>  v = String::from("Drakon"),
            Legendary::Drakaina =>  v = String::from("Drakaina"),
            Legendary::Dragon =>  v = String::from("Dragon"),
            Legendary::DragonTurtle =>  v = String::from("DragonTurtle"),
            Legendary::Drangue =>  v = String::from("Drangue"),
            Legendary::Draugr =>  v = String::from("Draugr"),
            Legendary::Drekavac =>  v = String::from("Drekavac"),
            Legendary::DropBear =>  v = String::from("DropBear"),
            Legendary::Drow =>  v = String::from("Drow"),
            Legendary::Drude =>  v = String::from("Drude"),
            Legendary::Druk =>  v = String::from("Druk"),
            Legendary::Dryad =>  v = String::from("Dryad"),
            Legendary::Duende =>  v = String::from("Duende"),
            Legendary::Duergar =>  v = String::from("Duergar"),
            Legendary::Dullahan =>  v = String::from("Dullahan"),
            Legendary::Duwende =>  v = String::from("Duwende"),
            Legendary::Dvergr =>  v = String::from("Dvergr"),
            Legendary::Dvorovoi =>  v = String::from("Dvorovoi"),
            Legendary::Dwarf =>  v = String::from("Dwarf"),
            Legendary::Dybbuk =>  v = String::from("Dybbuk"),
            Legendary::DzeeDzeeBonDa =>  v = String::from("DzeeDzeeBonDa"),
            Legendary::Dzunukwa =>  v = String::from("Dzunukwa"),
            Legendary::EasterBunny =>  v = String::from("EasterBunny"),
            Legendary::EasterBilby =>  v = String::from("EasterBilby"),
            Legendary::EachUisge =>  v = String::from("EachUisge"),
            Legendary::EagleSpirit =>  v = String::from("EagleSpirit"),
            Legendary::EbuGogo =>  v = String::from("EbuGogo"),
            Legendary::Echidna =>  v = String::from("Echidna"),
            Legendary::Echeneis =>  v = String::from("Echeneis"),
            Legendary::Edimmu =>  v = String::from("Edimmu"),
            Legendary::Egbere =>  v = String::from("Egbere"),
            Legendary::Eikthyrnir =>  v = String::from("Eikthyrnir"),
            Legendary::Einherjar =>  v = String::from("Einherjar"),
            Legendary::Ekek =>  v = String::from("Ekek"),
            Legendary::ElbowWitch =>  v = String::from("ElbowWitch"),
            Legendary::Eldjotnar =>  v = String::from("Eldjotnar"),
            Legendary::Eleionomae =>  v = String::from("Eleionomae"),
            Legendary::Elemental =>  v = String::from("Elemental"),
            Legendary::Elepaio =>  v = String::from("Elepaio"),
            Legendary::Elf =>  v = String::from("Elf"),
            Legendary::Eloko =>  v = String::from("Eloko"),
            Legendary::Emere =>  v = String::from("Emere"),
            Legendary::Emim =>  v = String::from("Emim"),
            Legendary::Empusa =>  v = String::from("Empusa"),
            Legendary::Encantado =>  v = String::from("Encantado"),
            Legendary::EnchantedMoor =>  v = String::from("EnchantedMoor"),
            Legendary::Enfield =>  v = String::from("Enfield"),
            Legendary::Engkanto =>  v = String::from("Engkanto"),
            Legendary::Enko =>  v = String::from("Enko"),
            Legendary::Ent =>  v = String::from("Ent"),
            Legendary::Epimeliad =>  v = String::from("Epimeliad"),
            Legendary::Erchitu =>  v = String::from("Erchitu"),
            Legendary::ErGui =>  v = String::from("ErGui"),
            Legendary::Erinyes =>  v = String::from("Erinyes"),
            Legendary::Erlking =>  v = String::from("Erlking"),
            Legendary::ErymanthianBoar =>  v = String::from("ErymanthianBoar"),
            Legendary::EthiopianPegasus =>  v = String::from("EthiopianPegasus"),
            Legendary::Etiainen =>  v = String::from("Etiainen"),
            Legendary::Ettin =>  v = String::from("Ettin"),
            Legendary::Eurynomos =>  v = String::from("Eurynomos"),
            Legendary::Ewah =>  v = String::from("Ewah"),
            Legendary::Eerinis =>  v = String::from("Eerinis"),
            Legendary::Fachen =>  v = String::from("Fachen"),
            Legendary::Fafnir =>  v = String::from("Fafnir"),
            Legendary::Fairy =>  v = String::from("Fairy"),
            Legendary::Familiar =>  v = String::from("Familiar"),
            Legendary::FarDarrig =>  v = String::from("FarDarrig"),
            Legendary::Farfadet =>  v = String::from("Farfadet"),
            Legendary::Fates =>  v = String::from("Fates"),
            Legendary::Faun =>  v = String::from("Faun"),
            Legendary::FearGorta =>  v = String::from("FearGorta"),
            Legendary::FeatheredSerpent =>  v = String::from("FeatheredSerpent"),
            Legendary::FeiLian =>  v = String::from("FeiLian"),
            Legendary::Fenghuang =>  v = String::from("Fenghuang"),
            Legendary::Fenodyree =>  v = String::from("Fenodyree"),
            Legendary::Fenrir =>  v = String::from("Fenrir"),
            Legendary::Fetch =>  v = String::from("Fetch"),
            Legendary::Fext =>  v = String::from("Fext"),
            Legendary::Finfolk =>  v = String::from("Finfolk"),
            Legendary::FirBolg =>  v = String::from("FirBolg"),
            Legendary::FireBird =>  v = String::from("FireBird"),
            Legendary::Firedrake =>  v = String::from("Firedrake"),
            Legendary::FishMan =>  v = String::from("FishMan"),
            Legendary::FlatwoodsMonster =>  v = String::from("FlatwoodsMonster"),
            Legendary::Fomorian =>  v = String::from("Fomorian"),
            Legendary::ForestBull =>  v = String::from("ForestBull"),
            Legendary::Freybug =>  v = String::from("Freybug"),
            Legendary::Fuath =>  v = String::from("Fuath"),
            Legendary::Fucanglong =>  v = String::from("Fucanglong"),
            Legendary::Funayurei =>  v = String::from("Funayurei"),
            Legendary::FuruUtsubo =>  v = String::from("FuruUtsubo"),
            Legendary::FutakuchiOnna =>  v = String::from("FutakuchiOnna"),
            Legendary::Fylgja =>  v = String::from("Fylgja"),
            Legendary::Gaasyendietha =>  v = String::from("Gaasyendietha"),
            Legendary::Gagana =>  v = String::from("Gagana"),
            Legendary::Gaki =>  v = String::from("Gaki"),
            Legendary::Gallu =>  v = String::from("Gallu"),
            Legendary::Galtzagorriak =>  v = String::from("Galtzagorriak"),
            Legendary::Gamayun =>  v = String::from("Gamayun"),
            Legendary::Gana =>  v = String::from("Gana"),
            Legendary::Gancanagh =>  v = String::from("Gancanagh"),
            Legendary::Gandabherunda =>  v = String::from("Gandabherunda"),
            Legendary::Gandharva =>  v = String::from("Gandharva"),
            Legendary::Gargouille =>  v = String::from("Gargouille"),
            Legendary::Garkain =>  v = String::from("Garkain"),
            Legendary::Garmr =>  v = String::from("Garmr"),
            Legendary::Garuda =>  v = String::from("Garuda"),
            Legendary::Gashadokuro =>  v = String::from("Gashadokuro"),
            Legendary::Gaueko =>  v = String::from("Gaueko"),
            Legendary::Geb =>  v = String::from("Geb"),
            Legendary::Ged =>  v = String::from("Ged"),
            Legendary::Gegenees =>  v = String::from("Gegenees"),
            Legendary::GeniusLoci =>  v = String::from("GeniusLoci"),
            Legendary::German =>  v = String::from("German"),
            Legendary::Geryon =>  v = String::from("Geryon"),
            Legendary::GhillieDhu =>  v = String::from("GhillieDhu"),
            Legendary::Ghost =>  v = String::from("Ghost"),
            Legendary::Ghoul =>  v = String::from("Ghoul"),
            Legendary::Giant =>  v = String::from("Giant"),
            Legendary::GiantAnimal =>  v = String::from("GiantAnimal"),
            Legendary::GichiAnamiEBizhiw =>  v = String::from("GichiAnamiEBizhiw"),
            Legendary::Gidim =>  v = String::from("Gidim"),
            Legendary::Gigantes =>  v = String::from("Gigantes"),
            Legendary::Gigelorum =>  v = String::from("Gigelorum"),
            Legendary::Girtablilu =>  v = String::from("Girtablilu"),
            Legendary::Gjenganger =>  v = String::from("Gjenganger"),
            Legendary::Glaistig =>  v = String::from("Glaistig"),
            Legendary::Glashtyn =>  v = String::from("Glashtyn"),
            Legendary::Gnome =>  v = String::from("Gnome"),
            Legendary::Goblin =>  v = String::from("Goblin"),
            Legendary::Gog =>  v = String::from("Gog"),
            Legendary::GoldDiggingAnt =>  v = String::from("GoldDiggingAnt"),
            Legendary::Golem =>  v = String::from("Golem"),
            Legendary::Gorgades =>  v = String::from("Gorgades"),
            Legendary::Gorgon =>  v = String::from("Gorgon"),
            Legendary::Goryo =>  v = String::from("Goryo"),
            Legendary::Grassman =>  v = String::from("Grassman"),
            Legendary::Gremlin =>  v = String::from("Gremlin"),
            Legendary::Griffin =>  v = String::from("Griffin"),
            Legendary::Grigori =>  v = String::from("Grigori"),
            Legendary::Grim =>  v = String::from("Grim"),
            Legendary::GrimReaper =>  v = String::from("GrimReaper"),
            Legendary::Grindylow =>  v = String::from("Grindylow"),
            Legendary::Gualichu =>  v = String::from("Gualichu"),
            Legendary::GuardianAngel =>  v = String::from("GuardianAngel"),
            Legendary::GudElim =>  v = String::from("GudElim"),
            Legendary::Guhin =>  v = String::from("Guhin"),
            Legendary::GuiPo =>  v = String::from("GuiPo"),
            Legendary::GuiShu =>  v = String::from("GuiShu"),
            Legendary::Gulon =>  v = String::from("Gulon"),
            Legendary::Gumiho =>  v = String::from("Gumiho"),
            Legendary::Gurangatch =>  v = String::from("Gurangatch"),
            Legendary::Gurumapa =>  v = String::from("Gurumapa"),
            Legendary::Gwyllgi =>  v = String::from("Gwyllgi"),
            Legendary::Gwyllion =>  v = String::from("Gwyllion"),
            Legendary::Gyascutus =>  v = String::from("Gyascutus"),
            Legendary::Gytrash =>  v = String::from("Gytrash"),
            Legendary::Gyuki =>  v = String::from("Gyuki"),
            Legendary::Habrok =>  v = String::from("Habrok"),
            Legendary::Hadhayosh =>  v = String::from("Hadhayosh"),
            Legendary::Hades =>  v = String::from("Hades"),
            Legendary::Haetae =>  v = String::from("Haetae"),
            Legendary::Hag =>  v = String::from("Hag"),
            Legendary::Haietlik =>  v = String::from("Haietlik"),
            Legendary::HaiUri =>  v = String::from("HaiUri"),
            Legendary::Hakutaku =>  v = String::from("Hakutaku"),
            Legendary::Hakuturi =>  v = String::from("Hakuturi"),
            Legendary::HalfElf =>  v = String::from("HalfElf"),
            Legendary::Haltija =>  v = String::from("Haltija"),
            Legendary::Hamadryad =>  v = String::from("Hamadryad"),
            Legendary::Hamingja =>  v = String::from("Hamingja"),
            Legendary::Hamsa =>  v = String::from("Hamsa"),
            Legendary::HanauEpe =>  v = String::from("HanauEpe"),
            Legendary::HantuAir =>  v = String::from("HantuAir"),
            Legendary::HantuDemon =>  v = String::from("HantuDemon"),
            Legendary::HantuRaya =>  v = String::from("HantuRaya"),
            Legendary::Harionago =>  v = String::from("Harionago"),
            Legendary::Harpy =>  v = String::from("Harpy"),
            Legendary::Haugbui =>  v = String::from("Haugbui"),
            Legendary::Havsrå =>  v = String::from("Havsrå"),
            Legendary::Helloi =>  v = String::from("Helloi"),
            Legendary::HeadlessHorseman =>  v = String::from("HeadlessHorseman"),
            Legendary::HeadlessMule =>  v = String::from("HeadlessMule"),
            Legendary::Hecatonchires =>  v = String::from("Hecatonchires"),
            Legendary::Heikegani =>  v = String::from("Heikegani"),
            Legendary::Heinzelmannchen =>  v = String::from("Heinzelmannchen"),
            Legendary::Helead =>  v = String::from("Helead"),
            Legendary::Hellhound =>  v = String::from("Hellhound"),
            Legendary::Heracles =>  v = String::from("Heracles"),
            Legendary::Hercinia =>  v = String::from("Hercinia"),
            Legendary::Herensuge =>  v = String::from("Herensuge"),
            Legendary::Hesperides =>  v = String::from("Hesperides"),
            Legendary::Hidebehind =>  v = String::from("Hidebehind"),
            Legendary::Hiderigami =>  v = String::from("Hiderigami"),
            Legendary::Hieracosphinx =>  v = String::from("Hieracosphinx"),
            Legendary::Hihi =>  v = String::from("Hihi"),
            Legendary::Hiisi =>  v = String::from("Hiisi"),
            Legendary::Hippalectryon =>  v = String::from("Hippalectryon"),
            Legendary::Hippocamp =>  v = String::from("Hippocamp"),
            Legendary::Hippogriff =>  v = String::from("Hippogriff"),
            Legendary::Hippopodes =>  v = String::from("Hippopodes"),
            Legendary::Hircocervus =>  v = String::from("Hircocervus"),
            Legendary::Hitodama =>  v = String::from("Hitodama"),
            Legendary::HitotsumeKozo =>  v = String::from("HitotsumeKozo"),
            Legendary::Hob =>  v = String::from("Hob"),
            Legendary::Hobbididance =>  v = String::from("Hobbididance"),
            Legendary::Hobgoblin =>  v = String::from("Hobgoblin"),
            Legendary::Hodag =>  v = String::from("Hodag"),
            Legendary::Hokhokw =>  v = String::from("Hokhokw"),
            Legendary::Hoko =>  v = String::from("Hoko"),
            Legendary::Homa =>  v = String::from("Homa"),
            Legendary::HombreCaiman =>  v = String::from("HombreCaiman"),
            Legendary::HombreGato =>  v = String::from("HombreGato"),
            Legendary::Homunculus =>  v = String::from("Homunculus"),
            Legendary::Hoo =>  v = String::from("Hoo"),
            Legendary::Hoopoe =>  v = String::from("Hoopoe"),
            Legendary::HoopSnake =>  v = String::from("HoopSnake"),
            Legendary::HornedSerpent =>  v = String::from("HornedSerpent"),
            Legendary::Hotoke =>  v = String::from("Hotoke"),
            Legendary::Houri =>  v = String::from("Houri"),
            Legendary::Hraesvelg =>  v = String::from("Hraesvelg"),
            Legendary::Hrímþursar =>  v = String::from("Hrímþursar"),
            Legendary::Huaychivo =>  v = String::from("Huaychivo"),
            Legendary::HuginnAndMuninn =>  v = String::from("HuginnAndMuninn"),
            Legendary::Huldufolk =>  v = String::from("Huldufolk"),
            Legendary::Hulder =>  v = String::from("Hulder"),
            Legendary::HuliJing =>  v = String::from("HuliJing"),
            Legendary::Huma =>  v = String::from("Huma"),
            Legendary::Humbaba =>  v = String::from("Humbaba"),
            Legendary::Hundun =>  v = String::from("Hundun"),
            Legendary::Hupia =>  v = String::from("Hupia"),
            Legendary::Hyakume =>  v = String::from("Hyakume"),
            Legendary::Hydra =>  v = String::from("Hydra"),
            Legendary::Hydros =>  v = String::from("Hydros"),
            Legendary::Hydrus =>  v = String::from("Hydrus"),
            Legendary::Hyosube =>  v = String::from("Hyosube"),
            Legendary::Hypnalis =>  v = String::from("Hypnalis"),
            Legendary::Hudhud =>  v = String::from("Hudhud"),
            Legendary::Ishigaq =>  v = String::from("Ishigaq"),
            Legendary::IslandSatyr =>  v = String::from("IslandSatyr"),
            Legendary::Isonade =>  v = String::from("Isonade"),
            Legendary::IttanMomen =>  v = String::from("IttanMomen"),
            Legendary::IwanaBozu =>  v = String::from("IwanaBozu"),
            Legendary::Jackalope =>  v = String::from("Jackalope"),
            Legendary::JackInIrons =>  v = String::from("JackInIrons"),
            Legendary::JackOLantern =>  v = String::from("JackOLantern"),
            Legendary::Jaculus =>  v = String::from("Jaculus"),
            Legendary::Jasconius =>  v = String::from("Jasconius"),
            Legendary::JasyJaterei =>  v = String::from("JasyJaterei"),
            Legendary::Jatayu =>  v = String::from("Jatayu"),
            Legendary::Jaud =>  v = String::from("Jaud"),
            Legendary::Jenglot =>  v = String::from("Jenglot"),
            Legendary::Jengu =>  v = String::from("Jengu"),
            Legendary::Jentil =>  v = String::from("Jentil"),
            Legendary::Jenu =>  v = String::from("Jenu"),
            Legendary::Jerff =>  v = String::from("Jerff"),
            Legendary::JerseyDevil =>  v = String::from("JerseyDevil"),
            Legendary::Jian =>  v = String::from("Jian"),
            Legendary::Jiangshi =>  v = String::from("Jiangshi"),
            Legendary::Jiaolong =>  v = String::from("Jiaolong"),
            Legendary::Jibakurei =>  v = String::from("Jibakurei"),
            Legendary::Jievaras =>  v = String::from("Jievaras"),
            Legendary::Jikininki =>  v = String::from("Jikininki"),
            Legendary::Jinn =>  v = String::from("Jinn"),
            Legendary::JipijkaM =>  v = String::from("JipijkaM"),
            Legendary::Jiufeng =>  v = String::from("Jiufeng"),
            Legendary::JiuTouNiao =>  v = String::from("JiuTouNiao"),
            Legendary::Jogah =>  v = String::from("Jogah"),
            Legendary::Jormungandr =>  v = String::from("Jormungandr"),
            Legendary::Jorogumo =>  v = String::from("Jorogumo"),
            Legendary::Jotai =>  v = String::from("Jotai"),
            Legendary::Jotunn =>  v = String::from("Jotunn"),
            Legendary::Jujak =>  v = String::from("Jujak"),
            Legendary::Jumbee =>  v = String::from("Jumbee"),
            Legendary::Kabouter =>  v = String::from("Kabouter"),
            Legendary::Kachina =>  v = String::from("Kachina"),
            Legendary::Kahaku =>  v = String::from("Kahaku"),
            Legendary::Kajsa =>  v = String::from("Kajsa"),
            Legendary::Kalakeyas =>  v = String::from("Kalakeyas"),
            Legendary::Kallikantzaroi =>  v = String::from("Kallikantzaroi"),
            Legendary::Kamaitachi =>  v = String::from("Kamaitachi"),
            Legendary::Kamatayan =>  v = String::from("Kamatayan"),
            Legendary::Kami =>  v = String::from("Kami"),
            Legendary::Kamikiri =>  v = String::from("Kamikiri"),
            Legendary::KanbariNyudo =>  v = String::from("KanbariNyudo"),
            Legendary::KanglaSha =>  v = String::from("KanglaSha"),
            Legendary::Kanbo =>  v = String::from("Kanbo"),
            Legendary::Kanedama =>  v = String::from("Kanedama"),
            Legendary::Kappa =>  v = String::from("Kappa"),
            Legendary::Kapre =>  v = String::from("Kapre"),
            Legendary::Karakoncolos =>  v = String::from("Karakoncolos"),
            Legendary::Karakura =>  v = String::from("Karakura"),
            Legendary::KarasuTengu =>  v = String::from("KarasuTengu"),
            Legendary::Karkadann =>  v = String::from("Karkadann"),
            Legendary::Karkinos =>  v = String::from("Karkinos"),
            Legendary::Karura =>  v = String::from("Karura"),
            Legendary::Karzelek =>  v = String::from("Karzelek"),
            Legendary::KasaObake =>  v = String::from("KasaObake"),
            Legendary::Kasha =>  v = String::from("Kasha"),
            Legendary::Kashanbo =>  v = String::from("Kashanbo"),
            Legendary::KatawaGuruma =>  v = String::from("KatawaGuruma"),
            Legendary::KatsuraOtoko =>  v = String::from("KatsuraOtoko"),
            Legendary::Katallan =>  v = String::from("Katallan"),
            Legendary::Kaukas =>  v = String::from("Kaukas"),
            Legendary::KawaUso =>  v = String::from("KawaUso"),
            Legendary::KawaZaru =>  v = String::from("KawaZaru"),
            Legendary::KeLets =>  v = String::from("KeLets"),
            Legendary::Keelut =>  v = String::from("Keelut"),
            Legendary::KeeWakw =>  v = String::from("KeeWakw"),
            Legendary::Kekkai =>  v = String::from("Kekkai"),
            Legendary::Kelpie =>  v = String::from("Kelpie"),
            Legendary::Ker =>  v = String::from("Ker"),
            Legendary::KesaranPasaran =>  v = String::from("KesaranPasaran"),
            Legendary::Keukegen =>  v = String::from("Keukegen"),
            Legendary::Keythong =>  v = String::from("Keythong"),
            Legendary::Khyah =>  v = String::from("Khyah"),
            Legendary::Kigatilik =>  v = String::from("Kigatilik"),
            Legendary::Kholomodumo =>  v = String::from("Kholomodumo"),
            Legendary::Kijimunaa =>  v = String::from("Kijimunaa"),
            Legendary::Kijo =>  v = String::from("Kijo"),
            Legendary::Kikimora =>  v = String::from("Kikimora"),
            Legendary::Killmoulis =>  v = String::from("Killmoulis"),
            Legendary::Kinnara =>  v = String::from("Kinnara"),
            Legendary::KinU =>  v = String::from("KinU"),
            Legendary::Kirin =>  v = String::from("Kirin"),
            Legendary::Kishi =>  v = String::from("Kishi"),
            Legendary::Kitsune =>  v = String::from("Kitsune"),
            Legendary::KitsuneTsuki =>  v = String::from("KitsuneTsuki"),
            Legendary::Kiyohime =>  v = String::from("Kiyohime"),
            Legendary::Klabautermann =>  v = String::from("Klabautermann"),
            Legendary::Knocker =>  v = String::from("Knocker"),
            Legendary::Knucker =>  v = String::from("Knucker"),
            Legendary::Kobalos =>  v = String::from("Kobalos"),
            Legendary::Kobold =>  v = String::from("Kobold"),
            Legendary::Kodama =>  v = String::from("Kodama"),
            Legendary::Kofewalt =>  v = String::from("Kofewalt"),
            Legendary::KoGok =>  v = String::from("KoGok"),
            Legendary::Kokakucho =>  v = String::from("Kokakucho"),
            Legendary::Komainu =>  v = String::from("Komainu"),
            Legendary::KonakiJiji =>  v = String::from("KonakiJiji"),
            Legendary::KonohaTengu =>  v = String::from("KonohaTengu"),
            Legendary::KoroPokGuru =>  v = String::from("KoroPokGuru"),
            Legendary::Korrigan =>  v = String::from("Korrigan"),
            Legendary::Kraken =>  v = String::from("Kraken"),
            Legendary::Krasnoludek =>  v = String::from("Krasnoludek"),
            Legendary::Krasue =>  v = String::from("Krasue"),
            Legendary::Krampus =>  v = String::from("Krampus"),
            Legendary::KuarahyJara =>  v = String::from("KuarahyJara"),
            Legendary::Kubikajiri =>  v = String::from("Kubikajiri"),
            Legendary::KuchisakeOnna =>  v = String::from("KuchisakeOnna"),
            Legendary::KudaGitsune =>  v = String::from("KudaGitsune"),
            Legendary::Kudan =>  v = String::from("Kudan"),
            Legendary::Kui =>  v = String::from("Kui"),
            Legendary::Kukudhi =>  v = String::from("Kukudhi"),
            Legendary::Kukwes =>  v = String::from("Kukwes"),
            Legendary::Kulshedra =>  v = String::from("Kulshedra"),
            Legendary::Kumakatok =>  v = String::from("Kumakatok"),
            Legendary::Kumiho =>  v = String::from("Kumiho"),
            Legendary::Kun =>  v = String::from("Kun"),
            Legendary::Kupua =>  v = String::from("Kupua"),
            Legendary::Kurabokko =>  v = String::from("Kurabokko"),
            Legendary::KurageNoHinotama =>  v = String::from("KurageNoHinotama"),
            Legendary::Kurma =>  v = String::from("Kurma"),
            Legendary::Kurupi =>  v = String::from("Kurupi"),
            Legendary::Kushtaka =>  v = String::from("Kushtaka"),
            Legendary::KyeRyong =>  v = String::from("KyeRyong"),
            Legendary::Kyourinrin =>  v = String::from("Kyourinrin"),
            Legendary::KyubiNoKitsune =>  v = String::from("KyubiNoKitsune"),
            Legendary::Kyuketsuki =>  v = String::from("Kyuketsuki"),
            Legendary::LaBarTu =>  v = String::from("LaBarTu"),
            Legendary::LabbMu =>  v = String::from("LabbMu"),
            Legendary::Ladyidday =>  v = String::from("Ladyidday"),
            Legendary::Ladon =>  v = String::from("Ladon"),
            Legendary::Laelaps =>  v = String::from("Laelaps"),
            Legendary::Laestrygonians =>  v = String::from("Laestrygonians"),
            Legendary::Lakanica =>  v = String::from("Lakanica"),
            Legendary::LakeMonster =>  v = String::from("LakeMonster"),
            Legendary::Lakhey =>  v = String::from("Lakhey"),
            Legendary::LaLlorona =>  v = String::from("LaLlorona"),
            Legendary::Lamassu =>  v = String::from("Lamassu"),
            Legendary::LambtonWorm =>  v = String::from("LambtonWorm"),
            Legendary::Lamia =>  v = String::from("Lamia"),
            Legendary::Lamiak =>  v = String::from("Lamiak"),
            Legendary::LaMojana =>  v = String::from("LaMojana"),
            Legendary::Lampades =>  v = String::from("Lampades"),
            Legendary::Landvaettir =>  v = String::from("Landvaettir"),
            Legendary::Langmeidong =>  v = String::from("Langmeidong"),
            Legendary::Lares =>  v = String::from("Lares"),
            Legendary::LaSayona =>  v = String::from("LaSayona"),
            Legendary::LaTunda =>  v = String::from("LaTunda"),
            Legendary::LavaBear =>  v = String::from("LavaBear"),
            Legendary::LaukuDvasios =>  v = String::from("LaukuDvasios"),
            Legendary::Lauma =>  v = String::from("Lauma"),
            Legendary::Lavellan =>  v = String::from("Lavellan"),
            Legendary::LeananSidhe =>  v = String::from("LeananSidhe"),
            Legendary::Leanashe =>  v = String::from("Leanashe"),
            Legendary::Leimakids =>  v = String::from("Leimakids"),
            Legendary::Leokampoi =>  v = String::from("Leokampoi"),
            Legendary::Leontophone =>  v = String::from("Leontophone"),
            Legendary::Leprechaun =>  v = String::from("Leprechaun"),
            Legendary::Leszi =>  v = String::from("Leszi"),
            Legendary::Leuce =>  v = String::from("Leuce"),
            Legendary::Leucrota =>  v = String::from("Leucrota"),
            Legendary::Leviathan =>  v = String::from("Leviathan"),
            Legendary::Leyak =>  v = String::from("Leyak"),
            Legendary::LibyanAegipanes =>  v = String::from("LibyanAegipanes"),
            Legendary::LibyanSatyr =>  v = String::from("LibyanSatyr"),
            Legendary::Liderc =>  v = String::from("Liderc"),
            Legendary::LightningBird =>  v = String::from("LightningBird"),
            Legendary::Likho =>  v = String::from("Likho"),
            Legendary::Lilin =>  v = String::from("Lilin"),
            Legendary::Lilitu =>  v = String::from("Lilitu"),
            Legendary::Limnades =>  v = String::from("Limnades"),
            Legendary::Lindworm =>  v = String::from("Lindworm"),
            Legendary::Ljosalfar =>  v = String::from("Ljosalfar"),
            Legendary::Ljubi =>  v = String::from("Ljubi"),
            Legendary::LlamhigynYDwr =>  v = String::from("LlamhigynYDwr"),
            Legendary::LochNessMonster =>  v = String::from("LochNessMonster"),
            Legendary::Loki =>  v = String::from("Loki"),
            Legendary::LoLol =>  v = String::from("LoLol"),
            Legendary::Long =>  v = String::from("Long"),
            Legendary::Longana =>  v = String::from("Longana"),
            Legendary::LongMa =>  v = String::from("LongMa"),
            Legendary::Loogaroo =>  v = String::from("Loogaroo"),
            Legendary::LouCarcolh =>  v = String::from("LouCarcolh"),
            Legendary::LoupGarou =>  v = String::from("LoupGarou"),
            Legendary::LovelandFrog =>  v = String::from("LovelandFrog"),
            Legendary::LubberFiend =>  v = String::from("LubberFiend"),
            Legendary::Luduan =>  v = String::from("Luduan"),
            Legendary::Lugat =>  v = String::from("Lugat"),
            Legendary::Luison =>  v = String::from("Luison"),
            Legendary::Lusca =>  v = String::from("Lusca"),
            Legendary::Lutin =>  v = String::from("Lutin"),
            Legendary::Lyngbakr =>  v = String::from("Lyngbakr"),
            Legendary::Lynx =>  v = String::from("Lynx"),
            Legendary::MaaAlused =>  v = String::from("MaaAlused"),
            Legendary::Machlyes =>  v = String::from("Machlyes"),
            Legendary::Macrocephali =>  v = String::from("Macrocephali"),
            Legendary::MadamKoiKoi =>  v = String::from("MadamKoiKoi"),
            Legendary::Madremonte =>  v = String::from("Madremonte"),
            Legendary::Maero =>  v = String::from("Maero"),
            Legendary::Magog =>  v = String::from("Magog"),
            Legendary::MahaPudma =>  v = String::from("MahaPudma"),
            Legendary::Mairu =>  v = String::from("Mairu"),
            Legendary::MajasGari =>  v = String::from("MajasGari"),
            Legendary::Majitu =>  v = String::from("Majitu"),
            Legendary::Makara =>  v = String::from("Makara"),
            Legendary::MakuraGaeshi =>  v = String::from("MakuraGaeshi"),
            Legendary::MalltYNos =>  v = String::from("MalltYNos"),
            Legendary::MamiWata =>  v = String::from("MamiWata"),
            Legendary::Manananggal =>  v = String::from("Manananggal"),
            Legendary::Mandi =>  v = String::from("Mandi"),
            Legendary::Mandrake =>  v = String::from("Mandrake"),
            Legendary::Manes =>  v = String::from("Manes"),
            Legendary::Mannegishi =>  v = String::from("Mannegishi"),
            Legendary::Manticore =>  v = String::from("Manticore"),
            Legendary::Mapinguari =>  v = String::from("Mapinguari"),
            Legendary::Mara =>  v = String::from("Mara"),
            Legendary::Marabbecca =>  v = String::from("Marabbecca"),
            Legendary::Mareikura =>  v = String::from("Mareikura"),
            Legendary::MaresOfDiomedes =>  v = String::from("MaresOfDiomedes"),
            Legendary::Marid =>  v = String::from("Marid"),
            Legendary::Marmennill =>  v = String::from("Marmennill"),
            Legendary::MaroDeives =>  v = String::from("MaroDeives"),
            Legendary::MaskiMonGweZoOs =>  v = String::from("MaskiMonGweZoOs"),
            Legendary::Matagot =>  v = String::from("Matagot"),
            Legendary::Matsya =>  v = String::from("Matsya"),
            Legendary::Mayura =>  v = String::from("Mayura"),
            Legendary::Mazzikin =>  v = String::from("Mazzikin"),
            Legendary::MboiTuI =>  v = String::from("MboiTuI"),
            Legendary::Mbwiri =>  v = String::from("Mbwiri"),
            Legendary::Medusa =>  v = String::from("Medusa"),
            Legendary::MelekTaus =>  v = String::from("MelekTaus"),
            Legendary::Meliae =>  v = String::from("Meliae"),
            Legendary::Melusine =>  v = String::from("Melusine"),
            Legendary::Menehune =>  v = String::from("Menehune"),
            Legendary::Menninkainen =>  v = String::from("Menninkainen"),
            Legendary::Merlion =>  v = String::from("Merlion"),
            Legendary::Mermaid =>  v = String::from("Mermaid"),
            Legendary::Merman =>  v = String::from("Merman"),
            Legendary::Merlin =>  v = String::from("Merlin"),
            Legendary::Merrow =>  v = String::from("Merrow"),
            Legendary::MeteeKolenOl =>  v = String::from("MeteeKolenOl"),
            Legendary::Mimi =>  v = String::from("Mimi"),
            Legendary::MinkaBird =>  v = String::from("MinkaBird"),
            Legendary::Minokawa =>  v = String::from("Minokawa"),
            Legendary::Minotaur =>  v = String::from("Minotaur"),
            Legendary::Mishibizhiw =>  v = String::from("Mishibizhiw"),
            Legendary::MisiGinebig =>  v = String::from("MisiGinebig"),
            Legendary::MisiKinepikw =>  v = String::from("MisiKinepikw"),
            Legendary::Mizuchi =>  v = String::from("Mizuchi"),
            Legendary::Mogwai =>  v = String::from("Mogwai"),
            Legendary::Mohan =>  v = String::from("Mohan"),
            Legendary::MokeleMbembe =>  v = String::from("MokeleMbembe"),
            Legendary::Mokoi =>  v = String::from("Mokoi"),
            Legendary::Mokorea =>  v = String::from("Mokorea"),
            Legendary::Monai =>  v = String::from("Monai"),
            Legendary::Monocerus =>  v = String::from("Monocerus"),
            Legendary::MonoGrande =>  v = String::from("MonoGrande"),
            Legendary::Monopod =>  v = String::from("Monopod"),
            Legendary::MooinjerVeggey =>  v = String::from("MooinjerVeggey"),
            Legendary::Mora =>  v = String::from("Mora"),
            Legendary::Morgens =>  v = String::from("Morgens"),
            Legendary::MorinjiNoOkama =>  v = String::from("MorinjiNoOkama"),
            Legendary::Mormolykeia =>  v = String::from("Mormolykeia"),
            Legendary::Moroi =>  v = String::from("Moroi"),
            Legendary::MossPeople =>  v = String::from("MossPeople"),
            Legendary::Mothman =>  v = String::from("Mothman"),
            Legendary::Mugwump =>  v = String::from("Mugwump"),
            Legendary::Mujina =>  v = String::from("Mujina"),
            Legendary::Muldjewangk =>  v = String::from("Muldjewangk"),
            Legendary::Multo =>  v = String::from("Multo"),
            Legendary::Mummy =>  v = String::from("Mummy"),
            Legendary::MumaPadurii =>  v = String::from("MumaPadurii"),
            Legendary::MungoonGali =>  v = String::from("MungoonGali"),
            Legendary::Muscaliet =>  v = String::from("Muscaliet"),
            Legendary::Muse =>  v = String::from("Muse"),
            Legendary::Mushusshu =>  v = String::from("Mushusshu"),
            Legendary::Musimon =>  v = String::from("Musimon"),
            Legendary::Myling =>  v = String::from("Myling"),
            Legendary::Myrmecoleon =>  v = String::from("Myrmecoleon"),
            Legendary::Nachzehrer =>  v = String::from("Nachzehrer"),
            Legendary::Naga =>  v = String::from("Naga"),
            Legendary::NagaFireballs =>  v = String::from("NagaFireballs"),
            Legendary::Nagual =>  v = String::from("Nagual"),
            Legendary::Naiad =>  v = String::from("Naiad"),
            Legendary::Nakki =>  v = String::from("Nakki"),
            Legendary::Namahage =>  v = String::from("Namahage"),
            Legendary::Namazu =>  v = String::from("Namazu"),
            Legendary::NandoBaba =>  v = String::from("NandoBaba"),
            Legendary::NangTakian =>  v = String::from("NangTakian"),
            Legendary::NanomKeeaPoDa =>  v = String::from("NanomKeeaPoDa"),
            Legendary::Napaeae =>  v = String::from("Napaeae"),
            Legendary::Narasimha =>  v = String::from("Narasimha"),
            Legendary::Narecnitsi =>  v = String::from("Narecnitsi"),
            Legendary::Nariphon =>  v = String::from("Nariphon"),
            Legendary::Nargun =>  v = String::from("Nargun"),
            Legendary::Nasnas =>  v = String::from("Nasnas"),
            Legendary::Nav =>  v = String::from("Nav"),
            Legendary::Nawao =>  v = String::from("Nawao"),
            Legendary::NDamKenoWet =>  v = String::from("NDamKenoWet"),
            Legendary::Neptune =>  v = String::from("Neptune"),
            Legendary::Neck =>  v = String::from("Neck"),
            Legendary::Negret =>  v = String::from("Negret"),
            Legendary::Nekomata =>  v = String::from("Nekomata"),
            Legendary::Nekomusume =>  v = String::from("Nekomusume"),
            Legendary::NemeanLion =>  v = String::from("NemeanLion"),
            Legendary::Nephilim =>  v = String::from("Nephilim"),
            Legendary::Nereid =>  v = String::from("Nereid"),
            Legendary::Ngen =>  v = String::from("Ngen"),
            Legendary::Nguruvilu =>  v = String::from("Nguruvilu"),
            Legendary::Nian =>  v = String::from("Nian"),
            Legendary::Nightmarchers =>  v = String::from("Nightmarchers"),
            Legendary::Nikusui =>  v = String::from("Nikusui"),
            Legendary::Nimerigar =>  v = String::from("Nimerigar"),
            Legendary::Ningyo =>  v = String::from("Ningyo"),
            Legendary::NinkiNanka =>  v = String::from("NinkiNanka"),
            Legendary::Nisse =>  v = String::from("Nisse"),
            Legendary::Niohoggr =>  v = String::from("Niohoggr"),
            Legendary::Nivatakavachas =>  v = String::from("Nivatakavachas"),
            Legendary::Nix =>  v = String::from("Nix"),
            Legendary::Nobusuma =>  v = String::from("Nobusuma"),
            Legendary::Nocnitsa =>  v = String::from("Nocnitsa"),
            Legendary::NopperaBo =>  v = String::from("NopperaBo"),
            Legendary::Nozuchi =>  v = String::from("Nozuchi"),
            Legendary::Nuckelavee =>  v = String::from("Nuckelavee"),
            Legendary::Nue =>  v = String::from("Nue"),
            Legendary::NuGui =>  v = String::from("NuGui"),
            Legendary::Nukekubi =>  v = String::from("Nukekubi"),
            Legendary::NukuMaiTore =>  v = String::from("NukuMaiTore"),
            Legendary::Nuli =>  v = String::from("Nuli"),
            Legendary::Numen =>  v = String::from("Numen"),
            Legendary::Nuno =>  v = String::from("Nuno"),
            Legendary::Nuppeppo =>  v = String::from("Nuppeppo"),
            Legendary::Nurarihyon =>  v = String::from("Nurarihyon"),
            Legendary::NureOnna =>  v = String::from("NureOnna"),
            Legendary::Nurikabe =>  v = String::from("Nurikabe"),
            Legendary::NyamiNyami =>  v = String::from("NyamiNyami"),
            Legendary::Nykstukas =>  v = String::from("Nykstukas"),
            Legendary::Nymph =>  v = String::from("Nymph"),
            Legendary::Obake =>  v = String::from("Obake"),
            Legendary::Obariyon =>  v = String::from("Obariyon"),
            Legendary::Obayifo =>  v = String::from("Obayifo"),
            Legendary::Obia =>  v = String::from("Obia"),
            Legendary::Oceanid =>  v = String::from("Oceanid"),
            Legendary::Odei =>  v = String::from("Odei"),
            Legendary::Odin =>  v = String::from("Odin"),
            Legendary::Odmience =>  v = String::from("Odmience"),
            Legendary::Og =>  v = String::from("Og"),
            Legendary::Ogopogo =>  v = String::from("Ogopogo"),
            Legendary::Ogun =>  v = String::from("Ogun"),
            Legendary::Ogre =>  v = String::from("Ogre"),
            Legendary::Oiwa =>  v = String::from("Oiwa"),
            Legendary::Ojancanu =>  v = String::from("Ojancanu"),
            Legendary::Okiku =>  v = String::from("Okiku"),
            Legendary::Okubi =>  v = String::from("Okubi"),
            Legendary::OkuriInu =>  v = String::from("OkuriInu"),
            Legendary::OleHigue =>  v = String::from("OleHigue"),
            Legendary::Omukade =>  v = String::from("Omukade"),
            Legendary::Oni =>  v = String::from("Oni"),
            Legendary::Onibi =>  v = String::from("Onibi"),
            Legendary::Onmoraki =>  v = String::from("Onmoraki"),
            Legendary::Onocentaur =>  v = String::from("Onocentaur"),
            Legendary::Onoskelis =>  v = String::from("Onoskelis"),
            Legendary::Onryo =>  v = String::from("Onryo"),
            Legendary::Onza =>  v = String::from("Onza"),
            Legendary::OozlumBird =>  v = String::from("OozlumBird"),
            Legendary::Ophiotaurus =>  v = String::from("Ophiotaurus"),
            Legendary::Opinicus =>  v = String::from("Opinicus"),
            Legendary::OrangBunian =>  v = String::from("OrangBunian"),
            Legendary::OrangMinyak =>  v = String::from("OrangMinyak"),
            Legendary::Ordog =>  v = String::from("Ordog"),
            Legendary::Oread =>  v = String::from("Oread"),
            Legendary::Ork =>  v = String::from("Ork"),
            Legendary::Orobas =>  v = String::from("Orobas"),
            Legendary::OrphanBird =>  v = String::from("OrphanBird"),
            Legendary::Orthrus =>  v = String::from("Orthrus"),
            Legendary::Osiris =>  v = String::from("Osiris"),
            Legendary::Oshun =>  v = String::from("Oshun"),
            Legendary::Otso =>  v = String::from("Otso"),
            Legendary::Ouroboros =>  v = String::from("Ouroboros"),
            Legendary::Ovinnik =>  v = String::from("Ovinnik"),
            Legendary::Owlman =>  v = String::from("Owlman"),
            Legendary::PaasselkaDevils =>  v = String::from("PaasselkaDevils"),
            Legendary::Pamola =>  v = String::from("Pamola"),
            Legendary::Panes =>  v = String::from("Panes"),
            Legendary::Pandi =>  v = String::from("Pandi"),
            Legendary::Panis =>  v = String::from("Panis"),
            Legendary::Panlong =>  v = String::from("Panlong"),
            Legendary::Panotti =>  v = String::from("Panotti"),
            Legendary::Panther =>  v = String::from("Panther"),
            Legendary::Parandrus =>  v = String::from("Parandrus"),
            Legendary::Pard =>  v = String::from("Pard"),
            Legendary::Pardalokampoi =>  v = String::from("Pardalokampoi"),
            Legendary::Patagon =>  v = String::from("Patagon"),
            Legendary::Patasola =>  v = String::from("Patasola"),
            Legendary::Patupairehe =>  v = String::from("Patupairehe"),
            Legendary::Pech =>  v = String::from("Pech"),
            Legendary::Pegaeae =>  v = String::from("Pegaeae"),
            Legendary::Pegasus =>  v = String::from("Pegasus"),
            Legendary::Pegacorn =>  v = String::from("Pegacorn"),
            Legendary::Pelesit =>  v = String::from("Pelesit"),
            Legendary::Peluda =>  v = String::from("Peluda"),
            Legendary::Penanggalan =>  v = String::from("Penanggalan"),
            Legendary::Peng =>  v = String::from("Peng"),
            Legendary::Penghou =>  v = String::from("Penghou"),
            Legendary::Peri =>  v = String::from("Peri"),
            Legendary::Peryton =>  v = String::from("Peryton"),
            Legendary::Pesanta =>  v = String::from("Pesanta"),
            Legendary::Peuchen =>  v = String::from("Peuchen"),
            Legendary::PhiTaiHong =>  v = String::from("PhiTaiHong"),
            Legendary::Phoenix =>  v = String::from("Phoenix"),
            Legendary::Piasa =>  v = String::from("Piasa"),
            Legendary::Piatek =>  v = String::from("Piatek"),
            Legendary::PictishBeast =>  v = String::from("PictishBeast"),
            Legendary::Pillan =>  v = String::from("Pillan"),
            Legendary::Plagg =>  v = String::from("Plagg"),
            Legendary::PimSkwaWagenOwad =>  v = String::from("PimSkwaWagenOwad"),
            Legendary::Piru =>  v = String::from("Piru"),
            Legendary::Pishacha =>  v = String::from("Pishacha"),
            Legendary::Pishtaco =>  v = String::from("Pishtaco"),
            Legendary::PitaSkog =>  v = String::from("PitaSkog"),
            Legendary::Pixie =>  v = String::from("Pixie"),
            Legendary::Pixiu =>  v = String::from("Pixiu"),
            Legendary::PiYao =>  v = String::from("PiYao"),
            Legendary::Plakavac =>  v = String::from("Plakavac"),
            Legendary::PokWejeeMen =>  v = String::from("PokWejeeMen"),
            Legendary::Polevik =>  v = String::from("Polevik"),
            Legendary::PolloMaligno =>  v = String::from("PolloMaligno"),
            Legendary::Polong =>  v = String::from("Polong"),
            Legendary::Poltergeist =>  v = String::from("Poltergeist"),
            Legendary::Pombero =>  v = String::from("Pombero"),
            Legendary::Ponaturi =>  v = String::from("Ponaturi"),
            Legendary::Pontianak =>  v = String::from("Pontianak"),
            Legendary::PopeLickMonster =>  v = String::from("PopeLickMonster"),
            Legendary::Poukai =>  v = String::from("Poukai"),
            Legendary::Preta =>  v = String::from("Preta"),
            Legendary::Pricolici =>  v = String::from("Pricolici"),
            Legendary::Psoglav =>  v = String::from("Psoglav"),
            Legendary::Psotnik =>  v = String::from("Psotnik"),
            Legendary::Psychai =>  v = String::from("Psychai"),
            Legendary::Psychopomp =>  v = String::from("Psychopomp"),
            Legendary::Puca =>  v = String::from("Puca"),
            Legendary::Puki =>  v = String::from("Puki"),
            Legendary::Puck =>  v = String::from("Puck"),
            Legendary::Putz =>  v = String::from("Putz"),
            Legendary::Pugot =>  v = String::from("Pugot"),
            Legendary::Puk =>  v = String::from("Puk"),
            Legendary::Pukis =>  v = String::from("Pukis"),
            Legendary::Puckwudgie =>  v = String::from("Puckwudgie"),
            Legendary::Pygmy =>  v = String::from("Pygmy"),
            Legendary::Pyrausta =>  v = String::from("Pyrausta"),
            Legendary::Python =>  v = String::from("Python"),
            Legendary::Qalupalik =>  v = String::from("Qalupalik"),
            Legendary::Qilin =>  v = String::from("Qilin"),
            Legendary::Qiqirn =>  v = String::from("Qiqirn"),
            Legendary::Qliphoth =>  v = String::from("Qliphoth"),
            Legendary::QuestingBeast =>  v = String::from("QuestingBeast"),
            Legendary::Quetzalcoatl =>  v = String::from("Quetzalcoatl"),
            Legendary::Quinotaur =>  v = String::from("Quinotaur"),
            Legendary::Ra =>  v = String::from("Ra"),
            Legendary::Rabisu =>  v = String::from("Rabisu"),
            Legendary::Radande =>  v = String::from("Radande"),
            Legendary::Ragana =>  v = String::from("Ragana"),
            Legendary::Raiju =>  v = String::from("Raiju"),
            Legendary::RainBird =>  v = String::from("RainBird"),
            Legendary::RainbowCrow =>  v = String::from("RainbowCrow"),
            Legendary::RainbowFish =>  v = String::from("RainbowFish"),
            Legendary::RainbowSerpent =>  v = String::from("RainbowSerpent"),
            Legendary::Rakshasa =>  v = String::from("Rakshasa"),
            Legendary::Ramidreju =>  v = String::from("Ramidreju"),
            Legendary::Rarog =>  v = String::from("Rarog"),
            Legendary::RavenMocker =>  v = String::from("RavenMocker"),
            Legendary::RavenSpirit =>  v = String::from("RavenSpirit"),
            Legendary::Ratatoskr =>  v = String::from("Ratatoskr"),
            Legendary::RaystownRay =>  v = String::from("RaystownRay"),
            Legendary::Redcap =>  v = String::from("Redcap"),
            Legendary::ReEm =>  v = String::from("ReEm"),
            Legendary::Reichsadler =>  v = String::from("Reichsadler"),
            Legendary::Rephaite =>  v = String::from("Rephaite"),
            Legendary::ReptilianHumanoid =>  v = String::from("ReptilianHumanoid"),
            Legendary::Revenant =>  v = String::from("Revenant"),
            Legendary::Roc =>  v = String::from("Roc"),
            Legendary::Rokurokubi =>  v = String::from("Rokurokubi"),
            Legendary::Rompo =>  v = String::from("Rompo"),
            Legendary::Rong =>  v = String::from("Rong"),
            Legendary::Rougarou =>  v = String::from("Rougarou"),
            Legendary::Rusalka =>  v = String::from("Rusalka"),
            Legendary::Ryu =>  v = String::from("Ryu"),
            Legendary::Saci =>  v = String::from("Saci"),
            Legendary::Sagari =>  v = String::from("Sagari"),
            Legendary::Sakabashira =>  v = String::from("Sakabashira"),
            Legendary::Salamander =>  v = String::from("Salamander"),
            Legendary::Samebito =>  v = String::from("Samebito"),
            Legendary::Samodiva =>  v = String::from("Samodiva"),
            Legendary::Sampati =>  v = String::from("Sampati"),
            Legendary::Sandman =>  v = String::from("Sandman"),
            Legendary::Sango =>  v = String::from("Sango"),
            Legendary::Santelmo =>  v = String::from("Santelmo"),
            Legendary::SantaClaus =>  v = String::from("SantaClaus"),
            Legendary::Sanziana =>  v = String::from("Sanziana"),
            Legendary::Sarimanok =>  v = String::from("Sarimanok"),
            Legendary::Sarngika =>  v = String::from("Sarngika"),
            Legendary::Sarugami =>  v = String::from("Sarugami"),
            Legendary::Satori =>  v = String::from("Satori"),
            Legendary::Satan =>  v = String::from("Satan"),
            Legendary::Satyr =>  v = String::from("Satyr"),
            Legendary::Satyrus =>  v = String::from("Satyrus"),
            Legendary::SazaeOni =>  v = String::from("SazaeOni"),
            Legendary::Sceadugenga =>  v = String::from("Sceadugenga"),
            Legendary::Scitalis =>  v = String::from("Scitalis"),
            Legendary::ScorpionMan =>  v = String::from("ScorpionMan"),
            Legendary::Scylla =>  v = String::from("Scylla"),
            Legendary::SeaBee =>  v = String::from("SeaBee"),
            Legendary::SeaLion =>  v = String::from("SeaLion"),
            Legendary::SeaMonk =>  v = String::from("SeaMonk"),
            Legendary::SeaMonster =>  v = String::from("SeaMonster"),
            Legendary::SeaSerpent =>  v = String::from("SeaSerpent"),
            Legendary::SeaWyvern =>  v = String::from("SeaWyvern"),
            Legendary::Seko =>  v = String::from("Seko"),
            Legendary::Selkie =>  v = String::from("Selkie"),
            Legendary::SenpokuKanpoku =>  v = String::from("SenpokuKanpoku"),
            Legendary::Seps =>  v = String::from("Seps"),
            Legendary::Serpent =>  v = String::from("Serpent"),
            Legendary::Serpopard =>  v = String::from("Serpopard"),
            Legendary::Shachihoko =>  v = String::from("Shachihoko"),
            Legendary::Shade =>  v = String::from("Shade"),
            Legendary::ShadowPeople =>  v = String::from("ShadowPeople"),
            Legendary::Shahbaz =>  v = String::from("Shahbaz"),
            Legendary::Shaitan =>  v = String::from("Shaitan"),
            Legendary::ShangYang =>  v = String::from("ShangYang"),
            Legendary::Shedim =>  v = String::from("Shedim"),
            Legendary::Shedu =>  v = String::from("Shedu"),
            Legendary::Shellycoat =>  v = String::from("Shellycoat"),
            Legendary::Shen =>  v = String::from("Shen"),
            Legendary::Shenlong =>  v = String::from("Shenlong"),
            Legendary::Shibaten =>  v = String::from("Shibaten"),
            Legendary::Shikigami =>  v = String::from("Shikigami"),
            Legendary::ShikiOji =>  v = String::from("ShikiOji"),
            Legendary::Shikome =>  v = String::from("Shikome"),
            Legendary::Shinigami =>  v = String::from("Shinigami"),
            Legendary::ShiroBozu =>  v = String::from("ShiroBozu"),
            Legendary::Shirouneri =>  v = String::from("Shirouneri"),
            Legendary::Shiryo =>  v = String::from("Shiryo"),
            Legendary::Shisa =>  v = String::from("Shisa"),
            Legendary::Shishi =>  v = String::from("Shishi"),
            Legendary::Shojo =>  v = String::from("Shojo"),
            Legendary::Shokera =>  v = String::from("Shokera"),
            Legendary::Shtriga =>  v = String::from("Shtriga"),
            Legendary::ShuiGui =>  v = String::from("ShuiGui"),
            Legendary::ShugMonkey =>  v = String::from("ShugMonkey"),
            Legendary::Shunoban =>  v = String::from("Shunoban"),
            Legendary::ShutenDoji =>  v = String::from("ShutenDoji"),
            Legendary::Sídhe =>  v = String::from("Sídhe"),
            Legendary::Sigbin =>  v = String::from("Sigbin"),
            Legendary::Sileni =>  v = String::from("Sileni"),
            Legendary::Simargl =>  v = String::from("Simargl"),
            Legendary::Simurgh =>  v = String::from("Simurgh"),
            Legendary::Singa =>  v = String::from("Singa"),
            Legendary::SintHolo =>  v = String::from("SintHolo"),
            Legendary::Siren =>  v = String::from("Siren"),
            Legendary::Sirin =>  v = String::from("Sirin"),
            Legendary::Sirrush =>  v = String::from("Sirrush"),
            Legendary::Sisiutl =>  v = String::from("Sisiutl"),
            Legendary::SiTeCah =>  v = String::from("SiTeCah"),
            Legendary::Sjora =>  v = String::from("Sjora"),
            Legendary::Sjovaettir =>  v = String::from("Sjovaettir"),
            Legendary::SkinWalker =>  v = String::from("SkinWalker"),
            Legendary::Skogsra =>  v = String::from("Skogsra"),
            Legendary::Skoll =>  v = String::from("Skoll"),
            Legendary::Skookum =>  v = String::from("Skookum"),
            Legendary::Skeleton =>  v = String::from("Skeleton"),
            Legendary::Skrzak =>  v = String::from("Skrzak"),
            Legendary::SkyWomen =>  v = String::from("SkyWomen"),
            Legendary::Sleipnir =>  v = String::from("Sleipnir"),
            Legendary::Sluagh =>  v = String::from("Sluagh"),
            Legendary::SodehikiKozo =>  v = String::from("SodehikiKozo"),
            Legendary::Sogenbi =>  v = String::from("Sogenbi"),
            Legendary::Soragami =>  v = String::from("Soragami"),
            Legendary::SorakiGaeshi =>  v = String::from("SorakiGaeshi"),
            Legendary::Sorobanbozu =>  v = String::from("Sorobanbozu"),
            Legendary::Sotangitsune =>  v = String::from("Sotangitsune"),
            Legendary::Soucouyant =>  v = String::from("Soucouyant"),
            Legendary::Spearfinger =>  v = String::from("Spearfinger"),
            Legendary::Spectre =>  v = String::from("Spectre"),
            Legendary::Sphinx =>  v = String::from("Sphinx"),
            Legendary::Spiridus =>  v = String::from("Spiridus"),
            Legendary::Spirit =>  v = String::from("Spirit"),
            Legendary::Spriggan =>  v = String::from("Spriggan"),
            Legendary::Sprite =>  v = String::from("Sprite"),
            Legendary::Squonk =>  v = String::from("Squonk"),
            Legendary::Stihi =>  v = String::from("Stihi"),
            Legendary::Strigoi =>  v = String::from("Strigoi"),
            Legendary::Strix =>  v = String::from("Strix"),
            Legendary::Struthopodes =>  v = String::from("Struthopodes"),
            Legendary::Strzyga =>  v = String::from("Strzyga"),
            Legendary::Stuhac =>  v = String::from("Stuhac"),
            Legendary::StymphalianBird =>  v = String::from("StymphalianBird"),
            Legendary::Suangi =>  v = String::from("Suangi"),
            Legendary::Succubus =>  v = String::from("Succubus"),
            Legendary::Sudice =>  v = String::from("Sudice"),
            Legendary::SunakakeBaba =>  v = String::from("SunakakeBaba"),
            Legendary::Sunekosuri =>  v = String::from("Sunekosuri"),
            Legendary::Surma =>  v = String::from("Surma"),
            Legendary::Suzaku =>  v = String::from("Suzaku"),
            Legendary::Svaoilfari =>  v = String::from("Svaoilfari"),
            Legendary::Svartalfar =>  v = String::from("Svartalfar"),
            Legendary::Swallower =>  v = String::from("Swallower"),
            Legendary::SwanMaiden =>  v = String::from("SwanMaiden"),
            Legendary::Sylph =>  v = String::from("Sylph"),
            Legendary::Sylvan =>  v = String::from("Sylvan"),
            Legendary::Syrbotae =>  v = String::from("Syrbotae"),
            Legendary::Syrictae =>  v = String::from("Syrictae"),
            Legendary::Tachash =>  v = String::from("Tachash"),
            Legendary::Tailypo =>  v = String::from("Tailypo"),
            Legendary::Taimatsumaru =>  v = String::from("Taimatsumaru"),
            Legendary::Takam =>  v = String::from("Takam"),
            Legendary::TakaOnna =>  v = String::from("TakaOnna"),
            Legendary::Talos =>  v = String::from("Talos"),
            Legendary::Tangie =>  v = String::from("Tangie"),
            Legendary::Taniwha =>  v = String::from("Taniwha"),
            Legendary::Tantankororin =>  v = String::from("Tantankororin"),
            Legendary::Tanuki =>  v = String::from("Tanuki"),
            Legendary::TaotaoMona =>  v = String::from("TaotaoMona"),
            Legendary::Taotie =>  v = String::from("Taotie"),
            Legendary::Tapairu =>  v = String::from("Tapairu"),
            Legendary::Tarasque =>  v = String::from("Tarasque"),
            Legendary::Tartalo =>  v = String::from("Tartalo"),
            Legendary::Tartaruchi =>  v = String::from("Tartaruchi"),
            Legendary::TatamiTataki =>  v = String::from("TatamiTataki"),
            Legendary::Tatzelwurm =>  v = String::from("Tatzelwurm"),
            Legendary::Tatsu =>  v = String::from("Tatsu"),
            Legendary::Taurokampoi =>  v = String::from("Taurokampoi"),
            Legendary::Tavara =>  v = String::from("Tavara"),
            Legendary::TejuJagua =>  v = String::from("TejuJagua"),
            Legendary::Tecumbalam =>  v = String::from("Tecumbalam"),
            Legendary::Tengu =>  v = String::from("Tengu"),
            Legendary::Tennin =>  v = String::from("Tennin"),
            Legendary::TeNoMe =>  v = String::from("TeNoMe"),
            Legendary::Tepegoz =>  v = String::from("Tepegoz"),
            Legendary::TerribleMonster =>  v = String::from("TerribleMonster"),
            Legendary::TeumessianFox =>  v = String::from("TeumessianFox"),
            Legendary::Theriocephalus =>  v = String::from("Theriocephalus"),
            Legendary::ThreeLeggedBird =>  v = String::from("ThreeLeggedBird"),
            Legendary::Thunderbird =>  v = String::from("Thunderbird"),
            Legendary::Thor =>  v = String::from("Thor"),
            Legendary::Tiangou =>  v = String::from("Tiangou"),
            Legendary::Tianlong =>  v = String::from("Tianlong"),
            Legendary::Tibicena =>  v = String::from("Tibicena"),
            Legendary::TiddyMun =>  v = String::from("TiddyMun"),
            Legendary::Tigmamanukan =>  v = String::from("Tigmamanukan"),
            Legendary::Tigris =>  v = String::from("Tigris"),
            Legendary::Tikbalang =>  v = String::from("Tikbalang"),
            Legendary::Tikoloshe =>  v = String::from("Tikoloshe"),
            Legendary::Timingila =>  v = String::from("Timingila"),
            Legendary::Tipua =>  v = String::from("Tipua"),
            Legendary::Titan =>  v = String::from("Titan"),
            Legendary::Tiyanak =>  v = String::from("Tiyanak"),
            Legendary::Tizheruk =>  v = String::from("Tizheruk"),
            Legendary::Tlahuelpuchi =>  v = String::from("Tlahuelpuchi"),
            Legendary::TofuKozo =>  v = String::from("TofuKozo"),
            Legendary::ToireNoHanakosan =>  v = String::from("ToireNoHanakosan"),
            Legendary::Tomte =>  v = String::from("Tomte"),
            Legendary::ToothFairy =>  v = String::from("ToothFairy"),
            Legendary::The tradition of leaving a tooth under a pillow for the Tooth Fairy or another fantasy figure to collect is practiced in various countries. =>  v = String::from("The tradition of leaving a tooth under a pillow for the Tooth Fairy or another fantasy figure to collect is practiced in various countries."),
            Legendary::Topielec =>  v = String::from("Topielec"),
            Legendary::Totetsu =>  v = String::from("Totetsu"),
            Legendary::Toyol =>  v = String::from("Toyol"),
            Legendary::Trasgo =>  v = String::from("Trasgo"),
            Legendary::Trauco =>  v = String::from("Trauco"),
            Legendary::Trenti =>  v = String::from("Trenti"),
            Legendary::Trickster =>  v = String::from("Trickster"),
            Legendary::Tripurasura =>  v = String::from("Tripurasura"),
            Legendary::Tritons =>  v = String::from("Tritons"),
            Legendary::Troll =>  v = String::from("Troll"),
            Legendary::Trow =>  v = String::from("Trow"),
            Legendary::TsiNoo =>  v = String::from("TsiNoo"),
            Legendary::Tsuchigumo =>  v = String::from("Tsuchigumo"),
            Legendary::Tsuchinoko =>  v = String::from("Tsuchinoko"),
            Legendary::Tsukumogami =>  v = String::from("Tsukumogami"),
            Legendary::TsulKalu =>  v = String::from("TsulKalu"),
            Legendary::TsuraraOnna =>  v = String::from("TsuraraOnna"),
            Legendary::TsurubeOtoshi =>  v = String::from("TsurubeOtoshi"),
            Legendary::TugarinZmeyevich =>  v = String::from("TugarinZmeyevich"),
            Legendary::TylwythTeg =>  v = String::from("TylwythTeg"),
            Legendary::Tupilaq =>  v = String::from("Tupilaq"),
            Legendary::Turehu =>  v = String::from("Turehu"),
            Legendary::Turst =>  v = String::from("Turst"),
            Legendary::Turul =>  v = String::from("Turul"),
            Legendary::Tyger =>  v = String::from("Tyger"),
            Legendary::Typhon =>  v = String::from("Typhon"),
            Legendary::Tzitzimitl =>  v = String::from("Tzitzimitl"),
            Legendary::Ubume =>  v = String::from("Ubume"),
            Legendary::UchekLangmeidong =>  v = String::from("UchekLangmeidong"),
            Legendary::UmaNoAshi =>  v = String::from("UmaNoAshi"),
            Legendary::Umibozu =>  v = String::from("Umibozu"),
            Legendary::UmiNyobo =>  v = String::from("UmiNyobo"),
            Legendary::Undead =>  v = String::from("Undead"),
            Legendary::UnderwaterPanther =>  v = String::from("UnderwaterPanther"),
            Legendary::Undine =>  v = String::from("Undine"),
            Legendary::Unhcegila =>  v = String::from("Unhcegila"),
            Legendary::Unicorn =>  v = String::from("Unicorn"),
            Legendary::Unktehi =>  v = String::from("Unktehi"),
            Legendary::Unktehila =>  v = String::from("Unktehila"),
            Legendary::Upinis =>  v = String::from("Upinis"),
            Legendary::Urayuli =>  v = String::from("Urayuli"),
            Legendary::Urias =>  v = String::from("Urias"),
            Legendary::Urmahlullu =>  v = String::from("Urmahlullu"),
            Legendary::UshiOni =>  v = String::from("UshiOni"),
            Legendary::Utukku =>  v = String::from("Utukku"),
            Legendary::Uwan =>  v = String::from("Uwan"),
            Legendary::Vadatajs =>  v = String::from("Vadatajs"),
            Legendary::Vahana =>  v = String::from("Vahana"),
            Legendary::Vaibhavi =>  v = String::from("Vaibhavi"),
            Legendary::Valkyrie =>  v = String::from("Valkyrie"),
            Legendary::Valva =>  v = String::from("Valva"),
            Legendary::Valravn =>  v = String::from("Valravn"),
            Legendary::Vampire =>  v = String::from("Vampire"),
            Legendary::Vanara =>  v = String::from("Vanara"),
            Legendary::Vantoase =>  v = String::from("Vantoase"),
            Legendary::Varaha =>  v = String::from("Varaha"),
            Legendary::Varcolac =>  v = String::from("Varcolac"),
            Legendary::Vardoger =>  v = String::from("Vardoger"),
            Legendary::Vedrfolnir =>  v = String::from("Vedrfolnir"),
            Legendary::Veli =>  v = String::from("Veli"),
            Legendary::VeriSelen =>  v = String::from("VeriSelen"),
            Legendary::Vetala =>  v = String::from("Vetala"),
            Legendary::Víbria =>  v = String::from("Víbria"),
            Legendary::Vielfras =>  v = String::from("Vielfras"),
            Legendary::Vila =>  v = String::from("Vila"),
            Legendary::Vilkacis =>  v = String::from("Vilkacis"),
            Legendary::Virunas =>  v = String::from("Virunas"),
            Legendary::VisionSerpent =>  v = String::from("VisionSerpent"),
            Legendary::Vídopnir =>  v = String::from("Vídopnir"),
            Legendary::Vodyanoy =>  v = String::from("Vodyanoy"),
            Legendary::Vrykolakas =>  v = String::from("Vrykolakas"),
            Legendary::Vaettir =>  v = String::from("Vaettir"),
            Legendary::Waldgeist =>  v = String::from("Waldgeist"),
            Legendary::WanaGamesAk =>  v = String::from("WanaGamesAk"),
            Legendary::Wani =>  v = String::from("Wani"),
            Legendary::Wanyudo =>  v = String::from("Wanyudo"),
            Legendary::WarakNgendog =>  v = String::from("WarakNgendog"),
            Legendary::Warg =>  v = String::from("Warg"),
            Legendary::Warlock =>  v = String::from("Warlock"),
            Legendary::WassanMonGaneehlaAk =>  v = String::from("WassanMonGaneehlaAk"),
            Legendary::WaterMonkey =>  v = String::from("WaterMonkey"),
            Legendary::WaterSprite =>  v = String::from("WaterSprite"),
            Legendary::WatiKutjara =>  v = String::from("WatiKutjara"),
            Legendary::WaWonDeeAMegw =>  v = String::from("WaWonDeeAMegw"),
            Legendary::WeisseFrauen =>  v = String::from("WeisseFrauen"),
            Legendary::Wekufe =>  v = String::from("Wekufe"),
            Legendary::Wendigo =>  v = String::from("Wendigo"),
            Legendary::Wentshukumishiteu =>  v = String::from("Wentshukumishiteu"),
            Legendary::Werecat =>  v = String::from("Werecat"),
            Legendary::Werehyena =>  v = String::from("Werehyena"),
            Legendary::Werewolf =>  v = String::from("Werewolf"),
            Legendary::WhiteLady =>  v = String::from("WhiteLady"),
            Legendary::Whowie =>  v = String::from("Whowie"),
            Legendary::WildMan =>  v = String::from("WildMan"),
            Legendary::WillOTheWisp =>  v = String::from("WillOTheWisp"),
            Legendary::WirryCow =>  v = String::from("WirryCow"),
            Legendary::Witch =>  v = String::from("Witch"),
            Legendary::WitteWieven =>  v = String::from("WitteWieven"),
            Legendary::Wolpertinger =>  v = String::from("Wolpertinger"),
            Legendary::Wondjina =>  v = String::from("Wondjina"),
            Legendary::Wraith =>  v = String::from("Wraith"),
            Legendary::Wulver =>  v = String::from("Wulver"),
            Legendary::WuTouGui =>  v = String::from("WuTouGui"),
            Legendary::Wyrm =>  v = String::from("Wyrm"),
            Legendary::Wyvern =>  v = String::from("Wyvern"),
            Legendary::Xana =>  v = String::from("Xana"),
            Legendary::Xanthus =>  v = String::from("Xanthus"),
            Legendary::Xecotcovach =>  v = String::from("Xecotcovach"),
            Legendary::Xelhua =>  v = String::from("Xelhua"),
            Legendary::Xiao =>  v = String::from("Xiao"),
            Legendary::XingTian =>  v = String::from("XingTian"),
            Legendary::Xiuhcoatl =>  v = String::from("Xiuhcoatl"),
            Legendary::Xhindi =>  v = String::from("Xhindi"),
            Legendary::Yacumama =>  v = String::from("Yacumama"),
            Legendary::Yacuruna =>  v = String::from("Yacuruna"),
            Legendary::Yadokai =>  v = String::from("Yadokai"),
            Legendary::YagyoSan =>  v = String::from("YagyoSan"),
            Legendary::Yaksha =>  v = String::from("Yaksha"),
            Legendary::Yakshi =>  v = String::from("Yakshi"),
            Legendary::Yakshini =>  v = String::from("Yakshini"),
            Legendary::YakubyoGami =>  v = String::from("YakubyoGami"),
            Legendary::Yale =>  v = String::from("Yale"),
            Legendary::Yazhi =>  v = String::from("Yazhi"),
            Legendary::YalleryBrown =>  v = String::from("YalleryBrown"),
            Legendary::Yama =>  v = String::from("Yama"),
            Legendary::YamaBiko =>  v = String::from("YamaBiko"),
            Legendary::YamaBito =>  v = String::from("YamaBito"),
            Legendary::YamaChichi =>  v = String::from("YamaChichi"),
            Legendary::YamaInu =>  v = String::from("YamaInu"),
            Legendary::YamaOtoko =>  v = String::from("YamaOtoko"),
            Legendary::YamataNoOrochi =>  v = String::from("YamataNoOrochi"),
            Legendary::YamaUba =>  v = String::from("YamaUba"),
            Legendary::YamaWaro =>  v = String::from("YamaWaro"),
            Legendary::Yanari =>  v = String::from("Yanari"),
            Legendary::Yaoguai =>  v = String::from("Yaoguai"),
            Legendary::YaraMaYhaWho =>  v = String::from("YaraMaYhaWho"),
            Legendary::Yatagarasu =>  v = String::from("Yatagarasu"),
            Legendary::YatoNoKami =>  v = String::from("YatoNoKami"),
            Legendary::YethHound =>  v = String::from("YethHound"),
            Legendary::Yeti =>  v = String::from("Yeti"),
            Legendary::Yilbegan =>  v = String::from("Yilbegan"),
            Legendary::Yobuko =>  v = String::from("Yobuko"),
            Legendary::Yokai =>  v = String::from("Yokai"),
            Legendary::YomotsuShikome =>  v = String::from("YomotsuShikome"),
            Legendary::Yong =>  v = String::from("Yong"),
            Legendary::Yosei =>  v = String::from("Yosei"),
            Legendary::Yosuzume =>  v = String::from("Yosuzume"),
            Legendary::YouHunYeGui =>  v = String::from("YouHunYeGui"),
            Legendary::Yowie =>  v = String::from("Yowie"),
            Legendary::Ypotryll =>  v = String::from("Ypotryll"),
            Legendary::YuanGui =>  v = String::from("YuanGui"),
            Legendary::Yukinko =>  v = String::from("Yukinko"),
            Legendary::YukiOnna =>  v = String::from("YukiOnna"),
            Legendary::Yurei =>  v = String::from("Yurei"),
            Legendary::Yuxa =>  v = String::from("Yuxa"),
            Legendary::Zahhak =>  v = String::from("Zahhak"),
            Legendary::Zaltys =>  v = String::from("Zaltys"),
            Legendary::Zamzummim =>  v = String::from("Zamzummim"),
            Legendary::ZanaEMalit =>  v = String::from("ZanaEMalit"),
            Legendary::Zână =>  v = String::from("Zână"),
            Legendary::ZashikiWarashi =>  v = String::from("ZashikiWarashi"),
            Legendary::Zburator =>  v = String::from("Zburator"),
            Legendary::Zduhac =>  v = String::from("Zduhac"),
            Legendary::Zeus =>  v = String::from("Zeus"),
            Legendary::ZennyoRyuo =>  v = String::from("ZennyoRyuo"),
            Legendary::ZharPtitsa =>  v = String::from("ZharPtitsa"),
            Legendary::Zhulong =>  v = String::from("Zhulong"),
            Legendary::ZhuQue =>  v = String::from("ZhuQue"),
            Legendary::Ziburinis =>  v = String::from("Ziburinis"),
            Legendary::Zilant =>  v = String::from("Zilant"),
            Legendary::Zin =>  v = String::from("Zin"),
            Legendary::Ziz =>  v = String::from("Ziz"),
            Legendary::Zlatorog =>  v = String::from("Zlatorog"),
            Legendary::Zmeu =>  v = String::from("Zmeu"),
            Legendary::Zmiy =>  v = String::from("Zmiy"),
            Legendary::Zombie =>  v = String::from("Zombie"),
            Legendary::Zorigami =>  v = String::from("Zorigami"),
            Legendary::Zuijin =>  v = String::from("Zuijin"),
            Legendary::ZunberaBo =>  v = String::from("ZunberaBo"),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Legendary {
    /// Get a short descriptive string of the `Legendary` creature
    pub fn short_description(&self) -> String {
        let v:String;
        match *self {
            Legendary::ABaoAQu =>  v = String::from("Á Bao A Qu – Entity that lives in the Tower of Victory in Chitor."),
            Legendary::Aatxe =>  v = String::from("Aatxe – Bull spirit."),
            Legendary::Abaasy =>  v = String::from("Abaasy – Iron-toothed demons."),
            Legendary::Abada =>  v = String::from("Äbädä – Forest spirit."),
            Legendary::Abaia =>  v = String::from("Abaia – Huge magical eel."),
            Legendary::Abarimon =>  v = String::from("Abarimon – Savage humanoid with backward feet."),
            Legendary::Abath =>  v = String::from("Abath – One-horned animal."),
            Legendary::AburaSumashi =>  v = String::from("Abura-sumashi – Creature from a mountain pass in Kumamoto Prefecture."),
            Legendary::Acephali =>  v = String::from("Acephali – Headless humanoids."),
            Legendary::Acheri =>  v = String::from("Acheri – Disease-bringing ghost."),
            Legendary::Achlis =>  v = String::from("Achlis – Curious elk."),
            Legendary::AdarLlwchGwin =>  v = String::from("Adar Llwch Gwin – Giant birds that understand human languages."),
            Legendary::Adaro =>  v = String::from("Adaro – Malevolent merfolk."),
            Legendary::Adhene =>  v = String::from("Adhene – Nature spirit."),
            Legendary::Adlet =>  v = String::from("Adlet – Vampiric dog-human hybrid"),
            Legendary::Adroanzi =>  v = String::from("Adroanzi – Nature spirit."),
            Legendary::Adze =>  v = String::from("Adze – African vampiric-forest being."),
            Legendary::Aerico =>  v = String::from("Aerico – Disease demon."),
            Legendary::AEsir =>  v = String::from("AEsir – Norse deities."),
            Legendary::Afanc =>  v = String::from("Afanc."),
            Legendary::Agni =>  v = String::from("Agni – God of fire and sacrifices."),
            Legendary::Agathodaemon =>  v = String::from("Agathodaemon – Spirit of vinefields and grainfields."),
            Legendary::Agloolik =>  v = String::from("Agloolik – Ice spirit that aids hunters and fishermen."),
            Legendary::Agogwe =>  v = String::from("Agogwe – Small, ape-like humanoid."),
            Legendary::Ahkiyyini =>  v = String::from("Ahkiyyini – Animated skeleton that causes shipwrecks."),
            Legendary::Ahuizotl =>  v = String::from("Ahuizotl – Anthropophagous dog-monkey hybrid."),
            Legendary::Ahura =>  v = String::from("Ahura – Zoroastrian spirits."),
            Legendary::Aigamuxa =>  v = String::from("Aigamuxa – Anthropophagous humanoid with eyes in its instep."),
            Legendary::Aigikampoi =>  v = String::from("Aigikampoi – Fish-tailed goat."),
            Legendary::Airavata =>  v = String::from("Airavata – Divine elephant."),
            Legendary::Aitu =>  v = String::from("Aitu – Malevolent spirits or demons."),
            Legendary::Aitvaras =>  v = String::from("Aitvaras – Household spirit."),
            Legendary::Ajatar =>  v = String::from("Ajatar – Dragon/snake female spirit, is said to spread diseases"),
            Legendary::Akateko =>  v = String::from("Akateko – Tree-dwelling monster."),
            Legendary::Akhlut =>  v = String::from("Akhlut – Orca-wolf shapeshifter."),
            Legendary::Akka =>  v = String::from("Akka – Female spirits or minor goddesses."),
            Legendary::Akki =>  v = String::from("Akki – Large, grotesque humanoid."),
            Legendary::Akkorokamui =>  v = String::from("Akkorokamui – Sea monster."),
            Legendary::Akuma =>  v = String::from("Akuma – Evil spirit or devil"),
            Legendary::Akupara =>  v = String::from("Akupara – Giant turtle that supports the world."),
            Legendary::AkurojinNoHi =>  v = String::from("AkurojinNoHi – Ghostly flame which causes disease."),
            Legendary::Al =>  v = String::from("Al – Spirit that steals unborn babies and livers from pregnant women."),
            Legendary::Ala =>  v = String::from("Ala – Bad weather demon."),
            Legendary::Alal =>  v = String::from("Alal – Queen of the full moon."),
            Legendary::Alan =>  v = String::from("Alan – Winged humanoid that steals reproductive waste to make children."),
            Legendary::Alce =>  v = String::from("Alce – Wingless griffin."),
            Legendary::Aleya =>  v = String::from("Aleya – Spirit of a dead fisherman."),
            Legendary::Alicanto =>  v = String::from("Alicanto – Bird that eats gold and silver."),
            Legendary::Alicorn =>  v = String::from("Alicorn – Winged unicorn."),
            Legendary::Alkonost =>  v = String::from("Alkonost – Angelic bird with human head and breasts."),
            Legendary::Allocamelus =>  v = String::from("Allocamelus – Ass-camel hybrid."),
            Legendary::Almas =>  v = String::from("Almas – Savage humanoid."),
            Legendary::AlMiRaj =>  v = String::from("AlMiRaj – One-horned rabbit."),
            Legendary::Aloja =>  v = String::from("Aloja – Female water spirit."),
            Legendary::AlomBagWinnosis =>  v = String::from("AlomBagWinnosis – Little people and tricksters."),
            Legendary::Alp =>  v = String::from("Alp – Male night-demon."),
            Legendary::Alphyn =>  v = String::from("Alphyn – Lion-like creature, sometimes with dragon or goat forelegs."),
            Legendary::AlpLuachra =>  v = String::from("AlpLuachra – Parasitic fairy."),
            Legendary::AlRakim =>  v = String::from("AlRakim – Guard dog of the Seven Sleepers."),
            Legendary::Alseid =>  v = String::from("Alseid – Grove nymph."),
            Legendary::Alu =>  v = String::from("Alu – Leprous demon."),
            Legendary::Alux =>  v = String::from("Alux – Little people."),
            Legendary::Amaburakosagi =>  v = String::from("Amaburakosagi – Ritual disciplinary demon from Shikoku."),
            Legendary::Amala =>  v = String::from("Amala – Giant who holds up the world."),
            Legendary::Amamehagi =>  v = String::from("Amamehagi – Ritual disciplinary demon from Hokuriku."),
            Legendary::Amanojaku =>  v = String::from("Amanojaku – Small demon."),
            Legendary::Amarok =>  v = String::from("Amarok – Giant wolf."),
            Legendary::Amarum =>  v = String::from("Amarum – Water boa spirit."),
            Legendary::AmazakeBabaa =>  v = String::from("AmazakeBabaa – Disease-causing hag."),
            Legendary::Amemasu =>  v = String::from("Amemasu – Lake monster."),
            Legendary::Ammit =>  v = String::from("Ammit – Female demon who was part lion, hippopotamus and crocodile and devoured the souls of the wicked."),
            Legendary::Amoronagu =>  v = String::from("Amoronagu – Tennyo from the island of Amami Ōshima."),
            Legendary::Amphiptere =>  v = String::from("Amphiptere – Winged serpent."),
            Legendary::Amphisbaena =>  v = String::from("Amphisbaena – Serpent with a head at each end."),
            Legendary::Anak =>  v = String::from("Anak – Giant."),
            Legendary::Androsphinx =>  v = String::from("Androsphinx – Human-headed sphinx."),
            Legendary::Angel =>  v = String::from("Angel – Divine beings of Heaven who act as mediators between God and humans; the counterparts of Demons."),
            Legendary::Anqa =>  v = String::from("Anqa – Legendary Huge Satanic Eagle with Human Face. sometimes can resurrect herself like phoenix did."),
            Legendary::AniHyuntikwalaski =>  v = String::from("AniHyuntikwalaski – Lightning spirit."),
            Legendary::Ankou =>  v = String::from("Ankou – Skeletal grave watcher with a lantern and scythe."),
            Legendary::Anmo =>  v = String::from("Anmo – Ritual disciplinary demon from Iwate Prefecture."),
            Legendary::Antaeus =>  v = String::from("Antaeus – Giant who was extremely strong as long as he remained in contact with the ground."),
            Legendary::Anubis =>  v = String::from("Anubis – God of the Underworld"),
            Legendary::AnteroVipunen =>  v = String::from("AnteroVipunen – Subterranean giant."),
            Legendary::Anzu =>  v = String::from("Anzu – Divine storm bird"),
            Legendary::AoAo =>  v = String::from("AoAo – Anthropophagous peccary or sheep."),
            Legendary::Aobozu =>  v = String::from("Aobozu – Blue monk who kidnaps children."),
            Legendary::Apkallu =>  v = String::from("Apkallu – Fish-human hybrid that attends the god Enki."),
            Legendary::Apsaras =>  v = String::from("Apsaras – Female cloud spirit."),
            Legendary::Aqrabuamelu =>  v = String::from("Aqrabuamelu – Human-scorpion hybrid."),
            Legendary::ArdatLili =>  v = String::from("ArdatLili – Disease demon."),
            Legendary::ArgusPanoptes =>  v = String::from("ArgusPanoptes – Hundred-eyed giant."),
            Legendary::ArikuraNoBaba =>  v = String::from("ArikuraNoBaba – Old woman with magical powers."),
            Legendary::Arimaspi =>  v = String::from("Arimaspi – One-eyed humanoid."),
            Legendary::Arion =>  v = String::from("Arion – Swift green-maned talking horse."),
            Legendary::ArkanSonney =>  v = String::from("ArkanSonney – Fairy hedgehog."),
            Legendary::Asag =>  v = String::from("Asag – Hideous rock demon."),
            Legendary::Asakku =>  v = String::from("Asakku – Demon."),
            Legendary::Asanbosam =>  v = String::from("Asanbosam – Iron-toothed vampire."),
            Legendary::Asena =>  v = String::from("Asena – Blue-maned wolf."),
            Legendary::ASeneeKiWakw =>  v = String::from("ASeneeKiWakw – Stone giant."),
            Legendary::AshiMagari =>  v = String::from("AshiMagari – Invisible tendril that impedes movement."),
            Legendary::Asiman =>  v = String::from("Asiman – Vampiric possession spirit."),
            Legendary::Askefrue =>  v = String::from("Askefrue – Female tree spirit."),
            Legendary::AskWeeDaEed =>  v = String::from("AskWeeDaEed – Fire elemental and spectral fire."),
            Legendary::Asobibi =>  v = String::from("Asobibi – Spectral fire from Kōchi Prefecture."),
            Legendary::Aspidochelone =>  v = String::from("Aspidochelone – Island-sized whale or sea turtle."),
            Legendary::Asrai =>  v = String::from("Asrai – Water spirit."),
            Legendary::Astomi =>  v = String::from("Astomi – Humanoid sustained by pleasant smells instead of food."),
            Legendary::Asura =>  v = String::from("Asura – Hindu malevolent divinities."),
            Legendary::Aswang =>  v = String::from("Aswang – Carrion-eating humanoid."),
            Legendary::Atomy =>  v = String::from("Atomy – Surprisingly small creature."),
            Legendary::AtoOiKozo =>  v = String::from("AtoOiKozo – Invisible spirit that follows people."),
            Legendary::Atshen =>  v = String::from("Atshen – Anthropophagous spirit."),
            Legendary::Auloniad =>  v = String::from("Auloniad – Pasture nymph."),
            Legendary::Avalerion =>  v = String::from("Avalerion – King of the birds."),
            Legendary::AwaHonDo =>  v = String::from("AwaHonDo – Insect spirit."),
            Legendary::Axex =>  v = String::from("Axex – Falcon-lion hybrid."),
            Legendary::Ayakashi =>  v = String::from("Ayakashi – Sea serpent that travels over boats in an arc while dripping oil."),
            Legendary::AyakashiNoAyashibi =>  v = String::from("AyakashiNoAyashibi – Spectral fire from Ishikawa Prefecture."),
            Legendary::Aziza =>  v = String::from("Aziza – Little people that help hunters."),
            Legendary::Azukiarai =>  v = String::from("Azukiarai – Spirit that washes azuki beans along riversides."),
            Legendary::Azukitogi =>  v = String::from("Azukitogi – Spirit that washes azuki beans along riversides."),
            Legendary::Azukibabaa =>  v = String::from("Azukibabaa – Bean-grinding hag who devours people."),
            Legendary::Ba =>  v = String::from("Ba – Soul of the deceased, depicted as a bird or a human-headed bird"),
            Legendary::BabaYaga =>  v = String::from("BabaYaga – Forest spirit and hag"),
            Legendary::Baccoo =>  v = String::from("Baccoo – Malevolent little people"),
            Legendary::Badalisc =>  v = String::from("Badalisc – Goat-like creature from the southern central Alps"),
            Legendary::Bagiennik =>  v = String::from("Bagiennik – Malevolent water spirit"),
            Legendary::Bahamut =>  v = String::from("Bahamut – Giant fish"),
            Legendary::BaiZe =>  v = String::from("BaiZe – Talking beast which handed down knowledge on harmful spirits"),
            Legendary::BaJiaoGui =>  v = String::from("BaJiaoGui – Banana tree spirit"),
            Legendary::Bak =>  v = String::from("Bak - Assamese shape-shifting aqueous creature"),
            Legendary::BakeKujira =>  v = String::from("BakeKujira – Ghostly whale skeleton that drifts along the coastline of Shimane Prefecture"),
            Legendary::Bakeneko =>  v = String::from("Bakeneko – Magical cat"),
            Legendary::Bakezori =>  v = String::from("Bakezori – Animated straw sandal"),
            Legendary::Bakhtak =>  v = String::from("Bakhtak – Night demon"),
            Legendary::Baku =>  v = String::from("Baku – Dream-devouring, tapir-like creature"),
            Legendary::Bakunawa =>  v = String::from("Bakunawa – Sea serpent that causes eclipses"),
            Legendary::Balaur =>  v = String::from("Balaur – Multi-headed dragon"),
            Legendary::Baloz =>  v = String::from("Baloz – Sea monster"),
            Legendary::Bannik =>  v = String::from("Bannik – Bathhouse spirit"),
            Legendary::Banshee =>  v = String::from("Banshee – Screaming death spirit"),
            Legendary::BaobhanSith =>  v = String::from("BaobhanSith – Beautiful vampiric seductresses who prey on young travelers"),
            Legendary::Barbegazi =>  v = String::from("Barbegazi – Dwarf with giant, snowshoe-like feet"),
            Legendary::Bardha =>  v = String::from("Bardha – Mountain spirit"),
            Legendary::Bardi =>  v = String::from("Bardi – Shapechanging death spirit"),
            Legendary::Barghest =>  v = String::from("Yorkshire black dogYorkshire black dog"),
            Legendary::BarJuchne =>  v = String::from("BarJuchne – Gigantic bird"),
            Legendary::BarnacleGeese =>  v = String::from("BarnacleGeese – Geese which hatch from barnacles"),
            Legendary::Barong =>  v = String::from("Barong – Tutelary spirit"),
            Legendary::Basajaun =>  v = String::from("Basajaun – Ancestral, megalith-building race"),
            Legendary::BasCelik =>  v = String::from("BasCelik – Powerful, evil winged man whose soul is not held by his body and can be subdued only by causing him to suffer dehydration"),
            Legendary::Bashe =>  v = String::from("Bashe – Elephant-swallowing serpent"),
            Legendary::BasiliscoChilote =>  v = String::from("BasiliscoChilote – Chicken-serpent hybrid"),
            Legendary::Basilisk =>  v = String::from("Basilisk – Multi-limbed, venomous lizard"),
            Legendary::Bathala =>  v = String::from("Bathala – Primordial god of creation"),
            Legendary::Batibat =>  v = String::from("Batibat – Female night-demon"),
            Legendary::Batsu =>  v = String::from("Batsu – Drought spirit"),
            Legendary::Baubas =>  v = String::from("Baubas – Malevolent spirit"),
            Legendary::Baykok =>  v = String::from("Baykok – Flying skeleton"),
            Legendary::BeastOfBrayRoad =>  v = String::from("BeastOfBrayRoad – Werewolf"),
            Legendary::BeanNighe =>  v = String::from("BeanNighe"),
            Legendary::Behemoth =>  v = String::from("Behemoth – Massive beast, possibly like a dinosaur"),
            Legendary::Bendigeidfran =>  v = String::from("Bendigeidfran – Giant king"),
            Legendary::Bennu =>  v = String::from("Bennu the Phoenix"),
            Legendary::Berehynia =>  v = String::from("Berehynia – Water spirit"),
            Legendary::Bergrisar =>  v = String::from("Bergrisar in Jotunheim"),
            Legendary::Bergsra =>  v = String::from("Bergsra – Mountain spirit"),
            Legendary::BestialBeast =>  v = String::from("BestialBeast – Centauroid specter"),
            Legendary::BetobetoSan =>  v = String::from("BetobetoSan – Invisible spirit which follows people at night, making the sound of footsteps"),
            Legendary::Bhuta =>  v = String::from("Bhuta – Ghost of someone killed by execution or suicide"),
            Legendary::BiBlouk =>  v = String::from("BiBlouk – Female, cannibalistic, partially invisible monster"),
            Legendary::Bies =>  v = String::from("Bies – Demon"),
            Legendary::Bigfoot =>  v = String::from("Bigfoot – Forest-dwelling hominid cryptid."),
            Legendary::Binbogami =>  v = String::from("Binbogami – Spirit of poverty"),
            Legendary::BishopFish =>  v = String::from("BishopFish – Fish-like humanoid"),
            Legendary::BiwaBokuboku =>  v = String::from("BiwaBokuboku – Animated biwa"),
            Legendary::BlackAnnis =>  v = String::from("BlackAnnis – Blue-faced hag"),
            Legendary::BlackDog =>  v = String::from("BlackDog – Canine death spirit"),
            Legendary::BlackShuck =>  v = String::from("Norfolk, Essex, and Suffolk black dogNorfolk, Essex, and Suffolk black dog"),
            Legendary::Blafard =>  v = String::from("Imaginary creature from the early United States of AmericaImaginary creature from the early United States of America"),
            Legendary::Blemmyae =>  v = String::from("Blemmyae – Headless humanoid with face in torso"),
            Legendary::BloodyBones =>  v = String::from("BloodyBones – Water bogeyman"),
            Legendary::Bludnik =>  v = String::from("Bludnik – Mischievous gnome"),
            Legendary::BlueCrow =>  v = String::from("BlueCrow – Giant amazonian bird"),
            Legendary::Bluecap =>  v = String::from("Bluecap – Mine-dwelling fairy"),
            Legendary::Bodach =>  v = String::from("Bodach – Malevolent spirit"),
            Legendary::Bogeyman =>  v = String::from("Bogeyman – Malevolent spirit"),
            Legendary::Boggart =>  v = String::from("Boggart – Malevolent household spirit"),
            Legendary::Boginki =>  v = String::from("Boginki – Nature spirit"),
            Legendary::Bogle =>  v = String::from("Bogle – Malevolent spirit"),
            Legendary::BoiTata =>  v = String::from("BoiTata – Giant snake"),
            Legendary::Bolla =>  v = String::from("Bolla – Dragon"),
            Legendary::Bonnacon =>  v = String::from("Bonnacon – Bull-horse hybrid with flaming dung"),
            Legendary::BooHag =>  v = String::from("BooHag – Vampire-like creature that steals energy from sleeping victims"),
            Legendary::Boobrie =>  v = String::from("Boobrie – Roaring water bird"),
            Legendary::Bozaloshtsh =>  v = String::from("Bozaloshtsh – Death spirit"),
            Legendary::Brag =>  v = String::from("Brag – Malevolent water horse"),
            Legendary::Brownie =>  v = String::from("Brownie – Benevolent household spirit"),
            Legendary::Broxa =>  v = String::from("Broxa – Nocturnal bird that drains goats of their milk"),
            Legendary::Bucca =>  v = String::from("Bucca – Male sea-spirit, a merman, that inhabited mines and coastal communities as a hobgoblin during storms"),
            Legendary::Bokkenrijders =>  v = String::from("Bokkenrijders – Ghosts/devils riding flying goats; co-opted by bandits to instil fear during raids"),
            Legendary::Bugbear =>  v = String::from("Bugbear – Bearlike goblin"),
            Legendary::Buggane =>  v = String::from("Buggane – Ogre-like humanoid"),
            Legendary::BugulNoz =>  v = String::from("BugulNoz – Extremely ugly, but kind, forest spirit"),
            Legendary::Bukavac =>  v = String::from("Bukavac – Six-legged lake monster"),
            Legendary::Bunyip =>  v = String::from("Bunyip – Horse-walrus hybrid lake monster"),
            Legendary::BunnyMan =>  v = String::from("BunnyMan West Virginia Urban Legend – Spirit/Maniac that wears a bunny costume and wields an axe"),
            Legendary::BushDaiDai =>  v = String::from("BushDaiDai – Spirit that seduces and kills men"),
            Legendary::Byangoma =>  v = String::from("Byangoma – Fortune-telling birds"),
            Legendary::Bysen =>  v = String::from("Bysen – Diminutive forest spirit"),
            Legendary::Cabeiri =>  v = String::from("Cabeiri – Smith and wine spirit"),
            Legendary::Cacus =>  v = String::from("Cacus – Fire-breathing giant"),
            Legendary::Cadejo =>  v = String::from("Cadejo – Cow-sized dog-goat hybrid"),
            Legendary::Cailleach =>  v = String::from("Cailleach – Divine creator and weather deity hag"),
            Legendary::Caipora =>  v = String::from("Caipora – Fox-human hybrid and nature spirit"),
            Legendary::Caladrius =>  v = String::from("Caladrius – White bird that can foretell if a sick person will recover or die"),
            Legendary::Calingi =>  v = String::from("Calingi – Humanoid with an eight-year lifespan"),
            Legendary::Callitrix =>  v = String::from("Callitrix – Apes who always bear twins, one the mother loves, the other it hates"),
            Legendary::CalydonianBoar =>  v = String::from("CalydonianBoar – Giant, chthonic boar"),
            Legendary::Calygreyhound =>  v = String::from("Calygreyhound – Wildcat-deer/antelope-eagle-ox-lion hybrid :>"),
            Legendary::Camahueto =>  v = String::from("Camahueto – One-horned calf"),
            Legendary::Cambion =>  v = String::from("Cambion – Offspring of a human and an incubus or succubus"),
            Legendary::Campe =>  v = String::from("Campe – Dragon-human-scorpion hybrid"),
            Legendary::Camulatz =>  v = String::from("Camulatz – Bird that ate the heads of the first men"),
            Legendary::Candileja =>  v = String::from("Candileja – Spectral, fiery hag"),
            Legendary::Canaima =>  v = String::from("Canaima – Were-jaguar"),
            Legendary::Canotila =>  v = String::from("Canotila – Little people and tree spirits"),
            Legendary::Caoineag =>  v = String::from("Caoineag"),
            Legendary::Chapa =>  v = String::from("Chapa – Beaver spirit"),
            Legendary::Chareng =>  v = String::from("///(Manipuri)-Semi-hornbill, semi-human creature-Semi-hornbill, semi-human creature"),
            Legendary::Capcaun =>  v = String::from("Capcaun – Large, monstrous humanoid"),
            Legendary::Carbuncle =>  v = String::from("Carbuncle – Small creature with a jewel on its head"),
            Legendary::Catoblepas =>  v = String::from("Catoblepas – Scaled buffalo-hog hybrid"),
            Legendary::CatSidhe =>  v = String::from("CatSidhe – Fairy cat"),
            Legendary::Ceasg =>  v = String::from("Ceasg — Benevolent Scottish mermaids"),
            Legendary::CeffylDwr =>  v = String::from("CeffylDwr – Malevolent water horse"),
            Legendary::Centaur =>  v = String::from("Centaur – Human-horse hybrid"),
            Legendary::Centicore =>  v = String::from("Centicore – Horse-Antelope-Lion-Bear hybrid"),
            Legendary::Cerastes =>  v = String::from("Cerastes – Extremely flexible, horned snake"),
            Legendary::Cerberus =>  v = String::from("Cerberus – Three-headed dog that guards the entrance to the underworld"),
            Legendary::Cercopes =>  v = String::from("Cercopes – Mischievous forest spirit"),
            Legendary::Cericopithicus =>  v = String::from("Cericopithicus – Apes who always bear twins, one the mother loves, the other it hates"),
            Legendary::CeryneianHind =>  v = String::from("CeryneianHind – Hind with golden antlers and bronze or brass hooves"),
            Legendary::Cetan =>  v = String::from("Cetan – Hawk spirit"),
            Legendary::Cetus =>  v = String::from("Cetus in length, its spines being a cubit in thickness, and its skeleton taller at the shoulder than an elephant."),
            Legendary::Chakora =>  v = String::from("Chakora – Lunar bird"),
            Legendary::Chalkydri =>  v = String::from("Chalkydri – Angelic birds"),
            Legendary::Chamrosh =>  v = String::from("Chamrosh – Dog-bird hybrid"),
            Legendary::Chaneque =>  v = String::from("Chaneque – Little people and nature spirits"),
            Legendary::Changeling =>  v = String::from("Changeling substituted for a kidnapped human child"),
            Legendary::Charybdis =>  v = String::from("Charybdis – Sea monster in the form of a giant mouth"),
            Legendary::Chenoo =>  v = String::from("Chenoo or were possessed by evil spirits, turning their hearts to ice"),
            Legendary::Chepi =>  v = String::from("Chepi – Ancestral spirit that instructs tribe members"),
            Legendary::Cherufe =>  v = String::from("Cherufe – Volcano-dwelling monster"),
            Legendary::ChevalMallet =>  v = String::from("ChevalMallet – Evil horse who runs away with travelers"),
            Legendary::ChevalGauvin =>  v = String::from("ChevalGauvin – Evil horse who drowns riders, similar to kelpie"),
            Legendary::Chibaiskweda =>  v = String::from("Chibaiskweda – Ghost of an improperly buried person"),
            Legendary::Chichevache =>  v = String::from("Human-faced cow that feeds on good womenHuman-faced cow that feeds on good women"),
            Legendary::Chickcharney =>  v = String::from("Chickcharney – Bird-mammal hybrid"),
            Legendary::Chimaera =>  v = String::from("Chimaera – Lion-goat-snake hybrid"),
            Legendary::Chindi =>  v = String::from("Chindi – Vengeful ghost that causes dust devils"),
            Legendary::Chinthe =>  v = String::from("Chinthe – Temple-guarding feline, similar to Chinese Shi and Japanese Shisa"),
            Legendary::Chitauli =>  v = String::from("Chitauli – Human-lizard hybrid"),
            Legendary::Chochinobake =>  v = String::from("Chochinobake – Animated paper lantern"),
            Legendary::Chol =>  v = String::from("Chol – Regenerative bird"),
            Legendary::Chollima =>  v = String::from("Chollima – Supernaturally fast horse"),
            Legendary::Chonchon =>  v = String::from("Chonchon – Disembodied, flying head"),
            Legendary::Choorile =>  v = String::from("Choorile – Ghost of a woman that died in childbirth"),
            Legendary::Chromandi =>  v = String::from("Chromandi – Hairy savage with dog teeth"),
            Legendary::Chrysaor =>  v = String::from("Chrysaor – The giant son of the gorgon Medusa."),
            Legendary::Chrysomallus =>  v = String::from("Chrysomallus – Golden winged ram"),
            Legendary::Chukwa =>  v = String::from("Chukwa – Giant turtle that supports the world"),
            Legendary::Chupacabra =>  v = String::from("Chupacabra – Cryptid beast named for its habit of sucking the blood of livestock"),
            Legendary::Churel =>  v = String::from("Churel – Vampiric, female ghost"),
            Legendary::Ciguapa =>  v = String::from("Ciguapa – Malevolent seductress"),
            Legendary::Cihuateteo =>  v = String::from("Cihuateteo – Ghost of women that died in childbirth"),
            Legendary::Cikavac =>  v = String::from("Cikavac – Bird that serves its owner"),
            Legendary::CinnamonBird =>  v = String::from("CinnamonBird – Giant bird that makes its nest out of cinnamon"),
            Legendary::Cipactli =>  v = String::from("Cipactli – Sea monster, crocodile-fish hybrid"),
            Legendary::CireinCroin =>  v = String::from("CireinCroin – Sea serpent"),
            Legendary::Coblynau =>  v = String::from("Coblynau – Little people and mine spirits"),
            Legendary::Cockatrice =>  v = String::from("Cockatrice – Chicken-lizard hybrid"),
            Legendary::Cofgod =>  v = String::from("Cofgod – Cove god"),
            Legendary::ColchisBull =>  v = String::from("ColchisBull – Bronze-hoofed bulls"),
            Legendary::ColoColo =>  v = String::from("ColoColo – Rat-bird hybrid that can shapeshift into a serpent"),
            Legendary::CorycianNymphs =>  v = String::from("CorycianNymphs – Nymph of the Corycian Cave"),
            Legendary::CretanBull =>  v = String::from("CretanBull – Monstrous bull"),
            Legendary::Crinaeae =>  v = String::from("Crinaeae – Fountain nymph"),
            Legendary::Criosphinx =>  v = String::from("Criosphinx – Ram-headed sphinx"),
            Legendary::Crocotta =>  v = String::from("Crocotta – Monstrous dog-wolf"),
            Legendary::TheCuBird =>  v = String::from("TheCuBird – El Pájaro Cu; a bird."),
            Legendary::Cuco =>  v = String::from("Cuco – Bogeyman"),
            Legendary::Cucuy =>  v = String::from("Cucuy – Malevolent spirit"),
            Legendary::Cuegle =>  v = String::from("Cuegle – Monstrous, three-armed humanoid"),
            Legendary::Cuelebre =>  v = String::from("Cuelebre – Dragon"),
            Legendary::Curupira =>  v = String::from("Curupira – Nature spirit"),
            Legendary::CuSith =>  v = String::from("CuSith – Gigantic fairy dog"),
            Legendary::CwnAnnwn =>  v = String::from("CwnAnnwn – Underworld hunting dog"),
            Legendary::Cyclops =>  v = String::from("Cyclops – One-eyed giant"),
            Legendary::Cyhyraeth =>  v = String::from("Cyhyraeth – Death spirit"),
            Legendary::Cynocephalus =>  v = String::from("Cynocephalus – Dog-headed humanoid"),
            Legendary::Dactyl =>  v = String::from("Dactyl – Little people and smith and healing spirits"),
            Legendary::Daemon =>  v = String::from("Daemon – Incorporeal spirit"),
            Legendary::Dahu =>  v = String::from("Dahu – Similar to a deer or ibex; legs on one side of its body are shorter than on the other side"),
            Legendary::Daidarabotchi =>  v = String::from("Daidarabotchi – Giant responsible for creating many geographical features in Japan"),
            Legendary::Daitengu =>  v = String::from("Daitengu – Most powerful class of tengu, each of whom lives on a separate mountain"),
            Legendary::Daitya =>  v = String::from("Daitya – Giant"),
            Legendary::Danava =>  v = String::from("Danava – Water demon"),
            Legendary::Daphnaie =>  v = String::from("Daphnaie – Laurel tree nymph"),
            Legendary::DatsueBa =>  v = String::from("DatsueBa – Old woman who steals clothes from the souls of the dead"),
            Legendary::DeadSeaApes =>  v = String::from("DeadSeaApes – Human tribe turned into apes for ignoring Moses' message"),
            Legendary::DedMoroz =>  v = String::from("DedMoroz – A winter spirit who delivers gifts to children on New Year's Eve"),
            Legendary::DeerWoman =>  v = String::from("DeerWoman – Human-deer hybrid"),
            Legendary::Deity =>  v = String::from("Deity – Preternatural or supernatural possibly immortal being"),
            Legendary::Demigod =>  v = String::from("Demigod – Half human, half god"),
            Legendary::Dhampir =>  v = String::from("Dhampir – Human/vampire hybrid"),
            Legendary::DiaoSiGui =>  v = String::from("DiaoSiGui – Hanged ghost"),
            Legendary::Dilong =>  v = String::from("Dilong – Earth dragon"),
            Legendary::Dip =>  v = String::from("Dip – Demonic and vampiric dog"),
            Legendary::DiPenates =>  v = String::from("DiPenates – House spirit"),
            Legendary::Dipsa =>  v = String::from("Dipsa – Extremely venomous snake"),
            Legendary::Dirawong =>  v = String::from("Dirawong – Goanna spirit"),
            Legendary::DiSmaUndarJordi =>  v = String::from("DiSmaUndarJordi – Little people and nature spirits"),
            Legendary::Diwata =>  v = String::from("Diwata – Tree spirit"),
            Legendary::Djall =>  v = String::from("Djall – Devil"),
            Legendary::DobharChu =>  v = String::from("DobharChu – King otter"),
            Legendary::DoGakwHoWad =>  v = String::from("DoGakwHoWad – Little people"),
            Legendary::Dokkaebi =>  v = String::from("Dokkaebi – Grotesque, horned humanoids"),
            Legendary::Dokkalfar =>  v = String::from("Dokkalfar – Male ancestral spirits; the Dark Elves"),
            Legendary::Dola =>  v = String::from("Dola – Tutelary and fate spirit"),
            Legendary::Domovoi =>  v = String::from("Domovoi – House spirit"),
            Legendary::Doppelganger =>  v = String::from("Doppelganger – Ghostly double"),
            Legendary::Drac =>  v = String::from("Drac – Winged sea serpent"),
            Legendary::Drakon =>  v = String::from("Drakon – Greek dragons"),
            Legendary::Drakaina =>  v = String::from("Drakaina – Dragons depicted with female characteristics"),
            Legendary::Dragon =>  v = String::from("Dragon winged reptiles"),
            Legendary::DragonTurtle =>  v = String::from("DragonTurtle – Giant turtle with dragon-like head"),
            Legendary::Drangue =>  v = String::from("Drangue – Semi-human winged warriors"),
            Legendary::Draugr =>  v = String::from("Draugr – Undead"),
            Legendary::Drekavac =>  v = String::from("Drekavac – Restless ghost of an unbaptised child"),
            Legendary::DropBear =>  v = String::from("DropBear – Large carnivorous koala that hunts by dropping on its prey from trees"),
            Legendary::Drow =>  v = String::from("Drow – Cavern spirit"),
            Legendary::Drude =>  v = String::from("Drude – Possessing demon"),
            Legendary::Druk =>  v = String::from("Druk – Dragon"),
            Legendary::Dryad =>  v = String::from("Dryad – Tree nymph"),
            Legendary::Duende =>  v = String::from("Duende – Little people and forest spirits"),
            Legendary::Duergar =>  v = String::from("Duergar – Malevolent little people"),
            Legendary::Dullahan =>  v = String::from("Dullahan – Headless death spirit"),
            Legendary::Duwende =>  v = String::from("Duwende – Little people, some are house spirits, others nature spirits"),
            Legendary::Dvergr =>  v = String::from("Dvergr – Subterranean little people smiths"),
            Legendary::Dvorovoi =>  v = String::from("Dvorovoi – Courtyard spirit"),
            Legendary::Dwarf =>  v = String::from("Dwarf – Little people nature spirits"),
            Legendary::Dybbuk =>  v = String::from("Dybbuk that possesses the living"),
            Legendary::DzeeDzeeBonDa =>  v = String::from("DzeeDzeeBonDa – Hideous monster"),
            Legendary::Dzunukwa =>  v = String::from("Dzunukwa – Child-eating hag"),
            Legendary::EasterBunny =>  v = String::from("Easter Bunny – Anthropomorphic lagomorph."),
            Legendary::EasterBilby =>  v = String::from("Easter Bilby – Anthropomorphic bilby."),
            Legendary::EachUisge =>  v = String::from("Each-uisge – Malevolent water horse"),
            Legendary::EagleSpirit =>  v = String::from("EagleSpirit – Leadership or guidance totem"),
            Legendary::EbuGogo =>  v = String::from("EbuGogo – Diminutive humanoids, possibly inspired by Homo floresiensis"),
            Legendary::Echidna =>  v = String::from("Echidna"),
            Legendary::Echeneis =>  v = String::from("Echeneis – Remora, said to attach to ships to slow them down"),
            Legendary::Edimmu =>  v = String::from("Edimmu – Ghosts of those not buried properly"),
            Legendary::Egbere =>  v = String::from("Egbere – Humanoid that carries a magical mat"),
            Legendary::Eikthyrnir =>  v = String::from("Eikthyrnir"),
            Legendary::Einherjar =>  v = String::from("Einherjar – Spirits of brave warriors"),
            Legendary::Ekek =>  v = String::from("Ekek – Flesh-eating, winged humanoids"),
            Legendary::ElbowWitch =>  v = String::from("ElbowWitch – Hags with awls in their elbows"),
            Legendary::Eldjotnar =>  v = String::from("Eldjotnar – Fire Giants who reside in Muspelheim, with Surtr as their leader"),
            Legendary::Eleionomae =>  v = String::from("Eleionomae – Marsh nymph"),
            Legendary::Elemental =>  v = String::from("Elemental – Personification of one of the Classical elements"),
            Legendary::Elepaio =>  v = String::from("Elepaio – Monarch flycatcher spirit that guides canoe-builders to the proper trees"),
            Legendary::Elf =>  v = String::from("Elf – Nature and fertility spirit"),
            Legendary::Eloko =>  v = String::from("Eloko – Little people and malevolent nature spirits"),
            Legendary::Emere =>  v = String::from("Emere – Child that can move back and forth between the material world and the afterlife at will"),
            Legendary::Emim =>  v = String::from("Emim – Giant"),
            Legendary::Empusa =>  v = String::from("Empusa – Female demon that waylays travelers and seduces and kills men"),
            Legendary::Encantado =>  v = String::from("Encantado – Dolphin-human shapeshifter"),
            Legendary::EnchantedMoor =>  v = String::from("EnchantedMoor – Enchanted princesses"),
            Legendary::Enfield =>  v = String::from("Enfield – Fox-greyhound-lion-wolf-eagle hybrid"),
            Legendary::Engkanto =>  v = String::from("Engkanto – Neutral nature spirit"),
            Legendary::Enko =>  v = String::from("Enko – Kappa of Shikoku and western Honshū"),
            Legendary::Ent =>  v = String::from("Ent -Living tree that is said to live for years"),
            Legendary::Epimeliad =>  v = String::from("Epimeliad – Apple tree nymph"),
            Legendary::Erchitu =>  v = String::from("Erchitu – Ox-human, wereox"),
            Legendary::ErGui =>  v = String::from("ErGui – Hungry ghost"),
            Legendary::Erinyes =>  v = String::from("Erinyes – Winged spirits of vengeance or justice, also known as Furies"),
            Legendary::Erlking =>  v = String::from("Erlking – Death spirit"),
            Legendary::ErymanthianBoar =>  v = String::from("ErymanthianBoar – Giant boar"),
            Legendary::EthiopianPegasus =>  v = String::from("EthiopianPegasus – Horned, winged horse"),
            Legendary::Etiainen =>  v = String::from("Etiainen – Spirit being of a living person"),
            Legendary::Ettin =>  v = String::from("Ettin – Three-headed giant"),
            Legendary::Eurynomos =>  v = String::from("Eurynomos – Blue-black, carrion-eater in the underworld"),
            Legendary::Ewah =>  v = String::from("Ewah – Human-cougar hybrid"),
            Legendary::Eerinis =>  v = String::from("Eerinis – Lake spirit"),
            Legendary::Fachen =>  v = String::from("Fachen – Monster with half a body"),
            Legendary::Fafnir =>  v = String::from("Fafnir – Dwarf who was cursed and turned into a dragon. He was later slain by Sigurd in the Saga of Nibelung."),
            Legendary::Fairy =>  v = String::from("Fairy – Nature spirits"),
            Legendary::Familiar =>  v = String::from("Familiar – Animal servant"),
            Legendary::FarDarrig =>  v = String::from("FarDarrig – Little people that constantly play pranks"),
            Legendary::Farfadet =>  v = String::from("Farfadet, wrinkled, and brown-skinned helpful sprites."),
            Legendary::Fates =>  v = String::from("Fates – Three time-controlling sisters"),
            Legendary::Faun =>  v = String::from("Faun – Human-goat hybrid nature spirit"),
            Legendary::FearGorta =>  v = String::from("FearGorta – Hunger ghost"),
            Legendary::FeatheredSerpent =>  v = String::from("Mesoamerican dragonMesoamerican dragon"),
            Legendary::FeiLian =>  v = String::from("FeiLian – Chinese wind god"),
            Legendary::Fenghuang =>  v = String::from("Fenghuang – Chinese Phoenix, female in marriage symbol"),
            Legendary::Fenodyree =>  v = String::from("Fenodyree – House spirit"),
            Legendary::Fenrir =>  v = String::from("Fenrir – Gigantic, ravenous wolf"),
            Legendary::Fetch =>  v = String::from("Fetch – Double or doppelgänger"),
            Legendary::Fext =>  v = String::from("Fext – Undead"),
            Legendary::Finfolk =>  v = String::from("Finfolk – Fish-human hybrid that kidnaps humans for servants"),
            Legendary::FirBolg =>  v = String::from("FirBolg – Ancestral race"),
            Legendary::FireBird =>  v = String::from("FireBird – Regenerative solar bird"),
            Legendary::Firedrake =>  v = String::from("Firedrake – Dragon"),
            Legendary::FishMan =>  v = String::from("FishMan – Amphibious, scaled humanoid"),
            Legendary::FlatwoodsMonster =>  v = String::from("FlatwoodsMonster – Alien, humanoid"),
            Legendary::Fomorian =>  v = String::from("Fomorian – Goat-headed giant"),
            Legendary::ForestBull =>  v = String::from("ForestBull – Giant horned red cattle"),
            Legendary::Freybug =>  v = String::from("// Norfolk black dog// Norfolk black dog"),
            Legendary::Fuath =>  v = String::from("Fuath – Malevolent water spirit"),
            Legendary::Fucanglong =>  v = String::from("Fucanglong – Underworld dragon"),
            Legendary::Funayurei =>  v = String::from("Funayurei – Ghosts of people who drowned at sea"),
            Legendary::FuruUtsubo =>  v = String::from("FuruUtsubo – Animated jar"),
            Legendary::FutakuchiOnna =>  v = String::from("FutakuchiOnna – Woman with a second mouth on the back of her head"),
            Legendary::Fylgja =>  v = String::from("Fylgja – Animal familiar"),
            Legendary::Gaasyendietha =>  v = String::from("Gaasyendietha – Dragon"),
            Legendary::Gagana =>  v = String::from("Gagana – Iron-beaked bird with copper talons"),
            Legendary::Gaki =>  v = String::from("Gaki – Ghosts of especially greedy people"),
            Legendary::Gallu =>  v = String::from("Gallu – Underworld demons"),
            Legendary::Galtzagorriak =>  v = String::from("Galtzagorriak – Small demonic servants"),
            Legendary::Gamayun =>  v = String::from("Gamayun – Prophetic human-headed bird"),
            Legendary::Gana =>  v = String::from("Gana – Attendants of Shiva"),
            Legendary::Gancanagh =>  v = String::from("Gancanagh – Male fairy that seduces human women"),
            Legendary::Gandabherunda =>  v = String::from("Gandabherunda – Double-headed bird"),
            Legendary::Gandharva =>  v = String::from("Gandharva – Male nature spirits, often depicted as part human, part animal"),
            Legendary::Gargouille =>  v = String::from("Gargouille – Water dragon"),
            Legendary::Garkain =>  v = String::from("Garkain – A flying humanoid who envelops his victims"),
            Legendary::Garmr =>  v = String::from("Garmr – Giant, ravenous hound"),
            Legendary::Garuda =>  v = String::from("Garuda – Human-eagle hybrid"),
            Legendary::Gashadokuro =>  v = String::from("Gashadokuro – Giant malevolent skeletons"),
            Legendary::Gaueko =>  v = String::from("Gaueko – Wolf capable of walking upright"),
            Legendary::Geb =>  v = String::from("Geb – God of the Earth, married to Nut"),
            Legendary::Ged =>  v = String::from("Ged – The fish pike"),
            Legendary::Gegenees =>  v = String::from("Gegenees – Six-armed giant"),
            Legendary::GeniusLoci =>  v = String::from("GeniusLoci – Spirit that protects a specific place"),
            Legendary::German =>  v = String::from("German – Male spirit associated with bringing rain and hail"),
            Legendary::Geryon =>  v = String::from("Geryon six legs"),
            Legendary::GhillieDhu =>  v = String::from("GhillieDhu – Tree guardian"),
            Legendary::Ghost =>  v = String::from("Disembodied spirits of those that have diedDisembodied spirits of those that have died"),
            Legendary::Ghoul =>  v = String::from("Ghoul – Cannibalistic shapeshifting desert genie often classified as undead."),
            Legendary::Giant =>  v = String::from("Giant – Immensely large and strong humanoids"),
            Legendary::GiantAnimal =>  v = String::from("GiantAnimal – Unusually large beasts"),
            Legendary::GichiAnamiEBizhiw =>  v = String::from("GichiAnamiEBizhiw – Bison-snake-bird-cougar hybrid water spirit"),
            Legendary::Gidim =>  v = String::from("Gidim – Ghost"),
            Legendary::Gigantes =>  v = String::from("Gigantes – Race of giants that fought the Olympian gods, sometimes depicted with snake-legs"),
            Legendary::Gigelorum =>  v = String::from("Gigelorum – Smallest animal"),
            Legendary::Girtablilu =>  v = String::from("Girtablilu – Human-scorpion hybrid"),
            Legendary::Gjenganger =>  v = String::from("Gjenganger – Corporeal ghost"),
            Legendary::Glaistig =>  v = String::from("Glaistig – Human-goat hybrid"),
            Legendary::Glashtyn =>  v = String::from("Glashtyn – Malevolent water horse"),
            Legendary::Gnome =>  v = String::from("Gnome – Diminutive Earth elemental"),
            Legendary::Goblin =>  v = String::from("Goblin – Grotesque, mischievous little people"),
            Legendary::Gog =>  v = String::from("Gog – Giant protector of London"),
            Legendary::GoldDiggingAnt =>  v = String::from("GoldDiggingAnt – Dog-sized ant that digs for gold in sandy areas"),
            Legendary::Golem =>  v = String::from("Golem – Animated construct"),
            Legendary::Gorgades =>  v = String::from("Gorgades – Hairy humanoid"),
            Legendary::Gorgon =>  v = String::from("Gorgon – Fanged, snake-haired humanoids that turn anyone who sees them into stone"),
            Legendary::Goryo =>  v = String::from("Goryo – Vengeful ghosts, usually of martyrs"),
            Legendary::Grassman =>  v = String::from("Grassman – Ape-like cryptid"),
            Legendary::Gremlin =>  v = String::from("Gremlin – Creatures that sabotage airplanes"),
            Legendary::Griffin =>  v = String::from("Griffin – Lion-eagle hybrid"),
            Legendary::Grigori =>  v = String::from("Grigori – Fallen angels, father of Nephilim"),
            Legendary::Grim =>  v = String::from("Grim – Tutelary spirits of churches"),
            Legendary::GrimReaper =>  v = String::from("GrimReaper – Death angel often thought to be God's/Satan's assistant"),
            Legendary::Grindylow =>  v = String::from("Grindylow – Malevolent water spirit"),
            Legendary::Gualichu =>  v = String::from("Gualichu – Malevolent spirit"),
            Legendary::GuardianAngel =>  v = String::from("GuardianAngel – Subclassification of angels that guard and protect a specific person or living being"),
            Legendary::GudElim =>  v = String::from("GudElim – Human-bull hybrid"),
            Legendary::Guhin =>  v = String::from("Guhin – Anthropomorphic bird"),
            Legendary::GuiPo =>  v = String::from("GuiPo – Ghost that manifests as an old woman"),
            Legendary::GuiShu =>  v = String::from("GuiShu – Ghostly tree that confuses travelers by moving"),
            Legendary::Gulon =>  v = String::from("Gulon – Gluttonous dog-cat-fox hybrid"),
            Legendary::Gumiho =>  v = String::from("Gumiho – Demonic fox with thousands of tails believed to possess an army of spirits and magic in its tails"),
            Legendary::Gurangatch =>  v = String::from("Gurangatch - An enormous reptile-fish whose movements carved out the landscape south of the Blue Mountains"),
            Legendary::Gurumapa =>  v = String::from("Gurumapa – Child-eating demon"),
            Legendary::Gwyllgi =>  v = String::from("Gwyllgi – Black dog"),
            Legendary::Gwyllion =>  v = String::from("Gwyllion – Malevolent spirit"),
            Legendary::Gyascutus =>  v = String::from("Gyascutus – Four-legged herbivore"),
            Legendary::Gytrash =>  v = String::from("Gytrash – Black dog"),
            Legendary::Gyuki =>  v = String::from("Gyuki – Bull-headed monster"),
            Legendary::Habrok =>  v = String::from("Habrok – listed as the 'best' hawk"),
            Legendary::Hadhayosh =>  v = String::from("Hadhayosh – gigantic land animal"),
            Legendary::Hades =>  v = String::from("Hades – Ruler of the Underworld"),
            Legendary::Haetae =>  v = String::from("Haetae – dog-lion hybrid"),
            Legendary::Hag =>  v = String::from("Hag – wise old woman who is usually a malevolent spirit or a disguised goddess"),
            Legendary::Haietlik =>  v = String::from("Haietlik – water serpent"),
            Legendary::HaiUri =>  v = String::from("HaiUri – male cannibalistic partially invisible monster"),
            Legendary::Hakutaku =>  v = String::from("Hakutaku – talking beast which handed down knowledge on harmful spirits"),
            Legendary::Hakuturi =>  v = String::from("Hakuturi – nature guardian"),
            Legendary::HalfElf =>  v = String::from("HalfElf – human-elf hybrid"),
            Legendary::Haltija =>  v = String::from("Haltija – spirit that protects a specific place"),
            Legendary::Hamadryad =>  v = String::from("Hamadryad – oak tree nymph"),
            Legendary::Hamingja =>  v = String::from("Hamingja – personal protection spirit"),
            Legendary::Hamsa =>  v = String::from("Hamsa – mystic bird"),
            Legendary::HanauEpe =>  v = String::from("HanauEpe – long-eared humanoid"),
            Legendary::HantuAir =>  v = String::from("HantuAir – shapeshifting water spirit"),
            Legendary::HantuDemon =>  v = String::from("HantuDemon – demon"),
            Legendary::HantuRaya =>  v = String::from("HantuRaya – demonic servant"),
            Legendary::Harionago =>  v = String::from("Harionago – humanoid female with barbed, prehensile hair"),
            Legendary::Harpy =>  v = String::from("Harpy – birdlike human-headed death spirit"),
            Legendary::Haugbui =>  v = String::from("Haugbui – undead being who cannot leave its burial mound"),
            Legendary::Havsrå =>  v = String::from("Havsrå – saltwater spirit"),
            Legendary::Helloi =>  v = String::from("Helloi – celestial maidens, daughters of the Sky God Soraren"),
            Legendary::HeadlessHorseman =>  v = String::from("HeadlessHorseman – humanoid spirit who haunts or kills"),
            Legendary::HeadlessMule =>  v = String::from("HeadlessMule – fire-spewing, headless, spectral mule"),
            Legendary::Hecatonchires =>  v = String::from("Hecatonchires – primordial giants with 100 hands and fifty heads"),
            Legendary::Heikegani =>  v = String::from("Heikegani – crabs with human-faced shells, the spirits of warriors killed in the Battle of Dan-no-ura"),
            Legendary::Heinzelmannchen =>  v = String::from("Heinzelmannchen – household spirit"),
            Legendary::Helead =>  v = String::from("Helead – fen nymph"),
            Legendary::Hellhound =>  v = String::from("Hellhound – underworld dog"),
            Legendary::Heracles =>  v = String::from("Heracles – gatekeeper of Olympus"),
            Legendary::Hercinia =>  v = String::from("Hercinia – glowing bird"),
            Legendary::Herensuge =>  v = String::from("Herensuge – dragon"),
            Legendary::Hesperides =>  v = String::from("Hesperides – nymph daughters of Atlas"),
            Legendary::Hidebehind =>  v = String::from("Hidebehind – nocturnal forest creature"),
            Legendary::Hiderigami =>  v = String::from("Hiderigami – drought spirit"),
            Legendary::Hieracosphinx =>  v = String::from("Hieracosphinx – falcon-headed sphinx"),
            Legendary::Hihi =>  v = String::from("Hihi – baboon monster"),
            Legendary::Hiisi =>  v = String::from("Hiisi – nature guardian"),
            Legendary::Hippalectryon =>  v = String::from("Hippalectryon"),
            Legendary::Hippocamp =>  v = String::from("Hippocamp – horse-fish hybrid"),
            Legendary::Hippogriff =>  v = String::from("Hippogriff – hybrid of a griffin and horse; a lion-eagle-horse hybrid"),
            Legendary::Hippopodes =>  v = String::from("Hippopodes – horse-hoofed humanoid"),
            Legendary::Hircocervus =>  v = String::from("Hircocervus – deer-goat hybrid"),
            Legendary::Hitodama =>  v = String::from("Hitodama – ghosts of the newly dead, which take the form of fireballs"),
            Legendary::HitotsumeKozo =>  v = String::from("HitotsumeKozo – one-eyed childlike spirit"),
            Legendary::Hob =>  v = String::from("Hob – house spirit"),
            Legendary::Hobbididance =>  v = String::from("Hobbididance – malevolent spirit"),
            Legendary::Hobgoblin =>  v = String::from("Hobgoblin – friendly or amusing goblin"),
            Legendary::Hodag =>  v = String::from("Hodag – frog-mammoth-lizard hybrid"),
            Legendary::Hokhokw =>  v = String::from("Hokhokw – bird"),
            Legendary::Hoko =>  v = String::from("Hoko – dog-like Chinese tree spirit"),
            Legendary::Homa =>  v = String::from("Homa – eagle-lion hybrid, similar to a griffin"),
            Legendary::HombreCaiman =>  v = String::from("HombreCaiman – human-alligator hybrid"),
            Legendary::HombreGato =>  v = String::from("HombreGato – human-cat hybrid"),
            Legendary::Homunculus =>  v = String::from("Homunculus – small animated construct"),
            Legendary::Hoo =>  v = String::from("Hoo – rooster-swallow-fowl-snake-goose-tortoise-stag-fish hybrid"),
            Legendary::Hoopoe =>  v = String::from("near passerine bird common to Africa and Eurasia that features in many mythologies in those continentsnear passerine bird common to Africa and Eurasia that features in many mythologies in those continents"),
            Legendary::HoopSnake =>  v = String::from("snake which rolls by taking its tail in its mouthsnake which rolls by taking its tail in its mouth"),
            Legendary::HornedSerpent =>  v = String::from("HornedSerpent – serpentine rain spirit"),
            Legendary::Hotoke =>  v = String::from("Hotoke – deceased person"),
            Legendary::Houri =>  v = String::from("Houri – heavenly beings"),
            Legendary::Hraesvelg =>  v = String::from("Hraesvelg – giant, who in eagle form, creates the wind by beating his wings"),
            Legendary::Hrímþursar =>  v = String::from("Hrímþursar – frost giants who are the main inhabitants of either Jotunheim or Niflheim"),
            Legendary::Huaychivo =>  v = String::from("Huaychivo – human-deer hybrid"),
            Legendary::HuginnAndMuninn =>  v = String::from("HuginnAndMuninn – pair of ravens associated with the Norse god Odin whose names mean Thought and Memory."),
            Legendary::Huldufolk =>  v = String::from("Huldufolk – secret mound/rock dwelling elves"),
            Legendary::Hulder =>  v = String::from("Hulder – forest spirit"),
            Legendary::HuliJing =>  v = String::from("HuliJing – nine-tailed fox spirit"),
            Legendary::Huma =>  v = String::from("Huma – regenerative fire bird"),
            Legendary::Humbaba =>  v = String::from("Humbaba – lion-faced giant"),
            Legendary::Hundun =>  v = String::from("Hundun – chaos spirit"),
            Legendary::Hupia =>  v = String::from("Hupia – nocturnal ghost"),
            Legendary::Hyakume =>  v = String::from("Hyakume – hundred-eyes creature"),
            Legendary::Hydra =>  v = String::from("Hydra – multi-headed water serpent/dragon"),
            Legendary::Hydros =>  v = String::from("Hydros – snake whose poison causes the victim to swell up"),
            Legendary::Hydrus =>  v = String::from("Hydrus – snake from the Nile River that would kill crocodiles from the inside"),
            Legendary::Hyosube =>  v = String::from("Hyosube – hair-covered kappa"),
            Legendary::Hypnalis =>  v = String::from("Hypnalis – snake that kills its victims in their sleep"),
            Legendary::Hudhud =>  v = String::from("Hudhud – Hoopoe"),
            Legendary::Ishigaq =>  v = String::from("Ishigaq – Little people"),
            Legendary::IslandSatyr =>  v = String::from("IslandSatyr – Savage human-goat hybrid from a remote island chain"),
            Legendary::Isonade =>  v = String::from("Isonade – Shark-like sea monster"),
            Legendary::IttanMomen =>  v = String::from("IttanMomen – Ghostly aerial phenomenon that attacks people"),
            Legendary::IwanaBozu =>  v = String::from("IwanaBozu – Char which appeared as a Buddhist monk"),
            Legendary::Jackalope =>  v = String::from("Jackalope – Rabbit with antlers"),
            Legendary::JackInIrons =>  v = String::from("JackInIrons – Malevolent giant"),
            Legendary::JackOLantern =>  v = String::from("JackOLantern – Vegetal lantern"),
            Legendary::Jaculus =>  v = String::from("Jaculus – Winged serpent or small dragon"),
            Legendary::Jasconius =>  v = String::from("Jasconius – Island-sized fish"),
            Legendary::JasyJaterei =>  v = String::from("JasyJaterei – Nature guardian and bogeyman"),
            Legendary::Jatayu =>  v = String::from("Jatayu – Vulture demigod"),
            Legendary::Jaud =>  v = String::from("Jaud – Vampirised premature baby"),
            Legendary::Jenglot =>  v = String::from("Jenglot – Vampiric little people"),
            Legendary::Jengu =>  v = String::from("Jengu – Water spirit"),
            Legendary::Jentil =>  v = String::from("Jentil – Megalith-building giant"),
            Legendary::Jenu =>  v = String::from("Jenu – Anthropophagous giant"),
            Legendary::Jerff =>  v = String::from("Jerff – Gluttonous dog-cat-fox hybrid"),
            Legendary::JerseyDevil =>  v = String::from("JerseyDevil – Demonic dragon or flying demon who was given birth to by an American living in New Jersey"),
            Legendary::Jian =>  v = String::from("Jian – One-eyed, one-winged bird who requires a mate for survival"),
            Legendary::Jiangshi =>  v = String::from("Jiangshi – Life-draining, reanimated corpse"),
            Legendary::Jiaolong =>  v = String::from("Jiaolong – Dragon"),
            Legendary::Jibakurei =>  v = String::from("Jibakurei – Spirit that protects a specific place"),
            Legendary::Jievaras =>  v = String::from("Jievaras – House spirit"),
            Legendary::Jikininki =>  v = String::from("Jikininki – Corpse-eating ghost"),
            Legendary::Jinn =>  v = String::from("Jinn – Spiritual creatures; genii"),
            Legendary::JipijkaM =>  v = String::from("JipijkaM – Underwater horned snake; lives in lakes and eats humans"),
            Legendary::Jiufeng =>  v = String::from("Jiufeng – Nine-headed bird worshiped by ancient natives in Hubei Province."),
            Legendary::JiuTouNiao =>  v = String::from("JiuTouNiao – Nine-headed, demonic bird"),
            Legendary::Jogah =>  v = String::from("Jogah – Little people nature spirit"),
            Legendary::Jormungandr =>  v = String::from("Jormungandr – Sea serpent"),
            Legendary::Jorogumo =>  v = String::from("Jorogumo – Spider woman"),
            Legendary::Jotai =>  v = String::from("Jotai – Animated folding screen cloth"),
            Legendary::Jotunn =>  v = String::from("Jotunn – Gigantic nature spirits"),
            Legendary::Jujak =>  v = String::from("Jujak – Bird"),
            Legendary::Jumbee =>  v = String::from("Jumbee – Malevolent spirit"),
            Legendary::Kabouter =>  v = String::from("Kabouter – Little people that live underground, in mushrooms, or as house spirits"),
            Legendary::Kachina =>  v = String::from("Kachina – Nature spirit"),
            Legendary::Kahaku =>  v = String::from("Kahaku – Little people and water spirits"),
            Legendary::Kajsa =>  v = String::from("Kajsa – Wind spirit"),
            Legendary::Kalakeyas =>  v = String::from("Kalakeyas – Descendants of Kala"),
            Legendary::Kallikantzaroi =>  v = String::from("Kallikantzaroi – Grotesque, malevolent spirit"),
            Legendary::Kamaitachi =>  v = String::from("Kamaitachi – Wind spirit"),
            Legendary::Kamatayan =>  v = String::from("Kamatayan – Philippine counterpart of Death"),
            Legendary::Kami =>  v = String::from("Kami – Nature spirit"),
            Legendary::Kamikiri =>  v = String::from("Kamikiri – Hair-cutting spirit"),
            Legendary::KanbariNyudo =>  v = String::from("KanbariNyudo – Bathroom spirit"),
            Legendary::KanglaSha =>  v = String::from("KanglaSha – Great Dragon in the Kangla Palace"),
            Legendary::Kanbo =>  v = String::from("Kanbo – Drought spirit"),
            Legendary::Kanedama =>  v = String::from("Kanedama – Money spirit"),
            Legendary::Kappa =>  v = String::from("Kappa – Little people and water spirit"),
            Legendary::Kapre =>  v = String::from("Kapre – Malevolent tree spirit"),
            Legendary::Karakoncolos =>  v = String::from("Karakoncolos, also in Bosnia and Herzegovina and Serbia known as Karanđoloz – Troublesome spirit"),
            Legendary::Karakura =>  v = String::from("Karakura – Male night-demon"),
            Legendary::KarasuTengu =>  v = String::from("KarasuTengu – Tengu with a bird's bill"),
            Legendary::Karkadann =>  v = String::from("Karkadann – One-horned giant animal"),
            Legendary::Karkinos =>  v = String::from("Karkinos – Giant crab"),
            Legendary::Karura =>  v = String::from("Karura – Eagle-human hybrid"),
            Legendary::Karzelek =>  v = String::from("Karzelek – Little people and mine spirits"),
            Legendary::KasaObake =>  v = String::from("KasaObake – Animated parasol"),
            Legendary::Kasha =>  v = String::from("Kasha – Cat-like demon which descends from the sky and carries away corpses"),
            Legendary::Kashanbo =>  v = String::from("Kashanbo – Kappa who climb into the mountains for the winter"),
            Legendary::KatawaGuruma =>  v = String::from("KatawaGuruma – Woman riding on a flaming wheel"),
            Legendary::KatsuraOtoko =>  v = String::from("KatsuraOtoko – Handsome man from the moon"),
            Legendary::Katallan =>  v = String::from("Katallan – Man-eating giant"),
            Legendary::Kaukas =>  v = String::from("Kaukas – Nature spirit"),
            Legendary::KawaUso =>  v = String::from("KawaUso – Supernatural river otter"),
            Legendary::KawaZaru =>  v = String::from("KawaZaru – Smelly, cowardly water spirit"),
            Legendary::KeLets =>  v = String::from("KeLets – Ogre or evil spirit"),
            Legendary::Keelut =>  v = String::from("Keelut – Hairless dog"),
            Legendary::KeeWakw =>  v = String::from("KeeWakw – Half-human half-animal cannibalistic giant"),
            Legendary::Kekkai =>  v = String::from("Kekkai – Amorphous afterbirth spirit"),
            Legendary::Kelpie =>  v = String::from("Kelpie – Malevolent water horse"),
            Legendary::Ker =>  v = String::from("Ker – Female death spirit"),
            Legendary::KesaranPasaran =>  v = String::from("KesaranPasaran – Mysterious, white, fluffy creature"),
            Legendary::Keukegen =>  v = String::from("Keukegen – Disease spirit"),
            Legendary::Keythong =>  v = String::from("Keythong – Wingless griffin"),
            Legendary::Khyah =>  v = String::from("Khyah – Fat, hairy ape-like creature"),
            Legendary::Kigatilik =>  v = String::from("Kigatilik – Night-demon"),
            Legendary::Kholomodumo =>  v = String::from("Kholomodumo – Gluttonous monster that was one of the first beasts of creation"),
            Legendary::Kijimunaa =>  v = String::from("Kijimunaa – Tree sprite from Okinawa"),
            Legendary::Kijo =>  v = String::from("Kijo – She-devil"),
            Legendary::Kikimora =>  v = String::from("Kikimora – Female house spirit"),
            Legendary::Killmoulis =>  v = String::from("Killmoulis – Ugly, mischievous mill spirit"),
            Legendary::Kinnara =>  v = String::from("Kinnara – Human-bird hybrid"),
            Legendary::KinU =>  v = String::from("KinU – Bird"),
            Legendary::Kirin =>  v = String::from("Kirin – Japanese Unicorn"),
            Legendary::Kishi =>  v = String::from("Kishi – Malevolent, two-faced seducer"),
            Legendary::Kitsune =>  v = String::from("Kitsune – Fox spirit"),
            Legendary::KitsuneTsuki =>  v = String::from("KitsuneTsuki – Person possessed by a fox spirit"),
            Legendary::Kiyohime =>  v = String::from("Kiyohime – Woman who transformed into a serpentine demon out of the rage of unrequited love"),
            Legendary::Klabautermann =>  v = String::from("Klabautermann – Ship spirit"),
            Legendary::Knocker =>  v = String::from("Knocker – Little people and mine spirits"),
            Legendary::Knucker =>  v = String::from("Knucker – Water dragon"),
            Legendary::Kobalos =>  v = String::from("Kobalos – Goblin like thieves and tricksters"),
            Legendary::Kobold =>  v = String::from("Kobold – Little people and mine or house spirits"),
            Legendary::Kodama =>  v = String::from("Kodama – Tree spirit"),
            Legendary::Kofewalt =>  v = String::from("Kofewalt – House spirit"),
            Legendary::KoGok =>  v = String::from("KoGok – Hideous monster"),
            Legendary::Kokakucho =>  v = String::from("Kokakucho – Ubume bird"),
            Legendary::Komainu =>  v = String::from("Komainu – Protective animal"),
            Legendary::KonakiJiji =>  v = String::from("KonakiJiji – Infant that cries until it is picked up, then increases its weight and crushes its victim"),
            Legendary::KonohaTengu =>  v = String::from("KonohaTengu – Bird-like creature"),
            Legendary::KoroPokGuru =>  v = String::from("KoroPokGuru – Little people"),
            Legendary::Korrigan =>  v = String::from("Korrigan – Little people and nature spirits"),
            Legendary::Kraken =>  v = String::from("Kraken – Sea monster"),
            Legendary::Krasnoludek =>  v = String::from("Krasnoludek – Little people nature spirits"),
            Legendary::Krasue =>  v = String::from("Krasue – Vampiric, floating head"),
            Legendary::Krampus =>  v = String::from("Krampus – Christmas Devil who punishes badly-behaved children"),
            Legendary::KuarahyJara =>  v = String::from("KuarahyJara – Forest spirit"),
            Legendary::Kubikajiri =>  v = String::from("Kubikajiri – Female corpse-chewing graveyard spirit"),
            Legendary::KuchisakeOnna =>  v = String::from("KuchisakeOnna – Vengeful ghost of a woman mutilated by her husband"),
            Legendary::KudaGitsune =>  v = String::from("KudaGitsune – Miniature fox spirit"),
            Legendary::Kudan =>  v = String::from("Kudan – Human-faced calf which predicts a calamity before dying"),
            Legendary::Kui =>  v = String::from("Kui – One-legged monster"),
            Legendary::Kukudhi =>  v = String::from("Kukudhi – Female demon who spreads sickness"),
            Legendary::Kukwes =>  v = String::from("Kukwes – Large, hairy, greedy, human-eating bipedal monsters whose scream can kill"),
            Legendary::Kulshedra =>  v = String::from("Kulshedra – Drought-causing dragon"),
            Legendary::Kumakatok =>  v = String::from("Kumakatok – Death spirits"),
            Legendary::Kumiho =>  v = String::from("Kumiho – Fox spirit"),
            Legendary::Kun =>  v = String::from("Kun – Giant fish"),
            Legendary::Kupua =>  v = String::from("Kupua – Shapeshifting tricksters"),
            Legendary::Kurabokko =>  v = String::from("Kurabokko – Guardian spirit of a warehouse"),
            Legendary::KurageNoHinotama =>  v = String::from("KurageNoHinotama – Jellyfish which floats through the air as a fireball"),
            Legendary::Kurma =>  v = String::from("Kurma – Second avatar of Vishnu in the form of a Turtle"),
            Legendary::Kurupi =>  v = String::from("Kurupi – Wild man and fertility spirit"),
            Legendary::Kushtaka =>  v = String::from("Kushtaka – Shapeshifting 'land otter man'"),
            Legendary::KyeRyong =>  v = String::from("KyeRyong – Chicken-lizard hybrid"),
            Legendary::Kyourinrin =>  v = String::from("Kyourinrin – Animated scroll or paper"),
            Legendary::KyubiNoKitsune =>  v = String::from("KyubiNoKitsune – Nine-tailed fox"),
            Legendary::Kyuketsuki =>  v = String::from("Kyuketsuki – Vampire"),
            Legendary::LaBarTu =>  v = String::from("LaBarTu – Disease demon"),
            Legendary::LabbMu =>  v = String::from("LabbMu – Sea snake"),
            Legendary::Ladyidday =>  v = String::from("Ladyidday – Sunstroke spirit"),
            Legendary::Ladon =>  v = String::from("Ladon – Dragon guarding the golden apples of the Hesperides"),
            Legendary::Laelaps =>  v = String::from("Laelaps – Enchanted dog that always caught his prey"),
            Legendary::Laestrygonians =>  v = String::from("Laestrygonians – Anthropophagic giants"),
            Legendary::Lakanica =>  v = String::from("Lakanica – Field spirit"),
            Legendary::LakeMonster =>  v = String::from("LakeMonster – Gigantic animals reported to inhabit various lakes around the world"),
            Legendary::Lakhey =>  v = String::from("Lakhey – Demon with fangs"),
            Legendary::LaLlorona =>  v = String::from("LaLlorona – Death spirit associated with drowning"),
            Legendary::Lamassu =>  v = String::from("Lamassu – Protective spirit with the form of a winged bull or human-headed lion"),
            Legendary::LambtonWorm =>  v = String::from("LambtonWorm – Giant worm"),
            Legendary::Lamia =>  v = String::from("Lamia – Child-devouring monster"),
            Legendary::Lamiak =>  v = String::from("Lamiak – Water spirit with duck-like feet"),
            Legendary::LaMojana =>  v = String::from("LaMojana – Shapeshifting, female water spirit"),
            Legendary::Lampades =>  v = String::from("Lampades – Underworld nymph"),
            Legendary::Landvaettir =>  v = String::from("Landvaettir – Nature spirits"),
            Legendary::Langmeidong =>  v = String::from("Langmeidong – Semi human, semi hornbill creature"),
            Legendary::Lares =>  v = String::from("Lares – House spirit"),
            Legendary::LaSayona =>  v = String::from("LaSayona – Female ghost that punishes unfaithful husbands"),
            Legendary::LaTunda =>  v = String::from("LaTunda – Nature spirit that seduces and kills men"),
            Legendary::LavaBear =>  v = String::from("Miniature bear thought to inhabit the lava beds of south central OregonMiniature bear thought to inhabit the lava beds of south central Oregon"),
            Legendary::LaukuDvasios =>  v = String::from("LaukuDvasios – Field spirit"),
            Legendary::Lauma =>  v = String::from("Lauma – Sky spirit"),
            Legendary::Lavellan =>  v = String::from("Lavellan – Gigantic water rat"),
            Legendary::LeananSidhe =>  v = String::from("LeananSidhe – Fairy lover"),
            Legendary::Leanashe =>  v = String::from("Leanashe – Possessing spirit or vampire"),
            Legendary::Leimakids =>  v = String::from("Leimakids – Meadow nymph"),
            Legendary::Leokampoi =>  v = String::from("Leokampoi – Fish-tailed lion"),
            Legendary::Leontophone =>  v = String::from("Leontophone – Tiny animal poisonous to lions"),
            Legendary::Leprechaun =>  v = String::from("Leprechaun – Cobbler spirit"),
            Legendary::Leszi =>  v = String::from("Leszi – Tree spirit"),
            Legendary::Leuce =>  v = String::from("Leuce – White poplar tree nymph"),
            Legendary::Leucrota =>  v = String::from("Leucrota – Crocotta-lion hybrid"),
            Legendary::Leviathan =>  v = String::from("Leviathan – Sea monster seen in Job 41"),
            Legendary::Leyak =>  v = String::from("Leyak – Anthropophagous flying head with entrails"),
            Legendary::LibyanAegipanes =>  v = String::from("LibyanAegipanes – Human-horse hybrid"),
            Legendary::LibyanSatyr =>  v = String::from("LibyanSatyr – Human-goat hybrid"),
            Legendary::Liderc =>  v = String::from("Liderc – Magical chicken that transforms into a humanoid"),
            Legendary::LightningBird =>  v = String::from("LightningBird – Magical bird found at sites of lightning strikes"),
            Legendary::Likho =>  v = String::from("Likho – One-eyed hag or goblin"),
            Legendary::Lilin =>  v = String::from("Lilin – Night-demoness"),
            Legendary::Lilitu =>  v = String::from("Lilitu – Winged demon"),
            Legendary::Limnades =>  v = String::from("Limnades – Lake nymph"),
            Legendary::Lindworm =>  v = String::from("Lindworm – Dragon"),
            Legendary::Ljosalfar =>  v = String::from("Ljosalfar – Sunlight spirits; the Light Elves"),
            Legendary::Ljubi =>  v = String::from("Ljubi- Demoness"),
            Legendary::LlamhigynYDwr =>  v = String::from("LlamhigynYDwr – Frog-bat-lizard hybrid"),
            Legendary::LochNessMonster =>  v = String::from("LochNessMonster – Serpentine sea monster"),
            Legendary::Loki =>  v = String::from("Loki – God of night"),
            Legendary::LoLol =>  v = String::from("LoLol – Hideous monster"),
            Legendary::Long =>  v = String::from("Chinese dragonChinese dragon"),
            Legendary::Longana =>  v = String::from("Longana – Female human-goat hybrid and water spirit"),
            Legendary::LongMa =>  v = String::from("LongMa – Dragon-horse hybrid"),
            Legendary::Loogaroo =>  v = String::from("Loogaroo – Shapeshifting, female vampire"),
            Legendary::LouCarcolh =>  v = String::from("LouCarcolh – Snake-mollusk hybrid"),
            Legendary::LoupGarou =>  v = String::from("LoupGarou – Werewolf"),
            Legendary::LovelandFrog =>  v = String::from("LovelandFrog – Cryptid, Humanoid Frog"),
            Legendary::LubberFiend =>  v = String::from("LubberFiend – House spirit"),
            Legendary::Luduan =>  v = String::from("Luduan – Truth-detecting animal"),
            Legendary::Lugat =>  v = String::from("Lugat – Vampire"),
            Legendary::Luison =>  v = String::from("Luison – Werewolf | Cadaver-eating dog"),
            Legendary::Lusca =>  v = String::from("Sea MonsterSea Monster"),
            Legendary::Lutin =>  v = String::from("Lutin – Amusing goblin"),
            Legendary::Lyngbakr =>  v = String::from("Lyngbakr Whale-like sea monster"),
            Legendary::Lynx =>  v = String::from("Lynx – Feline guide spirit"),
            Legendary::MaaAlused =>  v = String::from("MaaAlused – Subterranean spirit"),
            Legendary::Machlyes =>  v = String::from("Machlyes – Hermaphroditic humanoid"),
            Legendary::Macrocephali =>  v = String::from("Macrocephali – Giant-headed humanoid"),
            Legendary::MadamKoiKoi =>  v = String::from("MadamKoiKoi – Female ghost"),
            Legendary::Madremonte =>  v = String::from("Madremonte – Nature guardian"),
            Legendary::Maero =>  v = String::from("Maero – Savage, arboreal humanoids"),
            Legendary::Magog =>  v = String::from("Magog – Giant protector of London"),
            Legendary::MahaPudma =>  v = String::from("MahaPudma – Giant elephant that holds up the world"),
            Legendary::Mairu =>  v = String::from("Mairu – Megalith-building giant"),
            Legendary::MajasGari =>  v = String::from("MajasGari – Benevolent house spirit"),
            Legendary::Majitu =>  v = String::from("// in Swahili mythology, shape-shifting spirits that can pass as humans// in Swahili mythology, shape-shifting spirits that can pass as humans"),
            Legendary::Makara =>  v = String::from("Makara – Aquatic beings"),
            Legendary::MakuraGaeshi =>  v = String::from("MakuraGaeshi – Pillow-moving spirit"),
            Legendary::MalltYNos =>  v = String::from("MalltYNos – Spirit of the hunt"),
            Legendary::MamiWata =>  v = String::from("MamiWata – Supernaturally beautiful water spirits"),
            Legendary::Manananggal =>  v = String::from("Manananggal – Vampires that sever their torsos from their legs to fly around"),
            Legendary::Mandi =>  v = String::from("Mandi – Humanoid with a forty-year lifespan"),
            Legendary::Mandrake =>  v = String::from("Mandrake – Diminutive, animated construct"),
            Legendary::Manes =>  v = String::from("Manes – Ancestral spirits"),
            Legendary::Mannegishi =>  v = String::from("Mannegishi – Little people with six fingers and no noses"),
            Legendary::Manticore =>  v = String::from("Manticore – Lion-human-scorpion hybrid"),
            Legendary::Mapinguari =>  v = String::from("Mapinguari – Giant sloth"),
            Legendary::Mara =>  v = String::from("Mara – Female night-demon"),
            Legendary::Marabbecca =>  v = String::from("Marabbecca – Malevolent water spirit"),
            Legendary::Mareikura =>  v = String::from("Mareikura – Attendant of Kiho-tumu, the supreme god"),
            Legendary::MaresOfDiomedes =>  v = String::from("MaresOfDiomedes – Man-eating horses"),
            Legendary::Marid =>  v = String::from("Marid – Jinn associated fortune tellers"),
            Legendary::Marmennill =>  v = String::from("Marmennill – Mermen with prophetic abilities"),
            Legendary::MaroDeives =>  v = String::from("MaroDeives – Disease spirits"),
            Legendary::MaskiMonGweZoOs =>  v = String::from("MaskiMonGweZoOs – Shapeshifting toad spirit"),
            Legendary::Matagot =>  v = String::from("Matagot – Spirit that takes animal form; usually that of a black cat"),
            Legendary::Matsya =>  v = String::from("Matsya – First Avatar of Vishnu in the form of a half-fish and half-man"),
            Legendary::Mayura =>  v = String::from("Mayura – Peacock spirit"),
            Legendary::Mazzikin =>  v = String::from("Mazzikin – Invisible, malevolent spirit"),
            Legendary::MboiTuI =>  v = String::from("MboiTuI – Snake-parrot hybrid"),
            Legendary::Mbwiri =>  v = String::from("Mbwiri – Possessing demon"),
            Legendary::Medusa =>  v = String::from("Medusa with numerous snake heads"),
            Legendary::MelekTaus =>  v = String::from("// biblical bird// biblical bird"),
            Legendary::Meliae =>  v = String::from("Meliae – Ash tree nymph"),
            Legendary::Melusine =>  v = String::from("Melusine – Female water spirit, with the form of a winged mermaid or serpent"),
            Legendary::Menehune =>  v = String::from("Menehune – Little people and craftsmen"),
            Legendary::Menninkainen =>  v = String::from("Menninkainen – Little people and nature spirits"),
            Legendary::Merlion =>  v = String::from("Merlion – Combination of a lion and a fish, the symbol of Singapore"),
            Legendary::Mermaid =>  v = String::from("Mermaid – Human-fish hybrid"),
            Legendary::Merman =>  v = String::from("Merman – Human-fish hybrid"),
            Legendary::Merlin =>  v = String::from("Merlin – Elderly wizard"),
            Legendary::Merrow =>  v = String::from("Merrow – Human-fish hybrid"),
            Legendary::MeteeKolenOl =>  v = String::from("MeteeKolenOl – Ice-hearted wizards"),
            Legendary::Mimi =>  v = String::from("Mimi – Extremely elongated humanoid that has to live in rock crevasses to avoid blowing away"),
            Legendary::MinkaBird =>  v = String::from("MinkaBird – Death spirit"),
            Legendary::Minokawa =>  v = String::from("Minokawa – Giant swallow"),
            Legendary::Minotaur =>  v = String::from("Minotaur – Human-bull hybrid"),
            Legendary::Mishibizhiw =>  v = String::from("Mishibizhiw – Feline water spirit"),
            Legendary::MisiGinebig =>  v = String::from("MisiGinebig – Serpentine rain spirit"),
            Legendary::MisiKinepikw =>  v = String::from("MisiKinepikw – Serpentine rain spirit"),
            Legendary::Mizuchi =>  v = String::from("Mizuchi – Water dragon"),
            Legendary::Mogwai =>  v = String::from("Mogwai – Vengeful ghost or demon"),
            Legendary::Mohan =>  v = String::from("Mohan – Nature spirit"),
            Legendary::MokeleMbembe =>  v = String::from("MokeleMbembe – Water-dwelling creature"),
            Legendary::Mokoi =>  v = String::from("Mokoi – Malevolent spirit that kills sorcerers"),
            Legendary::Mokorea =>  v = String::from("Mokorea"),
            Legendary::Monai =>  v = String::from("Monai – Giant snake with antennae"),
            Legendary::Monocerus =>  v = String::from("Monocerus – One-horned stag-horse-elephant-boar hybrid, sometimes treated as distinct from the unicorn"),
            Legendary::MonoGrande =>  v = String::from("MonoGrande – Giant monkey"),
            Legendary::Monopod =>  v = String::from("Monopod – Dwarf with one giant foot"),
            Legendary::MooinjerVeggey =>  v = String::from("MooinjerVeggey – Nature spirit"),
            Legendary::Mora =>  v = String::from("Mora – Disembodied spirit"),
            Legendary::Morgens =>  v = String::from("Morgens – Water spirits"),
            Legendary::MorinjiNoOkama =>  v = String::from("MorinjiNoOkama – Animated tea kettle"),
            Legendary::Mormolykeia =>  v = String::from("Mormolykeia – Underworld spirit"),
            Legendary::Moroi =>  v = String::from("Moroi – Vampiric ghost"),
            Legendary::MossPeople =>  v = String::from("MossPeople – Little people and tree spirits"),
            Legendary::Mothman =>  v = String::from("Mothman – Large grey winged humanoid with glowing red eyes"),
            Legendary::Mugwump =>  v = String::from("Mugwump – Fish-like lake monster"),
            Legendary::Mujina =>  v = String::from("Mujina – Shapeshifting badger spirit"),
            Legendary::Muldjewangk =>  v = String::from("Muldjewangk – Water monster"),
            Legendary::Multo =>  v = String::from("Multo – Spirit of a deceased person seeking justice or has unfinished business"),
            Legendary::Mummy =>  v = String::from("Mummy – Undead creature who revives"),
            Legendary::MumaPadurii =>  v = String::from("MumaPadurii – Forest-dwelling hag"),
            Legendary::MungoonGali =>  v = String::from("MungoonGali – Giant goanna"),
            Legendary::Muscaliet =>  v = String::from("Muscaliet – Hare-squirrel-boar hybrid that has an intense body heat"),
            Legendary::Muse =>  v = String::from("Muse – Spirits that inspire artists"),
            Legendary::Mushusshu =>  v = String::from("Mushusshu"),
            Legendary::Musimon =>  v = String::from("Musimon – Sheep-goat hybrid"),
            Legendary::Myling =>  v = String::from("Myling – Ghosts of unbaptized children"),
            Legendary::Myrmecoleon =>  v = String::from("Myrmecoleon – Ant-lion hybrid"),
            Legendary::Nachzehrer =>  v = String::from("Nachzehrer – Anthropophagous undead"),
            Legendary::Naga =>  v = String::from("Naga – Nature and water spirits, serpentine or human-serpent hybrids"),
            Legendary::NagaFireballs =>  v = String::from("NagaFireballs – Spectral fire"),
            Legendary::Nagual =>  v = String::from("Nagual – Human-animal shapeshifter"),
            Legendary::Naiad =>  v = String::from("Naiad – Freshwater nymph"),
            Legendary::Nakki =>  v = String::from("Nakki – Water spirit"),
            Legendary::Namahage =>  v = String::from("Namahage – Ritual disciplinary demon from the Oga Peninsula"),
            Legendary::Namazu =>  v = String::from("Namazu – Giant catfish whose thrashing causing earthquakes"),
            Legendary::NandoBaba =>  v = String::from("NandoBaba – Old woman who hides under the floor in abandoned storerooms"),
            Legendary::NangTakian =>  v = String::from("NangTakian – Tree spirit"),
            Legendary::NanomKeeaPoDa =>  v = String::from("NanomKeeaPoDa – Earthquake spirit"),
            Legendary::Napaeae =>  v = String::from("Napaeae – Grotto nymph"),
            Legendary::Narasimha =>  v = String::from("Narasimha – Avatar of Vishnu in the form of half-man/half-lion"),
            Legendary::Narecnitsi =>  v = String::from("Narecnitsi – Fate spirit"),
            Legendary::Nariphon =>  v = String::from("Nariphon – Pod people"),
            Legendary::Nargun =>  v = String::from("Nargun – Water monster"),
            Legendary::Nasnas =>  v = String::from("Nasnas – Half-human, half-demon creature with half a body"),
            Legendary::Nav =>  v = String::from("Nav – Ghost"),
            Legendary::Nawao =>  v = String::from("Nawao – Savage humanoid"),
            Legendary::NDamKenoWet =>  v = String::from("NDamKenoWet – Fish-human hybrid"),
            Legendary::Neptune =>  v = String::from("Neptune – God of freshwater and sea"),
            Legendary::Neck =>  v = String::from("Neck – Female water spirit"),
            Legendary::Negret =>  v = String::from("Negret – Little people that turn into coins"),
            Legendary::Nekomata =>  v = String::from("Nekomata – Split-tailed magical cat"),
            Legendary::Nekomusume =>  v = String::from("Nekomusume – Cat in the form of a girl"),
            Legendary::NemeanLion =>  v = String::from("NemeanLion – Lion with impenetrable skin"),
            Legendary::Nephilim =>  v = String::from("Nephilim – Gigantic sons of Grigori and human women"),
            Legendary::Nereid =>  v = String::from("Nereid – Nymph daughters of Nereus"),
            Legendary::Ngen =>  v = String::from("Ngen – Nature spirit"),
            Legendary::Nguruvilu =>  v = String::from("Nguruvilu – Fox-like water snake"),
            Legendary::Nian =>  v = String::from("Nian – Predatory animal"),
            Legendary::Nightmarchers =>  v = String::from("Nightmarchers – Warrior ghosts"),
            Legendary::Nikusui =>  v = String::from("Nikusui – Monster which appears as a young woman and sucks all of the flesh off of its victim's body"),
            Legendary::Nimerigar =>  v = String::from("Nimerigar – Aggressive little people"),
            Legendary::Ningyo =>  v = String::from("Ningyo – Monkey-fish hybrid"),
            Legendary::NinkiNanka =>  v = String::from("NinkiNanka – Large reptile, possibly a dragon"),
            Legendary::Nisse =>  v = String::from("Nisse – House spirit"),
            Legendary::Niohoggr =>  v = String::from("Niohoggr – Dragon"),
            Legendary::Nivatakavachas =>  v = String::from("Nivatakavachas – Ocean demon"),
            Legendary::Nix =>  v = String::from("Nix – Female water spirit"),
            Legendary::Nobusuma =>  v = String::from("Nobusuma – Supernatural wall, also a monstrous flying squirrel"),
            Legendary::Nocnitsa =>  v = String::from("Nocnitsa – Nightmare spirit"),
            Legendary::NopperaBo =>  v = String::from("NopperaBo – Faceless ghost"),
            Legendary::Nozuchi =>  v = String::from("Nozuchi – Small sea serpent"),
            Legendary::Nuckelavee =>  v = String::from("Nuckelavee – Malevolent human-horse-fish hybrid"),
            Legendary::Nue =>  v = String::from("Nue – Monkey-raccoon dog-tiger-snake hybrid"),
            Legendary::NuGui =>  v = String::from("NuGui – Vengeful female ghost"),
            Legendary::Nukekubi =>  v = String::from("Nukekubi – Disembodied, flying head that attacks people"),
            Legendary::NukuMaiTore =>  v = String::from("NukuMaiTore – Forest spirit"),
            Legendary::Nuli =>  v = String::from("Nuli – Humanoid with backwards, eight-toed feet"),
            Legendary::Numen =>  v = String::from("Numen – Tutelary spirit"),
            Legendary::Nuno =>  v = String::from("Nuno – Malevolent little people"),
            Legendary::Nuppeppo =>  v = String::from("Nuppeppo – Animated chunk of dead flesh"),
            Legendary::Nurarihyon =>  v = String::from("Nurarihyon – Head-sized ball-like creature that floats in the sea and teases sailors"),
            Legendary::NureOnna =>  v = String::from("NureOnna – Female monster who appears on the beach"),
            Legendary::Nurikabe =>  v = String::from("Nurikabe – Spirit that manifests as an impassable, invisible wall"),
            Legendary::NyamiNyami =>  v = String::from("NyamiNyami – Snake-spirit of the Zambezi River"),
            Legendary::Nykstukas =>  v = String::from("Nykstukas – Cavern spirit"),
            Legendary::Nymph =>  v = String::from("Nymph – Nature spirit"),
            Legendary::Obake =>  v = String::from("Obake – Shapeshifting spirits"),
            Legendary::Obariyon =>  v = String::from("Obariyon – Spook which rides piggyback on a human victim and becomes unbearably heavy"),
            Legendary::Obayifo =>  v = String::from("Obayifo – Vampiric possession spirit"),
            Legendary::Obia =>  v = String::from("Obia – Gigantic animal that serves witches"),
            Legendary::Oceanid =>  v = String::from("Oceanid – Nymph daughters of Oceanus"),
            Legendary::Odei =>  v = String::from("Odei – Storm spirit"),
            Legendary::Odin =>  v = String::from("Odin – King of Asgard"),
            Legendary::Odmience =>  v = String::from("Odmience – Changeling"),
            Legendary::Og =>  v = String::from("Og – Giant king of the Amorites"),
            Legendary::Ogopogo =>  v = String::from("Ogopogo Canadian Lake Monster"),
            Legendary::Ogun =>  v = String::from("Ogun"),
            Legendary::Ogre =>  v = String::from("Ogre – Large, grotesque humanoid"),
            Legendary::Oiwa =>  v = String::from("Oiwa – Ghost of a woman with a distorted face who was murdered by her husband"),
            Legendary::Ojancanu =>  v = String::from("Ojancanu – Giant cyclops who embodies evil."),
            Legendary::Okiku =>  v = String::from("Okiku – Spirit of a plate-counting servant girl, associated with the 'Okiku-Mushi' worm"),
            Legendary::Okubi =>  v = String::from("Okubi – Death spirit"),
            Legendary::OkuriInu =>  v = String::from("OkuriInu – Dog or wolf that follows travelers at night, similar to the Black dog of English folklore"),
            Legendary::OleHigue =>  v = String::from("OleHigue – Vampiric hag who takes the form of a fireball at night"),
            Legendary::Omukade =>  v = String::from("Omukade – Giant, human-eating centipede that lives in the mountains"),
            Legendary::Oni =>  v = String::from("Oni – Large, grotesque humanoid demon, usually having red skin and horns"),
            Legendary::Onibi =>  v = String::from("Onibi – Spectral fire"),
            Legendary::Onmoraki =>  v = String::from("Onmoraki – Bird-demon created from the spirits of freshly dead corpses"),
            Legendary::Onocentaur =>  v = String::from("Onocentaur – Human-donkey hybrid"),
            Legendary::Onoskelis =>  v = String::from("Onoskelis – Shapeshifting demon"),
            Legendary::Onryo =>  v = String::from("Onryo – Vengeful ghost that manifests in a physical rather than a spectral form"),
            Legendary::Onza =>  v = String::from("Onza – Wild cat, possibly a subspecies of cougar"),
            Legendary::OozlumBird =>  v = String::from("OozlumBird – Bird that flies backwards"),
            Legendary::Ophiotaurus =>  v = String::from("Ophiotaurus – Bull-serpent hybrid"),
            Legendary::Opinicus =>  v = String::from("Opinicus – Lion-eagle hybrid, similar to a griffin, but with leonine forelimbs"),
            Legendary::OrangBunian =>  v = String::from("OrangBunian – Forest spirit"),
            Legendary::OrangMinyak =>  v = String::from("OrangMinyak – Spectral rapist"),
            Legendary::Ordog =>  v = String::from("Ordog – Shapeshifting demon"),
            Legendary::Oread =>  v = String::from("Oread – Mountain nymph"),
            Legendary::Ork =>  v = String::from("Ork – Little people and house spirits"),
            Legendary::Orobas =>  v = String::from("Orobas – Horse-headed, honest oracle classed as a demon"),
            Legendary::OrphanBird =>  v = String::from("OrphanBird – Peacock-eagle-swan-crane hybrid"),
            Legendary::Orthrus =>  v = String::from("Orthrus – Two-headed dog"),
            Legendary::Osiris =>  v = String::from("Osiris – God of the dead and the judge of the underworld"),
            Legendary::Oshun =>  v = String::from("Oshun – God of love and fertility"),
            Legendary::Otso =>  v = String::from("Otso – Bear spirit"),
            Legendary::Ouroboros =>  v = String::from("Ouroboros – Mystic serpent/dragon that eats its own tail"),
            Legendary::Ovinnik =>  v = String::from("Ovinnik – Malevolent threshing house spirit"),
            Legendary::Owlman =>  v = String::from("Owlman – Owl-like humanoid"),
            Legendary::PaasselkaDevils =>  v = String::from("PaasselkaDevils – Spectral fire"),
            Legendary::Pamola =>  v = String::from("Pamola – Weather spirit"),
            Legendary::Panes =>  v = String::from("Panes – Human-goat hybrids descended from the god Pan"),
            Legendary::Pandi =>  v = String::from("Pandi – White-haired humanoid with giant ears and eight fingers and toes"),
            Legendary::Panis =>  v = String::from("Panis – Demons with herds of stolen cows"),
            Legendary::Panlong =>  v = String::from("Panlong – Water dragon"),
            Legendary::Panotti =>  v = String::from("Panotti – Humanoid with gigantic ears"),
            Legendary::Panther =>  v = String::from("Panther – Feline with sweet breath"),
            Legendary::Parandrus =>  v = String::from("Parandrus – Shapeshifting animal whose natural form was a large ruminant"),
            Legendary::Pard =>  v = String::from("Pard – Fast, spotted feline believed to mate with lions to produce leopards"),
            Legendary::Pardalokampoi =>  v = String::from("Pardalokampoi – Fish-tailed leopard"),
            Legendary::Patagon =>  v = String::from("Patagon – Giant race reputed to live in the area of Patagonia"),
            Legendary::Patasola =>  v = String::from("Patasola – Anthropophagous, one-legged humanoid"),
            Legendary::Patupairehe =>  v = String::from("Patupairehe – White-skinned nature spirits"),
            Legendary::Pech =>  v = String::from("Pech – Strong little people"),
            Legendary::Pegaeae =>  v = String::from("Pegaeae – Spring nymph"),
            Legendary::Pegasus =>  v = String::from("Pegasus – Winged horse"),
            Legendary::Pegacorn =>  v = String::from("Pegasus-unicorn hybridPegasus-unicorn hybrid"),
            Legendary::Pelesit =>  v = String::from("Pelesit – Servant spirit"),
            Legendary::Peluda =>  v = String::from("Peluda – Dragon"),
            Legendary::Penanggalan =>  v = String::from("Penanggalan – Vampires that sever their heads from their bodies to fly around, usually with their intestines or other internal organs trailing behind"),
            Legendary::Peng =>  v = String::from("Peng – Giant bird"),
            Legendary::Penghou =>  v = String::from("Penghou – Tree spirit"),
            Legendary::Peri =>  v = String::from("Peri – Winged humanoid"),
            Legendary::Peryton =>  v = String::from("Peryton – Deer-bird hybrid"),
            Legendary::Pesanta =>  v = String::from("Pesanta – Nightmare demon in the form of a cat or dog"),
            Legendary::Peuchen =>  v = String::from("Peuchen – Vampiric, flying, shapeshifting serpent"),
            Legendary::PhiTaiHong =>  v = String::from("PhiTaiHong – Ghost of a person who has died suddenly of a violent or cruel death"),
            Legendary::Phoenix =>  v = String::from("Phoenix – Regenerative bird reborn from its own ashes"),
            Legendary::Piasa =>  v = String::from("Piasa – Winged, antlered feline-like dragon"),
            Legendary::Piatek =>  v = String::from("Piatek – Large land animal"),
            Legendary::PictishBeast =>  v = String::from("PictishBeast – Stylistic animal, possibly a dragon"),
            Legendary::Pillan =>  v = String::from("Pillan – Nature spirit"),
            Legendary::Plagg =>  v = String::from("Plagg"),
            Legendary::PimSkwaWagenOwad =>  v = String::from("PimSkwaWagenOwad – Water spirit"),
            Legendary::Piru =>  v = String::from("Piru – Minor demon"),
            Legendary::Pishacha =>  v = String::from("Pishacha – Carrion-eating demon"),
            Legendary::Pishtaco =>  v = String::from("Pishtaco – Monster man that steals its victim's body fat for cannibalistic purposes"),
            Legendary::PitaSkog =>  v = String::from("PitaSkog – Serpentine rain spirit"),
            Legendary::Pixie =>  v = String::from("Pixie – Little people and nature spirits"),
            Legendary::Pixiu =>  v = String::from("Pixiu – Winged lion"),
            Legendary::PiYao =>  v = String::from("PiYao – Horned, dragon-lion hybrid"),
            Legendary::Plakavac =>  v = String::from("Plakavac – Vampire created when a mother strangles her child"),
            Legendary::PokWejeeMen =>  v = String::from("PokWejeeMen – Tree spirit"),
            Legendary::Polevik =>  v = String::from("Polevik – Little people and field spirits"),
            Legendary::PolloMaligno =>  v = String::from("PolloMaligno – Man-eating chicken spirit"),
            Legendary::Polong =>  v = String::from("Polong – Invisible servant spirit"),
            Legendary::Poltergeist =>  v = String::from("Poltergeist – Ghost that moves objects"),
            Legendary::Pombero =>  v = String::from("Pombero – Wild man and nature spirit"),
            Legendary::Ponaturi =>  v = String::from("Ponaturi – Grotesque, malevolent humanoid"),
            Legendary::Pontianak =>  v = String::from("Pontianak – Undead, vampiric women who died in childbirth"),
            Legendary::PopeLickMonster =>  v = String::from("PopeLickMonster Kentucky Urban Legend – Cryptid, a murderous creature that is part man, sheep, and goat"),
            Legendary::Poukai =>  v = String::from("Poukai – Giant bird"),
            Legendary::Preta =>  v = String::from("Preta – Ghosts of especially greedy people"),
            Legendary::Pricolici =>  v = String::from("Pricolici – Undead wolf"),
            Legendary::Psoglav =>  v = String::from("Psoglav – Dog-headed monster"),
            Legendary::Psotnik =>  v = String::from("Psotnik – Mischievous spirit"),
            Legendary::Psychai =>  v = String::from("Psychai – Butterfly-winged nymphs, daughters of Psyche"),
            Legendary::Psychopomp =>  v = String::from("Psychopomp – Creatures, spirits, angels, or deities in many religions who escort newly deceased souls from Earth to the afterlife"),
            Legendary::Puca =>  v = String::from("Puca – Shapeshifting animal spirit"),
            Legendary::Puki =>  v = String::from("Puki – Malevolent little person"),
            Legendary::Puck =>  v = String::from("Puck – House spirit"),
            Legendary::Putz =>  v = String::from("Putz – House spirit"),
            Legendary::Pugot =>  v = String::from("Pugot – Headless humanoid"),
            Legendary::Puk =>  v = String::from("Puk – House spirit"),
            Legendary::Pukis =>  v = String::from("Pukis – Dragon"),
            Legendary::Puckwudgie =>  v = String::from("Puckwudgie – Troll-like gray-skinned being"),
            Legendary::Pygmy =>  v = String::from("Pygmy – Little people"),
            Legendary::Pyrausta =>  v = String::from("Pyrausta – Insect-dragon hybrid"),
            Legendary::Python =>  v = String::from("Python – Serpentine dragon"),
            Legendary::Qalupalik =>  v = String::from("Qalupalik – Aquatic human abductor"),
            Legendary::Qilin =>  v = String::from("Qilin – Dragon-ox-deer hybrid"),
            Legendary::Qiqirn =>  v = String::from("Qiqirn – Large, bald dog spirit"),
            Legendary::Qliphoth =>  v = String::from("Qliphoth – Evil spirits"),
            Legendary::QuestingBeast =>  v = String::from("QuestingBeast – Serpent-leopard-lion-hart hybrid"),
            Legendary::Quetzalcoatl =>  v = String::from("Quetzalcoatl – Important Aztec god whose name means 'feathered serpent'; he is not to be confused with the quetzal, a type of bird"),
            Legendary::Quinotaur =>  v = String::from("Quinotaur – Five-horned bull"),
            Legendary::Ra =>  v = String::from("Ra – Spirit that protects a specific place"),
            Legendary::Rabisu =>  v = String::from("Rabisu – Vampiric spirit that ambushes people"),
            Legendary::Radande =>  v = String::from("Radande – Tree spirit"),
            Legendary::Ragana =>  v = String::from("Ragana – Malevolent witch"),
            Legendary::Raiju =>  v = String::from("Raiju – Lightning spirit"),
            Legendary::RainBird =>  v = String::from("RainBird – Rain spirit"),
            Legendary::RainbowCrow =>  v = String::from("RainbowCrow – Crow spirit"),
            Legendary::RainbowFish =>  v = String::from("RainbowFish – Whale-sized, multi-colored fish"),
            Legendary::RainbowSerpent =>  v = String::from("RainbowSerpent – Snake"),
            Legendary::Rakshasa =>  v = String::from("Rakshasa – Shapeshifting demon"),
            Legendary::Ramidreju =>  v = String::from("Ramidreju – Extremely long, weasel-like animal"),
            Legendary::Rarog =>  v = String::from("Rarog – Whirlwind spirit"),
            Legendary::RavenMocker =>  v = String::from("RavenMocker – Life-draining spirit"),
            Legendary::RavenSpirit =>  v = String::from("RavenSpirit – Trickster spirit"),
            Legendary::Ratatoskr =>  v = String::from("Ratatoskr – Squirrel spirit"),
            Legendary::RaystownRay =>  v = String::from("RaystownRay – Possible plesiosaur or serpent"),
            Legendary::Redcap =>  v = String::from("Redcap – Evil, ugly humanoid"),
            Legendary::ReEm =>  v = String::from("ReEm – Gigantic land animal"),
            Legendary::Reichsadler =>  v = String::from("Reichsadler – Eagle, sometimes depicted with two heads"),
            Legendary::Rephaite =>  v = String::from("Rephaite – Giant"),
            Legendary::ReptilianHumanoid =>  v = String::from("ReptilianHumanoid – Human-lizard hybrid"),
            Legendary::Revenant =>  v = String::from("Revenant – Reanimated dead"),
            Legendary::Roc =>  v = String::from("Roc – Gigantic bird"),
            Legendary::Rokurokubi =>  v = String::from("Rokurokubi – Long-necked, humanoid trickster"),
            Legendary::Rompo =>  v = String::from("Rompo – Skeletal creature with elements of a rabbit, badger, and bear"),
            Legendary::Rong =>  v = String::from("Rong dragon"),
            Legendary::Rougarou =>  v = String::from("Rougarou – Human-wolf shapeshifter"),
            Legendary::Rusalka =>  v = String::from("Rusalka – Female water spirit"),
            Legendary::Ryu =>  v = String::from("Japanese dragonJapanese dragon"),
            Legendary::Saci =>  v = String::from("Saci – One-legged nature spirit"),
            Legendary::Sagari =>  v = String::from("Sagari – Horse head that dangles from trees on Kyūshū"),
            Legendary::Sakabashira =>  v = String::from("Sakabashira – Haunted pillar, installed upside-down"),
            Legendary::Salamander =>  v = String::from("Salamander – Fire elemental"),
            Legendary::Samebito =>  v = String::from("Samebito – Shark-man servant of the dragon king of the sea"),
            Legendary::Samodiva =>  v = String::from("Samodiva – Nature spirit"),
            Legendary::Sampati =>  v = String::from("Sampati – The demigod Jatayu's brother"),
            Legendary::Sandman =>  v = String::from("Sandman – Nursery spirit that induces sleep in children"),
            Legendary::Sango =>  v = String::from("Sango – Yoruba king of arts, music, dance and entertainment"),
            Legendary::Santelmo =>  v = String::from("Santelmo – Spirits in the form of fireballs that roam around the forest"),
            Legendary::SantaClaus =>  v = String::from("Santa Claus – Elderly man who delivers gifts to well-behaved children on the night of Christmas Eve"),
            Legendary::Sanziana =>  v = String::from("Sanziana – Nature spirit"),
            Legendary::Sarimanok =>  v = String::from("Sarimanok – Bird of good fortune"),
            Legendary::Sarngika =>  v = String::from("Sarngika – Bird spirit"),
            Legendary::Sarugami =>  v = String::from("Sarugami – Wicked monkey spirit who was defeated by a dog"),
            Legendary::Satori =>  v = String::from("Satori – Mind-reading humanoid"),
            Legendary::Satan =>  v = String::from("Satan – Ruler of Hell"),
            Legendary::Satyr =>  v = String::from("Satyr – Human-goat hybrid and fertility spirit"),
            Legendary::Satyrus =>  v = String::from("Satyrus – Apes who always bear twins, one the mother loves, the other it hates"),
            Legendary::SazaeOni =>  v = String::from("SazaeOni – Shapeshifting turban snail spirit"),
            Legendary::Sceadugenga =>  v = String::from("Sceadugenga – Shapeshifting undead"),
            Legendary::Scitalis =>  v = String::from("Scitalis – Snake which mesmerizes its prey"),
            Legendary::ScorpionMan =>  v = String::from("ScorpionMan – Human-scorpion hybrid"),
            Legendary::Scylla =>  v = String::from("Scylla – Human-snake hybrid with a snake's tail, twelve legs, and six long-necked snake heads"),
            Legendary::SeaBee =>  v = String::from("SeaBee – Fish-tailed bee"),
            Legendary::SeaLion =>  v = String::from("SeaLion a legendary creature that has the head and upper body of a lion, but with webbed forelimbs and a fish tail."),
            Legendary::SeaMonk =>  v = String::from("SeaMonk – Fish-like humanoid"),
            Legendary::SeaMonster =>  v = String::from("SeaMonster – Giant, marine animals"),
            Legendary::SeaSerpent =>  v = String::from("SeaSerpent – Serpentine sea monster"),
            Legendary::SeaWyvern =>  v = String::from("SeaWyvern – Fish-tailed wyvern"),
            Legendary::Seko =>  v = String::from("Seko – Water spirit which can be heard making merry at night"),
            Legendary::Selkie =>  v = String::from("Selkie – Human-seal shapeshifter"),
            Legendary::SenpokuKanpoku =>  v = String::from("SenpokuKanpoku – Human-faced frog which guides newly deceased souls to the graveyard"),
            Legendary::Seps =>  v = String::from("Seps – Snake with corrosive venom"),
            Legendary::Serpent =>  v = String::from("Serpent – Snake spirit"),
            Legendary::Serpopard =>  v = String::from("Serpopard – Serpent-leopard hybrid"),
            Legendary::Shachihoko =>  v = String::from("Shachihoko – Tiger-carp hybrid"),
            Legendary::Shade =>  v = String::from("Shade – Spiritual imprint"),
            Legendary::ShadowPeople =>  v = String::from("ShadowPeople – Malevolent ghost"),
            Legendary::Shahbaz =>  v = String::from("Shahbaz – Giant eagle or hawk"),
            Legendary::Shaitan =>  v = String::from("Shaitan from the Bible"),
            Legendary::ShangYang =>  v = String::from("ShangYang – Rain bird"),
            Legendary::Shedim =>  v = String::from("Shedim – Chicken-legged demon"),
            Legendary::Shedu =>  v = String::from("Shedu – Protective spirit who takes the form of a winged bull or human-headed lion"),
            Legendary::Shellycoat =>  v = String::from("Shellycoat – Water spirit"),
            Legendary::Shen =>  v = String::from("Shen – Shapeshifing sea monster"),
            Legendary::Shenlong =>  v = String::from("Shenlong – Weather dragon"),
            Legendary::Shibaten =>  v = String::from("Shibaten – Water spirit from Shikoku"),
            Legendary::Shikigami =>  v = String::from("Shikigami – Servant spirit"),
            Legendary::ShikiOji =>  v = String::from("ShikiOji – Child-sized servant spirit"),
            Legendary::Shikome =>  v = String::from("Shikome – Underworld hag"),
            Legendary::Shinigami =>  v = String::from("Shinigami – 'Death god'"),
            Legendary::ShiroBozu =>  v = String::from("ShiroBozu – White, faceless spirit"),
            Legendary::Shirouneri =>  v = String::from("Shirouneri – Animated mosquito netting or dust cloth"),
            Legendary::Shiryo =>  v = String::from("Shiryo – Spirit of a dead person"),
            Legendary::Shisa =>  v = String::from("Shisa – Lion-dog hybrid"),
            Legendary::Shishi =>  v = String::from("Shishi – Protective animal"),
            Legendary::Shojo =>  v = String::from("Shojo – Red-haired sea-sprites who love alcohol"),
            Legendary::Shokera =>  v = String::from("Shokera – Creature that peers in through skylights"),
            Legendary::Shtriga =>  v = String::from("Shtriga – Vampire witch that feeds on children"),
            Legendary::ShuiGui =>  v = String::from("ShuiGui – Drowned ghost"),
            Legendary::ShugMonkey =>  v = String::from("ShugMonkey – Dog/monkey"),
            Legendary::Shunoban =>  v = String::from("Shunoban – Red-faced ghoul"),
            Legendary::ShutenDoji =>  v = String::from("ShutenDoji – Ruler of the Oni"),
            Legendary::Sídhe =>  v = String::from("Sídhe – Ancestral or nature spirit"),
            Legendary::Sigbin =>  v = String::from("Sigbin – Goat-like vampire"),
            Legendary::Sileni =>  v = String::from("Sileni – Bald, fat, thick-lipped, and flat-nosed followers of Dionysus"),
            Legendary::Simargl =>  v = String::from("Simargl – Winged dog"),
            Legendary::Simurgh =>  v = String::from("Simurgh – Dog-lion-peacock hybrid"),
            Legendary::Singa =>  v = String::from("Singa – Feline animal"),
            Legendary::SintHolo =>  v = String::from("SintHolo – Serpentine rain spirit"),
            Legendary::Siren =>  v = String::from("Siren – Human-bird hybrid"),
            Legendary::Sirin =>  v = String::from("Sirin – Demonic human-headed bird"),
            Legendary::Sirrush =>  v = String::from("Sirrush – Dragon with aquiline hind legs and feline forelegs"),
            Legendary::Sisiutl =>  v = String::from("Sisiutl – Two-headed sea serpent"),
            Legendary::SiTeCah =>  v = String::from("SiTeCah – Red-haired giants"),
            Legendary::Sjora =>  v = String::from("Sjora – Freshwater spirit"),
            Legendary::Sjovaettir =>  v = String::from("Sjovaettir – Sea spirit"),
            Legendary::SkinWalker =>  v = String::from("SkinWalker – Animal-human shapeshifter"),
            Legendary::Skogsra =>  v = String::from("Skogsra – Forest spirit"),
            Legendary::Skoll =>  v = String::from("Skoll – Wolf that chases the Sun"),
            Legendary::Skookum =>  v = String::from("Skookum – Hairy giant"),
            Legendary::Skeleton =>  v = String::from("Skeleton – Living skeletons"),
            Legendary::Skrzak =>  v = String::from("Skrzak – Flying imp"),
            Legendary::SkyWomen =>  v = String::from("SkyWomen – Weather spirit"),
            Legendary::Sleipnir =>  v = String::from("Sleipnir – Eight-legged horse"),
            Legendary::Sluagh =>  v = String::from("Sluagh – Restless ghost"),
            Legendary::SodehikiKozo =>  v = String::from("SodehikiKozo – Invisible spirit which pulls on sleeves"),
            Legendary::Sogenbi =>  v = String::from("Sogenbi – Fiery ghost of an oil-stealing monk"),
            Legendary::Soragami =>  v = String::from("Soragami – Ritual disciplinary demon"),
            Legendary::SorakiGaeshi =>  v = String::from("SorakiGaeshi – Sound of trees being cut down, when later none seem to have been cut"),
            Legendary::Sorobanbozu =>  v = String::from("Sorobanbozu – Ghost with an abacus"),
            Legendary::Sotangitsune =>  v = String::from("Sotangitsune – Fox spirit from Kyoto"),
            Legendary::Soucouyant =>  v = String::from("Soucouyant – Vampiric hag who takes the form of a fireball at night"),
            Legendary::Spearfinger =>  v = String::from("Spearfinger – Sharp-fingered hag"),
            Legendary::Spectre =>  v = String::from("Spectre – Terrifying ghost"),
            Legendary::Sphinx =>  v = String::from("Sphinx – Winged woman-headed lion"),
            Legendary::Spiridus =>  v = String::from("Spiridus – Little people"),
            Legendary::Spirit =>  v = String::from("GhostsGhosts"),
            Legendary::Spriggan =>  v = String::from("Spriggan – Guardians of graveyards and ruins"),
            Legendary::Sprite =>  v = String::from("Sprite – little people, ghosts or elves"),
            Legendary::Squonk =>  v = String::from("Squonk – Ugly and lonely creature capable of evading capture by dissolving itself into a pool of tears"),
            Legendary::Stihi =>  v = String::from("Stihi – Demonic dragon who guards a treasure"),
            Legendary::Strigoi =>  v = String::from("Strigoi – Vampire"),
            Legendary::Strix =>  v = String::from("Strix – Vampiric bird"),
            Legendary::Struthopodes =>  v = String::from("Struthopodes – Humanoid whose males have enormous feet, and females have tiny feet"),
            Legendary::Strzyga =>  v = String::from("Strzyga – Vampiric undead"),
            Legendary::Stuhac =>  v = String::from("Stuhac – Malevolent mountain spirit"),
            Legendary::StymphalianBird =>  v = String::from("StymphalianBird – Metallic bird"),
            Legendary::Suangi =>  v = String::from("Suangi – Cannibalistic sorcerer"),
            Legendary::Succubus =>  v = String::from("Succubus – Female night-demon"),
            Legendary::Sudice =>  v = String::from("Sudice – Fortune spirit"),
            Legendary::SunakakeBaba =>  v = String::from("SunakakeBaba – Sand-throwing hag"),
            Legendary::Sunekosuri =>  v = String::from("Sunekosuri – Small dog- or cat-like creature that rubs against a person's legs at night"),
            Legendary::Surma =>  v = String::from("Surma – Hellhound"),
            Legendary::Suzaku =>  v = String::from("Suzaku – Japanese version of the Chinese Vermillion Bird"),
            Legendary::Svaoilfari =>  v = String::from("Svaoilfari – Unnatural strong horse, father of Sleipnir"),
            Legendary::Svartalfar =>  v = String::from("Svartalfar – Cavern spirits; the Black Elves"),
            Legendary::Swallower =>  v = String::from("Swallower – Crocodile-leopard-hippopotamus hybrid"),
            Legendary::SwanMaiden =>  v = String::from("SwanMaiden – Swan-human shapeshifter"),
            Legendary::Sylph =>  v = String::from("Sylph – Air elemental"),
            Legendary::Sylvan =>  v = String::from("Sylvan – Forest spirit"),
            Legendary::Syrbotae =>  v = String::from("Syrbotae – African giant"),
            Legendary::Syrictae =>  v = String::from("Syrictae – Reptilian humanoid"),
            Legendary::Tachash =>  v = String::from("Tachash – Large land animal"),
            Legendary::Tailypo =>  v = String::from("Tailypo – Powerful animal, that takes revenge on those who steal its tail"),
            Legendary::Taimatsumaru =>  v = String::from("Taimatsumaru – Tengu surrounded in demonic fire"),
            Legendary::Takam =>  v = String::from("Takam – Nature spirit"),
            Legendary::TakaOnna =>  v = String::from("TakaOnna – Female spirit which can stretch itself to peer into the second story of a building"),
            Legendary::Talos =>  v = String::from("Talos – Giant made of bronze"),
            Legendary::Tangie =>  v = String::from("Tangie – Shapeshifting water spirit"),
            Legendary::Taniwha =>  v = String::from("Taniwha – Water spirit"),
            Legendary::Tantankororin =>  v = String::from("Tantankororin – Unharvested persimmon which becomes a monster"),
            Legendary::Tanuki =>  v = String::from("Tanuki – Shapeshifting raccoon dog"),
            Legendary::TaotaoMona =>  v = String::from("TaotaoMona – Ancestral spirits"),
            Legendary::Taotie =>  v = String::from("Taotie – Greed spirit"),
            Legendary::Tapairu =>  v = String::from("Tapairu – Nature spirit"),
            Legendary::Tarasque =>  v = String::from("Tarasque – Dragon with leonine, turtle, bear, and human attributes"),
            Legendary::Tartalo =>  v = String::from("Tartalo – One-eyed giant"),
            Legendary::Tartaruchi =>  v = String::from("Tartaruchi – Demonic punisher"),
            Legendary::TatamiTataki =>  v = String::from("TatamiTataki – Poltergeist that hits the tatami mats at night"),
            Legendary::Tatzelwurm =>  v = String::from("Tatzelwurm lizard-like creature, often described as having the face of a cat, with a serpent-like body which may be slender or stubby, with four short legs or two forelegs"),
            Legendary::Tatsu =>  v = String::from("Japanese dragonJapanese dragon"),
            Legendary::Taurokampoi =>  v = String::from("Taurokampoi – Fish-tailed bull"),
            Legendary::Tavara =>  v = String::from("Tavara – Night-demon"),
            Legendary::TejuJagua =>  v = String::from("TejuJagua – Lizard with seven dog heads"),
            Legendary::Tecumbalam =>  v = String::from("Tecumbalam – Bird"),
            Legendary::Tengu =>  v = String::from("Tengu – Anthropomorphic bird"),
            Legendary::Tennin =>  v = String::from("Tennin – Angelic humanoid"),
            Legendary::TeNoMe =>  v = String::from("TeNoMe – Ghost of a blind man, with his eyes on his hands"),
            Legendary::Tepegoz =>  v = String::from("Tepegoz – Azerbaijani mythical creature similar to the cyclops Polyphemus"),
            Legendary::TerribleMonster =>  v = String::from("TerribleMonster – Lion-eagle-scorpion hybrid made from the blood of murder victims"),
            Legendary::TeumessianFox =>  v = String::from("TeumessianFox – Gigantic fox"),
            Legendary::Theriocephalus =>  v = String::from("Theriocephalus – Animal-headed humanoid"),
            Legendary::ThreeLeggedBird =>  v = String::from("ThreeLeggedBird – Solar bird"),
            Legendary::Thunderbird =>  v = String::from("Thunderbird – Avian lightning bird spirit"),
            Legendary::Thor =>  v = String::from("Thor – God of thunder and storm"),
            Legendary::Tiangou =>  v = String::from("Tiangou – Meteoric dog"),
            Legendary::Tianlong =>  v = String::from("Tianlong – Celestial dragon"),
            Legendary::Tibicena =>  v = String::from("Tibicena – Evil Dog"),
            Legendary::TiddyMun =>  v = String::from("TiddyMun – Bog spirit"),
            Legendary::Tigmamanukan =>  v = String::from("Tigmamanukan – Asian fairy bluebird"),
            Legendary::Tigris =>  v = String::from("Tigris – Giant lion"),
            Legendary::Tikbalang =>  v = String::from("Tikbalang – Anthropomorphic horse"),
            Legendary::Tikoloshe =>  v = String::from("Tikoloshe – Little people and water spirit"),
            Legendary::Timingila =>  v = String::from("Timingila – Sea monster"),
            Legendary::Tipua =>  v = String::from("Tipua – Spirit that protects a specific place"),
            Legendary::Titan =>  v = String::from("Titan – Primeval god"),
            Legendary::Tiyanak =>  v = String::from("Tiyanak – Demons that are souls of dead unbaptized babies"),
            Legendary::Tizheruk =>  v = String::from("Tizheruk – Sea serpent"),
            Legendary::Tlahuelpuchi =>  v = String::from("Tlahuelpuchi – Shapeshifting vampire"),
            Legendary::TofuKozo =>  v = String::from("TofuKozo – Spirit child carrying a block of tofu"),
            Legendary::ToireNoHanakosan =>  v = String::from("ToireNoHanakosan – Ghost who lurks in grade school restroom stalls"),
            Legendary::Tomte =>  v = String::from("Tomte – House spirit"),
            Legendary::ToothFairy =>  v = String::from("Tooth fairy a mythical creature who gives out money in exchange for teeth."),
            Legendary::The tradition of leaving a tooth under a pillow for the Tooth Fairy or another fantasy figure to collect is practiced in various countries. =>  v = String::from("The tradition of leaving a tooth under a pillow for the Tooth Fairy or another fantasy figure to collect is practiced in various countries."),
            Legendary::Topielec =>  v = String::from("Topielec – Water spirit"),
            Legendary::Totetsu =>  v = String::from("Totetsu – Greed spirit"),
            Legendary::Toyol =>  v = String::from("Toyol – Servant spirit"),
            Legendary::Trasgo =>  v = String::from("Trasgo – Grotesque, mischievous little people"),
            Legendary::Trauco =>  v = String::from("Trauco – Fertility spirit"),
            Legendary::Trenti =>  v = String::from("Trenti – Diminutive demon"),
            Legendary::Trickster =>  v = String::from("Character in a story which exhibits a great degree of intellect or secret knowledge, and uses it to play tricks or otherwise disobey normal rules and conventional behaviourCharacter in a story which exhibits a great degree of intellect or secret knowledge, and uses it to play tricks or otherwise disobey normal rules and conventional behaviour"),
            Legendary::Tripurasura =>  v = String::from("Tripurasura – Demonic inhabitants of Tripura"),
            Legendary::Tritons =>  v = String::from("Tritons – Male human-fish hybrid"),
            Legendary::Troll =>  v = String::from("Troll – Nature spirit"),
            Legendary::Trow =>  v = String::from("Trow – Little people and nature spirits"),
            Legendary::TsiNoo =>  v = String::from("TsiNoo – Vampiric demon"),
            Legendary::Tsuchigumo =>  v = String::from("Tsuchigumo – Shapeshifting, giant spider"),
            Legendary::Tsuchinoko =>  v = String::from("Tsuchinoko – Plump snake-like creature"),
            Legendary::Tsukumogami =>  v = String::from("Tsukumogami – Inanimate object that becomes animated after existing for 100 years"),
            Legendary::TsulKalu =>  v = String::from("TsulKalu – Giant nature spirit"),
            Legendary::TsuraraOnna =>  v = String::from("TsuraraOnna – Icicle woman"),
            Legendary::TsurubeOtoshi =>  v = String::from("TsurubeOtoshi – Monster which drops or lowers a bucket from the top of a tree to catch people"),
            Legendary::TugarinZmeyevich =>  v = String::from("TugarinZmeyevich – Evil shapeshifter"),
            Legendary::TylwythTeg =>  v = String::from("TylwythTeg – Nature spirit"),
            Legendary::Tupilaq =>  v = String::from("Tupilaq – Animated construct"),
            Legendary::Turehu =>  v = String::from("Turehu – Pale spirit"),
            Legendary::Turst =>  v = String::from("Turst – legendary figure who turns people into dogs"),
            Legendary::Turul =>  v = String::from("Turul – Giant falcon that helped shape the origins of the Magyars"),
            Legendary::Tyger =>  v = String::from("Tyger – Like a real tiger, but lacks stripes. It has the tufted tail of a lion and a thick mane along the neck like a horse"),
            Legendary::Typhon =>  v = String::from("Typhon – Winged, snake-legged giant"),
            Legendary::Tzitzimitl =>  v = String::from("Tzitzimitl – Skeletal star spirit"),
            Legendary::Ubume =>  v = String::from("Ubume – Ghosts of women who died in childbirth"),
            Legendary::UchekLangmeidong =>  v = String::from("///(Manipuri mythology) – Semi human, semi hornbill creature – Semi human, semi hornbill creature"),
            Legendary::UmaNoAshi =>  v = String::from("UmaNoAshi – Horse's leg which dangles from a tree and kicks passersby"),
            Legendary::Umibozu =>  v = String::from("Umibozu – Ghost of drowned priest"),
            Legendary::UmiNyobo =>  v = String::from("UmiNyobo – Female sea monster who steals fish"),
            Legendary::Undead =>  v = String::from("Undead – Dead that behave as if alive"),
            Legendary::UnderwaterPanther =>  v = String::from("UnderwaterPanther – Feline water spirit"),
            Legendary::Undine =>  v = String::from("Undine – Water elemental"),
            Legendary::Unhcegila =>  v = String::from("Unhcegila – Dragon"),
            Legendary::Unicorn =>  v = String::from("Unicorn – Unicorn that inhabits the African Congo."),
            Legendary::Unktehi =>  v = String::from("Unktehi – Serpentine rain spirit"),
            Legendary::Unktehila =>  v = String::from("Unktehila – Reptilian water monster"),
            Legendary::Upinis =>  v = String::from("Upinis – River spirit"),
            Legendary::Urayuli =>  v = String::from("Urayuli – Hairy giant"),
            Legendary::Urias =>  v = String::from("Urias – Giant"),
            Legendary::Urmahlullu =>  v = String::from("Urmahlullu – Lion-human hybrid guardian spirit"),
            Legendary::UshiOni =>  v = String::from("UshiOni – Bull-headed monster"),
            Legendary::Utukku =>  v = String::from("Utukku – ″Underworld messenger spirit″"),
            Legendary::Uwan =>  v = String::from("Uwan – Spirit that shouts to surprise people"),
            Legendary::Vadatajs =>  v = String::from("Vadatajs – Spirit that misleads people"),
            Legendary::Vahana =>  v = String::from("Vahana – Divine mounts"),
            Legendary::Vaibhavi =>  v = String::from("Vaibhavi – Deadly snake"),
            Legendary::Valkyrie =>  v = String::from("Valkyrie – Female spirit that leads souls of dead warriors to Valhalla"),
            Legendary::Valva =>  v = String::from("Valva – Female nature spirit"),
            Legendary::Valravn =>  v = String::from("Valravn – Supernatural raven"),
            Legendary::Vampire =>  v = String::from("Vampire – Reanimated corpse that feeds on blood"),
            Legendary::Vanara =>  v = String::from("Vanara – Human-ape hybrid"),
            Legendary::Vantoase =>  v = String::from("Vantoase – Female weather spirit"),
            Legendary::Varaha =>  v = String::from("Varaha – Third Avatar of Vishnu in the form of a boar"),
            Legendary::Varcolac =>  v = String::from("Varcolac – Vampire or werewolf"),
            Legendary::Vardoger =>  v = String::from("Vardoger – Ghostly double"),
            Legendary::Vedrfolnir =>  v = String::from("Vedrfolnir – Hawk sitting between the eyes of an eagle in the crown of the World Tree Yggdrasil"),
            Legendary::Veli =>  v = String::from("Veli – Ghost, shade, formed after a death of a human"),
            Legendary::VeriSelen =>  v = String::from("Chuvash dragonChuvash dragon"),
            Legendary::Vetala =>  v = String::from("Vetala – Corpses possessed by vampiric spirits"),
            Legendary::Víbria =>  v = String::from("Víbria – Dragon with breasts and an eagle's beak"),
            Legendary::Vielfras =>  v = String::from("Vielfras – Gluttonous dog-cat-fox hybrid"),
            Legendary::Vila =>  v = String::from("Vila – Weather spirit"),
            Legendary::Vilkacis =>  v = String::from("Vilkacis – Animalistic, werewolf-like monster"),
            Legendary::Virunas =>  v = String::from("Virunas – Handsome demon"),
            Legendary::VisionSerpent =>  v = String::from("VisionSerpent – Mystical dragon"),
            Legendary::Vídopnir =>  v = String::from("Vídopnir – Rooster that sits atop the tree"),
            Legendary::Vodyanoy =>  v = String::from("Vodyanoy – Male water spirit"),
            Legendary::Vrykolakas =>  v = String::from("Vrykolakas – Undead wolf-human hybrid"),
            Legendary::Vaettir =>  v = String::from("Vaettir – Nature spirit"),
            Legendary::Waldgeist =>  v = String::from("Waldgeist – Forest spirit"),
            Legendary::WanaGamesAk =>  v = String::from("WanaGamesAk – Water spirits"),
            Legendary::Wani =>  v = String::from("Wani – Crocodilian water monster"),
            Legendary::Wanyudo =>  v = String::from("Wanyudo – Demon in the form of a burning human-headed ox cart"),
            Legendary::WarakNgendog =>  v = String::from("WarakNgendog – Egg-laying bird"),
            Legendary::Warg =>  v = String::from("Warg – Giant, demonic wolf"),
            Legendary::Warlock =>  v = String::from("Warlock – Male witch"),
            Legendary::WassanMonGaneehlaAk =>  v = String::from("WassanMonGaneehlaAk – Aurora spirits"),
            Legendary::WaterMonkey =>  v = String::from("WaterMonkey – Water spirit"),
            Legendary::WaterSprite =>  v = String::from("WaterSprite – Water elemental"),
            Legendary::WatiKutjara =>  v = String::from("WatiKutjara – Goanna spirits"),
            Legendary::WaWonDeeAMegw =>  v = String::from("WaWonDeeAMegw – Shapeshifting snail spirit"),
            Legendary::WeisseFrauen =>  v = String::from("WeisseFrauen – Female spirit"),
            Legendary::Wekufe =>  v = String::from("Wekufe – Demon"),
            Legendary::Wendigo =>  v = String::from("Wendigo – Anthropophagous spirit"),
            Legendary::Wentshukumishiteu =>  v = String::from("Wentshukumishiteu – Water spirit"),
            Legendary::Werecat =>  v = String::from("Werecat – Feline-human shapeshifter"),
            Legendary::Werehyena =>  v = String::from("Werehyena – Hyena-human shapeshifter"),
            Legendary::Werewolf =>  v = String::from("Werewolf – Wolf-human shapeshifter"),
            Legendary::WhiteLady =>  v = String::from("WhiteLady – Ghost of a murdered or mistreated woman"),
            Legendary::Whowie =>  v = String::from("Whowie – Giant frog-headed goanna with six legs"),
            Legendary::WildMan =>  v = String::from("WildMan – Hairy, bipedal, man-like creature"),
            Legendary::WillOTheWisp =>  v = String::from("WillOTheWisp – Spectral fire"),
            Legendary::WirryCow =>  v = String::from("WirryCow – Malevolent spirit"),
            Legendary::Witch =>  v = String::from("Witch – Person who practices magic"),
            Legendary::WitteWieven =>  v = String::from("WitteWieven – Female, ancestral spirit"),
            Legendary::Wolpertinger =>  v = String::from("Wolpertinger"),
            Legendary::Wondjina =>  v = String::from("Wondjina – Weather spirit"),
            Legendary::Wraith =>  v = String::from("Wraith – Water spirit or ghostly apparition"),
            Legendary::Wulver =>  v = String::from("Wulver – Wolf-headed humanoid spirit"),
            Legendary::WuTouGui =>  v = String::from("WuTouGui – Beheaded ghost"),
            Legendary::Wyrm =>  v = String::from("English dragonEnglish dragon"),
            Legendary::Wyvern =>  v = String::from("Wyvern – Flying reptile, usually with two legs and two wings"),
            Legendary::Xana =>  v = String::from("Xana – Female water spirit"),
            Legendary::Xanthus =>  v = String::from("Xanthus"),
            Legendary::Xecotcovach =>  v = String::from("Xecotcovach – Bird"),
            Legendary::Xelhua =>  v = String::from("Xelhua – Giant"),
            Legendary::Xiao =>  v = String::from("Xiao – Ape or four-winged bird"),
            Legendary::XingTian =>  v = String::from("XingTian – Headless giant"),
            Legendary::Xiuhcoatl =>  v = String::from("Xiuhcoatl – Drought spirit"),
            Legendary::Xhindi =>  v = String::from("Xhindi – Elves"),
            Legendary::Yacumama =>  v = String::from("Yacumama – Sea monster"),
            Legendary::Yacuruna =>  v = String::from("Yacuruna – Mythical water people, with backwards heads and feet"),
            Legendary::Yadokai =>  v = String::from("Yadokai – Malevolent, nocturnal spirit"),
            Legendary::YagyoSan =>  v = String::from("YagyoSan – Demon who rides through the night on a headless horse"),
            Legendary::Yaksha =>  v = String::from("Yaksha – Male nature spirit"),
            Legendary::Yakshi =>  v = String::from("Yakshi – Vampire"),
            Legendary::Yakshini =>  v = String::from("Yakshini – Female nature spirit"),
            Legendary::YakubyoGami =>  v = String::from("YakubyoGami – Disease and misfortune spirit"),
            Legendary::Yale =>  v = String::from("Yale – Antelope- or goat-like animal with swiveling horns"),
            Legendary::Yazhi =>  v = String::from("Yazhi – Lion-like beast"),
            Legendary::YalleryBrown =>  v = String::from("YalleryBrown – Nature spirit"),
            Legendary::Yama =>  v = String::from("Yama – Wrathful god"),
            Legendary::YamaBiko =>  v = String::from("YamaBiko – Echo spirit"),
            Legendary::YamaBito =>  v = String::from("YamaBito – Savage, mountain-dwelling humanoid"),
            Legendary::YamaChichi =>  v = String::from("YamaChichi – Monkey-like mountain spirit"),
            Legendary::YamaInu =>  v = String::from("YamaInu – Dog-like mountain spirit"),
            Legendary::YamaOtoko =>  v = String::from("YamaOtoko – Mountain giant"),
            Legendary::YamataNoOrochi =>  v = String::from("YamataNoOrochi – Gigantic, eight-headed serpent"),
            Legendary::YamaUba =>  v = String::from("YamaUba – Malevolent, mountain-dwelling hag"),
            Legendary::YamaWaro =>  v = String::from("YamaWaro – Hairy, one-eyed spirit"),
            Legendary::Yanari =>  v = String::from("Yanari – Spirit which causes strange noises"),
            Legendary::Yaoguai =>  v = String::from("Yaoguai – Animalistic demon or fallen gods"),
            Legendary::YaraMaYhaWho =>  v = String::from("YaraMaYhaWho – Diminutive, sucker-fingered vampire"),
            Legendary::Yatagarasu =>  v = String::from("Yatagarasu – Three-legged crow of Amaterasu"),
            Legendary::YatoNoKami =>  v = String::from("YatoNoKami – Serpent spirits"),
            Legendary::YethHound =>  v = String::from("YethHound – Headless dog"),
            Legendary::Yeti =>  v = String::from("Yeti – Mountain bigfoot"),
            Legendary::Yilbegan =>  v = String::from("Yilbegan – Either a dragon or a giant"),
            Legendary::Yobuko =>  v = String::from("Yobuko – Mountain dwelling spirit"),
            Legendary::Yokai =>  v = String::from("Yokai – Supernatural monster"),
            Legendary::YomotsuShikome =>  v = String::from("YomotsuShikome – Underworld hag"),
            Legendary::Yong =>  v = String::from("Korean dragonKorean dragon"),
            Legendary::Yosei =>  v = String::from("Yosei – Fairy"),
            Legendary::Yosuzume =>  v = String::from("Yosuzume – Mysterious bird that sings at night, sometimes indicating that the okuri-inu is near"),
            Legendary::YouHunYeGui =>  v = String::from("YouHunYeGui – Wandering ghost"),
            Legendary::Yowie =>  v = String::from("Yowie – Nocturnal human-ape hybrid, also Yahoo"),
            Legendary::Ypotryll =>  v = String::from("Ypotryll – Boar-camel-ox-serpent hybrid"),
            Legendary::YuanGui =>  v = String::from("YuanGui – Distressed ghost"),
            Legendary::Yukinko =>  v = String::from("Yukinko – Childlike snow spirit"),
            Legendary::YukiOnna =>  v = String::from("YukiOnna – Female snow spirit"),
            Legendary::Yurei =>  v = String::from("Yurei – Ghost"),
            Legendary::Yuxa =>  v = String::from("Yuxa – 100-year-old snake that transforms into a beautiful human"),
            Legendary::Zahhak =>  v = String::from("Zahhak – Dragon"),
            Legendary::Zaltys =>  v = String::from("Zaltys – Serpentine fertility spirit"),
            Legendary::Zamzummim =>  v = String::from("Zamzummim – Giant"),
            Legendary::ZanaEMalit =>  v = String::from("ZanaEMalit – Mountain fairy who bless warriors"),
            Legendary::Zână =>  v = String::from("Zână – Nature spirit"),
            Legendary::ZashikiWarashi =>  v = String::from("ZashikiWarashi – House spirit"),
            Legendary::Zburator =>  v = String::from("Zburator – Wolf-headed dragon"),
            Legendary::Zduhac =>  v = String::from("Zduhac – Disembodied, heroic spirit"),
            Legendary::Zeus =>  v = String::from("Zeus – God of lightning and storms"),
            Legendary::ZennyoRyuo =>  v = String::from("ZennyoRyuo – Rain-making dragon"),
            Legendary::ZharPtitsa =>  v = String::from("ZharPtitsa – Glowing bird"),
            Legendary::Zhulong =>  v = String::from("Zhulong – Pig-headed dragon"),
            Legendary::ZhuQue =>  v = String::from("ZhuQue – Fire elemental bird"),
            Legendary::Ziburinis =>  v = String::from("Ziburinis – Forest spirit in the form of a glowing skeleton"),
            Legendary::Zilant =>  v = String::from("Zilant – Flying chicken-legged reptile"),
            Legendary::Zin =>  v = String::from("Zin – Water spirits"),
            Legendary::Ziz =>  v = String::from("Ziz – Giant bird"),
            Legendary::Zlatorog =>  v = String::from("Zlatorog – White golden-horned deer"),
            Legendary::Zmeu =>  v = String::from("Zmeu – Giant with a habit of kidnapping young girls"),
            Legendary::Zmiy =>  v = String::from("Slavic dragonSlavic dragon"),
            Legendary::Zombie =>  v = String::from("Zombie – Re-animated corpse"),
            Legendary::Zorigami =>  v = String::from("Zorigami – Animated clock"),
            Legendary::Zuijin =>  v = String::from("Zuijin – Tutelary spirit"),
            Legendary::ZunberaBo =>  v = String::from("ZunberaBo – Faceless ghost"),
        }
        // We **finally** return the string
        v
    }
    /// Get a long descriptive string of the `Legendary` creature
    pub fn long_description(&self) -> String {
        let v:String;
        match *self {
            Legendary::ABaoAQu =>  v = String::from("A Bao A Qu is a legendary Mewar creature described in Jorge Luis Borges's 1967 Book of Imaginary Beings. Borges claimed to have found it either in an introduction to the Arabian Nights by Richard Francis Burton, or in the book On Malay Witchcraft (1937) by C.C. Iturvuru. The Burton reference was given in the original Spanish, but it was changed to the Iturvuru reference in the English text, possibly to make it sound more exotic, or as a reference to Borges' friend C. C. Iturburu. The writer Antares conjectures that Borges's tale might be inspired by Orang Asli myth, and that 'A Bao A Qu' is a slurring of abang aku meaning 'my elder brother'.  In Borges's story, the A Bao A Qu lives on the steps of the Tower of Victory in Chitor, from the top of which one can see 'the loveliest landscape in the world'. The A Bao A Qu waits on the first step for a man brave enough to try to climb up. Until that point, it lies sleeping, shapeless and translucent, until someone passes. Then, when a man starts climbing, the creature wakes, and follows close behind. As it progresses further and further up, it begins to become clearer and more colorful. It gives off a blue light which increases as it ascends. But it only reaches perfection when the climber reaches the top, and achieves Nirvana, so his acts don't cast any shadows. But almost all the time, the climber cannot reach the top, for they are not perfect. When the A Bao A Qu realizes this, it hangs back, losing color and visibility, and tumbles back down the staircase until it reaches the bottom, once more dormant and shapeless. In doing so, it gives a small cry, so soft that it sounds similar to the rustling of silk. When touched, it feels like the fuzz on the skin of a peach. Only once in its everlasting life has the A Bao A Qu reached its destination at the top of the tower."),
            Legendary::Aatxe =>  v = String::from("Aatxe is a spirit in the folk mythology of the Basque people. His name is literally translated as 'Young Bull', and he is sometimes known as Etsai. He is a cave-dwelling spirit who adopts the form of a young red bull, but being a shapeshifter, sometimes takes the shape of a man. At night, more so in stormy weather, he arises from the hollow which is his lair. He attacks criminals and other malevolent people. He also protects people by making them stay home when danger is near. He is theorized to be a representative of the goddess Mari, or may be an enforcer of her will, punishing people who cheat her. Another name for him is Aatxegorri which means 'young red bull'. It is believed Aatxe inhabited caves and hollows; in many (Isturits, Sare, Errenteria, among others) engravings and paintings depicting aurochs, bulls, and oxen have been found; which implies that this Basque myth has its origins in the Paleolithic."),
            Legendary::Abaasy =>  v = String::from("The Abaasy (Abaahy or Abasy, Yakut: Aбаасы, Abaası; Dolgan: Абааһы, Abaahı; Azerbaijani: Abası; Turkish: Abası; Hungarian: Abaaszi; Bulgarian: Абааси, Abaasi; Russian: Абасы, Abasy; cognate of the Turkic word Abası) are demons in the mythology of the Sakha (also known as the Yakuts). Yakut Shamanism divides the universe into upper and lower layers, with the earth being 'a kind of indeterminate space or matter' in between. The abaasy occupy the lower level, referred to as the underworld or 'kingdom of darkness.'  The abaasy are alleged to be the spirits of the long-time deceased who dwell near graves or in deserted places who otherwise travel about causing destruction. They serve Arson-Duolai, the ruler of the dead, who also swallows people' souls and gives the living diseases. The abaasy can be appeased by blood sacrifices.  The abaasy have been depicted as causing sexual manifestations and madness."),
            Legendary::Abada =>  v = String::from("Äbädä is an innocent forest spirit in Tatar mythology. It looks like an old woman. Äbädä also is represented in mythologies of Siberian peoples.Äbädä is a demon or spirit. He is a Turkic forest being, similar in nature to the İyes. He protects the birds, trees, and animals of the forest; he appears in the shape of a human with blue skin, two great horns, green hair, and a long green beard across his face, carrying a club or whip indicating his mastery of the forest. He can shapeshift into many different forms. As a human, he looks like a peasant with glowing eyes, and his shoes are on backwards. Should one ever encounter an Äbädä, one must thwart him immediately by turning all one's clothes inside out and backwards, and placing one's shoes on the opposite feet."),
            Legendary::Abaia =>  v = String::from("Abaia is a huge, magical eel in Melanesian mythology.  According to Melanesian mythology the Abaia is a type of large eel which dwells at the bottom of freshwater lakes in the Fiji, Solomon and Vanuatu Islands. The Abaia is said to consider all creatures in the lake its children and protects them furiously against anyone who would harm or disturb them. It is said that those who are foolish enough to try to catch the fish from a lake containing the Abaia are immediately overwhelmed by a large wave caused by the thrashing of the Abaia's powerful tail.  Another version of the legend states that if someone were to harm a creature living in the Abaia's home, the Abaia would cause a great rain storm flooding the land and drowning those who had caused the harm. One example of this ability is illustrated in the following story:    One day a man discovered a lake in which were many fish, and at the bottom of the lake lived a magic eel, but the man knew it not. He caught many fish and returned the next day with the people of his village whom he had told of his discovery, and they also were very successful, while one woman even laid hold of the great eel, Abaia, who dwelt in the depths of the lake, though he escaped her. Now Abaia was angry that his fish had been caught and that he himself had been seized, so he caused a great rain to fall that night, and the waters of the lake also rose, and all the people were drowned except an old woman who had not eaten of the fish and who saved herself in a tree.  Although it would seem that the magical powers of the Abaia are the byproduct of human imagination and its fear of the unknown, it has been suggested that the Abaia legend may have stemmed from encounters with an actual undiscovered species of giant eel living at the bottom of these remote lakes."),
            Legendary::Abarimon =>  v = String::from("Abarimon or antipode in mythology are people whose feet are turned backwards, but in spite of this handicap were able to run at great speed.  In Europe, this tribe was first described by Pliny the Elder, in his book, Natural History (VII 11), who considered them to be native to India. A similar tale is recounted by Aulus Gellius in Attic Nights.  They lived side by side with wild animals and attempts to capture them failed because they were so savage. Pliny refers to information that originates from Baiton, which was Alexander the Great's Land Surveyor. Baiton says that the abarimons could only breathe the air in their own domestic valleys. Because of the special quality of air, which meant if it was breathed for a long period of time, it would be impossible to breathe any other type of air. Therefore the inhabitants were unable to leave the valley and live anywhere else. And so it was impossible to capture them and bring them to the courts of a distant ruler, or to the great Macedonian conquest.  It is possible that this is an overly designed ethnographic description of an ancient wildlife strain in the areas near Himalayas.  According to another legend, Abarimon is mentioned as a landscape in Scythia, a valley of Mount Imaus, (which may be identical to Hindukush or the Himalayan Mountains).Later, Abarimon has been briefly described in Thomas Cooper's Thesaurus Linguae Romanae et Britannicae, as a tribe in the country Tataria . During the Middle Ages, some mapdrawers, after a familiar heliocentric view, have made monsters in the form of Abarimon people and placed them at the outer border of the world."),
            Legendary::Abath =>  v = String::from("An Abath is a legendary creature resembling a unicorn, first appearing in records in the 16th century.  Accounts of the Abath were brought back by 16th-century European travellers to the Malay Peninsula. Described as female, with a single horn growing from its forehead, it is speculated that these were probably the result of a half-glimpsed Javan or Sumatran rhinoceros. Like the unicorn, a powder made from this horn supposedly served both as an aphrodisiac and as an antidote to poison. However, since the unicorn was invariably represented as male, and since there was only ever one in existence at any time, the Abath seems to have developed independently from the European myths of the one-horned creature."),
            Legendary::AburaSumashi =>  v = String::from("This spirit, which surprises people on the Kusazumigoe mountain pass, is thought to be the ghost of a human who stole oil.      In the days before electricity, oil was a very valuable commodity, necessary for lighting and heating a house. As such, the theft of oil, particularly from temples and shrines, could lead to punishment via reincarnation as a yōkai.  In modern media the abura-sumashi is often depicted as, 'a squat creature with a straw-coat covered body and a potato-like or stony head,' an appearance inspired by the artwork of Shigeru Mizuki."),
            Legendary::Acephali =>  v = String::from("Various species of mythical headless men were rumoured, in antiquity and later, to inhabit remote parts of the world. They are variously known as akephaloi (Greek ἀκέφαλοι, 'headless ones') or Blemmyes (Latin: Blemmyae; Greek: βλέμμυες) and described as lacking a head, with their facial features on their chest. These were at first described as inhabitants of ancient Libya or the Nile system (Aethiopia). Later traditions confined their habitat to a particular island in the Brisone River, or shifted it to India.  Blemmyes are said to occur in two types: with eyes on the chest or with the eyes on the shoulders. Epiphagi, a variant name for the headless people of the Brisone, is sometimes used as a term referring strictly to the eyes-on-the-shoulders type. "),
            Legendary::Acheri =>  v = String::from("Acheri – Disease-bringing ghost."),
            Legendary::Achlis =>  v = String::from("Achlis – Curious elk."),
            Legendary::AdarLlwchGwin =>  v = String::from("Adar Llwch Gwin – Giant birds that understand human languages."),
            Legendary::Adaro =>  v = String::from("Adaro – Malevolent merfolk."),
            Legendary::Adhene =>  v = String::from("Adhene – Nature spirit."),
            Legendary::Adlet =>  v = String::from("Adlet – Vampiric dog-human hybrid"),
            Legendary::Adroanzi =>  v = String::from("Adroanzi – Nature spirit."),
            Legendary::Adze =>  v = String::from("Adze – African vampiric-forest being."),
            Legendary::Aerico =>  v = String::from("Aerico – Disease demon."),
            Legendary::AEsir =>  v = String::from("AEsir – Norse deities."),
            Legendary::Afanc =>  v = String::from("Afanc."),
            Legendary::Agni =>  v = String::from("Agni – God of fire and sacrifices."),
            Legendary::Agathodaemon =>  v = String::from("Agathodaemon – Spirit of vinefields and grainfields."),
            Legendary::Agloolik =>  v = String::from("Agloolik – Ice spirit that aids hunters and fishermen."),
            Legendary::Agogwe =>  v = String::from("Agogwe – Small, ape-like humanoid."),
            Legendary::Ahkiyyini =>  v = String::from("Ahkiyyini – Animated skeleton that causes shipwrecks."),
            Legendary::Ahuizotl =>  v = String::from("Ahuizotl – Anthropophagous dog-monkey hybrid."),
            Legendary::Ahura =>  v = String::from("Ahura – Zoroastrian spirits."),
            Legendary::Aigamuxa =>  v = String::from("Aigamuxa – Anthropophagous humanoid with eyes in its instep."),
            Legendary::Aigikampoi =>  v = String::from("Aigikampoi – Fish-tailed goat."),
            Legendary::Airavata =>  v = String::from("Airavata – Divine elephant."),
            Legendary::Aitu =>  v = String::from("Aitu – Malevolent spirits or demons."),
            Legendary::Aitvaras =>  v = String::from("Aitvaras – Household spirit."),
            Legendary::Ajatar =>  v = String::from("Ajatar – Dragon/snake female spirit, is said to spread diseases"),
            Legendary::Akateko =>  v = String::from("Akateko – Tree-dwelling monster."),
            Legendary::Akhlut =>  v = String::from("Akhlut – Orca-wolf shapeshifter."),
            Legendary::Akka =>  v = String::from("Akka – Female spirits or minor goddesses."),
            Legendary::Akki =>  v = String::from("Akki – Large, grotesque humanoid."),
            Legendary::Akkorokamui =>  v = String::from("Akkorokamui – Sea monster."),
            Legendary::Akuma =>  v = String::from("Akuma – Evil spirit or devil"),
            Legendary::Akupara =>  v = String::from("Akupara – Giant turtle that supports the world."),
            Legendary::AkurojinNoHi =>  v = String::from("AkurojinNoHi – Ghostly flame which causes disease."),
            Legendary::Al =>  v = String::from("Al – Spirit that steals unborn babies and livers from pregnant women."),
            Legendary::Ala =>  v = String::from("Ala – Bad weather demon."),
            Legendary::Alal =>  v = String::from("Alal – Queen of the full moon."),
            Legendary::Alan =>  v = String::from("Alan – Winged humanoid that steals reproductive waste to make children."),
            Legendary::Alce =>  v = String::from("Alce – Wingless griffin."),
            Legendary::Aleya =>  v = String::from("Aleya – Spirit of a dead fisherman."),
            Legendary::Alicanto =>  v = String::from("Alicanto – Bird that eats gold and silver."),
            Legendary::Alicorn =>  v = String::from("Alicorn – Winged unicorn."),
            Legendary::Alkonost =>  v = String::from("Alkonost – Angelic bird with human head and breasts."),
            Legendary::Allocamelus =>  v = String::from("Allocamelus – Ass-camel hybrid."),
            Legendary::Almas =>  v = String::from("Almas – Savage humanoid."),
            Legendary::AlMiRaj =>  v = String::from("AlMiRaj – One-horned rabbit."),
            Legendary::Aloja =>  v = String::from("Aloja – Female water spirit."),
            Legendary::AlomBagWinnosis =>  v = String::from("AlomBagWinnosis – Little people and tricksters."),
            Legendary::Alp =>  v = String::from("Alp – Male night-demon."),
            Legendary::Alphyn =>  v = String::from("Alphyn – Lion-like creature, sometimes with dragon or goat forelegs."),
            Legendary::AlpLuachra =>  v = String::from("AlpLuachra – Parasitic fairy."),
            Legendary::AlRakim =>  v = String::from("AlRakim – Guard dog of the Seven Sleepers."),
            Legendary::Alseid =>  v = String::from("Alseid – Grove nymph."),
            Legendary::Alu =>  v = String::from("Alu – Leprous demon."),
            Legendary::Alux =>  v = String::from("Alux – Little people."),
            Legendary::Amaburakosagi =>  v = String::from("Amaburakosagi – Ritual disciplinary demon from Shikoku."),
            Legendary::Amala =>  v = String::from("Amala – Giant who holds up the world."),
            Legendary::Amamehagi =>  v = String::from("Amamehagi – Ritual disciplinary demon from Hokuriku."),
            Legendary::Amanojaku =>  v = String::from("Amanojaku – Small demon."),
            Legendary::Amarok =>  v = String::from("Amarok – Giant wolf."),
            Legendary::Amarum =>  v = String::from("Amarum – Water boa spirit."),
            Legendary::AmazakeBabaa =>  v = String::from("AmazakeBabaa – Disease-causing hag."),
            Legendary::Amemasu =>  v = String::from("Amemasu – Lake monster."),
            Legendary::Ammit =>  v = String::from("Ammit – Female demon who was part lion, hippopotamus and crocodile and devoured the souls of the wicked."),
            Legendary::Amoronagu =>  v = String::from("Amoronagu – Tennyo from the island of Amami Ōshima."),
            Legendary::Amphiptere =>  v = String::from("Amphiptere – Winged serpent."),
            Legendary::Amphisbaena =>  v = String::from("Amphisbaena – Serpent with a head at each end."),
            Legendary::Anak =>  v = String::from("Anak – Giant."),
            Legendary::Androsphinx =>  v = String::from("Androsphinx – Human-headed sphinx."),
            Legendary::Angel =>  v = String::from("Angel – Divine beings of Heaven who act as mediators between God and humans; the counterparts of Demons."),
            Legendary::Anqa =>  v = String::from("Anqa – Legendary Huge Satanic Eagle with Human Face. sometimes can resurrect herself like phoenix did."),
            Legendary::AniHyuntikwalaski =>  v = String::from("AniHyuntikwalaski – Lightning spirit."),
            Legendary::Ankou =>  v = String::from("Ankou – Skeletal grave watcher with a lantern and scythe."),
            Legendary::Anmo =>  v = String::from("Anmo – Ritual disciplinary demon from Iwate Prefecture."),
            Legendary::Antaeus =>  v = String::from("Antaeus – Giant who was extremely strong as long as he remained in contact with the ground."),
            Legendary::Anubis =>  v = String::from("Anubis – God of the Underworld"),
            Legendary::AnteroVipunen =>  v = String::from("AnteroVipunen – Subterranean giant."),
            Legendary::Anzu =>  v = String::from("Anzu – Divine storm bird"),
            Legendary::AoAo =>  v = String::from("AoAo – Anthropophagous peccary or sheep."),
            Legendary::Aobozu =>  v = String::from("Aobozu – Blue monk who kidnaps children."),
            Legendary::Apkallu =>  v = String::from("Apkallu – Fish-human hybrid that attends the god Enki."),
            Legendary::Apsaras =>  v = String::from("Apsaras – Female cloud spirit."),
            Legendary::Aqrabuamelu =>  v = String::from("Aqrabuamelu – Human-scorpion hybrid."),
            Legendary::ArdatLili =>  v = String::from("ArdatLili – Disease demon."),
            Legendary::ArgusPanoptes =>  v = String::from("ArgusPanoptes – Hundred-eyed giant."),
            Legendary::ArikuraNoBaba =>  v = String::from("ArikuraNoBaba – Old woman with magical powers."),
            Legendary::Arimaspi =>  v = String::from("Arimaspi – One-eyed humanoid."),
            Legendary::Arion =>  v = String::from("Arion – Swift green-maned talking horse."),
            Legendary::ArkanSonney =>  v = String::from("ArkanSonney – Fairy hedgehog."),
            Legendary::Asag =>  v = String::from("Asag – Hideous rock demon."),
            Legendary::Asakku =>  v = String::from("Asakku – Demon."),
            Legendary::Asanbosam =>  v = String::from("Asanbosam – Iron-toothed vampire."),
            Legendary::Asena =>  v = String::from("Asena – Blue-maned wolf."),
            Legendary::ASeneeKiWakw =>  v = String::from("ASeneeKiWakw – Stone giant."),
            Legendary::AshiMagari =>  v = String::from("AshiMagari – Invisible tendril that impedes movement."),
            Legendary::Asiman =>  v = String::from("Asiman – Vampiric possession spirit."),
            Legendary::Askefrue =>  v = String::from("Askefrue – Female tree spirit."),
            Legendary::AskWeeDaEed =>  v = String::from("AskWeeDaEed – Fire elemental and spectral fire."),
            Legendary::Asobibi =>  v = String::from("Asobibi – Spectral fire from Kōchi Prefecture."),
            Legendary::Aspidochelone =>  v = String::from("Aspidochelone – Island-sized whale or sea turtle."),
            Legendary::Asrai =>  v = String::from("Asrai – Water spirit."),
            Legendary::Astomi =>  v = String::from("Astomi – Humanoid sustained by pleasant smells instead of food."),
            Legendary::Asura =>  v = String::from("Asura – Hindu malevolent divinities."),
            Legendary::Aswang =>  v = String::from("Aswang – Carrion-eating humanoid."),
            Legendary::Atomy =>  v = String::from("Atomy – Surprisingly small creature."),
            Legendary::AtoOiKozo =>  v = String::from("AtoOiKozo – Invisible spirit that follows people."),
            Legendary::Atshen =>  v = String::from("Atshen – Anthropophagous spirit."),
            Legendary::Auloniad =>  v = String::from("Auloniad – Pasture nymph."),
            Legendary::Avalerion =>  v = String::from("Avalerion – King of the birds."),
            Legendary::AwaHonDo =>  v = String::from("AwaHonDo – Insect spirit."),
            Legendary::Axex =>  v = String::from("Axex – Falcon-lion hybrid."),
            Legendary::Ayakashi =>  v = String::from("Ayakashi – Sea serpent that travels over boats in an arc while dripping oil."),
            Legendary::AyakashiNoAyashibi =>  v = String::from("AyakashiNoAyashibi – Spectral fire from Ishikawa Prefecture."),
            Legendary::Aziza =>  v = String::from("Aziza – Little people that help hunters."),
            Legendary::Azukiarai =>  v = String::from("Azukiarai – Spirit that washes azuki beans along riversides."),
            Legendary::Azukitogi =>  v = String::from("Azukitogi – Spirit that washes azuki beans along riversides."),
            Legendary::Azukibabaa =>  v = String::from("Azukibabaa – Bean-grinding hag who devours people."),
            Legendary::Ba =>  v = String::from("Ba – Soul of the deceased, depicted as a bird or a human-headed bird"),
            Legendary::BabaYaga =>  v = String::from("BabaYaga – Forest spirit and hag"),
            Legendary::Baccoo =>  v = String::from("Baccoo – Malevolent little people"),
            Legendary::Badalisc =>  v = String::from("Badalisc – Goat-like creature from the southern central Alps"),
            Legendary::Bagiennik =>  v = String::from("Bagiennik – Malevolent water spirit"),
            Legendary::Bahamut =>  v = String::from("Bahamut – Giant fish"),
            Legendary::BaiZe =>  v = String::from("BaiZe – Talking beast which handed down knowledge on harmful spirits"),
            Legendary::BaJiaoGui =>  v = String::from("BaJiaoGui – Banana tree spirit"),
            Legendary::Bak =>  v = String::from("Bak - Assamese shape-shifting aqueous creature"),
            Legendary::BakeKujira =>  v = String::from("BakeKujira – Ghostly whale skeleton that drifts along the coastline of Shimane Prefecture"),
            Legendary::Bakeneko =>  v = String::from("Bakeneko – Magical cat"),
            Legendary::Bakezori =>  v = String::from("Bakezori – Animated straw sandal"),
            Legendary::Bakhtak =>  v = String::from("Bakhtak – Night demon"),
            Legendary::Baku =>  v = String::from("Baku – Dream-devouring, tapir-like creature"),
            Legendary::Bakunawa =>  v = String::from("Bakunawa – Sea serpent that causes eclipses"),
            Legendary::Balaur =>  v = String::from("Balaur – Multi-headed dragon"),
            Legendary::Baloz =>  v = String::from("Baloz – Sea monster"),
            Legendary::Bannik =>  v = String::from("Bannik – Bathhouse spirit"),
            Legendary::Banshee =>  v = String::from("Banshee – Screaming death spirit"),
            Legendary::BaobhanSith =>  v = String::from("BaobhanSith – Beautiful vampiric seductresses who prey on young travelers"),
            Legendary::Barbegazi =>  v = String::from("Barbegazi – Dwarf with giant, snowshoe-like feet"),
            Legendary::Bardha =>  v = String::from("Bardha – Mountain spirit"),
            Legendary::Bardi =>  v = String::from("Bardi – Shapechanging death spirit"),
            Legendary::Barghest =>  v = String::from("Yorkshire black dogYorkshire black dog"),
            Legendary::BarJuchne =>  v = String::from("BarJuchne – Gigantic bird"),
            Legendary::BarnacleGeese =>  v = String::from("BarnacleGeese – Geese which hatch from barnacles"),
            Legendary::Barong =>  v = String::from("Barong – Tutelary spirit"),
            Legendary::Basajaun =>  v = String::from("Basajaun – Ancestral, megalith-building race"),
            Legendary::BasCelik =>  v = String::from("BasCelik – Powerful, evil winged man whose soul is not held by his body and can be subdued only by causing him to suffer dehydration"),
            Legendary::Bashe =>  v = String::from("Bashe – Elephant-swallowing serpent"),
            Legendary::BasiliscoChilote =>  v = String::from("BasiliscoChilote – Chicken-serpent hybrid"),
            Legendary::Basilisk =>  v = String::from("Basilisk – Multi-limbed, venomous lizard"),
            Legendary::Bathala =>  v = String::from("Bathala – Primordial god of creation"),
            Legendary::Batibat =>  v = String::from("Batibat – Female night-demon"),
            Legendary::Batsu =>  v = String::from("Batsu – Drought spirit"),
            Legendary::Baubas =>  v = String::from("Baubas – Malevolent spirit"),
            Legendary::Baykok =>  v = String::from("Baykok – Flying skeleton"),
            Legendary::BeastOfBrayRoad =>  v = String::from("BeastOfBrayRoad – Werewolf"),
            Legendary::BeanNighe =>  v = String::from("BeanNighe"),
            Legendary::Behemoth =>  v = String::from("Behemoth – Massive beast, possibly like a dinosaur"),
            Legendary::Bendigeidfran =>  v = String::from("Bendigeidfran – Giant king"),
            Legendary::Bennu =>  v = String::from("Bennu the Phoenix"),
            Legendary::Berehynia =>  v = String::from("Berehynia – Water spirit"),
            Legendary::Bergrisar =>  v = String::from("Bergrisar in Jotunheim"),
            Legendary::Bergsra =>  v = String::from("Bergsra – Mountain spirit"),
            Legendary::BestialBeast =>  v = String::from("BestialBeast – Centauroid specter"),
            Legendary::BetobetoSan =>  v = String::from("BetobetoSan – Invisible spirit which follows people at night, making the sound of footsteps"),
            Legendary::Bhuta =>  v = String::from("Bhuta – Ghost of someone killed by execution or suicide"),
            Legendary::BiBlouk =>  v = String::from("BiBlouk – Female, cannibalistic, partially invisible monster"),
            Legendary::Bies =>  v = String::from("Bies – Demon"),
            Legendary::Bigfoot =>  v = String::from("Bigfoot – Forest-dwelling hominid cryptid."),
            Legendary::Binbogami =>  v = String::from("Binbogami – Spirit of poverty"),
            Legendary::BishopFish =>  v = String::from("BishopFish – Fish-like humanoid"),
            Legendary::BiwaBokuboku =>  v = String::from("BiwaBokuboku – Animated biwa"),
            Legendary::BlackAnnis =>  v = String::from("BlackAnnis – Blue-faced hag"),
            Legendary::BlackDog =>  v = String::from("BlackDog – Canine death spirit"),
            Legendary::BlackShuck =>  v = String::from("Norfolk, Essex, and Suffolk black dogNorfolk, Essex, and Suffolk black dog"),
            Legendary::Blafard =>  v = String::from("Imaginary creature from the early United States of AmericaImaginary creature from the early United States of America"),
            Legendary::Blemmyae =>  v = String::from("Blemmyae – Headless humanoid with face in torso"),
            Legendary::BloodyBones =>  v = String::from("BloodyBones – Water bogeyman"),
            Legendary::Bludnik =>  v = String::from("Bludnik – Mischievous gnome"),
            Legendary::BlueCrow =>  v = String::from("BlueCrow – Giant amazonian bird"),
            Legendary::Bluecap =>  v = String::from("Bluecap – Mine-dwelling fairy"),
            Legendary::Bodach =>  v = String::from("Bodach – Malevolent spirit"),
            Legendary::Bogeyman =>  v = String::from("Bogeyman – Malevolent spirit"),
            Legendary::Boggart =>  v = String::from("Boggart – Malevolent household spirit"),
            Legendary::Boginki =>  v = String::from("Boginki – Nature spirit"),
            Legendary::Bogle =>  v = String::from("Bogle – Malevolent spirit"),
            Legendary::BoiTata =>  v = String::from("BoiTata – Giant snake"),
            Legendary::Bolla =>  v = String::from("Bolla – Dragon"),
            Legendary::Bonnacon =>  v = String::from("Bonnacon – Bull-horse hybrid with flaming dung"),
            Legendary::BooHag =>  v = String::from("BooHag – Vampire-like creature that steals energy from sleeping victims"),
            Legendary::Boobrie =>  v = String::from("Boobrie – Roaring water bird"),
            Legendary::Bozaloshtsh =>  v = String::from("Bozaloshtsh – Death spirit"),
            Legendary::Brag =>  v = String::from("Brag – Malevolent water horse"),
            Legendary::Brownie =>  v = String::from("Brownie – Benevolent household spirit"),
            Legendary::Broxa =>  v = String::from("Broxa – Nocturnal bird that drains goats of their milk"),
            Legendary::Bucca =>  v = String::from("Bucca – Male sea-spirit, a merman, that inhabited mines and coastal communities as a hobgoblin during storms"),
            Legendary::Bokkenrijders =>  v = String::from("Bokkenrijders – Ghosts/devils riding flying goats; co-opted by bandits to instil fear during raids"),
            Legendary::Bugbear =>  v = String::from("Bugbear – Bearlike goblin"),
            Legendary::Buggane =>  v = String::from("Buggane – Ogre-like humanoid"),
            Legendary::BugulNoz =>  v = String::from("BugulNoz – Extremely ugly, but kind, forest spirit"),
            Legendary::Bukavac =>  v = String::from("Bukavac – Six-legged lake monster"),
            Legendary::Bunyip =>  v = String::from("Bunyip – Horse-walrus hybrid lake monster"),
            Legendary::BunnyMan =>  v = String::from("BunnyMan West Virginia Urban Legend – Spirit/Maniac that wears a bunny costume and wields an axe"),
            Legendary::BushDaiDai =>  v = String::from("BushDaiDai – Spirit that seduces and kills men"),
            Legendary::Byangoma =>  v = String::from("Byangoma – Fortune-telling birds"),
            Legendary::Bysen =>  v = String::from("Bysen – Diminutive forest spirit"),
            Legendary::Cabeiri =>  v = String::from("Cabeiri – Smith and wine spirit"),
            Legendary::Cacus =>  v = String::from("Cacus – Fire-breathing giant"),
            Legendary::Cadejo =>  v = String::from("Cadejo – Cow-sized dog-goat hybrid"),
            Legendary::Cailleach =>  v = String::from("Cailleach – Divine creator and weather deity hag"),
            Legendary::Caipora =>  v = String::from("Caipora – Fox-human hybrid and nature spirit"),
            Legendary::Caladrius =>  v = String::from("Caladrius – White bird that can foretell if a sick person will recover or die"),
            Legendary::Calingi =>  v = String::from("Calingi – Humanoid with an eight-year lifespan"),
            Legendary::Callitrix =>  v = String::from("Callitrix – Apes who always bear twins, one the mother loves, the other it hates"),
            Legendary::CalydonianBoar =>  v = String::from("CalydonianBoar – Giant, chthonic boar"),
            Legendary::Calygreyhound =>  v = String::from("Calygreyhound – Wildcat-deer/antelope-eagle-ox-lion hybrid :>"),
            Legendary::Camahueto =>  v = String::from("Camahueto – One-horned calf"),
            Legendary::Cambion =>  v = String::from("Cambion – Offspring of a human and an incubus or succubus"),
            Legendary::Campe =>  v = String::from("Campe – Dragon-human-scorpion hybrid"),
            Legendary::Camulatz =>  v = String::from("Camulatz – Bird that ate the heads of the first men"),
            Legendary::Candileja =>  v = String::from("Candileja – Spectral, fiery hag"),
            Legendary::Canaima =>  v = String::from("Canaima – Were-jaguar"),
            Legendary::Canotila =>  v = String::from("Canotila – Little people and tree spirits"),
            Legendary::Caoineag =>  v = String::from("Caoineag"),
            Legendary::Chapa =>  v = String::from("Chapa – Beaver spirit"),
            Legendary::Chareng =>  v = String::from("///(Manipuri)-Semi-hornbill, semi-human creature-Semi-hornbill, semi-human creature"),
            Legendary::Capcaun =>  v = String::from("Capcaun – Large, monstrous humanoid"),
            Legendary::Carbuncle =>  v = String::from("Carbuncle – Small creature with a jewel on its head"),
            Legendary::Catoblepas =>  v = String::from("Catoblepas – Scaled buffalo-hog hybrid"),
            Legendary::CatSidhe =>  v = String::from("CatSidhe – Fairy cat"),
            Legendary::Ceasg =>  v = String::from("Ceasg — Benevolent Scottish mermaids"),
            Legendary::CeffylDwr =>  v = String::from("CeffylDwr – Malevolent water horse"),
            Legendary::Centaur =>  v = String::from("Centaur – Human-horse hybrid"),
            Legendary::Centicore =>  v = String::from("Centicore – Horse-Antelope-Lion-Bear hybrid"),
            Legendary::Cerastes =>  v = String::from("Cerastes – Extremely flexible, horned snake"),
            Legendary::Cerberus =>  v = String::from("Cerberus – Three-headed dog that guards the entrance to the underworld"),
            Legendary::Cercopes =>  v = String::from("Cercopes – Mischievous forest spirit"),
            Legendary::Cericopithicus =>  v = String::from("Cericopithicus – Apes who always bear twins, one the mother loves, the other it hates"),
            Legendary::CeryneianHind =>  v = String::from("CeryneianHind – Hind with golden antlers and bronze or brass hooves"),
            Legendary::Cetan =>  v = String::from("Cetan – Hawk spirit"),
            Legendary::Cetus =>  v = String::from("Cetus in length, its spines being a cubit in thickness, and its skeleton taller at the shoulder than an elephant."),
            Legendary::Chakora =>  v = String::from("Chakora – Lunar bird"),
            Legendary::Chalkydri =>  v = String::from("Chalkydri – Angelic birds"),
            Legendary::Chamrosh =>  v = String::from("Chamrosh – Dog-bird hybrid"),
            Legendary::Chaneque =>  v = String::from("Chaneque – Little people and nature spirits"),
            Legendary::Changeling =>  v = String::from("Changeling substituted for a kidnapped human child"),
            Legendary::Charybdis =>  v = String::from("Charybdis – Sea monster in the form of a giant mouth"),
            Legendary::Chenoo =>  v = String::from("Chenoo or were possessed by evil spirits, turning their hearts to ice"),
            Legendary::Chepi =>  v = String::from("Chepi – Ancestral spirit that instructs tribe members"),
            Legendary::Cherufe =>  v = String::from("Cherufe – Volcano-dwelling monster"),
            Legendary::ChevalMallet =>  v = String::from("ChevalMallet – Evil horse who runs away with travelers"),
            Legendary::ChevalGauvin =>  v = String::from("ChevalGauvin – Evil horse who drowns riders, similar to kelpie"),
            Legendary::Chibaiskweda =>  v = String::from("Chibaiskweda – Ghost of an improperly buried person"),
            Legendary::Chichevache =>  v = String::from("Human-faced cow that feeds on good womenHuman-faced cow that feeds on good women"),
            Legendary::Chickcharney =>  v = String::from("Chickcharney – Bird-mammal hybrid"),
            Legendary::Chimaera =>  v = String::from("Chimaera – Lion-goat-snake hybrid"),
            Legendary::Chindi =>  v = String::from("Chindi – Vengeful ghost that causes dust devils"),
            Legendary::Chinthe =>  v = String::from("Chinthe – Temple-guarding feline, similar to Chinese Shi and Japanese Shisa"),
            Legendary::Chitauli =>  v = String::from("Chitauli – Human-lizard hybrid"),
            Legendary::Chochinobake =>  v = String::from("Chochinobake – Animated paper lantern"),
            Legendary::Chol =>  v = String::from("Chol – Regenerative bird"),
            Legendary::Chollima =>  v = String::from("Chollima – Supernaturally fast horse"),
            Legendary::Chonchon =>  v = String::from("Chonchon – Disembodied, flying head"),
            Legendary::Choorile =>  v = String::from("Choorile – Ghost of a woman that died in childbirth"),
            Legendary::Chromandi =>  v = String::from("Chromandi – Hairy savage with dog teeth"),
            Legendary::Chrysaor =>  v = String::from("Chrysaor – The giant son of the gorgon Medusa."),
            Legendary::Chrysomallus =>  v = String::from("Chrysomallus – Golden winged ram"),
            Legendary::Chukwa =>  v = String::from("Chukwa – Giant turtle that supports the world"),
            Legendary::Chupacabra =>  v = String::from("Chupacabra – Cryptid beast named for its habit of sucking the blood of livestock"),
            Legendary::Churel =>  v = String::from("Churel – Vampiric, female ghost"),
            Legendary::Ciguapa =>  v = String::from("Ciguapa – Malevolent seductress"),
            Legendary::Cihuateteo =>  v = String::from("Cihuateteo – Ghost of women that died in childbirth"),
            Legendary::Cikavac =>  v = String::from("Cikavac – Bird that serves its owner"),
            Legendary::CinnamonBird =>  v = String::from("CinnamonBird – Giant bird that makes its nest out of cinnamon"),
            Legendary::Cipactli =>  v = String::from("Cipactli – Sea monster, crocodile-fish hybrid"),
            Legendary::CireinCroin =>  v = String::from("CireinCroin – Sea serpent"),
            Legendary::Coblynau =>  v = String::from("Coblynau – Little people and mine spirits"),
            Legendary::Cockatrice =>  v = String::from("Cockatrice – Chicken-lizard hybrid"),
            Legendary::Cofgod =>  v = String::from("Cofgod – Cove god"),
            Legendary::ColchisBull =>  v = String::from("ColchisBull – Bronze-hoofed bulls"),
            Legendary::ColoColo =>  v = String::from("ColoColo – Rat-bird hybrid that can shapeshift into a serpent"),
            Legendary::CorycianNymphs =>  v = String::from("CorycianNymphs – Nymph of the Corycian Cave"),
            Legendary::CretanBull =>  v = String::from("CretanBull – Monstrous bull"),
            Legendary::Crinaeae =>  v = String::from("Crinaeae – Fountain nymph"),
            Legendary::Criosphinx =>  v = String::from("Criosphinx – Ram-headed sphinx"),
            Legendary::Crocotta =>  v = String::from("Crocotta – Monstrous dog-wolf"),
            Legendary::TheCuBird =>  v = String::from("TheCuBird – El Pájaro Cu; a bird."),
            Legendary::Cuco =>  v = String::from("Cuco – Bogeyman"),
            Legendary::Cucuy =>  v = String::from("Cucuy – Malevolent spirit"),
            Legendary::Cuegle =>  v = String::from("Cuegle – Monstrous, three-armed humanoid"),
            Legendary::Cuelebre =>  v = String::from("Cuelebre – Dragon"),
            Legendary::Curupira =>  v = String::from("Curupira – Nature spirit"),
            Legendary::CuSith =>  v = String::from("CuSith – Gigantic fairy dog"),
            Legendary::CwnAnnwn =>  v = String::from("CwnAnnwn – Underworld hunting dog"),
            Legendary::Cyclops =>  v = String::from("Cyclops – One-eyed giant"),
            Legendary::Cyhyraeth =>  v = String::from("Cyhyraeth – Death spirit"),
            Legendary::Cynocephalus =>  v = String::from("Cynocephalus – Dog-headed humanoid"),
            Legendary::Dactyl =>  v = String::from("Dactyl – Little people and smith and healing spirits"),
            Legendary::Daemon =>  v = String::from("Daemon – Incorporeal spirit"),
            Legendary::Dahu =>  v = String::from("Dahu – Similar to a deer or ibex; legs on one side of its body are shorter than on the other side"),
            Legendary::Daidarabotchi =>  v = String::from("Daidarabotchi – Giant responsible for creating many geographical features in Japan"),
            Legendary::Daitengu =>  v = String::from("Daitengu – Most powerful class of tengu, each of whom lives on a separate mountain"),
            Legendary::Daitya =>  v = String::from("Daitya – Giant"),
            Legendary::Danava =>  v = String::from("Danava – Water demon"),
            Legendary::Daphnaie =>  v = String::from("Daphnaie – Laurel tree nymph"),
            Legendary::DatsueBa =>  v = String::from("DatsueBa – Old woman who steals clothes from the souls of the dead"),
            Legendary::DeadSeaApes =>  v = String::from("DeadSeaApes – Human tribe turned into apes for ignoring Moses' message"),
            Legendary::DedMoroz =>  v = String::from("DedMoroz – A winter spirit who delivers gifts to children on New Year's Eve"),
            Legendary::DeerWoman =>  v = String::from("DeerWoman – Human-deer hybrid"),
            Legendary::Deity =>  v = String::from("Deity – Preternatural or supernatural possibly immortal being"),
            Legendary::Demigod =>  v = String::from("Demigod – Half human, half god"),
            Legendary::Dhampir =>  v = String::from("Dhampir – Human/vampire hybrid"),
            Legendary::DiaoSiGui =>  v = String::from("DiaoSiGui – Hanged ghost"),
            Legendary::Dilong =>  v = String::from("Dilong – Earth dragon"),
            Legendary::Dip =>  v = String::from("Dip – Demonic and vampiric dog"),
            Legendary::DiPenates =>  v = String::from("DiPenates – House spirit"),
            Legendary::Dipsa =>  v = String::from("Dipsa – Extremely venomous snake"),
            Legendary::Dirawong =>  v = String::from("Dirawong – Goanna spirit"),
            Legendary::DiSmaUndarJordi =>  v = String::from("DiSmaUndarJordi – Little people and nature spirits"),
            Legendary::Diwata =>  v = String::from("Diwata – Tree spirit"),
            Legendary::Djall =>  v = String::from("Djall – Devil"),
            Legendary::DobharChu =>  v = String::from("DobharChu – King otter"),
            Legendary::DoGakwHoWad =>  v = String::from("DoGakwHoWad – Little people"),
            Legendary::Dokkaebi =>  v = String::from("Dokkaebi – Grotesque, horned humanoids"),
            Legendary::Dokkalfar =>  v = String::from("Dokkalfar – Male ancestral spirits; the Dark Elves"),
            Legendary::Dola =>  v = String::from("Dola – Tutelary and fate spirit"),
            Legendary::Domovoi =>  v = String::from("Domovoi – House spirit"),
            Legendary::Doppelganger =>  v = String::from("Doppelganger – Ghostly double"),
            Legendary::Drac =>  v = String::from("Drac – Winged sea serpent"),
            Legendary::Drakon =>  v = String::from("Drakon – Greek dragons"),
            Legendary::Drakaina =>  v = String::from("Drakaina – Dragons depicted with female characteristics"),
            Legendary::Dragon =>  v = String::from("Dragon winged reptiles"),
            Legendary::DragonTurtle =>  v = String::from("DragonTurtle – Giant turtle with dragon-like head"),
            Legendary::Drangue =>  v = String::from("Drangue – Semi-human winged warriors"),
            Legendary::Draugr =>  v = String::from("Draugr – Undead"),
            Legendary::Drekavac =>  v = String::from("Drekavac – Restless ghost of an unbaptised child"),
            Legendary::DropBear =>  v = String::from("DropBear – Large carnivorous koala that hunts by dropping on its prey from trees"),
            Legendary::Drow =>  v = String::from("Drow – Cavern spirit"),
            Legendary::Drude =>  v = String::from("Drude – Possessing demon"),
            Legendary::Druk =>  v = String::from("Druk – Dragon"),
            Legendary::Dryad =>  v = String::from("Dryad – Tree nymph"),
            Legendary::Duende =>  v = String::from("Duende – Little people and forest spirits"),
            Legendary::Duergar =>  v = String::from("Duergar – Malevolent little people"),
            Legendary::Dullahan =>  v = String::from("Dullahan – Headless death spirit"),
            Legendary::Duwende =>  v = String::from("Duwende – Little people, some are house spirits, others nature spirits"),
            Legendary::Dvergr =>  v = String::from("Dvergr – Subterranean little people smiths"),
            Legendary::Dvorovoi =>  v = String::from("Dvorovoi – Courtyard spirit"),
            Legendary::Dwarf =>  v = String::from("Dwarf – Little people nature spirits"),
            Legendary::Dybbuk =>  v = String::from("Dybbuk that possesses the living"),
            Legendary::DzeeDzeeBonDa =>  v = String::from("DzeeDzeeBonDa – Hideous monster"),
            Legendary::Dzunukwa =>  v = String::from("Dzunukwa – Child-eating hag"),
            Legendary::EasterBunny =>  v = String::from("The Easter Bunny (also called the Easter Rabbit or Easter Hare) is a folkloric figure and symbol of Easter, depicted as a rabbit—sometimes dressed with clothes—bringing Easter eggs. Originating among German Lutherans, the 'Easter Hare' originally played the role of a judge, evaluating whether children were good or disobedient in behavior at the start of the season of Eastertide, similar to the 'naughty or nice' list made by Santa Claus. As part of the legend, the creature carries colored eggs in its basket, as well as candy, and sometimes toys, to the homes of children. As such, the Easter Bunny again shows similarities to Santa (or the Christkind) and Christmas by bringing gifts to children on the night before a holiday. The custom was first mentioned in Georg Franck von Franckenau's De ovis paschalibus ('About Easter eggs') in 1682, referring to a German tradition of an Easter Hare bringing eggs for the children."),
            Legendary::EasterBilby =>  v = String::from("Bilbies are native Australian marsupials that are endangered. To raise money and increase awareness of conservation efforts, bilby-shaped chocolates and related merchandise are sold within many stores throughout Australia as an alternative to Easter bunnies.  The first documented use of the Easter Bilby concept was in March 1968 when a 9-year-old girl Rose-Marie Dusting, wrote a story, 'Billy The Aussie Easter Bilby,' which she published as a book 11 years later. The story helped catalyse the public's interest in saving the bilby. In 1991, Nicholas Newland from the 'Foundation for Rabbit-Free Australia' also developed the idea of the Easter Bilby to raise awareness about the environmental damage that feral rabbits cause and to replace the Easter bunny with true native wildlife.  The first Chocolate Easter Bilbies were sold at the Warrawong Sanctuary when it was owned by John Wamsley"),
            Legendary::EachUisge =>  v = String::from("The each-uisge (Scottish Gaelic: literally 'water horse') is a water spirit in Scottish folklore, known as the each-uisce (anglicized as aughisky or ech-ushkya) in Ireland and cabyll-ushtey on the Isle of Man. It usually takes the form of a horse, and is similar to the kelpie but far more vicious."),
            Legendary::EagleSpirit =>  v = String::from("EagleSpirit – Leadership or guidance totem"),
            Legendary::EbuGogo =>  v = String::from("EbuGogo – Diminutive humanoids, possibly inspired by Homo floresiensis"),
            Legendary::Echidna =>  v = String::from("Echidna"),
            Legendary::Echeneis =>  v = String::from("Echeneis – Remora, said to attach to ships to slow them down"),
            Legendary::Edimmu =>  v = String::from("Edimmu – Ghosts of those not buried properly"),
            Legendary::Egbere =>  v = String::from("Egbere – Humanoid that carries a magical mat"),
            Legendary::Eikthyrnir =>  v = String::from("Eikthyrnir"),
            Legendary::Einherjar =>  v = String::from("Einherjar – Spirits of brave warriors"),
            Legendary::Ekek =>  v = String::from("Ekek – Flesh-eating, winged humanoids"),
            Legendary::ElbowWitch =>  v = String::from("ElbowWitch – Hags with awls in their elbows"),
            Legendary::Eldjotnar =>  v = String::from("Eldjotnar – Fire Giants who reside in Muspelheim, with Surtr as their leader"),
            Legendary::Eleionomae =>  v = String::from("Eleionomae – Marsh nymph"),
            Legendary::Elemental =>  v = String::from("Elemental – Personification of one of the Classical elements"),
            Legendary::Elepaio =>  v = String::from("Elepaio – Monarch flycatcher spirit that guides canoe-builders to the proper trees"),
            Legendary::Elf =>  v = String::from("Elf – Nature and fertility spirit"),
            Legendary::Eloko =>  v = String::from("Eloko – Little people and malevolent nature spirits"),
            Legendary::Emere =>  v = String::from("Emere – Child that can move back and forth between the material world and the afterlife at will"),
            Legendary::Emim =>  v = String::from("Emim – Giant"),
            Legendary::Empusa =>  v = String::from("Empusa – Female demon that waylays travelers and seduces and kills men"),
            Legendary::Encantado =>  v = String::from("Encantado – Dolphin-human shapeshifter"),
            Legendary::EnchantedMoor =>  v = String::from("EnchantedMoor – Enchanted princesses"),
            Legendary::Enfield =>  v = String::from("Enfield – Fox-greyhound-lion-wolf-eagle hybrid"),
            Legendary::Engkanto =>  v = String::from("Engkanto – Neutral nature spirit"),
            Legendary::Enko =>  v = String::from("Enko – Kappa of Shikoku and western Honshū"),
            Legendary::Ent =>  v = String::from("Ent -Living tree that is said to live for years"),
            Legendary::Epimeliad =>  v = String::from("Epimeliad – Apple tree nymph"),
            Legendary::Erchitu =>  v = String::from("Erchitu – Ox-human, wereox"),
            Legendary::ErGui =>  v = String::from("ErGui – Hungry ghost"),
            Legendary::Erinyes =>  v = String::from("Erinyes – Winged spirits of vengeance or justice, also known as Furies"),
            Legendary::Erlking =>  v = String::from("Erlking – Death spirit"),
            Legendary::ErymanthianBoar =>  v = String::from("ErymanthianBoar – Giant boar"),
            Legendary::EthiopianPegasus =>  v = String::from("EthiopianPegasus – Horned, winged horse"),
            Legendary::Etiainen =>  v = String::from("Etiainen – Spirit being of a living person"),
            Legendary::Ettin =>  v = String::from("Ettin – Three-headed giant"),
            Legendary::Eurynomos =>  v = String::from("Eurynomos – Blue-black, carrion-eater in the underworld"),
            Legendary::Ewah =>  v = String::from("Ewah – Human-cougar hybrid"),
            Legendary::Eerinis =>  v = String::from("Eerinis – Lake spirit"),
            Legendary::Fachen =>  v = String::from("Fachen – Monster with half a body"),
            Legendary::Fafnir =>  v = String::from("Fafnir – Dwarf who was cursed and turned into a dragon. He was later slain by Sigurd in the Saga of Nibelung."),
            Legendary::Fairy =>  v = String::from("Fairy – Nature spirits"),
            Legendary::Familiar =>  v = String::from("Familiar – Animal servant"),
            Legendary::FarDarrig =>  v = String::from("FarDarrig – Little people that constantly play pranks"),
            Legendary::Farfadet =>  v = String::from("Farfadet, wrinkled, and brown-skinned helpful sprites."),
            Legendary::Fates =>  v = String::from("Fates – Three time-controlling sisters"),
            Legendary::Faun =>  v = String::from("Faun – Human-goat hybrid nature spirit"),
            Legendary::FearGorta =>  v = String::from("FearGorta"),
            Legendary::FeatheredSerpent =>  v = String::from("Mesoamerican dragonMesoamerican dragon"),
            Legendary::FeiLian =>  v = String::from("FeiLian – Chinese wind god"),
            Legendary::Fenghuang =>  v = String::from("Fenghuang – Chinese Phoenix, female in marriage symbol"),
            Legendary::Fenodyree =>  v = String::from("Fenodyree – House spirit"),
            Legendary::Fenrir =>  v = String::from("Fenrir – Gigantic, ravenous wolf"),
            Legendary::Fetch =>  v = String::from("Fetch – Double or doppelgänger"),
            Legendary::Fext =>  v = String::from("Fext – Undead"),
            Legendary::Finfolk =>  v = String::from("Finfolk – Fish-human hybrid that kidnaps humans for servants"),
            Legendary::FirBolg =>  v = String::from("FirBolg – Ancestral race"),
            Legendary::FireBird =>  v = String::from("FireBird – Regenerative solar bird"),
            Legendary::Firedrake =>  v = String::from("Firedrake – Dragon"),
            Legendary::FishMan =>  v = String::from("FishMan – Amphibious, scaled humanoid"),
            Legendary::FlatwoodsMonster =>  v = String::from("FlatwoodsMonster – Alien, humanoid"),
            Legendary::Fomorian =>  v = String::from("Fomorian – Goat-headed giant"),
            Legendary::ForestBull =>  v = String::from("ForestBull"),
            Legendary::Freybug =>  v = String::from("// Norfolk black dog// Norfolk black dog"),
            Legendary::Fuath =>  v = String::from("Fuath – Malevolent water spirit"),
            Legendary::Fucanglong =>  v = String::from("Fucanglong – Underworld dragon"),
            Legendary::Funayurei =>  v = String::from("Funayurei – Ghosts of people who drowned at sea"),
            Legendary::FuruUtsubo =>  v = String::from("FuruUtsubo – Animated jar"),
            Legendary::FutakuchiOnna =>  v = String::from("FutakuchiOnna – Woman with a second mouth on the back of her head"),
            Legendary::Fylgja =>  v = String::from("Fylgja – Animal familiar"),
            Legendary::Gaasyendietha =>  v = String::from("Gaasyendietha – Dragon"),
            Legendary::Gagana =>  v = String::from("Gagana – Iron-beaked bird with copper talons"),
            Legendary::Gaki =>  v = String::from("Gaki – Ghosts of especially greedy people"),
            Legendary::Gallu =>  v = String::from("Gallu – Underworld demons"),
            Legendary::Galtzagorriak =>  v = String::from("Galtzagorriak – Small demonic servants"),
            Legendary::Gamayun =>  v = String::from("Gamayun – Prophetic human-headed bird"),
            Legendary::Gana =>  v = String::from("Gana – Attendants of Shiva"),
            Legendary::Gancanagh =>  v = String::from("Gancanagh – Male fairy that seduces human women"),
            Legendary::Gandabherunda =>  v = String::from("Gandabherunda – Double-headed bird"),
            Legendary::Gandharva =>  v = String::from("Gandharva – Male nature spirits, often depicted as part human, part animal"),
            Legendary::Gargouille =>  v = String::from("Gargouille – Water dragon"),
            Legendary::Garkain =>  v = String::from("Garkain – A flying humanoid who envelops his victims"),
            Legendary::Garmr =>  v = String::from("Garmr – Giant, ravenous hound"),
            Legendary::Garuda =>  v = String::from("Garuda – Human-eagle hybrid"),
            Legendary::Gashadokuro =>  v = String::from("Gashadokuro – Giant malevolent skeletons"),
            Legendary::Gaueko =>  v = String::from("Gaueko – Wolf capable of walking upright"),
            Legendary::Geb =>  v = String::from("Geb – God of the Earth, married to Nut"),
            Legendary::Ged =>  v = String::from("Ged – The fish pike"),
            Legendary::Gegenees =>  v = String::from("Gegenees – Six-armed giant"),
            Legendary::GeniusLoci =>  v = String::from("GeniusLoci – Spirit that protects a specific place"),
            Legendary::German =>  v = String::from("German – Male spirit associated with bringing rain and hail"),
            Legendary::Geryon =>  v = String::from("Geryon six legs"),
            Legendary::GhillieDhu =>  v = String::from("GhillieDhu – Tree guardian"),
            Legendary::Ghost =>  v = String::from("Disembodied spirits of those that have diedDisembodied spirits of those that have died"),
            Legendary::Ghoul =>  v = String::from("Ghoul – Cannibalistic shapeshifting desert genie often classified as undead."),
            Legendary::Giant =>  v = String::from("Giant – Immensely large and strong humanoids"),
            Legendary::GiantAnimal =>  v = String::from("GiantAnimal – Unusually large beasts"),
            Legendary::GichiAnamiEBizhiw =>  v = String::from("GichiAnamiEBizhiw – Bison-snake-bird-cougar hybrid water spirit"),
            Legendary::Gidim =>  v = String::from("Gidim – Ghost"),
            Legendary::Gigantes =>  v = String::from("Gigantes – Race of giants that fought the Olympian gods, sometimes depicted with snake-legs"),
            Legendary::Gigelorum =>  v = String::from("Gigelorum – Smallest animal"),
            Legendary::Girtablilu =>  v = String::from("Girtablilu – Human-scorpion hybrid"),
            Legendary::Gjenganger =>  v = String::from("Gjenganger – Corporeal ghost"),
            Legendary::Glaistig =>  v = String::from("Glaistig – Human-goat hybrid"),
            Legendary::Glashtyn =>  v = String::from("Glashtyn – Malevolent water horse"),
            Legendary::Gnome =>  v = String::from("Gnome – Diminutive Earth elemental"),
            Legendary::Goblin =>  v = String::from("Goblin – Grotesque, mischievous little people"),
            Legendary::Gog =>  v = String::from("Gog – Giant protector of London"),
            Legendary::GoldDiggingAnt =>  v = String::from("GoldDiggingAnt – Dog-sized ant that digs for gold in sandy areas"),
            Legendary::Golem =>  v = String::from("Golem – Animated construct"),
            Legendary::Gorgades =>  v = String::from("Gorgades – Hairy humanoid"),
            Legendary::Gorgon =>  v = String::from("Gorgon – Fanged, snake-haired humanoids that turn anyone who sees them into stone"),
            Legendary::Goryo =>  v = String::from("Goryo – Vengeful ghosts, usually of martyrs"),
            Legendary::Grassman =>  v = String::from("Grassman – Ape-like cryptid"),
            Legendary::Gremlin =>  v = String::from("Gremlin – Creatures that sabotage airplanes"),
            Legendary::Griffin =>  v = String::from("Griffin – Lion-eagle hybrid"),
            Legendary::Grigori =>  v = String::from("Grigori – Fallen angels, father of Nephilim"),
            Legendary::Grim =>  v = String::from("Grim – Tutelary spirits of churches"),
            Legendary::GrimReaper =>  v = String::from("GrimReaper – Death angel often thought to be God's/Satan's assistant"),
            Legendary::Grindylow =>  v = String::from("Grindylow – Malevolent water spirit"),
            Legendary::Gualichu =>  v = String::from("Gualichu – Malevolent spirit"),
            Legendary::GuardianAngel =>  v = String::from("GuardianAngel – Subclassification of angels that guard and protect a specific person or living being"),
            Legendary::GudElim =>  v = String::from("GudElim – Human-bull hybrid"),
            Legendary::Guhin =>  v = String::from("Guhin – Anthropomorphic bird"),
            Legendary::GuiPo =>  v = String::from("GuiPo – Ghost that manifests as an old woman"),
            Legendary::GuiShu =>  v = String::from("GuiShu – Ghostly tree that confuses travelers by moving"),
            Legendary::Gulon =>  v = String::from("Gulon – Gluttonous dog-cat-fox hybrid"),
            Legendary::Gumiho =>  v = String::from("Gumiho – Demonic fox with thousands of tails believed to possess an army of spirits and magic in its tails"),
            Legendary::Gurangatch =>  v = String::from("Gurangatch - An enormous reptile-fish whose movements carved out the landscape south of the Blue Mountains"),
            Legendary::Gurumapa =>  v = String::from("Gurumapa – Child-eating demon"),
            Legendary::Gwyllgi =>  v = String::from("Gwyllgi – Black dog"),
            Legendary::Gwyllion =>  v = String::from("Gwyllion – Malevolent spirit"),
            Legendary::Gyascutus =>  v = String::from("Gyascutus – Four-legged herbivore"),
            Legendary::Gytrash =>  v = String::from("Gytrash – Black dog"),
            Legendary::Gyuki =>  v = String::from("Gyuki – Bull-headed monster"),
            Legendary::Habrok =>  v = String::from("Habrok – listed as the 'best' hawk"),
            Legendary::Hadhayosh =>  v = String::from("Hadhayosh – gigantic land animal"),
            Legendary::Hades =>  v = String::from("Hades – Ruler of the Underworld"),
            Legendary::Haetae =>  v = String::from("Haetae – dog-lion hybrid"),
            Legendary::Hag =>  v = String::from("Hag – wise old woman who is usually a malevolent spirit or a disguised goddess"),
            Legendary::Haietlik =>  v = String::from("Haietlik – water serpent"),
            Legendary::HaiUri =>  v = String::from("HaiUri – male cannibalistic partially invisible monster"),
            Legendary::Hakutaku =>  v = String::from("Hakutaku – talking beast which handed down knowledge on harmful spirits"),
            Legendary::Hakuturi =>  v = String::from("Hakuturi – nature guardian"),
            Legendary::HalfElf =>  v = String::from("HalfElf – human-elf hybrid"),
            Legendary::Haltija =>  v = String::from("Haltija – spirit that protects a specific place"),
            Legendary::Hamadryad =>  v = String::from("Hamadryad – oak tree nymph"),
            Legendary::Hamingja =>  v = String::from("Hamingja – personal protection spirit"),
            Legendary::Hamsa =>  v = String::from("Hamsa – mystic bird"),
            Legendary::HanauEpe =>  v = String::from("HanauEpe – long-eared humanoid"),
            Legendary::HantuAir =>  v = String::from("HantuAir – shapeshifting water spirit"),
            Legendary::HantuDemon =>  v = String::from("HantuDemon – demon"),
            Legendary::HantuRaya =>  v = String::from("HantuRaya – demonic servant"),
            Legendary::Harionago =>  v = String::from("Harionago – humanoid female with barbed, prehensile hair"),
            Legendary::Harpy =>  v = String::from("Harpy – birdlike human-headed death spirit"),
            Legendary::Haugbui =>  v = String::from("Haugbui – undead being who cannot leave its burial mound"),
            Legendary::Havsrå =>  v = String::from("Havsrå – saltwater spirit"),
            Legendary::Helloi =>  v = String::from("Helloi – celestial maidens, daughters of the Sky God Soraren"),
            Legendary::HeadlessHorseman =>  v = String::from("HeadlessHorseman – humanoid spirit who haunts or kills"),
            Legendary::HeadlessMule =>  v = String::from("HeadlessMule – fire-spewing, headless, spectral mule"),
            Legendary::Hecatonchires =>  v = String::from("Hecatonchires – primordial giants with 100 hands and fifty heads"),
            Legendary::Heikegani =>  v = String::from("Heikegani – crabs with human-faced shells, the spirits of warriors killed in the Battle of Dan-no-ura"),
            Legendary::Heinzelmannchen =>  v = String::from("Heinzelmannchen – household spirit"),
            Legendary::Helead =>  v = String::from("Helead – fen nymph"),
            Legendary::Hellhound =>  v = String::from("Hellhound – underworld dog"),
            Legendary::Heracles =>  v = String::from("Heracles – gatekeeper of Olympus"),
            Legendary::Hercinia =>  v = String::from("Hercinia – glowing bird"),
            Legendary::Herensuge =>  v = String::from("Herensuge – dragon"),
            Legendary::Hesperides =>  v = String::from("Hesperides – nymph daughters of Atlas"),
            Legendary::Hidebehind =>  v = String::from("Hidebehind – nocturnal forest creature"),
            Legendary::Hiderigami =>  v = String::from("Hiderigami – drought spirit"),
            Legendary::Hieracosphinx =>  v = String::from("Hieracosphinx – falcon-headed sphinx"),
            Legendary::Hihi =>  v = String::from("Hihi – baboon monster"),
            Legendary::Hiisi =>  v = String::from("Hiisi – nature guardian"),
            Legendary::Hippalectryon =>  v = String::from("Hippalectryon"),
            Legendary::Hippocamp =>  v = String::from("Hippocamp – horse-fish hybrid"),
            Legendary::Hippogriff =>  v = String::from("Hippogriff – hybrid of a griffin and horse; a lion-eagle-horse hybrid"),
            Legendary::Hippopodes =>  v = String::from("Hippopodes – horse-hoofed humanoid"),
            Legendary::Hircocervus =>  v = String::from("Hircocervus – deer-goat hybrid"),
            Legendary::Hitodama =>  v = String::from("Hitodama – ghosts of the newly dead, which take the form of fireballs"),
            Legendary::HitotsumeKozo =>  v = String::from("HitotsumeKozo – one-eyed childlike spirit"),
            Legendary::Hob =>  v = String::from("Hob – house spirit"),
            Legendary::Hobbididance =>  v = String::from("Hobbididance – malevolent spirit"),
            Legendary::Hobgoblin =>  v = String::from("Hobgoblin – friendly or amusing goblin"),
            Legendary::Hodag =>  v = String::from("Hodag – frog-mammoth-lizard hybrid"),
            Legendary::Hokhokw =>  v = String::from("Hokhokw – bird"),
            Legendary::Hoko =>  v = String::from("Hoko – dog-like Chinese tree spirit"),
            Legendary::Homa =>  v = String::from("Homa – eagle-lion hybrid, similar to a griffin"),
            Legendary::HombreCaiman =>  v = String::from("HombreCaiman – human-alligator hybrid"),
            Legendary::HombreGato =>  v = String::from("HombreGato – human-cat hybrid"),
            Legendary::Homunculus =>  v = String::from("Homunculus – small animated construct"),
            Legendary::Hoo =>  v = String::from("Hoo – rooster-swallow-fowl-snake-goose-tortoise-stag-fish hybrid"),
            Legendary::Hoopoe =>  v = String::from("near passerine bird common to Africa and Eurasia that features in many mythologies in those continentsnear passerine bird common to Africa and Eurasia that features in many mythologies in those continents"),
            Legendary::HoopSnake =>  v = String::from("snake which rolls by taking its tail in its mouthsnake which rolls by taking its tail in its mouth"),
            Legendary::HornedSerpent =>  v = String::from("HornedSerpent – serpentine rain spirit"),
            Legendary::Hotoke =>  v = String::from("Hotoke – deceased person"),
            Legendary::Houri =>  v = String::from("Houri – heavenly beings"),
            Legendary::Hraesvelg =>  v = String::from("Hraesvelg – giant, who in eagle form, creates the wind by beating his wings"),
            Legendary::Hrímþursar =>  v = String::from("Hrímþursar – frost giants who are the main inhabitants of either Jotunheim or Niflheim"),
            Legendary::Huaychivo =>  v = String::from("Huaychivo – human-deer hybrid"),
            Legendary::HuginnAndMuninn =>  v = String::from("HuginnAndMuninn – pair of ravens associated with the Norse god Odin whose names mean Thought and Memory."),
            Legendary::Huldufolk =>  v = String::from("Huldufolk – secret mound/rock dwelling elves"),
            Legendary::Hulder =>  v = String::from("Hulder – forest spirit"),
            Legendary::HuliJing =>  v = String::from("HuliJing – nine-tailed fox spirit"),
            Legendary::Huma =>  v = String::from("Huma – regenerative fire bird"),
            Legendary::Humbaba =>  v = String::from("Humbaba – lion-faced giant"),
            Legendary::Hundun =>  v = String::from("Hundun – chaos spirit"),
            Legendary::Hupia =>  v = String::from("Hupia – nocturnal ghost"),
            Legendary::Hyakume =>  v = String::from("Hyakume – hundred-eyes creature"),
            Legendary::Hydra =>  v = String::from("Hydra – multi-headed water serpent/dragon"),
            Legendary::Hydros =>  v = String::from("Hydros – snake whose poison causes the victim to swell up"),
            Legendary::Hydrus =>  v = String::from("Hydrus – snake from the Nile River that would kill crocodiles from the inside"),
            Legendary::Hyosube =>  v = String::from("Hyosube – hair-covered kappa"),
            Legendary::Hypnalis =>  v = String::from("Hypnalis – snake that kills its victims in their sleep"),
            Legendary::Hudhud =>  v = String::from("Hudhud – Hoopoe"),
            Legendary::Ishigaq =>  v = String::from("Ishigaq – Little people"),
            Legendary::IslandSatyr =>  v = String::from("IslandSatyr – Savage human-goat hybrid from a remote island chain"),
            Legendary::Isonade =>  v = String::from("Isonade – Shark-like sea monster"),
            Legendary::IttanMomen =>  v = String::from("IttanMomen – Ghostly aerial phenomenon that attacks people"),
            Legendary::IwanaBozu =>  v = String::from("IwanaBozu – Char which appeared as a Buddhist monk"),
            Legendary::Jackalope =>  v = String::from("Jackalope – Rabbit with antlers"),
            Legendary::JackInIrons =>  v = String::from("JackInIrons – Malevolent giant"),
            Legendary::JackOLantern =>  v = String::from("JackOLantern – Vegetal lantern"),
            Legendary::Jaculus =>  v = String::from("Jaculus – Winged serpent or small dragon"),
            Legendary::Jasconius =>  v = String::from("Jasconius – Island-sized fish"),
            Legendary::JasyJaterei =>  v = String::from("JasyJaterei – Nature guardian and bogeyman"),
            Legendary::Jatayu =>  v = String::from("Jatayu – Vulture demigod"),
            Legendary::Jaud =>  v = String::from("Jaud – Vampirised premature baby"),
            Legendary::Jenglot =>  v = String::from("Jenglot – Vampiric little people"),
            Legendary::Jengu =>  v = String::from("Jengu – Water spirit"),
            Legendary::Jentil =>  v = String::from("Jentil – Megalith-building giant"),
            Legendary::Jenu =>  v = String::from("Jenu – Anthropophagous giant"),
            Legendary::Jerff =>  v = String::from("Jerff – Gluttonous dog-cat-fox hybrid"),
            Legendary::JerseyDevil =>  v = String::from("JerseyDevil – Demonic dragon or flying demon who was given birth to by an American living in New Jersey"),
            Legendary::Jian =>  v = String::from("Jian – One-eyed, one-winged bird who requires a mate for survival"),
            Legendary::Jiangshi =>  v = String::from("Jiangshi – Life-draining, reanimated corpse"),
            Legendary::Jiaolong =>  v = String::from("Jiaolong – Dragon"),
            Legendary::Jibakurei =>  v = String::from("Jibakurei – Spirit that protects a specific place"),
            Legendary::Jievaras =>  v = String::from("Jievaras – House spirit"),
            Legendary::Jikininki =>  v = String::from("Jikininki – Corpse-eating ghost"),
            Legendary::Jinn =>  v = String::from("Jinn – Spiritual creatures; genii"),
            Legendary::JipijkaM =>  v = String::from("JipijkaM – Underwater horned snake; lives in lakes and eats humans"),
            Legendary::Jiufeng =>  v = String::from("Jiufeng – Nine-headed bird worshiped by ancient natives in Hubei Province."),
            Legendary::JiuTouNiao =>  v = String::from("JiuTouNiao – Nine-headed, demonic bird"),
            Legendary::Jogah =>  v = String::from("Jogah – Little people nature spirit"),
            Legendary::Jormungandr =>  v = String::from("Jormungandr – Sea serpent"),
            Legendary::Jorogumo =>  v = String::from("Jorogumo – Spider woman"),
            Legendary::Jotai =>  v = String::from("Jotai – Animated folding screen cloth"),
            Legendary::Jotunn =>  v = String::from("Jotunn – Gigantic nature spirits"),
            Legendary::Jujak =>  v = String::from("Jujak – Bird"),
            Legendary::Jumbee =>  v = String::from("Jumbee – Malevolent spirit"),
            Legendary::Kabouter =>  v = String::from("Kabouter – Little people that live underground, in mushrooms, or as house spirits"),
            Legendary::Kachina =>  v = String::from("Kachina – Nature spirit"),
            Legendary::Kahaku =>  v = String::from("Kahaku – Little people and water spirits"),
            Legendary::Kajsa =>  v = String::from("Kajsa – Wind spirit"),
            Legendary::Kalakeyas =>  v = String::from("Kalakeyas – Descendants of Kala"),
            Legendary::Kallikantzaroi =>  v = String::from("Kallikantzaroi – Grotesque, malevolent spirit"),
            Legendary::Kamaitachi =>  v = String::from("Kamaitachi – Wind spirit"),
            Legendary::Kamatayan =>  v = String::from("Kamatayan – Philippine counterpart of Death"),
            Legendary::Kami =>  v = String::from("Kami – Nature spirit"),
            Legendary::Kamikiri =>  v = String::from("Kamikiri – Hair-cutting spirit"),
            Legendary::KanbariNyudo =>  v = String::from("KanbariNyudo – Bathroom spirit"),
            Legendary::KanglaSha =>  v = String::from("KanglaSha – Great Dragon in the Kangla Palace"),
            Legendary::Kanbo =>  v = String::from("Kanbo – Drought spirit"),
            Legendary::Kanedama =>  v = String::from("Kanedama – Money spirit"),
            Legendary::Kappa =>  v = String::from("Kappa – Little people and water spirit"),
            Legendary::Kapre =>  v = String::from("Kapre – Malevolent tree spirit"),
            Legendary::Karakoncolos =>  v = String::from("Karakoncolos, also in Bosnia and Herzegovina and Serbia known as Karanđoloz – Troublesome spirit"),
            Legendary::Karakura =>  v = String::from("Karakura – Male night-demon"),
            Legendary::KarasuTengu =>  v = String::from("KarasuTengu – Tengu with a bird's bill"),
            Legendary::Karkadann =>  v = String::from("Karkadann – One-horned giant animal"),
            Legendary::Karkinos =>  v = String::from("Karkinos – Giant crab"),
            Legendary::Karura =>  v = String::from("Karura – Eagle-human hybrid"),
            Legendary::Karzelek =>  v = String::from("Karzelek – Little people and mine spirits"),
            Legendary::KasaObake =>  v = String::from("KasaObake – Animated parasol"),
            Legendary::Kasha =>  v = String::from("Kasha – Cat-like demon which descends from the sky and carries away corpses"),
            Legendary::Kashanbo =>  v = String::from("Kashanbo – Kappa who climb into the mountains for the winter"),
            Legendary::KatawaGuruma =>  v = String::from("KatawaGuruma – Woman riding on a flaming wheel"),
            Legendary::KatsuraOtoko =>  v = String::from("KatsuraOtoko – Handsome man from the moon"),
            Legendary::Katallan =>  v = String::from("Katallan – Man-eating giant"),
            Legendary::Kaukas =>  v = String::from("Kaukas – Nature spirit"),
            Legendary::KawaUso =>  v = String::from("KawaUso – Supernatural river otter"),
            Legendary::KawaZaru =>  v = String::from("KawaZaru – Smelly, cowardly water spirit"),
            Legendary::KeLets =>  v = String::from("KeLets – Ogre or evil spirit"),
            Legendary::Keelut =>  v = String::from("Keelut – Hairless dog"),
            Legendary::KeeWakw =>  v = String::from("KeeWakw – Half-human half-animal cannibalistic giant"),
            Legendary::Kekkai =>  v = String::from("Kekkai – Amorphous afterbirth spirit"),
            Legendary::Kelpie =>  v = String::from("Kelpie – Malevolent water horse"),
            Legendary::Ker =>  v = String::from("Ker – Female death spirit"),
            Legendary::KesaranPasaran =>  v = String::from("KesaranPasaran – Mysterious, white, fluffy creature"),
            Legendary::Keukegen =>  v = String::from("Keukegen – Disease spirit"),
            Legendary::Keythong =>  v = String::from("Keythong – Wingless griffin"),
            Legendary::Khyah =>  v = String::from("Khyah – Fat, hairy ape-like creature"),
            Legendary::Kigatilik =>  v = String::from("Kigatilik – Night-demon"),
            Legendary::Kholomodumo =>  v = String::from("Kholomodumo – Gluttonous monster that was one of the first beasts of creation"),
            Legendary::Kijimunaa =>  v = String::from("Kijimunaa – Tree sprite from Okinawa"),
            Legendary::Kijo =>  v = String::from("Kijo – She-devil"),
            Legendary::Kikimora =>  v = String::from("Kikimora – Female house spirit"),
            Legendary::Killmoulis =>  v = String::from("Killmoulis – Ugly, mischievous mill spirit"),
            Legendary::Kinnara =>  v = String::from("Kinnara – Human-bird hybrid"),
            Legendary::KinU =>  v = String::from("KinU – Bird"),
            Legendary::Kirin =>  v = String::from("Kirin – Japanese Unicorn"),
            Legendary::Kishi =>  v = String::from("Kishi – Malevolent, two-faced seducer"),
            Legendary::Kitsune =>  v = String::from("Kitsune – Fox spirit"),
            Legendary::KitsuneTsuki =>  v = String::from("KitsuneTsuki – Person possessed by a fox spirit"),
            Legendary::Kiyohime =>  v = String::from("Kiyohime – Woman who transformed into a serpentine demon out of the rage of unrequited love"),
            Legendary::Klabautermann =>  v = String::from("Klabautermann – Ship spirit"),
            Legendary::Knocker =>  v = String::from("Knocker – Little people and mine spirits"),
            Legendary::Knucker =>  v = String::from("Knucker – Water dragon"),
            Legendary::Kobalos =>  v = String::from("Kobalos – Goblin like thieves and tricksters"),
            Legendary::Kobold =>  v = String::from("Kobold – Little people and mine or house spirits"),
            Legendary::Kodama =>  v = String::from("Kodama – Tree spirit"),
            Legendary::Kofewalt =>  v = String::from("Kofewalt – House spirit"),
            Legendary::KoGok =>  v = String::from("KoGok – Hideous monster"),
            Legendary::Kokakucho =>  v = String::from("Kokakucho – Ubume bird"),
            Legendary::Komainu =>  v = String::from("Komainu – Protective animal"),
            Legendary::KonakiJiji =>  v = String::from("KonakiJiji – Infant that cries until it is picked up, then increases its weight and crushes its victim"),
            Legendary::KonohaTengu =>  v = String::from("KonohaTengu – Bird-like creature"),
            Legendary::KoroPokGuru =>  v = String::from("KoroPokGuru – Little people"),
            Legendary::Korrigan =>  v = String::from("Korrigan – Little people and nature spirits"),
            Legendary::Kraken =>  v = String::from("Kraken – Sea monster"),
            Legendary::Krasnoludek =>  v = String::from("Krasnoludek – Little people nature spirits"),
            Legendary::Krasue =>  v = String::from("Krasue – Vampiric, floating head"),
            Legendary::Krampus =>  v = String::from("Krampus – Christmas Devil who punishes badly-behaved children"),
            Legendary::KuarahyJara =>  v = String::from("KuarahyJara – Forest spirit"),
            Legendary::Kubikajiri =>  v = String::from("Kubikajiri – Female corpse-chewing graveyard spirit"),
            Legendary::KuchisakeOnna =>  v = String::from("KuchisakeOnna – Vengeful ghost of a woman mutilated by her husband"),
            Legendary::KudaGitsune =>  v = String::from("KudaGitsune – Miniature fox spirit"),
            Legendary::Kudan =>  v = String::from("Kudan – Human-faced calf which predicts a calamity before dying"),
            Legendary::Kui =>  v = String::from("Kui – One-legged monster"),
            Legendary::Kukudhi =>  v = String::from("Kukudhi – Female demon who spreads sickness"),
            Legendary::Kukwes =>  v = String::from("Kukwes – Large, hairy, greedy, human-eating bipedal monsters whose scream can kill"),
            Legendary::Kulshedra =>  v = String::from("Kulshedra – Drought-causing dragon"),
            Legendary::Kumakatok =>  v = String::from("Kumakatok – Death spirits"),
            Legendary::Kumiho =>  v = String::from("Kumiho – Fox spirit"),
            Legendary::Kun =>  v = String::from("Kun – Giant fish"),
            Legendary::Kupua =>  v = String::from("Kupua – Shapeshifting tricksters"),
            Legendary::Kurabokko =>  v = String::from("Kurabokko – Guardian spirit of a warehouse"),
            Legendary::KurageNoHinotama =>  v = String::from("KurageNoHinotama – Jellyfish which floats through the air as a fireball"),
            Legendary::Kurma =>  v = String::from("Kurma – Second avatar of Vishnu in the form of a Turtle"),
            Legendary::Kurupi =>  v = String::from("Kurupi – Wild man and fertility spirit"),
            Legendary::Kushtaka =>  v = String::from("Kushtaka – Shapeshifting 'land otter man'"),
            Legendary::KyeRyong =>  v = String::from("KyeRyong – Chicken-lizard hybrid"),
            Legendary::Kyourinrin =>  v = String::from("Kyourinrin – Animated scroll or paper"),
            Legendary::KyubiNoKitsune =>  v = String::from("KyubiNoKitsune – Nine-tailed fox"),
            Legendary::Kyuketsuki =>  v = String::from("Kyuketsuki – Vampire"),
            Legendary::LaBarTu =>  v = String::from("LaBarTu – Disease demon"),
            Legendary::LabbMu =>  v = String::from("LabbMu – Sea snake"),
            Legendary::Ladyidday =>  v = String::from("Ladyidday – Sunstroke spirit"),
            Legendary::Ladon =>  v = String::from("Ladon – Dragon guarding the golden apples of the Hesperides"),
            Legendary::Laelaps =>  v = String::from("Laelaps – Enchanted dog that always caught his prey"),
            Legendary::Laestrygonians =>  v = String::from("Laestrygonians – Anthropophagic giants"),
            Legendary::Lakanica =>  v = String::from("Lakanica – Field spirit"),
            Legendary::LakeMonster =>  v = String::from("LakeMonster – Gigantic animals reported to inhabit various lakes around the world"),
            Legendary::Lakhey =>  v = String::from("Lakhey – Demon with fangs"),
            Legendary::LaLlorona =>  v = String::from("LaLlorona – Death spirit associated with drowning"),
            Legendary::Lamassu =>  v = String::from("Lamassu – Protective spirit with the form of a winged bull or human-headed lion"),
            Legendary::LambtonWorm =>  v = String::from("LambtonWorm – Giant worm"),
            Legendary::Lamia =>  v = String::from("Lamia – Child-devouring monster"),
            Legendary::Lamiak =>  v = String::from("Lamiak – Water spirit with duck-like feet"),
            Legendary::LaMojana =>  v = String::from("LaMojana – Shapeshifting, female water spirit"),
            Legendary::Lampades =>  v = String::from("Lampades – Underworld nymph"),
            Legendary::Landvaettir =>  v = String::from("Landvaettir – Nature spirits"),
            Legendary::Langmeidong =>  v = String::from("Langmeidong – Semi human, semi hornbill creature"),
            Legendary::Lares =>  v = String::from("Lares – House spirit"),
            Legendary::LaSayona =>  v = String::from("LaSayona – Female ghost that punishes unfaithful husbands"),
            Legendary::LaTunda =>  v = String::from("LaTunda – Nature spirit that seduces and kills men"),
            Legendary::LavaBear =>  v = String::from("Miniature bear thought to inhabit the lava beds of south central OregonMiniature bear thought to inhabit the lava beds of south central Oregon"),
            Legendary::LaukuDvasios =>  v = String::from("LaukuDvasios – Field spirit"),
            Legendary::Lauma =>  v = String::from("Lauma – Sky spirit"),
            Legendary::Lavellan =>  v = String::from("Lavellan – Gigantic water rat"),
            Legendary::LeananSidhe =>  v = String::from("LeananSidhe – Fairy lover"),
            Legendary::Leanashe =>  v = String::from("Leanashe – Possessing spirit or vampire"),
            Legendary::Leimakids =>  v = String::from("Leimakids – Meadow nymph"),
            Legendary::Leokampoi =>  v = String::from("Leokampoi – Fish-tailed lion"),
            Legendary::Leontophone =>  v = String::from("Leontophone – Tiny animal poisonous to lions"),
            Legendary::Leprechaun =>  v = String::from("Leprechaun – Cobbler spirit"),
            Legendary::Leszi =>  v = String::from("Leszi – Tree spirit"),
            Legendary::Leuce =>  v = String::from("Leuce – White poplar tree nymph"),
            Legendary::Leucrota =>  v = String::from("Leucrota – Crocotta-lion hybrid"),
            Legendary::Leviathan =>  v = String::from("Leviathan – Sea monster seen in Job 41"),
            Legendary::Leyak =>  v = String::from("Leyak – Anthropophagous flying head with entrails"),
            Legendary::LibyanAegipanes =>  v = String::from("LibyanAegipanes – Human-horse hybrid"),
            Legendary::LibyanSatyr =>  v = String::from("LibyanSatyr – Human-goat hybrid"),
            Legendary::Liderc =>  v = String::from("Liderc – Magical chicken that transforms into a humanoid"),
            Legendary::LightningBird =>  v = String::from("LightningBird – Magical bird found at sites of lightning strikes"),
            Legendary::Likho =>  v = String::from("Likho – One-eyed hag or goblin"),
            Legendary::Lilin =>  v = String::from("Lilin – Night-demoness"),
            Legendary::Lilitu =>  v = String::from("Lilitu – Winged demon"),
            Legendary::Limnades =>  v = String::from("Limnades – Lake nymph"),
            Legendary::Lindworm =>  v = String::from("Lindworm – Dragon"),
            Legendary::Ljosalfar =>  v = String::from("Ljosalfar – Sunlight spirits; the Light Elves"),
            Legendary::Ljubi =>  v = String::from("Ljubi- Demoness"),
            Legendary::LlamhigynYDwr =>  v = String::from("LlamhigynYDwr – Frog-bat-lizard hybrid"),
            Legendary::LochNessMonster =>  v = String::from("LochNessMonster – Serpentine sea monster"),
            Legendary::Loki =>  v = String::from("Loki – God of night"),
            Legendary::LoLol =>  v = String::from("LoLol – Hideous monster"),
            Legendary::Long =>  v = String::from("Chinese dragonChinese dragon"),
            Legendary::Longana =>  v = String::from("Longana – Female human-goat hybrid and water spirit"),
            Legendary::LongMa =>  v = String::from("LongMa – Dragon-horse hybrid"),
            Legendary::Loogaroo =>  v = String::from("Loogaroo – Shapeshifting, female vampire"),
            Legendary::LouCarcolh =>  v = String::from("LouCarcolh – Snake-mollusk hybrid"),
            Legendary::LoupGarou =>  v = String::from("LoupGarou – Werewolf"),
            Legendary::LovelandFrog =>  v = String::from("LovelandFrog – Cryptid, Humanoid Frog"),
            Legendary::LubberFiend =>  v = String::from("LubberFiend – House spirit"),
            Legendary::Luduan =>  v = String::from("Luduan – Truth-detecting animal"),
            Legendary::Lugat =>  v = String::from("Lugat – Vampire"),
            Legendary::Luison =>  v = String::from("Luison – Werewolf | Cadaver-eating dog"),
            Legendary::Lusca =>  v = String::from("Sea MonsterSea Monster"),
            Legendary::Lutin =>  v = String::from("Lutin – Amusing goblin"),
            Legendary::Lyngbakr =>  v = String::from("Lyngbakr Whale-like sea monster"),
            Legendary::Lynx =>  v = String::from("Lynx – Feline guide spirit"),
            Legendary::MaaAlused =>  v = String::from("MaaAlused – Subterranean spirit"),
            Legendary::Machlyes =>  v = String::from("Machlyes – Hermaphroditic humanoid"),
            Legendary::Macrocephali =>  v = String::from("Macrocephali – Giant-headed humanoid"),
            Legendary::MadamKoiKoi =>  v = String::from("MadamKoiKoi – Female ghost"),
            Legendary::Madremonte =>  v = String::from("Madremonte – Nature guardian"),
            Legendary::Maero =>  v = String::from("Maero – Savage, arboreal humanoids"),
            Legendary::Magog =>  v = String::from("Magog – Giant protector of London"),
            Legendary::MahaPudma =>  v = String::from("MahaPudma – Giant elephant that holds up the world"),
            Legendary::Mairu =>  v = String::from("Mairu – Megalith-building giant"),
            Legendary::MajasGari =>  v = String::from("MajasGari"),
            Legendary::Majitu =>  v = String::from("// in Swahili mythology, shape-shifting spirits that can pass as humans// in Swahili mythology, shape-shifting spirits that can pass as humans"),
            Legendary::Makara =>  v = String::from("Makara – Aquatic beings"),
            Legendary::MakuraGaeshi =>  v = String::from("MakuraGaeshi – Pillow-moving spirit"),
            Legendary::MalltYNos =>  v = String::from("MalltYNos – Spirit of the hunt"),
            Legendary::MamiWata =>  v = String::from("MamiWata – Supernaturally beautiful water spirits"),
            Legendary::Manananggal =>  v = String::from("Manananggal – Vampires that sever their torsos from their legs to fly around"),
            Legendary::Mandi =>  v = String::from("Mandi – Humanoid with a forty-year lifespan"),
            Legendary::Mandrake =>  v = String::from("Mandrake – Diminutive, animated construct"),
            Legendary::Manes =>  v = String::from("Manes – Ancestral spirits"),
            Legendary::Mannegishi =>  v = String::from("Mannegishi – Little people with six fingers and no noses"),
            Legendary::Manticore =>  v = String::from("Manticore – Lion-human-scorpion hybrid"),
            Legendary::Mapinguari =>  v = String::from("Mapinguari – Giant sloth"),
            Legendary::Mara =>  v = String::from("Mara – Female night-demon"),
            Legendary::Marabbecca =>  v = String::from("Marabbecca – Malevolent water spirit"),
            Legendary::Mareikura =>  v = String::from("Mareikura – Attendant of Kiho-tumu, the supreme god"),
            Legendary::MaresOfDiomedes =>  v = String::from("MaresOfDiomedes – Man-eating horses"),
            Legendary::Marid =>  v = String::from("Marid – Jinn associated fortune tellers"),
            Legendary::Marmennill =>  v = String::from("Marmennill – Mermen with prophetic abilities"),
            Legendary::MaroDeives =>  v = String::from("MaroDeives – Disease spirits"),
            Legendary::MaskiMonGweZoOs =>  v = String::from("MaskiMonGweZoOs – Shapeshifting toad spirit"),
            Legendary::Matagot =>  v = String::from("Matagot – Spirit that takes animal form; usually that of a black cat"),
            Legendary::Matsya =>  v = String::from("Matsya – First Avatar of Vishnu in the form of a half-fish and half-man"),
            Legendary::Mayura =>  v = String::from("Mayura – Peacock spirit"),
            Legendary::Mazzikin =>  v = String::from("Mazzikin – Invisible, malevolent spirit"),
            Legendary::MboiTuI =>  v = String::from("MboiTuI – Snake-parrot hybrid"),
            Legendary::Mbwiri =>  v = String::from("Mbwiri – Possessing demon"),
            Legendary::Medusa =>  v = String::from("Medusa"),
            Legendary::MelekTaus =>  v = String::from("// biblical bird// biblical bird"),
            Legendary::Meliae =>  v = String::from("Meliae – Ash tree nymph"),
            Legendary::Melusine =>  v = String::from("Melusine – Female water spirit, with the form of a winged mermaid or serpent"),
            Legendary::Menehune =>  v = String::from("Menehune – Little people and craftsmen"),
            Legendary::Menninkainen =>  v = String::from("Menninkainen – Little people and nature spirits"),
            Legendary::Merlion =>  v = String::from("Merlion – Combination of a lion and a fish, the symbol of Singapore"),
            Legendary::Mermaid =>  v = String::from("Mermaid – Human-fish hybrid"),
            Legendary::Merman =>  v = String::from("Merman – Human-fish hybrid"),
            Legendary::Merlin =>  v = String::from("Merlin – Elderly wizard"),
            Legendary::Merrow =>  v = String::from("Merrow – Human-fish hybrid"),
            Legendary::MeteeKolenOl =>  v = String::from("MeteeKolenOl – Ice-hearted wizards"),
            Legendary::Mimi =>  v = String::from("Mimi – Extremely elongated humanoid that has to live in rock crevasses to avoid blowing away"),
            Legendary::MinkaBird =>  v = String::from("MinkaBird – Death spirit"),
            Legendary::Minokawa =>  v = String::from("Minokawa – Giant swallow"),
            Legendary::Minotaur =>  v = String::from("Minotaur – Human-bull hybrid"),
            Legendary::Mishibizhiw =>  v = String::from("Mishibizhiw – Feline water spirit"),
            Legendary::MisiGinebig =>  v = String::from("MisiGinebig – Serpentine rain spirit"),
            Legendary::MisiKinepikw =>  v = String::from("MisiKinepikw – Serpentine rain spirit"),
            Legendary::Mizuchi =>  v = String::from("Mizuchi – Water dragon"),
            Legendary::Mogwai =>  v = String::from("Mogwai – Vengeful ghost or demon"),
            Legendary::Mohan =>  v = String::from("Mohan – Nature spirit"),
            Legendary::MokeleMbembe =>  v = String::from("MokeleMbembe – Water-dwelling creature"),
            Legendary::Mokoi =>  v = String::from("Mokoi – Malevolent spirit that kills sorcerers"),
            Legendary::Mokorea =>  v = String::from("Mokorea"),
            Legendary::Monai =>  v = String::from("Monai – Giant snake with antennae"),
            Legendary::Monocerus =>  v = String::from("Monocerus – One-horned stag-horse-elephant-boar hybrid, sometimes treated as distinct from the unicorn"),
            Legendary::MonoGrande =>  v = String::from("MonoGrande – Giant monkey"),
            Legendary::Monopod =>  v = String::from("Monopod – Dwarf with one giant foot"),
            Legendary::MooinjerVeggey =>  v = String::from("MooinjerVeggey – Nature spirit"),
            Legendary::Mora =>  v = String::from("Mora – Disembodied spirit"),
            Legendary::Morgens =>  v = String::from("Morgens – Water spirits"),
            Legendary::MorinjiNoOkama =>  v = String::from("MorinjiNoOkama – Animated tea kettle"),
            Legendary::Mormolykeia =>  v = String::from("Mormolykeia – Underworld spirit"),
            Legendary::Moroi =>  v = String::from("Moroi – Vampiric ghost"),
            Legendary::MossPeople =>  v = String::from("MossPeople – Little people and tree spirits"),
            Legendary::Mothman =>  v = String::from("Mothman – Large grey winged humanoid with glowing red eyes"),
            Legendary::Mugwump =>  v = String::from("Mugwump – Fish-like lake monster"),
            Legendary::Mujina =>  v = String::from("Mujina – Shapeshifting badger spirit"),
            Legendary::Muldjewangk =>  v = String::from("Muldjewangk – Water monster"),
            Legendary::Multo =>  v = String::from("Multo – Spirit of a deceased person seeking justice or has unfinished business"),
            Legendary::Mummy =>  v = String::from("Mummy – Undead creature who revives"),
            Legendary::MumaPadurii =>  v = String::from("MumaPadurii – Forest-dwelling hag"),
            Legendary::MungoonGali =>  v = String::from("MungoonGali – Giant goanna"),
            Legendary::Muscaliet =>  v = String::from("Muscaliet – Hare-squirrel-boar hybrid that has an intense body heat"),
            Legendary::Muse =>  v = String::from("Muse – Spirits that inspire artists"),
            Legendary::Mushusshu =>  v = String::from("Mushusshu"),
            Legendary::Musimon =>  v = String::from("Musimon – Sheep-goat hybrid"),
            Legendary::Myling =>  v = String::from("Myling – Ghosts of unbaptized children"),
            Legendary::Myrmecoleon =>  v = String::from("Myrmecoleon – Ant-lion hybrid"),
            Legendary::Nachzehrer =>  v = String::from("Nachzehrer – Anthropophagous undead"),
            Legendary::Naga =>  v = String::from("Naga – Nature and water spirits, serpentine or human-serpent hybrids"),
            Legendary::NagaFireballs =>  v = String::from("NagaFireballs – Spectral fire"),
            Legendary::Nagual =>  v = String::from("Nagual – Human-animal shapeshifter"),
            Legendary::Naiad =>  v = String::from("Naiad – Freshwater nymph"),
            Legendary::Nakki =>  v = String::from("Nakki – Water spirit"),
            Legendary::Namahage =>  v = String::from("Namahage – Ritual disciplinary demon from the Oga Peninsula"),
            Legendary::Namazu =>  v = String::from("Namazu – Giant catfish whose thrashing causing earthquakes"),
            Legendary::NandoBaba =>  v = String::from("NandoBaba – Old woman who hides under the floor in abandoned storerooms"),
            Legendary::NangTakian =>  v = String::from("NangTakian – Tree spirit"),
            Legendary::NanomKeeaPoDa =>  v = String::from("NanomKeeaPoDa – Earthquake spirit"),
            Legendary::Napaeae =>  v = String::from("Napaeae – Grotto nymph"),
            Legendary::Narasimha =>  v = String::from("Narasimha – Avatar of Vishnu in the form of half-man/half-lion"),
            Legendary::Narecnitsi =>  v = String::from("Narecnitsi – Fate spirit"),
            Legendary::Nariphon =>  v = String::from("Nariphon – Pod people"),
            Legendary::Nargun =>  v = String::from("Nargun – Water monster"),
            Legendary::Nasnas =>  v = String::from("Nasnas – Half-human, half-demon creature with half a body"),
            Legendary::Nav =>  v = String::from("Nav – Ghost"),
            Legendary::Nawao =>  v = String::from("Nawao – Savage humanoid"),
            Legendary::NDamKenoWet =>  v = String::from("NDamKenoWet – Fish-human hybrid"),
            Legendary::Neptune =>  v = String::from("Neptune – God of freshwater and sea"),
            Legendary::Neck =>  v = String::from("Neck – Female water spirit"),
            Legendary::Negret =>  v = String::from("Negret – Little people that turn into coins"),
            Legendary::Nekomata =>  v = String::from("Nekomata – Split-tailed magical cat"),
            Legendary::Nekomusume =>  v = String::from("Nekomusume – Cat in the form of a girl"),
            Legendary::NemeanLion =>  v = String::from("NemeanLion – Lion with impenetrable skin"),
            Legendary::Nephilim =>  v = String::from("Nephilim – Gigantic sons of Grigori and human women"),
            Legendary::Nereid =>  v = String::from("Nereid – Nymph daughters of Nereus"),
            Legendary::Ngen =>  v = String::from("Ngen – Nature spirit"),
            Legendary::Nguruvilu =>  v = String::from("Nguruvilu – Fox-like water snake"),
            Legendary::Nian =>  v = String::from("Nian – Predatory animal"),
            Legendary::Nightmarchers =>  v = String::from("Nightmarchers – Warrior ghosts"),
            Legendary::Nikusui =>  v = String::from("Nikusui – Monster which appears as a young woman and sucks all of the flesh off of its victim's body"),
            Legendary::Nimerigar =>  v = String::from("Nimerigar – Aggressive little people"),
            Legendary::Ningyo =>  v = String::from("Ningyo – Monkey-fish hybrid"),
            Legendary::NinkiNanka =>  v = String::from("NinkiNanka – Large reptile, possibly a dragon"),
            Legendary::Nisse =>  v = String::from("Nisse – House spirit"),
            Legendary::Niohoggr =>  v = String::from("Niohoggr – Dragon"),
            Legendary::Nivatakavachas =>  v = String::from("Nivatakavachas – Ocean demon"),
            Legendary::Nix =>  v = String::from("Nix – Female water spirit"),
            Legendary::Nobusuma =>  v = String::from("Nobusuma – Supernatural wall, also a monstrous flying squirrel"),
            Legendary::Nocnitsa =>  v = String::from("Nocnitsa – Nightmare spirit"),
            Legendary::NopperaBo =>  v = String::from("NopperaBo – Faceless ghost"),
            Legendary::Nozuchi =>  v = String::from("Nozuchi – Small sea serpent"),
            Legendary::Nuckelavee =>  v = String::from("Nuckelavee – Malevolent human-horse-fish hybrid"),
            Legendary::Nue =>  v = String::from("Nue – Monkey-raccoon dog-tiger-snake hybrid"),
            Legendary::NuGui =>  v = String::from("NuGui – Vengeful female ghost"),
            Legendary::Nukekubi =>  v = String::from("Nukekubi – Disembodied, flying head that attacks people"),
            Legendary::NukuMaiTore =>  v = String::from("NukuMaiTore – Forest spirit"),
            Legendary::Nuli =>  v = String::from("Nuli – Humanoid with backwards, eight-toed feet"),
            Legendary::Numen =>  v = String::from("Numen – Tutelary spirit"),
            Legendary::Nuno =>  v = String::from("Nuno – Malevolent little people"),
            Legendary::Nuppeppo =>  v = String::from("Nuppeppo – Animated chunk of dead flesh"),
            Legendary::Nurarihyon =>  v = String::from("Nurarihyon – Head-sized ball-like creature that floats in the sea and teases sailors"),
            Legendary::NureOnna =>  v = String::from("NureOnna – Female monster who appears on the beach"),
            Legendary::Nurikabe =>  v = String::from("Nurikabe – Spirit that manifests as an impassable, invisible wall"),
            Legendary::NyamiNyami =>  v = String::from("NyamiNyami – Snake-spirit of the Zambezi River"),
            Legendary::Nykstukas =>  v = String::from("Nykstukas – Cavern spirit"),
            Legendary::Nymph =>  v = String::from("Nymph – Nature spirit"),
            Legendary::Obake =>  v = String::from("Obake – Shapeshifting spirits"),
            Legendary::Obariyon =>  v = String::from("Obariyon – Spook which rides piggyback on a human victim and becomes unbearably heavy"),
            Legendary::Obayifo =>  v = String::from("Obayifo – Vampiric possession spirit"),
            Legendary::Obia =>  v = String::from("Obia – Gigantic animal that serves witches"),
            Legendary::Oceanid =>  v = String::from("Oceanid – Nymph daughters of Oceanus"),
            Legendary::Odei =>  v = String::from("Odei – Storm spirit"),
            Legendary::Odin =>  v = String::from("Odin – King of Asgard"),
            Legendary::Odmience =>  v = String::from("Odmience – Changeling"),
            Legendary::Og =>  v = String::from("Og – Giant king of the Amorites"),
            Legendary::Ogopogo =>  v = String::from("Ogopogo Canadian Lake Monster"),
            Legendary::Ogun =>  v = String::from("Ogun"),
            Legendary::Ogre =>  v = String::from("Ogre – Large, grotesque humanoid"),
            Legendary::Oiwa =>  v = String::from("Oiwa – Ghost of a woman with a distorted face who was murdered by her husband"),
            Legendary::Ojancanu =>  v = String::from("Ojancanu – Giant cyclops who embodies evil."),
            Legendary::Okiku =>  v = String::from("Okiku – Spirit of a plate-counting servant girl, associated with the 'Okiku-Mushi' worm"),
            Legendary::Okubi =>  v = String::from("Okubi – Death spirit"),
            Legendary::OkuriInu =>  v = String::from("OkuriInu – Dog or wolf that follows travelers at night, similar to the Black dog of English folklore"),
            Legendary::OleHigue =>  v = String::from("OleHigue – Vampiric hag who takes the form of a fireball at night"),
            Legendary::Omukade =>  v = String::from("Omukade – Giant, human-eating centipede that lives in the mountains"),
            Legendary::Oni =>  v = String::from("Oni – Large, grotesque humanoid demon, usually having red skin and horns"),
            Legendary::Onibi =>  v = String::from("Onibi – Spectral fire"),
            Legendary::Onmoraki =>  v = String::from("Onmoraki – Bird-demon created from the spirits of freshly dead corpses"),
            Legendary::Onocentaur =>  v = String::from("Onocentaur – Human-donkey hybrid"),
            Legendary::Onoskelis =>  v = String::from("Onoskelis – Shapeshifting demon"),
            Legendary::Onryo =>  v = String::from("Onryo – Vengeful ghost that manifests in a physical rather than a spectral form"),
            Legendary::Onza =>  v = String::from("Onza – Wild cat, possibly a subspecies of cougar"),
            Legendary::OozlumBird =>  v = String::from("OozlumBird – Bird that flies backwards"),
            Legendary::Ophiotaurus =>  v = String::from("Ophiotaurus – Bull-serpent hybrid"),
            Legendary::Opinicus =>  v = String::from("Opinicus – Lion-eagle hybrid, similar to a griffin, but with leonine forelimbs"),
            Legendary::OrangBunian =>  v = String::from("OrangBunian – Forest spirit"),
            Legendary::OrangMinyak =>  v = String::from("OrangMinyak – Spectral rapist"),
            Legendary::Ordog =>  v = String::from("Ordog – Shapeshifting demon"),
            Legendary::Oread =>  v = String::from("Oread – Mountain nymph"),
            Legendary::Ork =>  v = String::from("Ork – Little people and house spirits"),
            Legendary::Orobas =>  v = String::from("Orobas – Horse-headed, honest oracle classed as a demon"),
            Legendary::OrphanBird =>  v = String::from("OrphanBird – Peacock-eagle-swan-crane hybrid"),
            Legendary::Orthrus =>  v = String::from("Orthrus – Two-headed dog"),
            Legendary::Osiris =>  v = String::from("Osiris – God of the dead and the judge of the underworld"),
            Legendary::Oshun =>  v = String::from("Oshun – God of love and fertility"),
            Legendary::Otso =>  v = String::from("Otso – Bear spirit"),
            Legendary::Ouroboros =>  v = String::from("Ouroboros – Mystic serpent/dragon that eats its own tail"),
            Legendary::Ovinnik =>  v = String::from("Ovinnik – Malevolent threshing house spirit"),
            Legendary::Owlman =>  v = String::from("Owlman – Owl-like humanoid"),
            Legendary::PaasselkaDevils =>  v = String::from("PaasselkaDevils – Spectral fire"),
            Legendary::Pamola =>  v = String::from("Pamola – Weather spirit"),
            Legendary::Panes =>  v = String::from("Panes – Human-goat hybrids descended from the god Pan"),
            Legendary::Pandi =>  v = String::from("Pandi – White-haired humanoid with giant ears and eight fingers and toes"),
            Legendary::Panis =>  v = String::from("Panis – Demons with herds of stolen cows"),
            Legendary::Panlong =>  v = String::from("Panlong – Water dragon"),
            Legendary::Panotti =>  v = String::from("Panotti – Humanoid with gigantic ears"),
            Legendary::Panther =>  v = String::from("Panther – Feline with sweet breath"),
            Legendary::Parandrus =>  v = String::from("Parandrus – Shapeshifting animal whose natural form was a large ruminant"),
            Legendary::Pard =>  v = String::from("Pard – Fast, spotted feline believed to mate with lions to produce leopards"),
            Legendary::Pardalokampoi =>  v = String::from("Pardalokampoi – Fish-tailed leopard"),
            Legendary::Patagon =>  v = String::from("Patagon – Giant race reputed to live in the area of Patagonia"),
            Legendary::Patasola =>  v = String::from("Patasola – Anthropophagous, one-legged humanoid"),
            Legendary::Patupairehe =>  v = String::from("Patupairehe – White-skinned nature spirits"),
            Legendary::Pech =>  v = String::from("Pech – Strong little people"),
            Legendary::Pegaeae =>  v = String::from("Pegaeae – Spring nymph"),
            Legendary::Pegasus =>  v = String::from("Pegasus – Winged horse"),
            Legendary::Pegacorn =>  v = String::from("Pegasus-unicorn hybridPegasus-unicorn hybrid"),
            Legendary::Pelesit =>  v = String::from("Pelesit – Servant spirit"),
            Legendary::Peluda =>  v = String::from("Peluda – Dragon"),
            Legendary::Penanggalan =>  v = String::from("Penanggalan – Vampires that sever their heads from their bodies to fly around, usually with their intestines or other internal organs trailing behind"),
            Legendary::Peng =>  v = String::from("Peng – Giant bird"),
            Legendary::Penghou =>  v = String::from("Penghou – Tree spirit"),
            Legendary::Peri =>  v = String::from("Peri – Winged humanoid"),
            Legendary::Peryton =>  v = String::from("Peryton – Deer-bird hybrid"),
            Legendary::Pesanta =>  v = String::from("Pesanta – Nightmare demon in the form of a cat or dog"),
            Legendary::Peuchen =>  v = String::from("Peuchen – Vampiric, flying, shapeshifting serpent"),
            Legendary::PhiTaiHong =>  v = String::from("PhiTaiHong – Ghost of a person who has died suddenly of a violent or cruel death"),
            Legendary::Phoenix =>  v = String::from("Phoenix – Regenerative bird reborn from its own ashes"),
            Legendary::Piasa =>  v = String::from("Piasa – Winged, antlered feline-like dragon"),
            Legendary::Piatek =>  v = String::from("Piatek – Large land animal"),
            Legendary::PictishBeast =>  v = String::from("PictishBeast – Stylistic animal, possibly a dragon"),
            Legendary::Pillan =>  v = String::from("Pillan – Nature spirit"),
            Legendary::Plagg =>  v = String::from("Plagg"),
            Legendary::PimSkwaWagenOwad =>  v = String::from("PimSkwaWagenOwad – Water spirit"),
            Legendary::Piru =>  v = String::from("Piru – Minor demon"),
            Legendary::Pishacha =>  v = String::from("Pishacha – Carrion-eating demon"),
            Legendary::Pishtaco =>  v = String::from("Pishtaco – Monster man that steals its victim's body fat for cannibalistic purposes"),
            Legendary::PitaSkog =>  v = String::from("PitaSkog – Serpentine rain spirit"),
            Legendary::Pixie =>  v = String::from("Pixie – Little people and nature spirits"),
            Legendary::Pixiu =>  v = String::from("Pixiu – Winged lion"),
            Legendary::PiYao =>  v = String::from("PiYao – Horned, dragon-lion hybrid"),
            Legendary::Plakavac =>  v = String::from("Plakavac – Vampire created when a mother strangles her child"),
            Legendary::PokWejeeMen =>  v = String::from("PokWejeeMen – Tree spirit"),
            Legendary::Polevik =>  v = String::from("Polevik – Little people and field spirits"),
            Legendary::PolloMaligno =>  v = String::from("PolloMaligno – Man-eating chicken spirit"),
            Legendary::Polong =>  v = String::from("Polong – Invisible servant spirit"),
            Legendary::Poltergeist =>  v = String::from("Poltergeist – Ghost that moves objects"),
            Legendary::Pombero =>  v = String::from("Pombero – Wild man and nature spirit"),
            Legendary::Ponaturi =>  v = String::from("Ponaturi – Grotesque, malevolent humanoid"),
            Legendary::Pontianak =>  v = String::from("Pontianak – Undead, vampiric women who died in childbirth"),
            Legendary::PopeLickMonster =>  v = String::from("PopeLickMonster Kentucky Urban Legend – Cryptid, a murderous creature that is part man, sheep, and goat"),
            Legendary::Poukai =>  v = String::from("Poukai – Giant bird"),
            Legendary::Preta =>  v = String::from("Preta – Ghosts of especially greedy people"),
            Legendary::Pricolici =>  v = String::from("Pricolici – Undead wolf"),
            Legendary::Psoglav =>  v = String::from("Psoglav – Dog-headed monster"),
            Legendary::Psotnik =>  v = String::from("Psotnik – Mischievous spirit"),
            Legendary::Psychai =>  v = String::from("Psychai – Butterfly-winged nymphs, daughters of Psyche"),
            Legendary::Psychopomp =>  v = String::from("Psychopomp – Creatures, spirits, angels, or deities in many religions who escort newly deceased souls from Earth to the afterlife"),
            Legendary::Puca =>  v = String::from("Puca – Shapeshifting animal spirit"),
            Legendary::Puki =>  v = String::from("Puki – Malevolent little person"),
            Legendary::Puck =>  v = String::from("Puck – House spirit"),
            Legendary::Putz =>  v = String::from("Putz – House spirit"),
            Legendary::Pugot =>  v = String::from("Pugot – Headless humanoid"),
            Legendary::Puk =>  v = String::from("Puk – House spirit"),
            Legendary::Pukis =>  v = String::from("Pukis – Dragon"),
            Legendary::Puckwudgie =>  v = String::from("Puckwudgie – Troll-like gray-skinned being"),
            Legendary::Pygmy =>  v = String::from("Pygmy – Little people"),
            Legendary::Pyrausta =>  v = String::from("Pyrausta – Insect-dragon hybrid"),
            Legendary::Python =>  v = String::from("Python – Serpentine dragon"),
            Legendary::Qalupalik =>  v = String::from("Qalupalik – Aquatic human abductor"),
            Legendary::Qilin =>  v = String::from("Qilin – Dragon-ox-deer hybrid"),
            Legendary::Qiqirn =>  v = String::from("Qiqirn – Large, bald dog spirit"),
            Legendary::Qliphoth =>  v = String::from("Qliphoth – Evil spirits"),
            Legendary::QuestingBeast =>  v = String::from("QuestingBeast – Serpent-leopard-lion-hart hybrid"),
            Legendary::Quetzalcoatl =>  v = String::from("Quetzalcoatl – Important Aztec god whose name means 'feathered serpent'; he is not to be confused with the quetzal, a type of bird"),
            Legendary::Quinotaur =>  v = String::from("Quinotaur – Five-horned bull"),
            Legendary::Ra =>  v = String::from("Ra – Spirit that protects a specific place"),
            Legendary::Rabisu =>  v = String::from("Rabisu – Vampiric spirit that ambushes people"),
            Legendary::Radande =>  v = String::from("Radande – Tree spirit"),
            Legendary::Ragana =>  v = String::from("Ragana – Malevolent witch"),
            Legendary::Raiju =>  v = String::from("Raiju – Lightning spirit"),
            Legendary::RainBird =>  v = String::from("RainBird – Rain spirit"),
            Legendary::RainbowCrow =>  v = String::from("RainbowCrow – Crow spirit"),
            Legendary::RainbowFish =>  v = String::from("RainbowFish – Whale-sized, multi-colored fish"),
            Legendary::RainbowSerpent =>  v = String::from("RainbowSerpent – Snake"),
            Legendary::Rakshasa =>  v = String::from("Rakshasa – Shapeshifting demon"),
            Legendary::Ramidreju =>  v = String::from("Ramidreju – Extremely long, weasel-like animal"),
            Legendary::Rarog =>  v = String::from("Rarog – Whirlwind spirit"),
            Legendary::RavenMocker =>  v = String::from("RavenMocker – Life-draining spirit"),
            Legendary::RavenSpirit =>  v = String::from("RavenSpirit – Trickster spirit"),
            Legendary::Ratatoskr =>  v = String::from("Ratatoskr – Squirrel spirit"),
            Legendary::RaystownRay =>  v = String::from("RaystownRay – Possible plesiosaur or serpent"),
            Legendary::Redcap =>  v = String::from("Redcap – Evil, ugly humanoid"),
            Legendary::ReEm =>  v = String::from("ReEm – Gigantic land animal"),
            Legendary::Reichsadler =>  v = String::from("Reichsadler – Eagle, sometimes depicted with two heads"),
            Legendary::Rephaite =>  v = String::from("Rephaite – Giant"),
            Legendary::ReptilianHumanoid =>  v = String::from("ReptilianHumanoid – Human-lizard hybrid"),
            Legendary::Revenant =>  v = String::from("Revenant – Reanimated dead"),
            Legendary::Roc =>  v = String::from("Roc – Gigantic bird"),
            Legendary::Rokurokubi =>  v = String::from("Rokurokubi – Long-necked, humanoid trickster"),
            Legendary::Rompo =>  v = String::from("Rompo – Skeletal creature with elements of a rabbit, badger, and bear"),
            Legendary::Rong =>  v = String::from("Rong dragon"),
            Legendary::Rougarou =>  v = String::from("Rougarou – Human-wolf shapeshifter"),
            Legendary::Rusalka =>  v = String::from("Rusalka – Female water spirit"),
            Legendary::Ryu =>  v = String::from("Japanese dragonJapanese dragon"),
            Legendary::Saci =>  v = String::from("Saci – One-legged nature spirit"),
            Legendary::Sagari =>  v = String::from("Sagari – Horse head that dangles from trees on Kyūshū"),
            Legendary::Sakabashira =>  v = String::from("Sakabashira – Haunted pillar, installed upside-down"),
            Legendary::Salamander =>  v = String::from("Salamander – Fire elemental"),
            Legendary::Samebito =>  v = String::from("Samebito – Shark-man servant of the dragon king of the sea"),
            Legendary::Samodiva =>  v = String::from("Samodiva – Nature spirit"),
            Legendary::Sampati =>  v = String::from("Sampati – The demigod Jatayu's brother"),
            Legendary::Sandman =>  v = String::from("Sandman – Nursery spirit that induces sleep in children"),
            Legendary::Sango =>  v = String::from("Sango – Yoruba king of arts, music, dance and entertainment"),
            Legendary::Santelmo =>  v = String::from("Santelmo – Spirits in the form of fireballs that roam around the forest"),
            Legendary::SantaClaus =>  v = String::from("Santa Claus, also known as Father Christmas, Saint Nicholas, Saint Nick, Kris Kringle, or simply Santa, is a legendary character originating in Western Christian culture who is said to bring gifts on Christmas Eve of toys and candy to well-behaved children, and either coal or nothing to naughty children. He is said to accomplish this with the aid of Christmas elves, who make the toys in his workshop at the North Pole, and flying reindeer who pull his sleigh through the air.  The modern character of Santa Claus was based on traditions surrounding the historical Saint Nicholas (a fourth-century Greek bishop and gift-giver of Myra), the English figure of Father Christmas, and the Dutch figure of Sinterklaas (also based on Saint Nicholas).  Santa Claus is generally depicted as a portly, jolly, white-bearded man, often with spectacles, wearing a red coat with white fur collar and cuffs, white-fur-cuffed red trousers, red hat with white fur, and black leather belt and boots, carrying a bag full of gifts for children. He is commonly portrayed as laughing in a way that sounds like 'ho ho ho'. This image became popular in the United States and Canada in the 19th century due to the significant influence of the 1823 poem 'A Visit from St. Nicholas'. Caricaturist and political cartoonist Thomas Nast also played a role in the creation of Santa's image. This image has been maintained and reinforced through song, radio, television, children's books, family Christmas traditions, films, and advertising. "),
            Legendary::Sanziana =>  v = String::from("Sanziana – Nature spirit"),
            Legendary::Sarimanok =>  v = String::from("Sarimanok – Bird of good fortune"),
            Legendary::Sarngika =>  v = String::from("Sarngika – Bird spirit"),
            Legendary::Sarugami =>  v = String::from("Sarugami – Wicked monkey spirit who was defeated by a dog"),
            Legendary::Satori =>  v = String::from("Satori – Mind-reading humanoid"),
            Legendary::Satan =>  v = String::from("Satan – Ruler of Hell"),
            Legendary::Satyr =>  v = String::from("Satyr – Human-goat hybrid and fertility spirit"),
            Legendary::Satyrus =>  v = String::from("Satyrus – Apes who always bear twins, one the mother loves, the other it hates"),
            Legendary::SazaeOni =>  v = String::from("SazaeOni – Shapeshifting turban snail spirit"),
            Legendary::Sceadugenga =>  v = String::from("Sceadugenga – Shapeshifting undead"),
            Legendary::Scitalis =>  v = String::from("Scitalis – Snake which mesmerizes its prey"),
            Legendary::ScorpionMan =>  v = String::from("ScorpionMan – Human-scorpion hybrid"),
            Legendary::Scylla =>  v = String::from("Scylla – Human-snake hybrid with a snake's tail, twelve legs, and six long-necked snake heads"),
            Legendary::SeaBee =>  v = String::from("SeaBee – Fish-tailed bee"),
            Legendary::SeaLion =>  v = String::from("SeaLion a legendary creature that has the head and upper body of a lion, but with webbed forelimbs and a fish tail."),
            Legendary::SeaMonk =>  v = String::from("SeaMonk – Fish-like humanoid"),
            Legendary::SeaMonster =>  v = String::from("SeaMonster – Giant, marine animals"),
            Legendary::SeaSerpent =>  v = String::from("SeaSerpent – Serpentine sea monster"),
            Legendary::SeaWyvern =>  v = String::from("SeaWyvern – Fish-tailed wyvern"),
            Legendary::Seko =>  v = String::from("Seko – Water spirit which can be heard making merry at night"),
            Legendary::Selkie =>  v = String::from("Selkie – Human-seal shapeshifter"),
            Legendary::SenpokuKanpoku =>  v = String::from("SenpokuKanpoku – Human-faced frog which guides newly deceased souls to the graveyard"),
            Legendary::Seps =>  v = String::from("Seps – Snake with corrosive venom"),
            Legendary::Serpent =>  v = String::from("Serpent – Snake spirit"),
            Legendary::Serpopard =>  v = String::from("Serpopard – Serpent-leopard hybrid"),
            Legendary::Shachihoko =>  v = String::from("Shachihoko – Tiger-carp hybrid"),
            Legendary::Shade =>  v = String::from("Shade – Spiritual imprint"),
            Legendary::ShadowPeople =>  v = String::from("ShadowPeople – Malevolent ghost"),
            Legendary::Shahbaz =>  v = String::from("Shahbaz – Giant eagle or hawk"),
            Legendary::Shaitan =>  v = String::from("Shaitan from the Bible"),
            Legendary::ShangYang =>  v = String::from("ShangYang – Rain bird"),
            Legendary::Shedim =>  v = String::from("Shedim – Chicken-legged demon"),
            Legendary::Shedu =>  v = String::from("Shedu – Protective spirit who takes the form of a winged bull or human-headed lion"),
            Legendary::Shellycoat =>  v = String::from("Shellycoat – Water spirit"),
            Legendary::Shen =>  v = String::from("Shen – Shapeshifing sea monster"),
            Legendary::Shenlong =>  v = String::from("Shenlong – Weather dragon"),
            Legendary::Shibaten =>  v = String::from("Shibaten – Water spirit from Shikoku"),
            Legendary::Shikigami =>  v = String::from("Shikigami – Servant spirit"),
            Legendary::ShikiOji =>  v = String::from("ShikiOji – Child-sized servant spirit"),
            Legendary::Shikome =>  v = String::from("Shikome – Underworld hag"),
            Legendary::Shinigami =>  v = String::from("Shinigami – 'Death god'"),
            Legendary::ShiroBozu =>  v = String::from("ShiroBozu – White, faceless spirit"),
            Legendary::Shirouneri =>  v = String::from("Shirouneri – Animated mosquito netting or dust cloth"),
            Legendary::Shiryo =>  v = String::from("Shiryo – Spirit of a dead person"),
            Legendary::Shisa =>  v = String::from("Shisa – Lion-dog hybrid"),
            Legendary::Shishi =>  v = String::from("Shishi – Protective animal"),
            Legendary::Shojo =>  v = String::from("Shojo – Red-haired sea-sprites who love alcohol"),
            Legendary::Shokera =>  v = String::from("Shokera – Creature that peers in through skylights"),
            Legendary::Shtriga =>  v = String::from("Shtriga – Vampire witch that feeds on children"),
            Legendary::ShuiGui =>  v = String::from("ShuiGui – Drowned ghost"),
            Legendary::ShugMonkey =>  v = String::from("ShugMonkey – Dog/monkey"),
            Legendary::Shunoban =>  v = String::from("Shunoban – Red-faced ghoul"),
            Legendary::ShutenDoji =>  v = String::from("ShutenDoji – Ruler of the Oni"),
            Legendary::Sídhe =>  v = String::from("Sídhe – Ancestral or nature spirit"),
            Legendary::Sigbin =>  v = String::from("Sigbin – Goat-like vampire"),
            Legendary::Sileni =>  v = String::from("Sileni – Bald, fat, thick-lipped, and flat-nosed followers of Dionysus"),
            Legendary::Simargl =>  v = String::from("Simargl – Winged dog"),
            Legendary::Simurgh =>  v = String::from("Simurgh – Dog-lion-peacock hybrid"),
            Legendary::Singa =>  v = String::from("Singa – Feline animal"),
            Legendary::SintHolo =>  v = String::from("SintHolo – Serpentine rain spirit"),
            Legendary::Siren =>  v = String::from("Siren – Human-bird hybrid"),
            Legendary::Sirin =>  v = String::from("Sirin – Demonic human-headed bird"),
            Legendary::Sirrush =>  v = String::from("Sirrush – Dragon with aquiline hind legs and feline forelegs"),
            Legendary::Sisiutl =>  v = String::from("Sisiutl – Two-headed sea serpent"),
            Legendary::SiTeCah =>  v = String::from("SiTeCah – Red-haired giants"),
            Legendary::Sjora =>  v = String::from("Sjora – Freshwater spirit"),
            Legendary::Sjovaettir =>  v = String::from("Sjovaettir – Sea spirit"),
            Legendary::SkinWalker =>  v = String::from("SkinWalker – Animal-human shapeshifter"),
            Legendary::Skogsra =>  v = String::from("Skogsra – Forest spirit"),
            Legendary::Skoll =>  v = String::from("Skoll – Wolf that chases the Sun"),
            Legendary::Skookum =>  v = String::from("Skookum – Hairy giant"),
            Legendary::Skeleton =>  v = String::from("Skeleton – Living skeletons"),
            Legendary::Skrzak =>  v = String::from("Skrzak – Flying imp"),
            Legendary::SkyWomen =>  v = String::from("SkyWomen – Weather spirit"),
            Legendary::Sleipnir =>  v = String::from("Sleipnir – Eight-legged horse"),
            Legendary::Sluagh =>  v = String::from("Sluagh – Restless ghost"),
            Legendary::SodehikiKozo =>  v = String::from("SodehikiKozo – Invisible spirit which pulls on sleeves"),
            Legendary::Sogenbi =>  v = String::from("Sogenbi – Fiery ghost of an oil-stealing monk"),
            Legendary::Soragami =>  v = String::from("Soragami – Ritual disciplinary demon"),
            Legendary::SorakiGaeshi =>  v = String::from("SorakiGaeshi – Sound of trees being cut down, when later none seem to have been cut"),
            Legendary::Sorobanbozu =>  v = String::from("Sorobanbozu – Ghost with an abacus"),
            Legendary::Sotangitsune =>  v = String::from("Sotangitsune – Fox spirit from Kyoto"),
            Legendary::Soucouyant =>  v = String::from("Soucouyant – Vampiric hag who takes the form of a fireball at night"),
            Legendary::Spearfinger =>  v = String::from("Spearfinger – Sharp-fingered hag"),
            Legendary::Spectre =>  v = String::from("Spectre – Terrifying ghost"),
            Legendary::Sphinx =>  v = String::from("Sphinx – Winged woman-headed lion"),
            Legendary::Spiridus =>  v = String::from("Spiridus – Little people"),
            Legendary::Spirit =>  v = String::from("GhostsGhosts"),
            Legendary::Spriggan =>  v = String::from("Spriggan – Guardians of graveyards and ruins"),
            Legendary::Sprite =>  v = String::from("Sprite – little people, ghosts or elves"),
            Legendary::Squonk =>  v = String::from("Squonk – Ugly and lonely creature capable of evading capture by dissolving itself into a pool of tears"),
            Legendary::Stihi =>  v = String::from("Stihi – Demonic dragon who guards a treasure"),
            Legendary::Strigoi =>  v = String::from("Strigoi – Vampire"),
            Legendary::Strix =>  v = String::from("Strix – Vampiric bird"),
            Legendary::Struthopodes =>  v = String::from("Struthopodes – Humanoid whose males have enormous feet, and females have tiny feet"),
            Legendary::Strzyga =>  v = String::from("Strzyga – Vampiric undead"),
            Legendary::Stuhac =>  v = String::from("Stuhac – Malevolent mountain spirit"),
            Legendary::StymphalianBird =>  v = String::from("StymphalianBird – Metallic bird"),
            Legendary::Suangi =>  v = String::from("Suangi – Cannibalistic sorcerer"),
            Legendary::Succubus =>  v = String::from("Succubus – Female night-demon"),
            Legendary::Sudice =>  v = String::from("Sudice – Fortune spirit"),
            Legendary::SunakakeBaba =>  v = String::from("SunakakeBaba – Sand-throwing hag"),
            Legendary::Sunekosuri =>  v = String::from("Sunekosuri – Small dog- or cat-like creature that rubs against a person's legs at night"),
            Legendary::Surma =>  v = String::from("Surma – Hellhound"),
            Legendary::Suzaku =>  v = String::from("Suzaku – Japanese version of the Chinese Vermillion Bird"),
            Legendary::Svaoilfari =>  v = String::from("Svaoilfari – Unnatural strong horse, father of Sleipnir"),
            Legendary::Svartalfar =>  v = String::from("Svartalfar – Cavern spirits; the Black Elves"),
            Legendary::Swallower =>  v = String::from("Swallower – Crocodile-leopard-hippopotamus hybrid"),
            Legendary::SwanMaiden =>  v = String::from("SwanMaiden – Swan-human shapeshifter"),
            Legendary::Sylph =>  v = String::from("Sylph – Air elemental"),
            Legendary::Sylvan =>  v = String::from("Sylvan – Forest spirit"),
            Legendary::Syrbotae =>  v = String::from("Syrbotae – African giant"),
            Legendary::Syrictae =>  v = String::from("Syrictae – Reptilian humanoid"),
            Legendary::Tachash =>  v = String::from("Tachash – Large land animal"),
            Legendary::Tailypo =>  v = String::from("Tailypo – Powerful animal, that takes revenge on those who steal its tail"),
            Legendary::Taimatsumaru =>  v = String::from("Taimatsumaru – Tengu surrounded in demonic fire"),
            Legendary::Takam =>  v = String::from("Takam – Nature spirit"),
            Legendary::TakaOnna =>  v = String::from("TakaOnna – Female spirit which can stretch itself to peer into the second story of a building"),
            Legendary::Talos =>  v = String::from("Talos – Giant made of bronze"),
            Legendary::Tangie =>  v = String::from("Tangie – Shapeshifting water spirit"),
            Legendary::Taniwha =>  v = String::from("Taniwha – Water spirit"),
            Legendary::Tantankororin =>  v = String::from("Tantankororin – Unharvested persimmon which becomes a monster"),
            Legendary::Tanuki =>  v = String::from("Tanuki – Shapeshifting raccoon dog"),
            Legendary::TaotaoMona =>  v = String::from("TaotaoMona – Ancestral spirits"),
            Legendary::Taotie =>  v = String::from("Taotie – Greed spirit"),
            Legendary::Tapairu =>  v = String::from("Tapairu – Nature spirit"),
            Legendary::Tarasque =>  v = String::from("Tarasque – Dragon with leonine, turtle, bear, and human attributes"),
            Legendary::Tartalo =>  v = String::from("Tartalo – One-eyed giant"),
            Legendary::Tartaruchi =>  v = String::from("Tartaruchi – Demonic punisher"),
            Legendary::TatamiTataki =>  v = String::from("TatamiTataki – Poltergeist that hits the tatami mats at night"),
            Legendary::Tatzelwurm =>  v = String::from("Tatzelwurm lizard-like creature, often described as having the face of a cat, with a serpent-like body which may be slender or stubby, with four short legs or two forelegs"),
            Legendary::Tatsu =>  v = String::from("Japanese dragonJapanese dragon"),
            Legendary::Taurokampoi =>  v = String::from("Taurokampoi – Fish-tailed bull"),
            Legendary::Tavara =>  v = String::from("Tavara – Night-demon"),
            Legendary::TejuJagua =>  v = String::from("TejuJagua – Lizard with seven dog heads"),
            Legendary::Tecumbalam =>  v = String::from("Tecumbalam – Bird"),
            Legendary::Tengu =>  v = String::from("Tengu – Anthropomorphic bird"),
            Legendary::Tennin =>  v = String::from("Tennin – Angelic humanoid"),
            Legendary::TeNoMe =>  v = String::from("TeNoMe – Ghost of a blind man, with his eyes on his hands"),
            Legendary::Tepegoz =>  v = String::from("Tepegoz – Azerbaijani mythical creature similar to the cyclops Polyphemus"),
            Legendary::TerribleMonster =>  v = String::from("TerribleMonster – Lion-eagle-scorpion hybrid made from the blood of murder victims"),
            Legendary::TeumessianFox =>  v = String::from("TeumessianFox – Gigantic fox"),
            Legendary::Theriocephalus =>  v = String::from("Theriocephalus – Animal-headed humanoid"),
            Legendary::ThreeLeggedBird =>  v = String::from("ThreeLeggedBird – Solar bird"),
            Legendary::Thunderbird =>  v = String::from("Thunderbird – Avian lightning bird spirit"),
            Legendary::Thor =>  v = String::from("Thor – God of thunder and storm"),
            Legendary::Tiangou =>  v = String::from("Tiangou – Meteoric dog"),
            Legendary::Tianlong =>  v = String::from("Tianlong – Celestial dragon"),
            Legendary::Tibicena =>  v = String::from("Tibicena – Evil Dog"),
            Legendary::TiddyMun =>  v = String::from("TiddyMun – Bog spirit"),
            Legendary::Tigmamanukan =>  v = String::from("Tigmamanukan – Asian fairy bluebird"),
            Legendary::Tigris =>  v = String::from("Tigris – Giant lion"),
            Legendary::Tikbalang =>  v = String::from("Tikbalang – Anthropomorphic horse"),
            Legendary::Tikoloshe =>  v = String::from("Tikoloshe – Little people and water spirit"),
            Legendary::Timingila =>  v = String::from("Timingila – Sea monster"),
            Legendary::Tipua =>  v = String::from("Tipua – Spirit that protects a specific place"),
            Legendary::Titan =>  v = String::from("Titan – Primeval god"),
            Legendary::Tiyanak =>  v = String::from("Tiyanak – Demons that are souls of dead unbaptized babies"),
            Legendary::Tizheruk =>  v = String::from("Tizheruk – Sea serpent"),
            Legendary::Tlahuelpuchi =>  v = String::from("Tlahuelpuchi – Shapeshifting vampire"),
            Legendary::TofuKozo =>  v = String::from("TofuKozo – Spirit child carrying a block of tofu"),
            Legendary::ToireNoHanakosan =>  v = String::from("ToireNoHanakosan – Ghost who lurks in grade school restroom stalls"),
            Legendary::Tomte =>  v = String::from("Tomte – House spirit"),
            Legendary::ToothFairy =>  v = String::from("The Tooth Fairy is a fantasy figure of early childhood in Western and Western-influenced cultures. The folklore states that when children lose one of their baby teeth, they should place it underneath their pillow or on their bedside table and the Tooth Fairy will visit while they sleep, replacing the lost tooth with a small payment."),
            Legendary::The tradition of leaving a tooth under a pillow for the Tooth Fairy or another fantasy figure to collect is practiced in various countries. =>  v = String::from("The tradition of leaving a tooth under a pillow for the Tooth Fairy or another fantasy figure to collect is practiced in various countries."),
            Legendary::Topielec =>  v = String::from("Topielec – Water spirit"),
            Legendary::Totetsu =>  v = String::from("Totetsu – Greed spirit"),
            Legendary::Toyol =>  v = String::from("Toyol – Servant spirit"),
            Legendary::Trasgo =>  v = String::from("Trasgo – Grotesque, mischievous little people"),
            Legendary::Trauco =>  v = String::from("Trauco – Fertility spirit"),
            Legendary::Trenti =>  v = String::from("Trenti – Diminutive demon"),
            Legendary::Trickster =>  v = String::from("Character in a story which exhibits a great degree of intellect or secret knowledge, and uses it to play tricks or otherwise disobey normal rules and conventional behaviourCharacter in a story which exhibits a great degree of intellect or secret knowledge, and uses it to play tricks or otherwise disobey normal rules and conventional behaviour"),
            Legendary::Tripurasura =>  v = String::from("Tripurasura – Demonic inhabitants of Tripura"),
            Legendary::Tritons =>  v = String::from("Tritons – Male human-fish hybrid"),
            Legendary::Troll =>  v = String::from("Troll – Nature spirit"),
            Legendary::Trow =>  v = String::from("Trow – Little people and nature spirits"),
            Legendary::TsiNoo =>  v = String::from("TsiNoo – Vampiric demon"),
            Legendary::Tsuchigumo =>  v = String::from("Tsuchigumo – Shapeshifting, giant spider"),
            Legendary::Tsuchinoko =>  v = String::from("Tsuchinoko – Plump snake-like creature"),
            Legendary::Tsukumogami =>  v = String::from("Tsukumogami – Inanimate object that becomes animated after existing for 100 years"),
            Legendary::TsulKalu =>  v = String::from("TsulKalu – Giant nature spirit"),
            Legendary::TsuraraOnna =>  v = String::from("TsuraraOnna – Icicle woman"),
            Legendary::TsurubeOtoshi =>  v = String::from("TsurubeOtoshi – Monster which drops or lowers a bucket from the top of a tree to catch people"),
            Legendary::TugarinZmeyevich =>  v = String::from("TugarinZmeyevich – Evil shapeshifter"),
            Legendary::TylwythTeg =>  v = String::from("TylwythTeg – Nature spirit"),
            Legendary::Tupilaq =>  v = String::from("Tupilaq – Animated construct"),
            Legendary::Turehu =>  v = String::from("Turehu – Pale spirit"),
            Legendary::Turst =>  v = String::from("Turst – legendary figure who turns people into dogs"),
            Legendary::Turul =>  v = String::from("Turul – Giant falcon that helped shape the origins of the Magyars"),
            Legendary::Tyger =>  v = String::from("Tyger – Like a real tiger, but lacks stripes. It has the tufted tail of a lion and a thick mane along the neck like a horse"),
            Legendary::Typhon =>  v = String::from("Typhon – Winged, snake-legged giant"),
            Legendary::Tzitzimitl =>  v = String::from("Tzitzimitl – Skeletal star spirit"),
            Legendary::Ubume =>  v = String::from("Ubume – Ghosts of women who died in childbirth"),
            Legendary::UchekLangmeidong =>  v = String::from("///(Manipuri mythology) – Semi human, semi hornbill creature – Semi human, semi hornbill creature"),
            Legendary::UmaNoAshi =>  v = String::from("UmaNoAshi – Horse's leg which dangles from a tree and kicks passersby"),
            Legendary::Umibozu =>  v = String::from("Umibozu – Ghost of drowned priest"),
            Legendary::UmiNyobo =>  v = String::from("UmiNyobo – Female sea monster who steals fish"),
            Legendary::Undead =>  v = String::from("Undead – Dead that behave as if alive"),
            Legendary::UnderwaterPanther =>  v = String::from("UnderwaterPanther – Feline water spirit"),
            Legendary::Undine =>  v = String::from("Undine – Water elemental"),
            Legendary::Unhcegila =>  v = String::from("Unhcegila – Dragon"),
            Legendary::Unicorn =>  v = String::from("Unicorn – Unicorn that inhabits the African Congo."),
            Legendary::Unktehi =>  v = String::from("Unktehi – Serpentine rain spirit"),
            Legendary::Unktehila =>  v = String::from("Unktehila – Reptilian water monster"),
            Legendary::Upinis =>  v = String::from("Upinis – River spirit"),
            Legendary::Urayuli =>  v = String::from("Urayuli – Hairy giant"),
            Legendary::Urias =>  v = String::from("Urias – Giant"),
            Legendary::Urmahlullu =>  v = String::from("Urmahlullu – Lion-human hybrid guardian spirit"),
            Legendary::UshiOni =>  v = String::from("UshiOni – Bull-headed monster"),
            Legendary::Utukku =>  v = String::from("Utukku – ″Underworld messenger spirit″"),
            Legendary::Uwan =>  v = String::from("Uwan – Spirit that shouts to surprise people"),
            Legendary::Vadatajs =>  v = String::from("Vadatajs – Spirit that misleads people"),
            Legendary::Vahana =>  v = String::from("Vahana – Divine mounts"),
            Legendary::Vaibhavi =>  v = String::from("Vaibhavi – Deadly snake"),
            Legendary::Valkyrie =>  v = String::from("Valkyrie – Female spirit that leads souls of dead warriors to Valhalla"),
            Legendary::Valva =>  v = String::from("Valva – Female nature spirit"),
            Legendary::Valravn =>  v = String::from("Valravn – Supernatural raven"),
            Legendary::Vampire =>  v = String::from("Vampire – Reanimated corpse that feeds on blood"),
            Legendary::Vanara =>  v = String::from("Vanara – Human-ape hybrid"),
            Legendary::Vantoase =>  v = String::from("Vantoase – Female weather spirit"),
            Legendary::Varaha =>  v = String::from("Varaha – Third Avatar of Vishnu in the form of a boar"),
            Legendary::Varcolac =>  v = String::from("Varcolac – Vampire or werewolf"),
            Legendary::Vardoger =>  v = String::from("Vardoger – Ghostly double"),
            Legendary::Vedrfolnir =>  v = String::from("Vedrfolnir – Hawk sitting between the eyes of an eagle in the crown of the World Tree Yggdrasil"),
            Legendary::Veli =>  v = String::from("Veli – Ghost, shade, formed after a death of a human"),
            Legendary::VeriSelen =>  v = String::from("Chuvash dragonChuvash dragon"),
            Legendary::Vetala =>  v = String::from("Vetala – Corpses possessed by vampiric spirits"),
            Legendary::Víbria =>  v = String::from("Víbria – Dragon with breasts and an eagle's beak"),
            Legendary::Vielfras =>  v = String::from("Vielfras – Gluttonous dog-cat-fox hybrid"),
            Legendary::Vila =>  v = String::from("Vila – Weather spirit"),
            Legendary::Vilkacis =>  v = String::from("Vilkacis – Animalistic, werewolf-like monster"),
            Legendary::Virunas =>  v = String::from("Virunas – Handsome demon"),
            Legendary::VisionSerpent =>  v = String::from("VisionSerpent – Mystical dragon"),
            Legendary::Vídopnir =>  v = String::from("Vídopnir – Rooster that sits atop the tree"),
            Legendary::Vodyanoy =>  v = String::from("Vodyanoy – Male water spirit"),
            Legendary::Vrykolakas =>  v = String::from("Vrykolakas – Undead wolf-human hybrid"),
            Legendary::Vaettir =>  v = String::from("Vaettir – Nature spirit"),
            Legendary::Waldgeist =>  v = String::from("Waldgeist – Forest spirit"),
            Legendary::WanaGamesAk =>  v = String::from("WanaGamesAk – Water spirits"),
            Legendary::Wani =>  v = String::from("Wani – Crocodilian water monster"),
            Legendary::Wanyudo =>  v = String::from("Wanyudo – Demon in the form of a burning human-headed ox cart"),
            Legendary::WarakNgendog =>  v = String::from("WarakNgendog – Egg-laying bird"),
            Legendary::Warg =>  v = String::from("Warg – Giant, demonic wolf"),
            Legendary::Warlock =>  v = String::from("Warlock – Male witch"),
            Legendary::WassanMonGaneehlaAk =>  v = String::from("WassanMonGaneehlaAk – Aurora spirits"),
            Legendary::WaterMonkey =>  v = String::from("WaterMonkey – Water spirit"),
            Legendary::WaterSprite =>  v = String::from("WaterSprite – Water elemental"),
            Legendary::WatiKutjara =>  v = String::from("WatiKutjara – Goanna spirits"),
            Legendary::WaWonDeeAMegw =>  v = String::from("WaWonDeeAMegw – Shapeshifting snail spirit"),
            Legendary::WeisseFrauen =>  v = String::from("WeisseFrauen – Female spirit"),
            Legendary::Wekufe =>  v = String::from("Wekufe – Demon"),
            Legendary::Wendigo =>  v = String::from("Wendigo – Anthropophagous spirit"),
            Legendary::Wentshukumishiteu =>  v = String::from("Wentshukumishiteu – Water spirit"),
            Legendary::Werecat =>  v = String::from("A werecat (also written in a hyphenated form as were-cat) is an analogy to 'werewolf' for a feline therianthropic creature. "),
            Legendary::Werehyena =>  v = String::from("Were-hyena is a neologism coined in analogy to werewolf for therianthropy involving hyenas. It is common in the folklore of the Arabian Peninsula, the Levant, North Africa, the Horn of Africa, and the Near East as well as some adjacent territories. Unlike werewolves and other therianthropes, which are usually portrayed as being originally human, some werehyena lore tells of how they can also be hyenas disguised as humans."),
            Legendary::Werewolf =>  v = String::from("In folklore, a werewolf (Old English: werwulf, 'man-wolf'), or occasionally lycanthrope /ˈlaɪkənˌθroʊp/ (Greek: λυκάνθρωπος lukánthrōpos, 'wolf-human'), is a human with the ability to shapeshift into a wolf (or, especially in modern film, a therianthropic hybrid wolflike creature), either purposely or after being placed under a curse or affliction (often a bite or scratch from another werewolf) with the transformations occurring on the night of a full moon. Early sources for belief in this ability or affliction, called lycanthropy /laɪˈkænθrəpi/, are Petronius (27–66) and Gervase of Tilbury (1150–1228).  The werewolf is a widespread concept in European folklore, existing in many variants, which are related by a common development of a Christian interpretation of underlying European folklore developed during the medieval period. From the early modern period, werewolf beliefs also spread to the New World with colonialism. Belief in werewolves developed in parallel to the belief in witches, in the course of the Late Middle Ages and the Early Modern period. Like the witchcraft trials as a whole, the trial of supposed werewolves emerged in what is now Switzerland (especially the Valais and Vaud) in the early 15th century and spread throughout Europe in the 16th, peaking in the 17th and subsiding by the 18th century.  The persecution of werewolves and the associated folklore is an integral part of the 'witch-hunt' phenomenon, albeit a marginal one, accusations of lycanthropy being involved in only a small fraction of witchcraft trials.  During the early period, accusations of lycanthropy (transformation into a wolf) were mixed with accusations of wolf-riding or wolf-charming. The case of Peter Stumpp (1589) led to a significant peak in both interest in and persecution of supposed werewolves, primarily in French-speaking and German-speaking Europe. The phenomenon persisted longest in Bavaria and Austria, with persecution of wolf-charmers recorded until well after 1650, the final cases taking place in the early 18th century in Carinthia and Styria.  After the end of the witch-trials, the werewolf became of interest in folklore studies and in the emerging Gothic horror genre; werewolf fiction as a genre has pre-modern precedents in medieval romances (e.g. Bisclavret and Guillaume de Palerme) and developed in the 18th century out of the 'semi-fictional' chap book tradition. The trappings of horror literature in the 20th century became part of the horror and fantasy genre of modern popular culture."),
            Legendary::WhiteLady =>  v = String::from("WhiteLady – Ghost of a murdered or mistreated woman"),
            Legendary::Whowie =>  v = String::from("Whowie – Giant frog-headed goanna with six legs"),
            Legendary::WildMan =>  v = String::from("WildMan – Hairy, bipedal, man-like creature"),
            Legendary::WillOTheWisp =>  v = String::from("WillOTheWisp – Spectral fire"),
            Legendary::WirryCow =>  v = String::from("WirryCow – Malevolent spirit"),
            Legendary::Witch =>  v = String::from("Witch – Person who practices magic"),
            Legendary::WitteWieven =>  v = String::from("WitteWieven – Female, ancestral spirit"),
            Legendary::Wolpertinger =>  v = String::from("Wolpertinger"),
            Legendary::Wondjina =>  v = String::from("Wondjina – Weather spirit"),
            Legendary::Wraith =>  v = String::from("Wraith – Water spirit or ghostly apparition"),
            Legendary::Wulver =>  v = String::from("Wulver – Wolf-headed humanoid spirit"),
            Legendary::WuTouGui =>  v = String::from("WuTouGui – Beheaded ghost"),
            Legendary::Wyrm =>  v = String::from("English dragonEnglish dragon"),
            Legendary::Wyvern =>  v = String::from("Wyvern – Flying reptile, usually with two legs and two wings"),
            Legendary::Xana =>  v = String::from("Xana – Female water spirit"),
            Legendary::Xanthus =>  v = String::from("Xanthus"),
            Legendary::Xecotcovach =>  v = String::from("Xecotcovach – Bird"),
            Legendary::Xelhua =>  v = String::from("Xelhua – Giant"),
            Legendary::Xiao =>  v = String::from("Xiao – Ape or four-winged bird"),
            Legendary::XingTian =>  v = String::from("XingTian – Headless giant"),
            Legendary::Xiuhcoatl =>  v = String::from("Xiuhcoatl – Drought spirit"),
            Legendary::Xhindi =>  v = String::from("Xhindi – Elves"),
            Legendary::Yacumama =>  v = String::from("Yacumama – Sea monster"),
            Legendary::Yacuruna =>  v = String::from("Yacuruna – Mythical water people, with backwards heads and feet"),
            Legendary::Yadokai =>  v = String::from("Yadokai – Malevolent, nocturnal spirit"),
            Legendary::YagyoSan =>  v = String::from("YagyoSan – Demon who rides through the night on a headless horse"),
            Legendary::Yaksha =>  v = String::from("Yaksha – Male nature spirit"),
            Legendary::Yakshi =>  v = String::from("Yakshi – Vampire"),
            Legendary::Yakshini =>  v = String::from("Yakshini – Female nature spirit"),
            Legendary::YakubyoGami =>  v = String::from("YakubyoGami – Disease and misfortune spirit"),
            Legendary::Yale =>  v = String::from("Yale – Antelope- or goat-like animal with swiveling horns"),
            Legendary::Yazhi =>  v = String::from("Yazhi – Lion-like beast"),
            Legendary::YalleryBrown =>  v = String::from("YalleryBrown – Nature spirit"),
            Legendary::Yama =>  v = String::from("Yama – Wrathful god"),
            Legendary::YamaBiko =>  v = String::from("YamaBiko – Echo spirit"),
            Legendary::YamaBito =>  v = String::from("YamaBito – Savage, mountain-dwelling humanoid"),
            Legendary::YamaChichi =>  v = String::from("YamaChichi – Monkey-like mountain spirit"),
            Legendary::YamaInu =>  v = String::from("YamaInu – Dog-like mountain spirit"),
            Legendary::YamaOtoko =>  v = String::from("YamaOtoko – Mountain giant"),
            Legendary::YamataNoOrochi =>  v = String::from("YamataNoOrochi – Gigantic, eight-headed serpent"),
            Legendary::YamaUba =>  v = String::from("YamaUba – Malevolent, mountain-dwelling hag"),
            Legendary::YamaWaro =>  v = String::from("YamaWaro – Hairy, one-eyed spirit"),
            Legendary::Yanari =>  v = String::from("Yanari – Spirit which causes strange noises"),
            Legendary::Yaoguai =>  v = String::from("Yaoguai – Animalistic demon or fallen gods"),
            Legendary::YaraMaYhaWho =>  v = String::from("YaraMaYhaWho – Diminutive, sucker-fingered vampire"),
            Legendary::Yatagarasu =>  v = String::from("Yatagarasu – Three-legged crow of Amaterasu"),
            Legendary::YatoNoKami =>  v = String::from("YatoNoKami – Serpent spirits"),
            Legendary::YethHound =>  v = String::from("YethHound – Headless dog"),
            Legendary::Yeti =>  v = String::from("Yeti – Mountain bigfoot"),
            Legendary::Yilbegan =>  v = String::from("Yilbegan – Either a dragon or a giant"),
            Legendary::Yobuko =>  v = String::from("Yobuko – Mountain dwelling spirit"),
            Legendary::Yokai =>  v = String::from("Yokai – Supernatural monster"),
            Legendary::YomotsuShikome =>  v = String::from("YomotsuShikome – Underworld hag"),
            Legendary::Yong =>  v = String::from("Korean dragonKorean dragon"),
            Legendary::Yosei =>  v = String::from("Yosei – Fairy"),
            Legendary::Yosuzume =>  v = String::from("Yosuzume – Mysterious bird that sings at night, sometimes indicating that the okuri-inu is near"),
            Legendary::YouHunYeGui =>  v = String::from("YouHunYeGui – Wandering ghost"),
            Legendary::Yowie =>  v = String::from("Yowie – Nocturnal human-ape hybrid, also Yahoo"),
            Legendary::Ypotryll =>  v = String::from("Ypotryll – Boar-camel-ox-serpent hybrid"),
            Legendary::YuanGui =>  v = String::from("YuanGui – Distressed ghost"),
            Legendary::Yukinko =>  v = String::from("Yukinko – Childlike snow spirit"),
            Legendary::YukiOnna =>  v = String::from("YukiOnna – Female snow spirit"),
            Legendary::Yurei =>  v = String::from("Yurei – Ghost"),
            Legendary::Yuxa =>  v = String::from("Yuxa – 100-year-old snake that transforms into a beautiful human"),
            Legendary::Zahhak =>  v = String::from("Zahhak – Dragon"),
            Legendary::Zaltys =>  v = String::from("Zaltys – Serpentine fertility spirit"),
            Legendary::Zamzummim =>  v = String::from("Zamzummim – Giant"),
            Legendary::ZanaEMalit =>  v = String::from("ZanaEMalit – Mountain fairy who bless warriors"),
            Legendary::Zână =>  v = String::from("Zână – Nature spirit"),
            Legendary::ZashikiWarashi =>  v = String::from("ZashikiWarashi – House spirit"),
            Legendary::Zburator =>  v = String::from("Zburator – Wolf-headed dragon"),
            Legendary::Zduhac =>  v = String::from("Zduhac – Disembodied, heroic spirit"),
            Legendary::Zeus =>  v = String::from("Zeus – God of lightning and storms"),
            Legendary::ZennyoRyuo =>  v = String::from("ZennyoRyuo – Rain-making dragon"),
            Legendary::ZharPtitsa =>  v = String::from("ZharPtitsa – Glowing bird"),
            Legendary::Zhulong =>  v = String::from("Zhulong – Pig-headed dragon"),
            Legendary::ZhuQue =>  v = String::from("ZhuQue – Fire elemental bird"),
            Legendary::Ziburinis =>  v = String::from("Ziburinis – Forest spirit in the form of a glowing skeleton"),
            Legendary::Zilant =>  v = String::from("Zilant – Flying chicken-legged reptile"),
            Legendary::Zin =>  v = String::from("Zin – Water spirits"),
            Legendary::Ziz =>  v = String::from("Ziz – Giant bird"),
            Legendary::Zlatorog =>  v = String::from("Zlatorog – White golden-horned deer"),
            Legendary::Zmeu =>  v = String::from("Zmeu – Giant with a habit of kidnapping young girls"),
            Legendary::Zmiy =>  v = String::from("Slavic dragonSlavic dragon"),
            Legendary::Zombie =>  v = String::from("Zombie – Re-animated corpse"),
            Legendary::Zorigami =>  v = String::from("Zorigami – Animated clock"),
            Legendary::Zuijin =>  v = String::from("Zuijin – Tutelary spirit"),
            Legendary::ZunberaBo =>  v = String::from("ZunberaBo"),
        }
        // We **finally** return the *looooooong* string
        v
    }
}
impl<T:Copy 
    + Default
    + AddAssign
    + Add<Output = T>
    + Div<Output = T>
    + DivAssign
    + Mul<Output = T>
    + MulAssign
    + Neg<Output = T>
    + Rem<Output = T>
    + RemAssign
    + Sub<Output = T>
    + SubAssign
    + std::cmp::PartialOrd
    + num::NumCast> Builder<T> for Legendary {
    /// Build a `Basic` stat
    fn build_basic(&self, id:T, level:T) -> Basic<T>{
        let mut hp:T = num::cast(10).unwrap();
        let mut mp:T = num::cast(5).unwrap();
        let mut xp:T = num::cast(1).unwrap();
        let mut xp_next:T = num::cast(10).unwrap();
        let mut gp:T = num::cast(5).unwrap();
        let mut speed:T = num::cast(5).unwrap();
        //TODO OR ue legendary.ini + serde
        match *self {
            _=> {},
        }
        hp *= level;
        mp *= level;
        // TODO fixme:
        xp *= level;
        // TODO fixme:
        xp_next *= level;
        gp *= level;
        speed += level;
        Basic {
            id:id,
            xp:xp,
            xp_next:xp_next,
            level:level,
            gp:gp,
            hp: hp,
            mp: mp,
            hp_max: hp,
            mp_max: mp,
            speed: speed,
        }
        
    }
    // Build a `Normal` stat
    fn build_normal(&self, id:T, level:T) -> Normal<T>{
        let mut hp:T = num::cast(10).unwrap();
        let mut mp:T = num::cast(5).unwrap();
        let mut xp:T = num::cast(1).unwrap();
        let mut xp_next:T = num::cast(10).unwrap();
        let mut gp:T = num::cast(5).unwrap();
        let mut speed:T = num::cast(5).unwrap();
        let mut atk:T = num::cast(10).unwrap();
        let mut def:T = num::cast(10).unwrap();
        let mut m_atk:T = num::cast(10).unwrap();
        let mut m_def:T = num::cast(10).unwrap();
        //TODO OR use legendary.ini + serde
        match *self {
            _=> {},
        }
        hp *= level;
        mp *= level;
        // TODO fixme:
        xp *= level;
        // TODO fixme:
        xp_next *= level;
        gp *= level;
        speed += level;
        Normal {
            id:id,
            xp:xp,
            xp_next:xp_next,
            level:level,
            gp:gp,
            hp: hp,
            mp: mp,
            hp_max: hp,
            mp_max: mp,
            speed: speed,
            atk:atk,
            def:def,
            m_atk:m_atk,
            m_def:m_def,
        }
    }

    // Build an `Advanced` stat
    fn build_advanced(&self, id:T, level:T) -> Advanced<T>{
        let mut hp:T = num::cast(10).unwrap();
        let mut mp:T = num::cast(5).unwrap();
        let mut xp:T = num::cast(1).unwrap();
        let mut xp_next:T = num::cast(10).unwrap();
        let mut gp:T = num::cast(5).unwrap();
        let mut speed:T = num::cast(5).unwrap();
        let mut atk:T = num::cast(10).unwrap();
        let mut def:T = num::cast(10).unwrap();
        let mut m_atk:T = num::cast(10).unwrap();
        let mut m_def:T = num::cast(10).unwrap();
        let mut agility:T = num::cast(10).unwrap();
        let mut strength:T = num::cast(10).unwrap();
        let mut dexterity:T = num::cast(10).unwrap();
        let mut constitution:T = num::cast(10).unwrap();
        let mut intelligence:T = num::cast(10).unwrap();
        let mut charisma:T = num::cast(10).unwrap();
        let mut wisdom:T = num::cast(10).unwrap();
        let mut age:T = num::cast(10).unwrap();
        //TODO OR use legendary.ini + serde
        match *self {
            _=> {},
        }
        hp *= level;
        mp *= level;
        // TODO fixme:
        xp *= level;
        // TODO fixme:
        xp_next *= level;
        gp *= level;
        speed += level;
        Advanced {
            id:id,
            xp:xp,
            xp_next:xp_next,
            level:level,
            gp:gp,
            hp: hp,
            mp: mp,
            hp_max: hp,
            mp_max: mp,
            speed: speed,
            atk:atk,
            def:def,
            m_atk:m_atk,
            m_def:m_def,
            agility:agility,
            strength:strength,
            dexterity:dexterity,
            constitution:constitution,
            intelligence:intelligence,
            charisma:charisma,
            wisdom:wisdom,
            age:age,
        }
    }
}
