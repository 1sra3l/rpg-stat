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
    /// (Malay) – Entity that lives in the Tower of Victory in Chitor.
    ABaoAQu,
    /// (Basque) – Bull spirit.
    Aatxe,
    /// (Yakuts) – Iron-toothed demons.
    Abaasy,
    /// (African) – Unicorn that inhabits the African Congo.
    Abada,
    /// (Tatar) – Forest spirit. (Melanesia) – Huge magical eel.
    Abaia,
    /// (Medieval Bestiaries) – Savage humanoid with backward feet.
    Abarimon,
    /// (Malay) – One-horned animal.
    Abath,
    /// (Japanese) – Creature from a mountain pass in Kumamoto Prefecture.
    AburaSumashi,
    /// (Greek) – Headless humanoids.
    Acephali,
    /// (Mitologia Hindu) – Disease-bringing ghost.
    Acheri,
    /// (Roman) – Curious elk.
    Achlis,
    /// (Welsh) – Giant birds that understand human languages.
    AdarLlwchGwin,
    /// (Solomon Islands) – Malevolent merfolk.
    Adaro,
    /// (Manx) – Nature spirit.
    Adhene,
    /// (Inuit) – Vampiric dog-human hybrid
    Adlet,
    /// (Lugbara) – Nature spirit.
    Adroanzi,
    /// (Ewe people) – African vampiric-forest being.
    Adze,
    /// (Greek) – Disease demon.
    Aerico,
    /// (Norse) – Norse deities.
    AEsir,
    /// (Welsh) – Lake monster (exact lake varies by story).
    Afanc,
    /// (Hindu) – God of fire and sacrifices.
    Agni,
    /// (Greek) – Spirit of vinefields and grainfields.
    Agathodaemon,
    /// (Inuit) – Ice spirit that aids hunters and fishermen.
    Agloolik,
    /// (East Africa) – Small, ape-like humanoid.
    Agogwe,
    /// (Inuit) – Animated skeleton that causes shipwrecks.
    Ahkiyyini,
    /// (Aztec) – Anthropophagous dog-monkey hybrid.
    Ahuizotl,
    /// (Zoroastrianism) – Zoroastrian spirits.
    Ahura,
    /// (Khoikhoi) – Anthropophagous humanoid with eyes in its instep.
    Aigamuxa,
    /// (Etruscan) – Fish-tailed goat.
    Aigikampoi,
    /// (Hindu) – Divine elephant.
    Airavata,
    /// (Polynesian) – Malevolent spirits or demons.
    Aitu,
    /// (Lithuanian) – Household spirit.
    Aitvaras,
    /// (Finnish) – Dragon/snake female spirit, is said to spread diseases
    Ajatar,
    /// (Japanese) – Tree-dwelling monster.
    Akateko,
    /// (Inuit) – Orca-wolf shapeshifter.
    Akhlut,
    /// (Finnish) – Female spirits or minor goddesses.
    Akka,
    /// (Japanese) – Large, grotesque humanoid.
    Akki,
    /// (Ainu) – Sea monster.
    Akkorokamui,
    /// (Japanese) – Evil spirit or devil
    Akuma,
    /// (Hindu) – Giant turtle that supports the world.
    Akupara,
    /// (Japanese) – Ghostly flame which causes disease.
    AkurojinNoHi,
    /// (Armenian and Persian) – Spirit that steals unborn babies and livers from pregnant women.
    Al,
    /// (Slavic) – Bad weather demon.
    Ala,
    /// (Chaldean) – Queen of the full moon.
    Alal,
    /// (Philippine) – Winged humanoid that steals reproductive waste to make children.
    Alan,
    /// (Heraldic) – Wingless griffin.
    Alce,
    /// (Bengali) – Spirit of a dead fisherman.
    Aleya,
    /// (Chilean) – Bird that eats gold and silver.
    Alicanto,
    /// (Bestiario medieval) – Winged unicorn.
    Alicorn,
    /// (Slavic) – Angelic bird with human head and breasts.
    Alkonost,
    /// (Heraldic) – Ass-camel hybrid.
    Allocamelus,
    /// (Mongolian) – Savage humanoid.
    Almas,
    /// (Islamic) – One-horned rabbit.
    AlMiRaj,
    /// (Catalan) – Female water spirit.
    Aloja,
    /// (Abenaki) – Little people and tricksters.
    AlomBagWinnosis,
    /// (German) – Male night-demon.
    Alp,
    /// (Heraldic) – Lion-like creature, sometimes with dragon or goat forelegs.
    Alphyn,
    /// (Irish) – Parasitic fairy.
    AlpLuachra,
    /// (Islamic) – Guard dog of the Seven Sleepers.
    AlRakim,
    /// (Greek) – Grove nymph.
    Alseid,
    /// (Assyrian) – Leprous demon.
    Alu,
    /// (Mayan) – Little people.
    Alux,
    /// (Japanese) – Ritual disciplinary demon from Shikoku.
    Amaburakosagi,
    /// (Tsimshian) – Giant who holds up the world.
    Amala,
    /// (Japanese) – Ritual disciplinary demon from Hokuriku.
    Amamehagi,
    /// (Japanese) – Small demon.
    Amanojaku,
    /// (Inuit) – Giant wolf.
    Amarok,
    /// (Quechua) – Water boa spirit.
    Amarum,
    /// (Japanese) – Disease-causing hag.
    AmazakeBabaa,
    /// (Ainu) – Lake monster.
    Amemasu,
    /// (Ancient Egyptian) – Female demon who was part lion, hippopotamus and crocodile and devoured the souls of the wicked.
    Ammit,
    /// (Japanese) – Tennyo from the island of Amami Ōshima.
    Amoronagu,
    /// (Heraldic) – Winged serpent.
    Amphiptere,
    /// (Greek) – Serpent with a head at each end.
    Amphisbaena,
    /// (Jewish) – Giant.
    Anak,
    /// (Ancient Egyptian) – Human-headed sphinx.
    Androsphinx,
    /// (mainly Christian, Jewish, Islamic traditions) – Divine beings of Heaven who act as mediators between God and humans; the counterparts of Demons.
    Angel,
    /// (Arabian) – Legendary Huge Satanic Eagle with Human Face. sometimes can resurrect herself like phoenix did.
    Anqa,
    /// (Cherokee) – Lightning spirit.
    AniHyuntikwalaski,
    /// (French) – Skeletal grave watcher with a lantern and scythe.
    Ankou,
    /// (Japanese) – Ritual disciplinary demon from Iwate Prefecture.
    Anmo,
    /// (Greek) – Giant who was extremely strong as long as he remained in contact with the ground.
    Antaeus,
    /// (Ancient Egyptian) – God of the Underworld
    Anubis,
    /// (Finnish) – Subterranean giant.
    AnteroVipunen,
    /// (Sumerian) – Divine storm bird
    Anzu,
    /// (Guaraní) – Anthropophagous peccary or sheep.
    AoAo,
    /// (Japanese) – Blue monk who kidnaps children.
    Aobozu,
    /// (Sumerian) – Fish-human hybrid that attends the god Enki.
    Apkallu,
    /// (Buddhist and Hindu) – Female cloud spirit.
    Apsaras,
    /// (Akkadian) – Human-scorpion hybrid.
    Aqrabuamelu,
    /// (Akkadian) – Disease demon.
    ArdatLili,
    /// (Greek) – Hundred-eyed giant.
    ArgusPanoptes,
    /// (Japanese) – Old woman with magical powers.
    ArikuraNoBaba,
    /// (Greek) – One-eyed humanoid.
    Arimaspi,
    /// (Greek) – Swift green-maned talking horse.
    Arion,
    /// (Manx) – Fairy hedgehog.
    ArkanSonney,
    /// (Sumerian) – Hideous rock demon.
    Asag,
    /// (Sumerian) – Demon.
    Asakku,
    /// (West Africa) – Iron-toothed vampire.
    Asanbosam,
    /// (Turkic) – Blue-maned wolf.
    Asena,
    /// (Abenaki) – Stone giant.
    ASeneeKiWakw,
    /// (Japanese) – Invisible tendril that impedes movement.
    AshiMagari,
    /// (Dahomey) – Vampiric possession spirit.
    Asiman,
    /// (Germanic) – Female tree spirit.
    Askefrue,
    /// (Abenaki) – Fire elemental and spectral fire.
    AskWeeDaEed,
    /// (Japanese) – Spectral fire from Kōchi Prefecture.
    Asobibi,
    /// (Medieval Bestiaries) – Island-sized whale or sea turtle.
    Aspidochelone,
    /// (English) – Water spirit.
    Asrai,
    /// (Greek) – Humanoid sustained by pleasant smells instead of food.
    Astomi,
    /// (Hindu) – Hindu malevolent divinities.
    Asura,
    /// (Philippine) – Carrion-eating humanoid.
    Aswang,
    /// (English) – Surprisingly small creature.
    Atomy,
    /// (Japanese) – Invisible spirit that follows people.
    AtoOiKozo,
    /// (Inuit) – Anthropophagous spirit.
    Atshen,
    /// (Greek) – Pasture nymph.
    Auloniad,
    /// (Medieval Bestiary) – King of the birds.
    Avalerion,
    /// (Abenaki) – Insect spirit.
    AwaHonDo,
    /// (Ancient Egyptian) – Falcon-lion hybrid.
    Axex,
    /// (Japanese) – Sea serpent that travels over boats in an arc while dripping oil.
    Ayakashi,
    /// (Japanese) – Spectral fire from Ishikawa Prefecture.
    AyakashiNoAyashibi,
    /// (Dahomey) – Little people that help hunters.
    Aziza,
    /// (Japanese) – Spirit that washes azuki beans along riversides.
    Azukiarai,
    /// (Japanese) – Spirit that washes azuki beans along riversides.
    Azukitogi,
    /// (Japanese) – Bean-grinding hag who devours people.
    Azukibabaa,
    /// (Egyptian) – Soul of the deceased, depicted as a bird or a human-headed bird
    Ba,
    /// (Slavic) – Forest spirit and hag
    BabaYaga,
    /// (Guyanese/Surinamese) – Malevolent little people
    Baccoo,
    /// (Italian) – Goat-like creature from the southern central Alps
    Badalisc,
    /// (Slavic) – Malevolent water spirit
    Bagiennik,
    /// (Arabian) – Giant fish
    Bahamut,
    /// (Chinese) – Talking beast which handed down knowledge on harmful spirits
    BaiZe,
    /// (Chinese) – Banana tree spirit
    BaJiaoGui,
    /// (Indian) - Assamese shape-shifting aqueous creature
    Bak,
    /// (Japanese) – Ghostly whale skeleton that drifts along the coastline of Shimane Prefecture
    BakeKujira,
    /// (Japanese) – Magical cat
    Bakeneko,
    /// (Japanese) – Animated straw sandal
    Bakezori,
    /// (Iranian) – Night demon
    Bakhtak,
    /// (Japanese) – Dream-devouring, tapir-like creature
    Baku,
    /// (Philippine) – Sea serpent that causes eclipses
    Bakunawa,
    /// (Romanian) – Multi-headed dragon
    Balaur,
    /// (Albanian) – Sea monster
    Baloz,
    /// (Slavic) – Bathhouse spirit
    Bannik,
    /// (Irish) – Screaming death spirit
    Banshee,
    /// (Celtic Mythology) – Beautiful vampiric seductresses who prey on young travelers
    BaobhanSith,
    /// (Swiss) – Dwarf with giant, snowshoe-like feet
    Barbegazi,
    /// (Albanian) – Mountain spirit
    Bardha,
    /// (Trabzon) – Shapechanging death spirit
    Bardi,
    /// Yorkshire black dog
    Barghest,
    /// (Jewish) – Gigantic bird
    BarJuchne,
    /// (Medieval folklore) – Geese which hatch from barnacles
    BarnacleGeese,
    /// (Balinese) – Tutelary spirit
    Barong,
    /// (Basque) – Ancestral, megalith-building race
    Basajaun,
    /// (Serbian) – Powerful, evil winged man whose soul is not held by his body and can be subdued only by causing him to suffer dehydration
    BasCelik,
    /// (Chinese) – Elephant-swallowing serpent
    Bashe,
    /// (Chilota) – Chicken-serpent hybrid
    BasiliscoChilote,
    /// (Italian) – Multi-limbed, venomous lizard
    Basilisk,
    /// (Philippine) – Primordial god of creation
    Bathala,
    /// (Philippine) – Female night-demon
    Batibat,
    /// (Chinese) – Drought spirit
    Batsu,
    /// (Lithuanian) – Malevolent spirit
    Baubas,
    /// (Ojibwa) – Flying skeleton
    Baykok,
    /// (American Folklore) – Werewolf
    BeastOfBrayRoad,
    /// (Irish) – Death spirit; a type of Banshee/Bean Sídhe)
    BeanNighe,
    /// (Jewish) – Massive beast, possibly like a dinosaur
    Behemoth,
    /// (Welsh) – Giant king
    Bendigeidfran,
    /// (Egyptian) – Heron-like, regenerative bird, equivalent to (or inspiration for) the Phoenix
    Bennu,
    /// (Slavic) – Water spirit
    Berehynia,
    /// (Norse) – Mountain giants who live alongside the Hrimthursar (lit. "Rime-Giants") in Jotunheim
    Bergrisar,
    /// (Norse) – Mountain spirit
    Bergsra,
    /// (Brazilian) – Centauroid specter
    BestialBeast,
    /// (Japanese) – Invisible spirit which follows people at night, making the sound of footsteps
    BetobetoSan,
    /// (Buddhist and Hindu) – Ghost of someone killed by execution or suicide
    Bhuta,
    /// (Khoikhoi) – Female, cannibalistic, partially invisible monster
    BiBlouk,
    /// (Slavic) – Demon
    Bies,
    /// (American Folklore) – Forest-dwelling hominid cryptid.
    Bigfoot,
    /// (Japanese) – Spirit of poverty
    Binbogami,
    /// (Medieval Bestiaries) – Fish-like humanoid
    BishopFish,
    /// (Japanese) – Animated biwa
    BiwaBokuboku,
    /// (English) – Blue-faced hag
    BlackAnnis,
    /// (British) – Canine death spirit
    BlackDog,
    /// Norfolk, Essex, and Suffolk black dog
    BlackShuck,
    /// Imaginary creature from the early United States of America
    Blafard,
    /// (Medieval Bestiary) – Headless humanoid with face in torso
    Blemmyae,
    /// (Irish) – Water bogeyman
    BloodyBones,
    /// (Slavic) – Mischievous gnome
    Bludnik,
    /// (Brazilian) – Giant amazonian bird
    BlueCrow,
    /// (English) – Mine-dwelling fairy
    Bluecap,
    /// (Scottish) – Malevolent spirit
    Bodach,
    /// (English) – Malevolent spirit
    Bogeyman,
    /// (English) – Malevolent household spirit
    Boggart,
    /// (Slavic) – Nature spirit
    Boginki,
    /// (Scottish) – Malevolent spirit
    Bogle,
    /// (Brazilian) – Giant snake
    BoiTata,
    /// (Albanian) – Dragon
    Bolla,
    /// (Medieval Bestiaries) – Bull-horse hybrid with flaming dung
    Bonnacon,
    /// (American Folklore) – Vampire-like creature that steals energy from sleeping victims
    BooHag,
    /// (Scottish) – Roaring water bird
    Boobrie,
    /// (Slavic) – Death spirit
    Bozaloshtsh,
    /// (English) – Malevolent water horse
    Brag,
    /// (English and Scottish) – Benevolent household spirit
    Brownie,
    /// (Jewish) – Nocturnal bird that drains goats of their milk
    Broxa,
    /// (Cornish) – Male sea-spirit, a merman, that inhabited mines and coastal communities as a hobgoblin during storms
    Bucca,
    /// (Dutch) – Ghosts/devils riding flying goats; co-opted by bandits to instil fear during raids
    Bokkenrijders,
    /// (English) – Bearlike goblin
    Bugbear,
    /// (Manx) – Ogre-like humanoid
    Buggane,
    /// (Celtic) – Extremely ugly, but kind, forest spirit
    BugulNoz,
    /// (Serbia) – Six-legged lake monster
    Bukavac,
    /// (Australian Aboriginal) – Horse-walrus hybrid lake monster
    Bunyip,
    /// (American Folklore) West Virginia Urban Legend – Spirit/Maniac that wears a bunny costume and wields an axe
    BunnyMan,
    /// (Guyanese) – Spirit that seduces and kills men
    BushDaiDai,
    /// (Bengali) – Fortune-telling birds
    Byangoma,
    /// (Scandinavian) – Diminutive forest spirit
    Bysen,
    /// (Greek) – Smith and wine spirit
    Cabeiri,
    /// (Roman) – Fire-breathing giant
    Cacus,
    /// (Central America) – Cow-sized dog-goat hybrid
    Cadejo,
    /// (Scottish) – Divine creator and weather deity hag
    Cailleach,
    /// (Tupi) – Fox-human hybrid and nature spirit
    Caipora,
    /// (Medieval Bestiary) – White bird that can foretell if a sick person will recover or die
    Caladrius,
    /// (Medieval Bestiary) – Humanoid with an eight-year lifespan
    Calingi,
    /// (Medieval Bestiary) – Apes who always bear twins, one the mother loves, the other it hates
    Callitrix,
    /// (Greek) – Giant, chthonic boar
    CalydonianBoar,
    /// (Heraldic) – Wildcat-deer/antelope-eagle-ox-lion hybrid :>
    Calygreyhound,
    /// (Chilota) – One-horned calf
    Camahueto,
    /// (Medieval folklore) – Offspring of a human and an incubus or succubus
    Cambion,
    /// (Greek) – Dragon-human-scorpion hybrid
    Campe,
    /// (Mayan) – Bird that ate the heads of the first men
    Camulatz,
    /// (Colombian) – Spectral, fiery hag
    Candileja,
    /// (Guyanese) – Were-jaguar
    Canaima,
    /// (Lakota) – Little people and tree spirits
    Canotila,
    /// (Scottish) – Death spirit (a particular type of Banshee/Bean Sídhe)
    Caoineag,
    /// (Lakota) – Beaver spirit
    Chapa,
    ///(Manipuri)-Semi-hornbill, semi-human creature
    Chareng,
    /// (Romanian) – Large, monstrous humanoid
    Capcaun,
    /// (Latin America) – Small creature with a jewel on its head
    Carbuncle,
    /// (Medieval Bestiary) – Scaled buffalo-hog hybrid
    Catoblepas,
    /// (Scottish) – Fairy cat
    CatSidhe,
    /// (Scottish) — Benevolent Scottish mermaids
    Ceasg,
    /// (Welsh) – Malevolent water horse
    CeffylDwr,
    /// (Greek) – Human-horse hybrid
    Centaur,
    /// (Indian) – Horse-Antelope-Lion-Bear hybrid
    Centicore,
    /// (Greek) – Extremely flexible, horned snake
    Cerastes,
    /// (Greek) – Three-headed dog that guards the entrance to the underworld
    Cerberus,
    /// (Greek) – Mischievous forest spirit
    Cercopes,
    /// (Medieval Bestiary) – Apes who always bear twins, one the mother loves, the other it hates
    Cericopithicus,
    /// (Greek) – Hind with golden antlers and bronze or brass hooves
    CeryneianHind,
    /// (Lakota) – Hawk spirit
    Cetan,
    /// (Greek) The Cetus was variously described as a sea monster or sea serpent. Other versions describe Cetus as a monster with the head of a boar or a greyhound and the body of a whale or dolphin, and a divided, fan-like tail. Cetus was said to be a colossal beast the size of a ship, its skull alone measuring 40 feet (12.2 meters) in length, its spines being a cubit in thickness, and its skeleton taller at the shoulder than an elephant.
    Cetus,
    /// (Hindu) – Lunar bird
    Chakora,
    /// (Apocryphal writings) – Angelic birds
    Chalkydri,
    /// (Persian) – Dog-bird hybrid
    Chamrosh,
    /// (Aztec) – Little people and nature spirits
    Chaneque,
    /// (European) – Humanoid child (fairy, elf, troll, etc.) substituted for a kidnapped human child
    Changeling,
    /// (Greek) – Sea monster in the form of a giant mouth
    Charybdis,
    /// (Mi'kmaq/Algonquian) – Giant, human-eating ice monsters; former humans who either committed terrible crime(s) or were possessed by evil spirits, turning their hearts to ice
    Chenoo,
    /// (Narragansett) – Ancestral spirit that instructs tribe members
    Chepi,
    /// (Mapuche) – Volcano-dwelling monster
    Cherufe,
    /// (French) – Evil horse who runs away with travelers
    ChevalMallet,
    /// (French) – Evil horse who drowns riders, similar to kelpie
    ChevalGauvin,
    /// (Abenaki) – Ghost of an improperly buried person
    Chibaiskweda,
    /// Human-faced cow that feeds on good women
    Chichevache,
    /// (Bahamian) – Bird-mammal hybrid
    Chickcharney,
    /// (Greek) – Lion-goat-snake hybrid
    Chimaera,
    /// (Navajo) – Vengeful ghost that causes dust devils
    Chindi,
    /// (Burmese) – Temple-guarding feline, similar to Chinese Shi and Japanese Shisa
    Chinthe,
    /// (Zulu) – Human-lizard hybrid
    Chitauli,
    /// (Japanese) – Animated paper lantern
    Chochinobake,
    /// (Biblical mythology) – Regenerative bird
    Chol,
    /// (Korean) – Supernaturally fast horse
    Chollima,
    /// (Mapuche) – Disembodied, flying head
    Chonchon,
    /// (Guyanese) – Ghost of a woman that died in childbirth
    Choorile,
    /// (Medieval Bestiary) – Hairy savage with dog teeth
    Chromandi,
    /// (Greek) – The giant son of the gorgon Medusa.
    Chrysaor,
    /// (Greek mythology) – Golden winged ram
    Chrysomallus,
    /// (Hindu) – Giant turtle that supports the world
    Chukwa,
    /// (Latin America) – Cryptid beast named for its habit of sucking the blood of livestock
    Chupacabra,
    /// (Hindu) – Vampiric, female ghost
    Churel,
    /// (Dominican Republic) – Malevolent seductress
    Ciguapa,
    /// (Aztec) – Ghost of women that died in childbirth
    Cihuateteo,
    /// (Serbian) – Bird that serves its owner
    Cikavac,
    /// (Medieval Bestiaries) – Giant bird that makes its nest out of cinnamon
    CinnamonBird,
    /// (Aztec) – Sea monster, crocodile-fish hybrid
    Cipactli,
    /// (Scottish) – Sea serpent
    CireinCroin,
    /// (Welsh) – Little people and mine spirits
    Coblynau,
    /// (Medieval Bestiaries) – Chicken-lizard hybrid
    Cockatrice,
    /// (English) – Cove god
    Cofgod,
    /// (Greek) – Bronze-hoofed bulls
    ColchisBull,
    /// (Mapuche) – Rat-bird hybrid that can shapeshift into a serpent
    ColoColo,
    /// (Greek) – Nymph of the Corycian Cave
    CorycianNymphs,
    /// (Greek) – Monstrous bull
    CretanBull,
    /// (Greek) – Fountain nymph
    Crinaeae,
    /// (Ancient Egypt) – Ram-headed sphinx
    Criosphinx,
    /// (Medieval Bestiaries) – Monstrous dog-wolf
    Crocotta,
    /// (Mexican) – El Pájaro Cu; a bird.
    TheCuBird,
    /// (Latin America) – Bogeyman
    Cuco,
    /// (Latin America) – Malevolent spirit
    Cucuy,
    /// (Cantabrian) – Monstrous, three-armed humanoid
    Cuegle,
    /// (Asturian and Cantabrian) – Dragon
    Cuelebre,
    /// (Tupi) – Nature spirit
    Curupira,
    /// (Scottish) – Gigantic fairy dog
    CuSith,
    /// (Welsh) – Underworld hunting dog
    CwnAnnwn,
    /// (Greek) – One-eyed giant
    Cyclops,
    /// (Welsh) – Death spirit
    Cyhyraeth,
    /// (Medieval Bestiaries) – Dog-headed humanoid
    Cynocephalus,
    /// (Greek) – Little people and smith and healing spirits
    Dactyl,
    /// (Greek) – Incorporeal spirit
    Daemon,
    /// (France, Switzerland and the north of Italy) – Similar to a deer or ibex; legs on one side of its body are shorter than on the other side
    Dahu,
    /// (Japanese) – Giant responsible for creating many geographical features in Japan
    Daidarabotchi,
    /// (Japanese) – Most powerful class of tengu, each of whom lives on a separate mountain
    Daitengu,
    /// (Hindu) – Giant
    Daitya,
    /// (Hindu) – Water demon
    Danava,
    /// (Greek) – Laurel tree nymph
    Daphnaie,
    /// (Japanese) – Old woman who steals clothes from the souls of the dead
    DatsueBa,
    /// (Islamic) – Human tribe turned into apes for ignoring Moses' message
    DeadSeaApes,
    /// (Russia) – A winter spirit who delivers gifts to children on New Year's Eve
    DedMoroz,
    /// (Native American) – Human-deer hybrid
    DeerWoman,
    /// (Global) – Preternatural or supernatural possibly immortal being
    Deity,
    /// (Global) – Half human, half god
    Demigod,
    /// (Balkans) – Human/vampire hybrid
    Dhampir,
    /// (Chinese) – Hanged ghost
    DiaoSiGui,
    /// (Chinese) – Earth dragon
    Dilong,
    /// (Catalan) – Demonic and vampiric dog
    Dip,
    /// (Roman) – House spirit
    DiPenates,
    /// (Medieval Bestiaries) – Extremely venomous snake
    Dipsa,
    /// (Australian Aboriginal) – Goanna spirit
    Dirawong,
    /// (Gotland) – Little people and nature spirits
    DiSmaUndarJordi,
    /// (Philippine) – Tree spirit
    Diwata,
    /// (Albanian) – Devil
    Djall,
    /// (Irish) – King otter
    DobharChu,
    /// (Abenaki) – Little people
    DoGakwHoWad,
    /// (Korean) – Grotesque, horned humanoids
    Dokkaebi,
    /// (Norse) – Male ancestral spirits; the Dark Elves
    Dokkalfar,
    /// (Slavic) – Tutelary and fate spirit
    Dola,
    /// (Slavic) – House spirit
    Domovoi,
    /// (German) – Ghostly double
    Doppelganger,
    /// (Catalan) – Lion or bull-faced dragon
    /// (French) – Winged sea serpent
    Drac,
    /// (Greek) – Greek dragons
    Drakon,
    /// (Greek) – Dragons depicted with female characteristics
    Drakaina,
    /// (Many cultures worldwide) – Fire-breathing and,/// (normally) winged reptiles
    Dragon,
    /// (Chinese) – Giant turtle with dragon-like head
    DragonTurtle,
    /// (Albanian) – Semi-human winged warriors
    Drangue,
    /// (Norse) – Undead
    Draugr,
    /// (Slavic) – Restless ghost of an unbaptised child
    Drekavac,
    /// (Australian) – Large carnivorous koala that hunts by dropping on its prey from trees
    DropBear,
    /// (Scottish) – Cavern spirit
    Drow,
    /// (German) – Possessing demon
    Drude,
    /// (Bhutanese) – Dragon
    Druk,
    /// (Greek) – Tree nymph
    Dryad,
    /// (Spanish and Portuguese) – Little people and forest spirits
    Duende,
    /// (English) – Malevolent little people
    Duergar,
    /// (Irish) – Headless death spirit
    Dullahan,
    /// (Philippine) – Little people, some are house spirits, others nature spirits
    Duwende,
    /// (Norse) – Subterranean little people smiths
    Dvergr,
    /// (Slavic) – Courtyard spirit
    Dvorovoi,
    /// (Germanic) – Little people nature spirits
    Dwarf,
    /// (Jewish) – Spirit,/// (sometimes the soul of a wicked deceased) that possesses the living
    Dybbuk,
    /// (Abenaki) – Hideous monster
    DzeeDzeeBonDa,
    /// (Kwakwaka'wakw) – Child-eating hag
    Dzunukwa,
    /// (Christianity) – Anthropomorphic lagomorph.
    EasterBunny,
    /// (Australian) – Anthropomorphic bilby.
    EasterBilby,
    /// (Scottish) – Malevolent water horse
    EachUisge,
    /// (Many cultures worldwide) – Leadership or guidance totem
    EagleSpirit,
    /// (Flores) – Diminutive humanoids, possibly inspired by Homo floresiensis
    EbuGogo,
    /// (Greek)
    Echidna,
    /// (Medieval Bestiaries) – Remora, said to attach to ships to slow them down
    Echeneis,
    /// (Sumerian) – Ghosts of those not buried properly
    Edimmu,
    /// (Yoruba) – Humanoid that carries a magical mat
    Egbere,
    /// (Norse)
    Eikthyrnir,
    /// (Norse) – Spirits of brave warriors
    Einherjar,
    /// (Philippine) – Flesh-eating, winged humanoids
    Ekek,
    /// (Ojibwa) – Hags with awls in their elbows
    ElbowWitch,
    /// (Norse) – Fire Giants who reside in Muspelheim, with Surtr as their leader
    Eldjotnar,
    /// (Greek) – Marsh nymph
    Eleionomae,
    /// (Alchemy) – Personification of one of the Classical elements
    Elemental,
    /// (Hawaiian) – Monarch flycatcher spirit that guides canoe-builders to the proper trees
    Elepaio,
    /// (Germanic) – Nature and fertility spirit
    Elf,
    /// (Central Africa) – Little people and malevolent nature spirits
    Eloko,
    /// (Yoruba) – Child that can move back and forth between the material world and the afterlife at will
    Emere,
    /// (Jewish) – Giant
    Emim,
    /// (Greek) – Female demon that waylays travelers and seduces and kills men
    Empusa,
    /// (Brazilian) – Dolphin-human shapeshifter
    Encantado,
    /// (Portuguese) – Enchanted princesses
    EnchantedMoor,
    /// (Heraldic) – Fox-greyhound-lion-wolf-eagle hybrid
    Enfield,
    /// (Philippine) – Neutral nature spirit
    Engkanto,
    /// (Japanese) – Kappa of Shikoku and western Honshū
    Enko,
    /// (worldwide/fantasy) -Living tree that is said to live for years
    Ent,
    /// (Greek) – Apple tree nymph
    Epimeliad,
    /// (Sardinia) – Ox-human, wereox
    Erchitu,
    /// (Chinese) – Hungry ghost
    ErGui,
    /// (Greek) – Winged spirits of vengeance or justice, also known as Furies
    Erinyes,
    /// (German) – Death spirit
    Erlking,
    /// (Greek) – Giant boar
    ErymanthianBoar,
    /// (Medieval Bestiaries) – Horned, winged horse
    EthiopianPegasus,
    /// (Finnish mythology) – Spirit being of a living person
    Etiainen,
    /// (English) – Three-headed giant
    Ettin,
    /// (Greek) – Blue-black, carrion-eater in the underworld
    Eurynomos,
    /// (Cherokee) – Human-cougar hybrid
    Ewah,
    /// (Lithuanian) – Lake spirit
    Eerinis,
    /// (Irish and Scottish) – Monster with half a body
    Fachen,
    /// (Germanic mythology) – Dwarf who was cursed and turned into a dragon. He was later slain by Sigurd in the Saga of Nibelung.
    Fafnir,
    /// (many cultures worldwide, esp. Germanic mythology/folklore) – Nature spirits
    Fairy,
    /// (English) – Animal servant
    Familiar,
    /// (Irish) – Little people that constantly play pranks
    FarDarrig,
    /// (French) – Small,/// (some half-meter tall), wrinkled, and brown-skinned helpful sprites.
    Farfadet,
    /// (Greek) – Three time-controlling sisters
    Fates,
    /// (Roman) – Human-goat hybrid nature spirit
    Faun,
    /// (Irish) – Hunger ghost
    FearGorta,
    /// Mesoamerican dragon
    FeatheredSerpent,
    /// (Chinese) – Chinese wind god
    FeiLian,
    /// (Chinese) – Chinese Phoenix, female in marriage symbol
    Fenghuang,
    /// (Manx) – House spirit
    Fenodyree,
    /// (Norse) – Gigantic, ravenous wolf
    Fenrir,
    /// (Irish) – Double or doppelgänger
    Fetch,
    /// (Slavic) – Undead
    Fext,
    /// (Orkney) – Fish-human hybrid that kidnaps humans for servants
    Finfolk,
    /// (Irish) – Ancestral race
    FirBolg,
    /// (Many cultures worldwide) – Regenerative solar bird
    FireBird,
    /// (Germanic) – Dragon
    Firedrake,
    /// (Cantabrian) – Amphibious, scaled humanoid
    FishMan,
    /// (American Folklore),/// (West Virginia) – Alien, humanoid
    FlatwoodsMonster,
    /// (Irish) – Goat-headed giant
    Fomorian,
    /// (Medieval Bestiaries) – Giant horned red cattle
    ForestBull,
    // Norfolk black dog
    Freybug,
    /// (Celtic) – Malevolent water spirit
    Fuath,
    /// (Chinese) – Underworld dragon
    Fucanglong,
    /// (Japanese) – Ghosts of people who drowned at sea
    Funayurei,
    /// (Japanese) – Animated jar
    FuruUtsubo,
    /// (Japanese) – Woman with a second mouth on the back of her head
    FutakuchiOnna,
    /// (Scandinavian) – Animal familiar
    Fylgja,
    /// (Seneca) – Dragon
    Gaasyendietha,
    /// (Russian) – Iron-beaked bird with copper talons
    Gagana,
    /// (Japanese) – Ghosts of especially greedy people
    Gaki,
    /// (Mesopotamian) – Underworld demons
    Gallu,
    /// (Basque) – Small demonic servants
    Galtzagorriak,
    /// (Russian) – Prophetic human-headed bird
    Gamayun,
    /// (Hindu) – Attendants of Shiva
    Gana,
    /// (Irish) – Male fairy that seduces human women
    Gancanagh,
    /// (Hindu) – Double-headed bird
    Gandabherunda,
    /// (Hindu) – Male nature spirits, often depicted as part human, part animal
    Gandharva,
    /// (French) – Water dragon
    Gargouille,
    /// (Australian Aboriginal) – A flying humanoid who envelops his victims
    Garkain,
    /// (Norse) – Giant, ravenous hound
    Garmr,
    /// (Hindu) – Human-eagle hybrid
    Garuda,
    /// (Japanese) – Giant malevolent skeletons
    Gashadokuro,
    /// (Basque) – Wolf capable of walking upright
    Gaueko,
    /// (Egyptian) – God of the Earth, married to Nut
    Geb,
    /// (Heraldic) – The fish pike
    Ged,
    /// (Greek) – Six-armed giant
    Gegenees,
    /// (Roman) – Spirit that protects a specific place
    GeniusLoci,
    /// (Slavic) – Male spirit associated with bringing rain and hail
    German,
    /// (Greek) – Three-headed six-armed giant with three torsos and (in some sources) six legs
    Geryon,
    /// (Scottish) – Tree guardian
    GhillieDhu,
    /// Disembodied spirits of those that have died
    Ghost,
    /// (Arabian) – Cannibalistic shapeshifting desert genie often classified as undead.
    Ghoul,
    /// (Worldwide) – Immensely large and strong humanoids
    Giant,
    /// (Worldwide) – Unusually large beasts
    GiantAnimal,
    /// (Ojibwa) – Bison-snake-bird-cougar hybrid water spirit
    GichiAnamiEBizhiw,
    /// (Sumerian) – Ghost
    Gidim,
    /// (Greek) – Race of giants that fought the Olympian gods, sometimes depicted with snake-legs
    Gigantes,
    /// (Scottish) – Smallest animal
    Gigelorum,
    /// (Akkadian) – Human-scorpion hybrid
    Girtablilu,
    /// (Scandinavian) – Corporeal ghost
    Gjenganger,
    /// (Scottish) – Human-goat hybrid
    Glaistig,
    /// (Manx) – Malevolent water horse
    Glashtyn,
    /// (Alchemy) – Diminutive Earth elemental
    Gnome,
    /// (Medieval) – Grotesque, mischievous little people
    Goblin,
    /// (English) – Giant protector of London
    Gog,
    /// (Medieval Bestiaries) – Dog-sized ant that digs for gold in sandy areas
    GoldDiggingAnt,
    /// (Jewish) – Animated construct
    Golem,
    /// (Medieval Bestiary) – Hairy humanoid
    Gorgades,
    /// (Greek) – Fanged, snake-haired humanoids that turn anyone who sees them into stone
    Gorgon,
    /// (Japanese) – Vengeful ghosts, usually of martyrs
    Goryo,
    /// (Ohio, USA) – Ape-like cryptid
    Grassman,
    /// (Folklore) – Creatures that sabotage airplanes
    Gremlin,
    /// (Heraldic) – Lion-eagle hybrid
    Griffin,
    /// (Christian, Jewish, and Islamic mythology) – Fallen angels, father of Nephilim
    Grigori,
    /// (English and Scandinavian) – Tutelary spirits of churches
    Grim,
    /// (Worldwide) – Death angel often thought to be God's/Satan's assistant
    GrimReaper,
    /// (English) – Malevolent water spirit
    Grindylow,
    /// (Mapuche) – Malevolent spirit
    Gualichu,
    /// (Christian, Jewish, and Islamic belief) – Subclassification of angels that guard and protect a specific person or living being
    GuardianAngel,
    /// (Akkadian) – Human-bull hybrid
    GudElim,
    /// (Japanese) – Anthropomorphic bird
    Guhin,
    /// (Chinese) – Ghost that manifests as an old woman
    GuiPo,
    /// (Chinese) – Ghostly tree that confuses travelers by moving
    GuiShu,
    /// (Germanic) – Gluttonous dog-cat-fox hybrid
    Gulon,
    /// (Korean mythology) – Demonic fox with thousands of tails believed to possess an army of spirits and magic in its tails
    Gumiho,
    /// (Australian Aboriginal) - An enormous reptile-fish whose movements carved out the landscape south of the Blue Mountains
    Gurangatch,
    /// (Nepalese) – Child-eating demon
    Gurumapa,
    /// (Welsh) – Black dog
    Gwyllgi,
    /// (Welsh) – Malevolent spirit
    Gwyllion,
    /// (American folklore) – Four-legged herbivore
    Gyascutus,
    /// (Lincolnshire and Yorkshire) – Black dog
    Gytrash,
    /// (Japanese) – Bull-headed monster
    Gyuki,
    /// (Norse) – listed as the "best" hawk
    Habrok,
    /// (Persian) – gigantic land animal
    Hadhayosh,
    /// (Greek) – Ruler of the Underworld
    Hades,
    /// (Korean) – dog-lion hybrid
    Haetae,
    /// (Many cultures worldwide) – wise old woman who is usually a malevolent spirit or a disguised goddess
    Hag,
    /// (Nuu-chah-nulth) – water serpent
    Haietlik,
    /// (Khoikhoi) – male cannibalistic partially invisible monster
    HaiUri,
    /// (Japanese) – talking beast which handed down knowledge on harmful spirits
    Hakutaku,
    /// (Māori) – nature guardian
    Hakuturi,
    /// (Norse) – human-elf hybrid
    HalfElf,
    /// (Finnish) – spirit that protects a specific place
    Haltija,
    /// (Greek) – oak tree nymph
    Hamadryad,
    /// (Scandinavian) – personal protection spirit
    Hamingja,
    /// (Buddhist, Hindu and Jainism) – mystic bird
    Hamsa,
    /// (Rapa Nui) – long-eared humanoid
    HanauEpe,
    /// (Malay) – shapeshifting water spirit
    HantuAir,
    /// (Philippine) – demon
    HantuDemon,
    /// (Malay) – demonic servant
    HantuRaya,
    /// (Japanese) – humanoid female with barbed, prehensile hair
    Harionago,
    /// (Greek) – birdlike human-headed death spirit
    Harpy,
    /// (Norse) – undead being who cannot leave its burial mound
    Haugbui,
    /// (Norse) – saltwater spirit
    Havsrå,
    /// (Manipuri mythology) – celestial maidens, daughters of the Sky God Soraren
    Helloi,
    /// (European) – humanoid spirit who haunts or kills
    HeadlessHorseman,
    /// (Brazilian) – fire-spewing, headless, spectral mule
    HeadlessMule,
    /// (Greek) – primordial giants with 100 hands and fifty heads
    Hecatonchires,
    /// (Japanese) – crabs with human-faced shells, the spirits of warriors killed in the Battle of Dan-no-ura
    Heikegani,
    /// (German) – household spirit
    Heinzelmannchen,
    /// (Greek) – fen nymph
    Helead,
    /// (Many cultures worldwide) – underworld dog
    Hellhound,
    /// (Greek) – gatekeeper of Olympus
    Heracles,
    /// (Medieval Bestiaries) – glowing bird
    Hercinia,
    /// (Basque) – dragon
    Herensuge,
    /// (Greek) – nymph daughters of Atlas
    Hesperides,
    /// (United States) – nocturnal forest creature
    Hidebehind,
    /// (Japanese) – drought spirit
    Hiderigami,
    /// (Ancient Egypt) – falcon-headed sphinx
    Hieracosphinx,
    /// (Japanese) – baboon monster
    Hihi,
    /// (Finnish) – nature guardian
    Hiisi,
    /// (Greek)
    Hippalectryon,
    /// (Etruscan, Greek and Phoenician) – horse-fish hybrid
    Hippocamp,
    /// (Medieval Bestiaries) – hybrid of a griffin and horse; a lion-eagle-horse hybrid
    Hippogriff,
    /// (Medieval Bestiary) – horse-hoofed humanoid
    Hippopodes,
    /// (Medieval Bestiary) – deer-goat hybrid
    Hircocervus,
    /// (Japanese) – ghosts of the newly dead, which take the form of fireballs
    Hitodama,
    /// (Japanese) – one-eyed childlike spirit
    HitotsumeKozo,
    /// (English) – house spirit
    Hob,
    /// (English) – malevolent spirit
    Hobbididance,
    /// (Medieval) – friendly or amusing goblin
    Hobgoblin,
    /// (Native American) – frog-mammoth-lizard hybrid
    Hodag,
    /// (Kwakiutl) – bird
    Hokhokw,
    /// (Japanese) – dog-like Chinese tree spirit
    Hoko,
    /// (Persian) – eagle-lion hybrid, similar to a griffin
    Homa,
    /// (Colombian) – human-alligator hybrid
    HombreCaiman,
    /// (Latin America) – human-cat hybrid
    HombreGato,
    /// (Alchemy) – small animated construct
    Homunculus,
    /// (Japanese) – rooster-swallow-fowl-snake-goose-tortoise-stag-fish hybrid
    Hoo,
    /// near passerine bird common to Africa and Eurasia that features in many mythologies in those continents
    Hoopoe,
    /// snake which rolls by taking its tail in its mouth
    HoopSnake,
    /// (Native American) – serpentine rain spirit
    HornedSerpent,
    /// (Japanese) – deceased person
    Hotoke,
    /// (Islamic) – heavenly beings
    Houri,
    /// (Norse) – giant, who in eagle form, creates the wind by beating his wings
    Hraesvelg,
    /// (Norse) – frost giants who are the main inhabitants of either Jotunheim or Niflheim
    Hrímþursar,
    /// (Mayan) – human-deer hybrid
    Huaychivo,
    /// (Norse) – pair of ravens associated with the Norse god Odin whose names mean Thought and Memory.
    HuginnAndMuninn,
    /// (Icelandic/Faroese) – secret mound/rock dwelling elves
    Huldufolk,
    /// (Scandinavian) – forest spirit
    Hulder,
    /// (Chinese) – nine-tailed fox spirit
    HuliJing,
    /// (Persian) – regenerative fire bird
    Huma,
    /// (Akkadian) – lion-faced giant
    Humbaba,
    /// (Chinese) – chaos spirit
    Hundun,
    /// (Taíno) – nocturnal ghost
    Hupia,
    /// (Japanese) – hundred-eyes creature
    Hyakume,
    /// (Greek) – multi-headed water serpent/dragon
    Hydra,
    /// (Medieval Bestiary) – snake whose poison causes the victim to swell up
    Hydros,
    /// (Medieval Bestiary) – snake from the Nile River that would kill crocodiles from the inside
    Hydrus,
    /// (Japanese) – hair-covered kappa
    Hyosube,
    /// (Medieval Bestiary) – snake that kills its victims in their sleep
    Hypnalis,
    /// (mythology) – Hoopoe
    Hudhud,
    /// (Inuit) – Little people
    Ishigaq,
    /// (Medieval Bestiaries) – Savage human-goat hybrid from a remote island chain
    IslandSatyr,
    /// (Japanese) – Shark-like sea monster
    Isonade,
    /// (Japanese) – Ghostly aerial phenomenon that attacks people
    IttanMomen,
    /// (Japanese) – Char which appeared as a Buddhist monk
    IwanaBozu,
    /// (American) – Rabbit with antlers
    Jackalope,
    /// (English) – Malevolent giant
    JackInIrons,
    /// (Medieval folklore) – Vegetal lantern
    JackOLantern,
    /// (Medieval Bestiaries) – Winged serpent or small dragon
    Jaculus,
    /// (Medieval folklore) – Island-sized fish
    Jasconius,
    /// (Guaraní) – Nature guardian and bogeyman
    JasyJaterei,
    /// (Hindu mythology) – Vulture demigod
    Jatayu,
    /// (Slavic) – Vampirised premature baby
    Jaud,
    /// (Java) – Vampiric little people
    Jenglot,
    /// (Sawa) – Water spirit
    Jengu,
    /// (Basque) – Megalith-building giant
    Jentil,
    /// (Mi'kmaq) – Anthropophagous giant
    Jenu,
    /// (Swedish) – Gluttonous dog-cat-fox hybrid
    Jerff,
    /// (American) – Demonic dragon or flying demon who was given birth to by an American living in New Jersey
    JerseyDevil,
    /// (Chinese) – One-eyed, one-winged bird who requires a mate for survival
    Jian,
    /// (Chinese) – Life-draining, reanimated corpse
    Jiangshi,
    /// (Chinese) – Dragon
    Jiaolong,
    /// (Japanese) – Spirit that protects a specific place
    Jibakurei,
    /// (Lithuanian) – House spirit
    Jievaras,
    /// (Japanese) – Corpse-eating ghost
    Jikininki,
    /// (Arabian, Islamic) – Spiritual creatures; genii
    Jinn,
    /// (Mi'kmaq) – Underwater horned snake; lives in lakes and eats humans
    JipijkaM,
    /// (Chinese) – Nine-headed bird worshiped by ancient natives in Hubei Province.
    Jiufeng,
    /// (Chinese) – Nine-headed, demonic bird
    JiuTouNiao,
    /// (Iroquois) – Little people nature spirit
    Jogah,
    /// (Norse) – Sea serpent
    Jormungandr,
    /// (Japanese) – Spider woman
    Jorogumo,
    /// (Japanese) – Animated folding screen cloth
    Jotai,
    /// (Norse) – Gigantic nature spirits
    Jotunn,
    /// (Korean) – Bird
    Jujak,
    /// (Guyanese) – Malevolent spirit
    Jumbee,
    /// (Dutch) – Little people that live underground, in mushrooms, or as house spirits
    Kabouter,
    /// (Hopi and Puebloan) – Nature spirit
    Kachina,
    /// (Japanese) – Little people and water spirits
    Kahaku,
    /// (Scandinavian) – Wind spirit
    Kajsa,
    /// (Hindu) – Descendants of Kala
    Kalakeyas,
    /// (Greek) – Grotesque, malevolent spirit
    Kallikantzaroi,
    /// (Japanese) – Wind spirit
    Kamaitachi,
    /// (Philippine) – Philippine counterpart of Death
    Kamatayan,
    /// (Japanese) – Nature spirit
    Kami,
    /// (Japanese) – Hair-cutting spirit
    Kamikiri,
    /// (Japanese) – Bathroom spirit
    KanbariNyudo,
    /// (Manipuri mythology) – Great Dragon in the Kangla Palace
    KanglaSha,
    /// (Japanese) – Drought spirit
    Kanbo,
    /// (Japanese) – Money spirit
    Kanedama,
    /// (Japanese) – Little people and water spirit
    Kappa,
    /// (Philippine) – Malevolent tree spirit
    Kapre,
    /// (Bulgarian and Turkish), also in Bosnia and Herzegovina and Serbia known as Karanđoloz – Troublesome spirit
    Karakoncolos,
    /// (Turkish) – Male night-demon
    Karakura,
    /// (Japanese) – Tengu with a bird's bill
    KarasuTengu,
    /// (Persian) – One-horned giant animal
    Karkadann,
    /// (Greek) – Giant crab
    Karkinos,
    /// (Japanese) – Eagle-human hybrid
    Karura,
    /// (Polish) – Little people and mine spirits
    Karzelek,
    /// (Japanese) – Animated parasol
    KasaObake,
    /// (Japanese) – Cat-like demon which descends from the sky and carries away corpses
    Kasha,
    /// (Japanese) – Kappa who climb into the mountains for the winter
    Kashanbo,
    /// (Japanese) – Woman riding on a flaming wheel
    KatawaGuruma,
    /// (Japanese) – Handsome man from the moon
    KatsuraOtoko,
    /// (Albanian) – Man-eating giant
    Katallan,
    /// (Lithuanian) – Nature spirit
    Kaukas,
    /// (Japanese) – Supernatural river otter
    KawaUso,
    /// (Japanese) – Smelly, cowardly water spirit
    KawaZaru,
    /// (Chukchi mythology) – Ogre or evil spirit
    KeLets,
    /// (Inuit) – Hairless dog
    Keelut,
    /// (Abenaki) – Half-human half-animal cannibalistic giant
    KeeWakw,
    /// (Japanese) – Amorphous afterbirth spirit
    Kekkai,
    /// (Irish and Scottish) – Malevolent water horse
    Kelpie,
    /// (Greek) – Female death spirit
    Ker,
    /// (Japanese) – Mysterious, white, fluffy creature
    KesaranPasaran,
    /// (Japanese) – Disease spirit
    Keukegen,
    /// (Heraldic) – Wingless griffin
    Keythong,
    /// (Nepalese) – Fat, hairy ape-like creature
    Khyah,
    /// (Inuit) – Night-demon
    Kigatilik,
    /// (Sotho) – Gluttonous monster that was one of the first beasts of creation
    Kholomodumo,
    /// (Japanese) – Tree sprite from Okinawa
    Kijimunaa,
    /// (Japanese) – She-devil
    Kijo,
    /// (Slavic) – Female house spirit
    Kikimora,
    /// (English and Scottish) – Ugly, mischievous mill spirit
    Killmoulis,
    /// (Hindu) – Human-bird hybrid
    Kinnara,
    /// (Japanese) – Bird
    KinU,
    /// (Japanese) – Japanese Unicorn
    Kirin,
    /// (Angola) – Malevolent, two-faced seducer
    Kishi,
    /// (Japanese) – Fox spirit
    Kitsune,
    /// (Japanese) – Person possessed by a fox spirit
    KitsuneTsuki,
    /// (Japanese) – Woman who transformed into a serpentine demon out of the rage of unrequited love
    Kiyohime,
    /// (German) – Ship spirit
    Klabautermann,
    /// (folklore),/// (Cornish and Welsh) – Little people and mine spirits
    Knocker,
    /// (English) – Water dragon
    Knucker,
    /// (Greek) – Goblin like thieves and tricksters
    Kobalos,
    /// (German) – Little people and mine or house spirits
    Kobold,
    /// (Japanese) – Tree spirit
    Kodama,
    /// (Germanic) – House spirit
    Kofewalt,
    /// (Abenaki) – Hideous monster
    KoGok,
    /// (Japanese) – Ubume bird
    Kokakucho,
    /// (Japanese) – Protective animal
    Komainu,
    /// (Japanese) – Infant that cries until it is picked up, then increases its weight and crushes its victim
    KonakiJiji,
    /// (Japanese) – Bird-like creature
    KonohaTengu,
    /// (Ainu) – Little people
    KoroPokGuru,
    /// (Breton) – Little people and nature spirits
    Korrigan,
    /// (Scandinavian) – Sea monster
    Kraken,
    /// (Slavic) – Little people nature spirits
    Krasnoludek,
    /// (Southeast Asian) – Vampiric, floating head
    Krasue,
    /// (Germany) – Christmas Devil who punishes badly-behaved children
    Krampus,
    /// (Guaraní) – Forest spirit
    KuarahyJara,
    /// (Japanese) – Female corpse-chewing graveyard spirit
    Kubikajiri,
    /// (Japanese) – Vengeful ghost of a woman mutilated by her husband
    KuchisakeOnna,
    /// (Japanese) – Miniature fox spirit
    KudaGitsune,
    /// (Japanese) – Human-faced calf which predicts a calamity before dying
    Kudan,
    /// (Chinese) – One-legged monster
    Kui,
    /// (Albanian) – Female demon who spreads sickness
    Kukudhi,
    /// (Mi'kmaq) – Large, hairy, greedy, human-eating bipedal monsters whose scream can kill
    Kukwes,
    /// (Albanian) – Drought-causing dragon
    Kulshedra,
    /// (Philippine) – Death spirits
    Kumakatok,
    /// (Korean) – Fox spirit
    Kumiho,
    /// (Chinese) – Giant fish
    Kun,
    /// (Hawaiian) – Shapeshifting tricksters
    Kupua,
    /// (Japanese) – Guardian spirit of a warehouse
    Kurabokko,
    /// (Japanese) – Jellyfish which floats through the air as a fireball
    KurageNoHinotama,
    /// (Hindu mythology) – Second avatar of Vishnu in the form of a Turtle
    Kurma,
    /// (Guaraní) – Wild man and fertility spirit
    Kurupi,
    /// (Tlingit) – Shapeshifting "land otter man"
    Kushtaka,
    /// (Korean) – Chicken-lizard hybrid
    KyeRyong,
    /// (Japanese) – Animated scroll or paper
    Kyourinrin,
    /// (Japanese) – Nine-tailed fox
    KyubiNoKitsune,
    /// (Japanese) – Vampire
    Kyuketsuki,
    /// (Assyrian) – Disease demon
    LaBarTu,
    /// (Akkadian) – Sea snake
    LabbMu,
    /// (Slavic) – Sunstroke spirit
    Ladyidday,
    /// (Greek) – Dragon guarding the golden apples of the Hesperides
    Ladon,
    /// (Greek) – Enchanted dog that always caught his prey
    Laelaps,
    /// (Greek) – Anthropophagic giants
    Laestrygonians,
    /// (Slavic) – Field spirit
    Lakanica,
    /// (Worldwide) – Gigantic animals reported to inhabit various lakes around the world
    LakeMonster,
    /// (Nepalese) – Demon with fangs
    Lakhey,
    /// (Latin America) – Death spirit associated with drowning
    LaLlorona,
    /// (Akkadian and Sumerian) – Protective spirit with the form of a winged bull or human-headed lion
    Lamassu,
    /// (English) – Giant worm
    LambtonWorm,
    /// (Greek) – Child-devouring monster
    Lamia,
    /// (Basque) – Water spirit with duck-like feet
    Lamiak,
    /// (Colombian) – Shapeshifting, female water spirit
    LaMojana,
    /// (Greek) – Underworld nymph
    Lampades,
    /// (Norse) – Nature spirits
    Landvaettir,
    /// (Manipuri mythology) – Semi human, semi hornbill creature
    Langmeidong,
    /// (Roman) – House spirit
    Lares,
    /// (Venezuela) – Female ghost that punishes unfaithful husbands
    LaSayona,
    /// (Colombian) – Nature spirit that seduces and kills men
    LaTunda,
    /// Miniature bear thought to inhabit the lava beds of south central Oregon
    LavaBear,
    /// (Lithuanian) – Field spirit
    LaukuDvasios,
    /// (Baltic) – Sky spirit
    Lauma,
    /// (Scottish) – Gigantic water rat
    Lavellan,
    /// (Celtic) – Fairy lover
    LeananSidhe,
    /// (Irish) – Possessing spirit or vampire
    Leanashe,
    /// (Greek) – Meadow nymph
    Leimakids,
    /// (Etruscan) – Fish-tailed lion
    Leokampoi,
    /// (Medieval Bestiary) – Tiny animal poisonous to lions
    Leontophone,
    /// (Irish) – Cobbler spirit
    Leprechaun,
    /// (Slavic) – Tree spirit
    Leszi,
    /// (Greek) – White poplar tree nymph
    Leuce,
    /// (Medieval Bestiary) – Crocotta-lion hybrid
    Leucrota,
    /// (Jewish) – Sea monster seen in Job 41
    Leviathan,
    /// (Balinese) – Anthropophagous flying head with entrails
    Leyak,
    /// (Medieval Bestiaries) – Human-horse hybrid
    LibyanAegipanes,
    /// (Medieval Bestiaries) – Human-goat hybrid
    LibyanSatyr,
    /// (Hungary) – Magical chicken that transforms into a humanoid
    Liderc,
    /// (Southern Africa) – Magical bird found at sites of lightning strikes
    LightningBird,
    /// (Slavic) – One-eyed hag or goblin
    Likho,
    /// (Jewish) – Night-demoness
    Lilin,
    /// (Assyrian) – Winged demon
    Lilitu,
    /// (Greek) – Lake nymph
    Limnades,
    /// (Germanic) – Dragon
    Lindworm,
    /// (Norse) – Sunlight spirits; the Light Elves
    Ljosalfar,
    /// (Albanian)- Demoness
    Ljubi,
    /// (Welsh) – Frog-bat-lizard hybrid
    LlamhigynYDwr,
    /// (Scottish) – Serpentine sea monster
    LochNessMonster,
    /// (Norse mythology) – God of night
    Loki,
    /// (Abenaki) – Hideous monster
    LoLol,
    /// Chinese dragon
    Long,
    /// (Italian) – Female human-goat hybrid and water spirit
    Longana,
    /// (Chinese) – Dragon-horse hybrid
    LongMa,
    /// (French America) – Shapeshifting, female vampire
    Loogaroo,
    /// (French) – Snake-mollusk hybrid
    LouCarcolh,
    /// (French) – Werewolf
    LoupGarou,
    /// (American Folklore),/// (Ohio) – Cryptid, Humanoid Frog
    LovelandFrog,
    /// (English) – House spirit
    LubberFiend,
    /// (Chinese) – Truth-detecting animal
    Luduan,
    /// (Albanian) – Vampire
    Lugat,
    /// (Guaraní) – Werewolf | Cadaver-eating dog
    Luison,
    /// Sea Monster
    Lusca,
    /// (French) – Amusing goblin
    Lutin,
    /// (Icelandic) Whale-like sea monster
    Lyngbakr,
    /// (Medieval Bestiaries) – Feline guide spirit
    Lynx,
    /// (Estonian mythology) – Subterranean spirit
    MaaAlused,
    /// (Medieval bestiaries) – Hermaphroditic humanoid
    Machlyes,
    /// (Medieval bestiaries) – Giant-headed humanoid
    Macrocephali,
    /// (West African Mythology ) – Female ghost
    MadamKoiKoi,
    /// (Colombian folklore) – Nature guardian
    Madremonte,
    /// (Māori) – Savage, arboreal humanoids
    Maero,
    /// (English folklore) – Giant protector of London
    Magog,
    /// (Hindu mythology) – Giant elephant that holds up the world
    MahaPudma,
    /// (Basque mythology) – Megalith-building giant
    Mairu,
    /// (Latvian mythology) – Benevolent house spirit
    MajasGari,
    // in Swahili mythology, shape-shifting spirits that can pass as humans
    Majitu,
    /// (Indian mythology) – Aquatic beings
    Makara,
    /// (Japanese mythology) – Pillow-moving spirit
    MakuraGaeshi,
    /// (Welsh mythology) – Spirit of the hunt
    MalltYNos,
    /// (Africa and the African diaspora) – Supernaturally beautiful water spirits
    MamiWata,
    /// (Philippine mythology) – Vampires that sever their torsos from their legs to fly around
    Manananggal,
    /// (Medieval bestiaries) – Humanoid with a forty-year lifespan
    Mandi,
    /// (Medieval folklore) – Diminutive, animated construct
    Mandrake,
    /// (Roman mythology) – Ancestral spirits
    Manes,
    /// (Cree) – Little people with six fingers and no noses
    Mannegishi,
    /// (Persian mythology) – Lion-human-scorpion hybrid
    Manticore,
    /// (Brazilian mythology) – Giant sloth
    Mapinguari,
    /// (Scandinavian folklore) – Female night-demon
    Mara,
    /// (Italian folklore) – Malevolent water spirit
    Marabbecca,
    /// (Tuamotu) – Attendant of Kiho-tumu, the supreme god
    Mareikura,
    /// (Greek mythology) – Man-eating horses
    MaresOfDiomedes,
    /// (Arabian mythology) – Jinn associated fortune tellers
    Marid,
    /// (Norse mythology) – Mermen with prophetic abilities
    Marmennill,
    /// (Lithuanian mythology) – Disease spirits
    MaroDeives,
    /// (Abenaki mythology) – Shapeshifting toad spirit
    MaskiMonGweZoOs,
    /// (French mythology) – Spirit that takes animal form; usually that of a black cat
    Matagot,
    /// (Hindu mythology) – First Avatar of Vishnu in the form of a half-fish and half-man
    Matsya,
    /// (Hindu mythology) – Peacock spirit
    Mayura,
    /// (Jewish mythology) – Invisible, malevolent spirit
    Mazzikin,
    /// (Guaraní mythology) – Snake-parrot hybrid
    MboiTuI,
    /// (Central Africa) – Possessing demon
    Mbwiri,
    /// (Greek mythology) – Serpent-female hybrid,/// (Gorgon) with numerous snake heads
    Medusa,
    // biblical bird
    MelekTaus,
    /// (Greek mythology) – Ash tree nymph
    Meliae,
    /// (Medieval folklore) – Female water spirit, with the form of a winged mermaid or serpent
    Melusine,
    /// (Hawaiian mythology) – Little people and craftsmen
    Menehune,
    /// (Finnish mythology) – Little people and nature spirits
    Menninkainen,
    /// (Singapore) – Combination of a lion and a fish, the symbol of Singapore
    Merlion,
    /// (multiple cultures) – Human-fish hybrid
    Mermaid,
    /// (multiple cultures) – Human-fish hybrid
    Merman,
    /// (English mythology) – Elderly wizard
    Merlin,
    /// (Irish mythology and Scottish) – Human-fish hybrid
    Merrow,
    /// (Abenaki mythology) – Ice-hearted wizards
    MeteeKolenOl,
    /// (Australian Aboriginal mythology) – Extremely elongated humanoid that has to live in rock crevasses to avoid blowing away
    Mimi,
    /// (Australian Aboriginal mythology) – Death spirit
    MinkaBird,
    /// (Philippine) – Giant swallow
    Minokawa,
    /// (Greek mythology) – Human-bull hybrid
    Minotaur,
    /// (Ojibwa) – Feline water spirit
    Mishibizhiw,
    /// (Ojibwa) – Serpentine rain spirit
    MisiGinebig,
    /// (Cree) – Serpentine rain spirit
    MisiKinepikw,
    /// (Japanese mythology) – Water dragon
    Mizuchi,
    /// (Chinese mythology) – Vengeful ghost or demon
    Mogwai,
    /// (Latin American folklore) – Nature spirit
    Mohan,
    /// (Congo) – Water-dwelling creature
    MokeleMbembe,
    /// (Australian Aboriginal mythology) – Malevolent spirit that kills sorcerers
    Mokoi,
    /// (Polynesian mythology) – Amphibious humanoid living in the spirit world,/// (underground world)
    Mokorea,
    /// (Guaraní mythology) – Giant snake with antennae
    Monai,
    /// (Medieval bestiaries) – One-horned stag-horse-elephant-boar hybrid, sometimes treated as distinct from the unicorn
    Monocerus,
    /// (South America) – Giant monkey
    MonoGrande,
    /// (Medieval bestiaries) – Dwarf with one giant foot
    Monopod,
    /// (Manx folklore) – Nature spirit
    MooinjerVeggey,
    /// (Slavic mythology) – Disembodied spirit
    Mora,
    /// (Breton and Welsh mythology) – Water spirits
    Morgens,
    /// (Japanese mythology) – Animated tea kettle
    MorinjiNoOkama,
    /// (Greek) – Underworld spirit
    Mormolykeia,
    /// (Romanian) – Vampiric ghost
    Moroi,
    /// (Continental Germanic mythology) – Little people and tree spirits
    MossPeople,
    /// (American folklore) – Large grey winged humanoid with glowing red eyes
    Mothman,
    /// (Canadian folklore) – Fish-like lake monster
    Mugwump,
    /// (Japanese mythology) – Shapeshifting badger spirit
    Mujina,
    /// (Australian Aboriginal mythology) – Water monster
    Muldjewangk,
    /// (Philippine mythology) – Spirit of a deceased person seeking justice or has unfinished business
    Multo,
    /// (Egyptian) – Undead creature who revives
    Mummy,
    /// (Romanian folklore) – Forest-dwelling hag
    MumaPadurii,
    /// (Australian Aboriginal) – Giant goanna
    MungoonGali,
    /// (Medieval bestiaries) – Hare-squirrel-boar hybrid that has an intense body heat
    Muscaliet,
    /// (Greek mythology) – Spirits that inspire artists
    Muse,
    /// (Mesopotamian mythology)
    Mushusshu,
    /// (Heraldic) – Sheep-goat hybrid
    Musimon,
    /// (Scandinavian folklore) – Ghosts of unbaptized children
    Myling,
    /// (Medieval bestiaries) – Ant-lion hybrid
    Myrmecoleon,
    /// (German) – Anthropophagous undead
    Nachzehrer,
    /// (Buddhist and Hindu) – Nature and water spirits, serpentine or human-serpent hybrids
    Naga,
    /// (Thai) – Spectral fire
    NagaFireballs,
    /// (Mesoamerica) – Human-animal shapeshifter
    Nagual,
    /// (Greek) – Freshwater nymph
    Naiad,
    /// (Finnish) – Water spirit
    Nakki,
    /// (Japanese) – Ritual disciplinary demon from the Oga Peninsula
    Namahage,
    /// (Japanese) – Giant catfish whose thrashing causing earthquakes
    Namazu,
    /// (Japanese) – Old woman who hides under the floor in abandoned storerooms
    NandoBaba,
    /// (Thai) – Tree spirit
    NangTakian,
    /// (Abenaki) – Earthquake spirit
    NanomKeeaPoDa,
    /// (Greek) – Grotto nymph
    Napaeae,
    /// (Hindu mythology) – Avatar of Vishnu in the form of half-man/half-lion
    Narasimha,
    /// (Slavic) – Fate spirit
    Narecnitsi,
    /// (Thai) – Pod people
    Nariphon,
    /// (Gunai) – Water monster
    Nargun,
    /// (Arabian) – Half-human, half-demon creature with half a body
    Nasnas,
    /// (Slavic) – Ghost
    Nav,
    /// (Hawaiian) – Savage humanoid
    Nawao,
    /// (Abenaki) – Fish-human hybrid
    NDamKenoWet,
    /// (Roman mythology) – God of freshwater and sea
    Neptune,
    /// (Germanic mythology) – Female water spirit
    Neck,
    /// (Catalan) – Little people that turn into coins
    Negret,
    /// (Japanese) – Split-tailed magical cat
    Nekomata,
    /// (Japanese) – Cat in the form of a girl
    Nekomusume,
    /// (Greek) – Lion with impenetrable skin
    NemeanLion,
    /// (Abrahamic mythology) – Gigantic sons of Grigori and human women
    Nephilim,
    /// (Greek) – Nymph daughters of Nereus
    Nereid,
    /// (Mapuche) – Nature spirit
    Ngen,
    /// (Mapuche) – Fox-like water snake
    Nguruvilu,
    /// (Chinese) – Predatory animal
    Nian,
    /// (Hawaiian) – Warrior ghosts
    Nightmarchers,
    /// (Japanese) – Monster which appears as a young woman and sucks all of the flesh off of its victim's body
    Nikusui,
    /// (Shoshone) – Aggressive little people
    Nimerigar,
    /// (Japanese) – Monkey-fish hybrid
    Ningyo,
    /// (Western Africa) – Large reptile, possibly a dragon
    NinkiNanka,
    /// (Scandinavian) – House spirit
    Nisse,
    /// (Norse) – Dragon
    Niohoggr,
    /// (Hindu) – Ocean demon
    Nivatakavachas,
    /// (Germanic) – Female water spirit
    Nix,
    /// (Japanese) – Supernatural wall, also a monstrous flying squirrel
    Nobusuma,
    /// (Slavic) – Nightmare spirit
    Nocnitsa,
    /// (Japanese) – Faceless ghost
    NopperaBo,
    /// (Japanese) – Small sea serpent
    Nozuchi,
    /// (Scottish) – Malevolent human-horse-fish hybrid
    Nuckelavee,
    /// (Japanese) – Monkey-raccoon dog-tiger-snake hybrid
    Nue,
    /// (Chinese) – Vengeful female ghost
    NuGui,
    /// (Japanese) – Disembodied, flying head that attacks people
    Nukekubi,
    /// (Māori) – Forest spirit
    NukuMaiTore,
    /// (Medieval Bestiary) – Humanoid with backwards, eight-toed feet
    Nuli,
    /// (Roman) – Tutelary spirit
    Numen,
    /// (Philippine) – Malevolent little people
    Nuno,
    /// (Japanese) – Animated chunk of dead flesh
    Nuppeppo,
    /// (Japanese) – Head-sized ball-like creature that floats in the sea and teases sailors
    Nurarihyon,
    /// (Japanese) – Female monster who appears on the beach
    NureOnna,
    /// (Japanese) – Spirit that manifests as an impassable, invisible wall
    Nurikabe,
    /// (Tonga,/// (Zimbabwean) mythology) – Snake-spirit of the Zambezi River
    NyamiNyami,
    /// (Lithuanian) – Cavern spirit
    Nykstukas,
    /// (Greek) – Nature spirit
    Nymph,
    /// (Japanese) – Shapeshifting spirits
    Obake,
    /// (Japanese) – Spook which rides piggyback on a human victim and becomes unbearably heavy
    Obariyon,
    /// (Ashanti) – Vampiric possession spirit
    Obayifo,
    /// (West Africa) – Gigantic animal that serves witches
    Obia,
    /// (Greek) – Nymph daughters of Oceanus
    Oceanid,
    /// (Basque) – Storm spirit
    Odei,
    /// (Norse mythology) – King of Asgard
    Odin,
    /// (Slavic) – Changeling
    Odmience,
    /// (Jewish) – Giant king of the Amorites
    Og,
    /// (Canadian) Canadian Lake Monster
    Ogopogo,
    /// (Nigeria) – Iron god for the Yoruba people,/// (South Western Nigeria)
    Ogun,
    /// (Medieval folklore) – Large, grotesque humanoid
    Ogre,
    /// (Japanese) – Ghost of a woman with a distorted face who was murdered by her husband
    Oiwa,
    /// (Cantabrian) – Giant cyclops who embodies evil.
    Ojancanu,
    /// (Japanese) – Spirit of a plate-counting servant girl, associated with the "Okiku-Mushi" worm
    Okiku,
    /// (Japanese) – Death spirit
    Okubi,
    /// (Japanese) – Dog or wolf that follows travelers at night, similar to the Black dog of English folklore
    OkuriInu,
    /// (Guyanese) – Vampiric hag who takes the form of a fireball at night
    OleHigue,
    /// (Japanese) – Giant, human-eating centipede that lives in the mountains
    Omukade,
    /// (Japanese) – Large, grotesque humanoid demon, usually having red skin and horns
    Oni,
    /// (Japanese) – Spectral fire
    Onibi,
    /// (Japanese) – Bird-demon created from the spirits of freshly dead corpses
    Onmoraki,
    /// (Medieval Bestiaries) – Human-donkey hybrid
    Onocentaur,
    /// (Greek) – Shapeshifting demon
    Onoskelis,
    /// (Japanese) – Vengeful ghost that manifests in a physical rather than a spectral form
    Onryo,
    /// (Aztec and Latin American folklore) – Wild cat, possibly a subspecies of cougar
    Onza,
    /// (Unknown origin) – Bird that flies backwards
    OozlumBird,
    /// (Greek) – Bull-serpent hybrid
    Ophiotaurus,
    /// (Heraldic) – Lion-eagle hybrid, similar to a griffin, but with leonine forelimbs
    Opinicus,
    /// (Malay) – Forest spirit
    OrangBunian,
    /// (Malay) – Spectral rapist
    OrangMinyak,
    /// (Hungarian) – Shapeshifting demon
    Ordog,
    /// (Greek) – Mountain nymph
    Oread,
    /// (Tyrolean) – Little people and house spirits
    Ork,
    /// (European) – Horse-headed, honest oracle classed as a demon
    Orobas,
    /// (Medieval Bestiaries) – Peacock-eagle-swan-crane hybrid
    OrphanBird,
    /// (Greek) – Two-headed dog
    Orthrus,
    /// (Hellenized) – God of the dead and the judge of the underworld
    Osiris,
    /// (Nigeria) – God of love and fertility
    Oshun,
    /// (Finnish) – Bear spirit
    Otso,
    /// (Worldwide) – Mystic serpent/dragon that eats its own tail
    Ouroboros,
    /// (Slavic) – Malevolent threshing house spirit
    Ovinnik,
    /// (Cornish) – Owl-like humanoid
    Owlman,
    /// (Finnish) – Spectral fire
    PaasselkaDevils,
    /// (Abenaki) – Weather spirit
    Pamola,
    /// (Greek) – Human-goat hybrids descended from the god Pan
    Panes,
    /// (Medieval Bestiary) – White-haired humanoid with giant ears and eight fingers and toes
    Pandi,
    /// (Hindu) – Demons with herds of stolen cows
    Panis,
    /// (Chinese) – Water dragon
    Panlong,
    /// (Medieval Bestiaries) – Humanoid with gigantic ears
    Panotti,
    /// (Medieval Bestiaries) – Feline with sweet breath
    Panther,
    /// (Medieval Bestiaries) – Shapeshifting animal whose natural form was a large ruminant
    Parandrus,
    /// (Medieval Bestiaries) – Fast, spotted feline believed to mate with lions to produce leopards
    Pard,
    /// (Etruscan) – Fish-tailed leopard
    Pardalokampoi,
    /// (Medieval folklore) – Giant race reputed to live in the area of Patagonia
    Patagon,
    /// (Latin America) – Anthropophagous, one-legged humanoid
    Patasola,
    /// (Māori) – White-skinned nature spirits
    Patupairehe,
    /// (Scottish) – Strong little people
    Pech,
    /// (Greek) – Spring nymph
    Pegaeae,
    /// (Greek) – Winged horse
    Pegasus,
    /// Pegasus-unicorn hybrid
    Pegacorn,
    /// (Malay) – Servant spirit
    Pelesit,
    /// (French) – Dragon
    Peluda,
    /// (Malay) – Vampires that sever their heads from their bodies to fly around, usually with their intestines or other internal organs trailing behind
    Penanggalan,
    /// (Chinese) – Giant bird
    Peng,
    /// (Chinese) – Tree spirit
    Penghou,
    /// (Persian) – Winged humanoid
    Peri,
    /// (Allegedly Medieval folklore) – Deer-bird hybrid
    Peryton,
    /// (Catalan) – Nightmare demon in the form of a cat or dog
    Pesanta,
    /// (Chilota and Mapuche) – Vampiric, flying, shapeshifting serpent
    Peuchen,
    /// (Thai) – Ghost of a person who has died suddenly of a violent or cruel death
    PhiTaiHong,
    /// (Phoenician) – Regenerative bird reborn from its own ashes
    Phoenix,
    /// (Native American mythology) – Winged, antlered feline-like dragon
    Piasa,
    /// (Armenian) – Large land animal
    Piatek,
    /// (Pictish stones) – Stylistic animal, possibly a dragon
    PictishBeast,
    /// (Mapuche) – Nature spirit
    Pillan,
    /// ([Japanese spirit])
    Plagg,
    /// (Abenaki) – Water spirit
    PimSkwaWagenOwad,
    /// (Finnish) – Minor demon
    Piru,
    /// (Hindu) – Carrion-eating demon
    Pishacha,
    /// (Peru) – Monster man that steals its victim's body fat for cannibalistic purposes
    Pishtaco,
    /// (Abenaki) – Serpentine rain spirit
    PitaSkog,
    /// (Cornish) – Little people and nature spirits
    Pixie,
    /// (Chinese) – Winged lion
    Pixiu,
    /// (Chinese) – Horned, dragon-lion hybrid
    PiYao,
    /// (Slavic) – Vampire created when a mother strangles her child
    Plakavac,
    /// (Abenaki) – Tree spirit
    PokWejeeMen,
    /// (Polish) – Little people and field spirits
    Polevik,
    /// (Colombian) – Man-eating chicken spirit
    PolloMaligno,
    /// (Malay) – Invisible servant spirit
    Polong,
    /// (German) – Ghost that moves objects
    Poltergeist,
    /// (Guaraní) – Wild man and nature spirit
    Pombero,
    /// (Māori) – Grotesque, malevolent humanoid
    Ponaturi,
    /// (Malay) – Undead, vampiric women who died in childbirth
    Pontianak,
    /// (American Folklore) Kentucky Urban Legend – Cryptid, a murderous creature that is part man, sheep, and goat
    PopeLickMonster,
    /// (Māori) – Giant bird
    Poukai,
    /// (Buddhist, Hindu, and Jain) – Ghosts of especially greedy people
    Preta,
    /// (Romanian – Roman) – Undead wolf
    Pricolici,
    /// (Serbia) – Dog-headed monster
    Psoglav,
    /// (Slavic) – Mischievous spirit
    Psotnik,
    /// (Greek) – Butterfly-winged nymphs, daughters of Psyche
    Psychai,
    /// (Greek) – Creatures, spirits, angels, or deities in many religions who escort newly deceased souls from Earth to the afterlife
    Psychopomp,
    /// (Welsh) – Shapeshifting animal spirit
    Puca,
    /// (Icelandic) – Malevolent little person
    Puki,
    /// (English) – House spirit
    Puck,
    /// (German) – House spirit
    Putz,
    /// (Philippine) – Headless humanoid
    Pugot,
    /// (Frisian) – House spirit
    Puk,
    /// (Latvian) – Dragon
    Pukis,
    /// (Native American mythology) – Troll-like gray-skinned being
    Puckwudgie,
    /// (Greek) – Little people
    Pygmy,
    /// (Greek) – Insect-dragon hybrid
    Pyrausta,
    /// (Greek) – Serpentine dragon
    Python,
    /// (Inuit mythology) – Aquatic human abductor
    Qalupalik,
    /// (Chinese) – Dragon-ox-deer hybrid
    Qilin,
    /// (Inuit) – Large, bald dog spirit
    Qiqirn,
    /// (Jewish) – Evil spirits
    Qliphoth,
    /// (Arthurian legend) – Serpent-leopard-lion-hart hybrid
    QuestingBeast,
    /// (Aztec) – Important Aztec god whose name means "feathered serpent"; he is not to be confused with the quetzal, a type of bird
    Quetzalcoatl,
    /// (Frankish) – Five-horned bull
    Quinotaur,
    /// (Norse) – Spirit that protects a specific place
    Ra,
    /// (Akkadian) – Vampiric spirit that ambushes people
    Rabisu,
    /// (Swedish) – Tree spirit
    Radande,
    /// (Lithuanian) – Malevolent witch
    Ragana,
    /// (Japanese) – Lightning spirit
    Raiju,
    /// (Native American) – Rain spirit
    RainBird,
    /// (Lenape) – Crow spirit
    RainbowCrow,
    /// (Hindu) – Whale-sized, multi-colored fish
    RainbowFish,
    /// (Australian Aboriginal) – Snake
    RainbowSerpent,
    /// (Buddhist and Hindu) – Shapeshifting demon
    Rakshasa,
    /// (Cantabrian) – Extremely long, weasel-like animal
    Ramidreju,
    /// (Slavic) – Whirlwind spirit
    Rarog,
    /// (Cherokee) – Life-draining spirit
    RavenMocker,
    /// (Native American, Norse, and Siberian) – Trickster spirit
    RavenSpirit,
    /// (Norse) – Squirrel spirit
    Ratatoskr,
    /// (American Folklore) – Possible plesiosaur or serpent
    RaystownRay,
    /// (English) – Evil, ugly humanoid
    Redcap,
    /// (Jewish) – Gigantic land animal
    ReEm,
    /// (Heraldic) – Eagle, sometimes depicted with two heads
    Reichsadler,
    /// (Jewish) – Giant
    Rephaite,
    /// (Global) – Human-lizard hybrid
    ReptilianHumanoid,
    /// (Medieval folklore) – Reanimated dead
    Revenant,
    /// (Arabian and Persian) – Gigantic bird
    Roc,
    /// (Japanese) – Long-necked, humanoid trickster
    Rokurokubi,
    /// (Africa and India) – Skeletal creature with elements of a rabbit, badger, and bear
    Rompo,
    /// (Vietnamese) dragon
    Rong,
    /// (French America) – Human-wolf shapeshifter
    Rougarou,
    /// (Slavic) – Female water spirit
    Rusalka,
    /// Japanese dragon
    Ryu,
    /// (Brazilian) – One-legged nature spirit
    Saci,
    /// (Japanese) – Horse head that dangles from trees on Kyūshū
    Sagari,
    /// (Japanese) – Haunted pillar, installed upside-down
    Sakabashira,
    /// (Alchemy) – Fire elemental
    Salamander,
    /// (Japanese) – Shark-man servant of the dragon king of the sea
    Samebito,
    /// (Slavic) – Nature spirit
    Samodiva,
    /// (Hindu) – The demigod Jatayu's brother
    Sampati,
    /// (Northern Europe) – Nursery spirit that induces sleep in children
    Sandman,
    /// (South Western Nigeria) – Yoruba king of arts, music, dance and entertainment
    Sango,
    /// (Philippine) – Spirits in the form of fireballs that roam around the forest
    Santelmo,
    /// (North Pole-European folklore) – Elderly man who delivers gifts to well-behaved children on the night of Christmas Eve
    SantaClaus,
    /// (Romanian) – Nature spirit
    Sanziana,
    /// (Philippine) – Bird of good fortune
    Sarimanok,
    /// (Hindu) – Bird spirit
    Sarngika,
    /// (Japanese) – Wicked monkey spirit who was defeated by a dog
    Sarugami,
    /// (Japanese) – Mind-reading humanoid
    Satori,
    /// (Heaven--Abrahamic mythology) – Ruler of Hell
    Satan,
    /// (Greek) – Human-goat hybrid and fertility spirit
    Satyr,
    /// (Medieval Bestiary) – Apes who always bear twins, one the mother loves, the other it hates
    Satyrus,
    /// (Japanese) – Shapeshifting turban snail spirit
    SazaeOni,
    /// (English) – Shapeshifting undead
    Sceadugenga,
    /// (Medieval Bestiaries) – Snake which mesmerizes its prey
    Scitalis,
    /// (Sumerian) – Human-scorpion hybrid
    ScorpionMan,
    /// (Greek) – Human-snake hybrid with a snake's tail, twelve legs, and six long-necked snake heads
    Scylla,
    /// (Heraldic) – Fish-tailed bee
    SeaBee,
    /// (Heraldic) a legendary creature that has the head and upper body of a lion, but with webbed forelimbs and a fish tail.
    SeaLion,
    /// (Medieval folklore) – Fish-like humanoid
    SeaMonk,
    /// (Worldwide) – Giant, marine animals
    SeaMonster,
    /// (Worldwide) – Serpentine sea monster
    SeaSerpent,
    /// (Heraldic) – Fish-tailed wyvern
    SeaWyvern,
    /// (Japanese) – Water spirit which can be heard making merry at night
    Seko,
    /// (Faroese, Icelandic, Irish, and Scottish) – Human-seal shapeshifter
    Selkie,
    /// (Japanese) – Human-faced frog which guides newly deceased souls to the graveyard
    SenpokuKanpoku,
    /// (Medieval Bestiaries) – Snake with corrosive venom
    Seps,
    /// (Worldwide) – Snake spirit
    Serpent,
    /// (Ancient Egypt) – Serpent-leopard hybrid
    Serpopard,
    /// (Japanese) – Tiger-carp hybrid
    Shachihoko,
    /// (Worldwide) – Spiritual imprint
    Shade,
    /// (American) – Malevolent ghost
    ShadowPeople,
    /// (Persian) – Giant eagle or hawk
    Shahbaz,
    /// (Islam) – Islamic version of the Devil (Satan) from the Bible
    Shaitan,
    /// (Chinese) – Rain bird
    ShangYang,
    /// (Jewish) – Chicken-legged demon
    Shedim,
    /// (Akkadian and Sumerian) – Protective spirit who takes the form of a winged bull or human-headed lion
    Shedu,
    /// (English, Scottish and German, as schellenrocc) – Water spirit
    Shellycoat,
    /// (Chinese) – Shapeshifing sea monster
    Shen,
    /// (Chinese) – Weather dragon
    Shenlong,
    /// (Japanese) – Water spirit from Shikoku
    Shibaten,
    /// (Japanese) – Servant spirit
    Shikigami,
    /// (Japanese) – Child-sized servant spirit
    ShikiOji,
    /// (Japanese) – Underworld hag
    Shikome,
    /// (Japanese) – "Death god"
    Shinigami,
    /// (Japanese) – White, faceless spirit
    ShiroBozu,
    /// (Japanese) – Animated mosquito netting or dust cloth
    Shirouneri,
    /// (Japanese) – Spirit of a dead person
    Shiryo,
    /// (Japanese) – Lion-dog hybrid
    Shisa,
    /// (Chinese) – Protective animal
    Shishi,
    /// (Japanese) – Red-haired sea-sprites who love alcohol
    Shojo,
    /// (Japanese) – Creature that peers in through skylights
    Shokera,
    /// (Albanian) – Vampire witch that feeds on children
    Shtriga,
    /// (Chinese) – Drowned ghost
    ShuiGui,
    /// (English) – Dog/monkey
    ShugMonkey,
    /// (Japanese) – Red-faced ghoul
    Shunoban,
    /// (Japanese) – Ruler of the Oni
    ShutenDoji,
    /// (Irish and Scottish) – Ancestral or nature spirit
    Sídhe,
    /// (Philippine) – Goat-like vampire
    Sigbin,
    /// (Greek) – Bald, fat, thick-lipped, and flat-nosed followers of Dionysus
    Sileni,
    /// (Slavic) – Winged dog
    Simargl,
    /// (Persian) – Dog-lion-peacock hybrid
    Simurgh,
    /// (Batak) – Feline animal
    Singa,
    /// (Choctaw) – Serpentine rain spirit
    SintHolo,
    /// (Greek) – Human-bird hybrid
    Siren,
    /// (Slavic) – Demonic human-headed bird
    Sirin,
    /// (Akkadian) – Dragon with aquiline hind legs and feline forelegs
    Sirrush,
    /// (American Indian) – Two-headed sea serpent
    Sisiutl,
    /// (Paiute) – Red-haired giants
    SiTeCah,
    /// (Norse) – Freshwater spirit
    Sjora,
    /// (Norse) – Sea spirit
    Sjovaettir,
    /// (American Indian) – Animal-human shapeshifter
    SkinWalker,
    /// (Scandinavian) – Forest spirit
    Skogsra,
    /// (Norse) – Wolf that chases the Sun
    Skoll,
    /// (Chinook Jargon) – Hairy giant
    Skookum,
    /// (Medieval folklore) – Living skeletons
    Skeleton,
    /// (Slavic) – Flying imp
    Skrzak,
    /// (Polish) – Weather spirit
    SkyWomen,
    /// (Norse) – Eight-legged horse
    Sleipnir,
    /// (Irish and Scottish) – Restless ghost
    Sluagh,
    /// (Japanese) – Invisible spirit which pulls on sleeves
    SodehikiKozo,
    /// (Japanese) – Fiery ghost of an oil-stealing monk
    Sogenbi,
    /// (Japanese) – Ritual disciplinary demon
    Soragami,
    /// (Japanese) – Sound of trees being cut down, when later none seem to have been cut
    SorakiGaeshi,
    /// (Japanese) – Ghost with an abacus
    Sorobanbozu,
    /// (Japanese) – Fox spirit from Kyoto
    Sotangitsune,
    /// (Trinidad and Tobago) – Vampiric hag who takes the form of a fireball at night
    Soucouyant,
    /// (Cherokee) – Sharp-fingered hag
    Spearfinger,
    /// (Worldwide) – Terrifying ghost
    Spectre,
    /// (Greek) – Winged woman-headed lion
    Sphinx,
    /// (Romanian) – Little people
    Spiridus,
    /// Ghosts
    Spirit,
    /// (Cornish) – Guardians of graveyards and ruins
    Spriggan,
    /// (Medieval folklore) – little people, ghosts or elves
    Sprite,
    /// (American) – Ugly and lonely creature capable of evading capture by dissolving itself into a pool of tears
    Squonk,
    /// (Albanian) – Demonic dragon who guards a treasure
    Stihi,
    /// (Romanian) – Vampire
    Strigoi,
    /// (Roman) – Vampiric bird
    Strix,
    /// (Medieval Bestiaries) – Humanoid whose males have enormous feet, and females have tiny feet
    Struthopodes,
    /// (Slavic) – Vampiric undead
    Strzyga,
    /// (Slavic) – Malevolent mountain spirit
    Stuhac,
    /// (Greek) – Metallic bird
    StymphalianBird,
    /// (New Guinea) – Cannibalistic sorcerer
    Suangi,
    /// (Medieval folklore) – Female night-demon
    Succubus,
    /// (Slavic) – Fortune spirit
    Sudice,
    /// (Japanese) – Sand-throwing hag
    SunakakeBaba,
    /// (Japanese) – Small dog- or cat-like creature that rubs against a person's legs at night
    Sunekosuri,
    /// (Finnish) – Hellhound
    Surma,
    /// (Japanese) – Japanese version of the Chinese Vermillion Bird
    Suzaku,
    /// (Norse) – Unnatural strong horse, father of Sleipnir
    Svaoilfari,
    /// (Norse) – Cavern spirits; the Black Elves
    Svartalfar,
    /// (Ancient Egyptian) – Crocodile-leopard-hippopotamus hybrid
    Swallower,
    /// (Worldwide) – Swan-human shapeshifter
    SwanMaiden,
    /// (Alchemy) – Air elemental
    Sylph,
    /// (Medieval folklore) – Forest spirit
    Sylvan,
    /// (Medieval Bestiaries) – African giant
    Syrbotae,
    /// (Medieval Bestiaries) – Reptilian humanoid
    Syrictae,
    /// (Jewish) – Large land animal
    Tachash,
    /// (American Folklore),/// (Appalachia) – Powerful animal, that takes revenge on those who steal its tail
    Tailypo,
    /// (Japanese) – Tengu surrounded in demonic fire
    Taimatsumaru,
    /// (Persian) – Nature spirit
    Takam,
    /// (Japanese) – Female spirit which can stretch itself to peer into the second story of a building
    TakaOnna,
    /// (Greek) – Giant made of bronze
    Talos,
    /// (Scottish) – Shapeshifting water spirit
    Tangie,
    /// (Māori) – Water spirit
    Taniwha,
    /// (Japanese) – Unharvested persimmon which becomes a monster
    Tantankororin,
    /// (Japanese) – Shapeshifting raccoon dog
    Tanuki,
    /// (Mariana Islands) – Ancestral spirits
    TaotaoMona,
    /// (Chinese) – Greed spirit
    Taotie,
    /// (Mangaia) – Nature spirit
    Tapairu,
    /// (French) – Dragon with leonine, turtle, bear, and human attributes
    Tarasque,
    /// (Basque) – One-eyed giant
    Tartalo,
    /// (Christian) – Demonic punisher
    Tartaruchi,
    /// (Japanese) – Poltergeist that hits the tatami mats at night
    TatamiTataki,
    /// (Alpine Folklore) lizard-like creature, often described as having the face of a cat, with a serpent-like body which may be slender or stubby, with four short legs or two forelegs
    Tatzelwurm,
    /// Japanese dragon
    Tatsu,
    /// (Etruscan) – Fish-tailed bull
    Taurokampoi,
    /// (Trabzon) – Night-demon[citation needed]
    Tavara,
    /// (Guaraní) – Lizard with seven dog heads
    TejuJagua,
    /// (Mayan) – Bird
    Tecumbalam,
    /// (Japanese) – Anthropomorphic bird
    Tengu,
    /// (Japanese) – Angelic humanoid
    Tennin,
    /// (Japanese) – Ghost of a blind man, with his eyes on his hands
    TeNoMe,
    /// (Azerbaijani) – Azerbaijani mythical creature similar to the cyclops Polyphemus
    Tepegoz,
    /// (Jewish) – Lion-eagle-scorpion hybrid made from the blood of murder victims
    TerribleMonster,
    /// (Greek) – Gigantic fox
    TeumessianFox,
    /// (Medieval folklore) – Animal-headed humanoid
    Theriocephalus,
    /// (Asia and Africa) – Solar bird
    ThreeLeggedBird,
    /// (Native American) – Avian lightning bird spirit
    Thunderbird,
    /// (Norse mythology) – God of thunder and storm
    Thor,
    /// (Chinese) – Meteoric dog
    Tiangou,
    /// (Chinese) – Celestial dragon
    Tianlong,
    /// (Canarian) – Evil Dog
    Tibicena,
    /// (English) – Bog spirit
    TiddyMun,
    /// (Philippine) – Asian fairy bluebird
    Tigmamanukan,
    /// (Jewish) – Giant lion
    Tigris,
    /// (Philippine) – Anthropomorphic horse
    Tikbalang,
    /// (Zulu) – Little people and water spirit
    Tikoloshe,
    /// (Hindu) – Sea monster
    Timingila,
    /// (Māori) – Spirit that protects a specific place
    Tipua,
    /// (Greek) – Primeval god
    Titan,
    /// (Philippine) – Demons that are souls of dead unbaptized babies
    Tiyanak,
    /// (Inuit) – Sea serpent
    Tizheruk,
    /// (Tlaxcalan) – Shapeshifting vampire
    Tlahuelpuchi,
    /// (Japanese) – Spirit child carrying a block of tofu
    TofuKozo,
    /// (Japanese) – Ghost who lurks in grade school restroom stalls
    ToireNoHanakosan,
    /// (Scandinavian) – House spirit
    Tomte,
    /// (Slavic) – Water spirit
    Topielec,
    /// (Japanese) – Greed spirit
    Totetsu,
    /// (Malay) – Servant spirit
    Toyol,
    /// (Spanish and Portuguese) – Grotesque, mischievous little people
    Trasgo,
    /// (Chilota) – Fertility spirit
    Trauco,
    /// (Cantabrian) – Diminutive demon
    Trenti,
    /// Character in a story which exhibits a great degree of intellect or secret knowledge, and uses it to play tricks or otherwise disobey normal rules and conventional behaviour
    Trickster,
    /// (Hindu) – Demonic inhabitants of Tripura
    Tripurasura,
    /// (Greek) – Male human-fish hybrid
    Tritons,
    /// (Norse) – Nature spirit
    Troll,
    /// (Orkney and Shetland) – Little people and nature spirits
    Trow,
    /// (Abenaki) – Vampiric demon
    TsiNoo,
    /// (Japanese) – Shapeshifting, giant spider
    Tsuchigumo,
    /// (Japanese) – Plump snake-like creature
    Tsuchinoko,
    /// (Japanese) – Inanimate object that becomes animated after existing for 100 years
    Tsukumogami,
    /// (Cherokee) – Giant nature spirit
    TsulKalu,
    /// (Japanese) – Icicle woman
    TsuraraOnna,
    /// (Japanese) – Monster which drops or lowers a bucket from the top of a tree to catch people
    TsurubeOtoshi,
    /// (Slavic) – Evil shapeshifter
    TugarinZmeyevich,
    /// (Welsh) – Nature spirit
    TylwythTeg,
    /// (Inuit) – Animated construct
    Tupilaq,
    /// (Māori) – Pale spirit
    Turehu,
    /// (Swiss) – legendary figure who turns people into dogs
    Turst,
    /// (Hungarian) – Giant falcon that helped shape the origins of the Magyars
    Turul,
    /// (Heraldry) – Like a real tiger, but lacks stripes. It has the tufted tail of a lion and a thick mane along the neck like a horse
    Tyger,
    /// (Greek) – Winged, snake-legged giant
    Typhon,
    /// (Aztec) – Skeletal star spirit
    Tzitzimitl,
    /// (Japanese) – Ghosts of women who died in childbirth
    Ubume,
    ///(Manipuri mythology) – Semi human, semi hornbill creature
    UchekLangmeidong,
    /// (Japanese) – Horse's leg which dangles from a tree and kicks passersby
    UmaNoAshi,
    /// (Japanese) – Ghost of drowned priest
    Umibozu,
    /// (Japanese) – Female sea monster who steals fish
    UmiNyobo,
    /// (Worldwide) – Dead that behave as if alive
    Undead,
    /// (Native American) – Feline water spirit
    UnderwaterPanther,
    /// (Alchemy) – Water elemental
    Undine,
    /// (Lakota) – Dragon
    Unhcegila,
    /// (Medieval Bestiaries) – Horse-like creature with the legs of an antelope, the tail of a lion and a single magical healing horn.
    Unicorn,
    /// (Lakota) – Serpentine rain spirit
    Unktehi,
    /// (Lakota) – Reptilian water monster
    Unktehila,
    /// (Lithuanian) – River spirit
    Upinis,
    /// (Native American) – Hairy giant
    Urayuli,
    /// (Romanian) – Giant
    Urias,
    /// (Mesopotamian) – Lion-human hybrid guardian spirit
    Urmahlullu,
    /// (Japanese) – Bull-headed monster
    UshiOni,
    /// (Akkadian) – ″Underworld messenger spirit″
    Utukku,
    /// (Japanese) – Spirit that shouts to surprise people
    Uwan,
    /// (Latvian) – Spirit that misleads people
    Vadatajs,
    /// (Hindu) – Divine mounts
    Vahana,
    /// (Indian) – Deadly snake
    Vaibhavi,
    /// (Norse) – Female spirit that leads souls of dead warriors to Valhalla
    Valkyrie,
    /// (Romanian) – Female nature spirit
    Valva,
    /// (Danish) – Supernatural raven
    Valravn,
    /// (Slavic) – Reanimated corpse that feeds on blood
    Vampire,
    /// (Hindu) – Human-ape hybrid
    Vanara,
    /// (Romanian) – Female weather spirit
    Vantoase,
    /// (Hindu mythology) – Third Avatar of Vishnu in the form of a boar
    Varaha,
    /// (Romanian) – Vampire or werewolf
    Varcolac,
    /// (Scandinavian) – Ghostly double
    Vardoger,
    /// (Norse) – Hawk sitting between the eyes of an eagle in the crown of the World Tree Yggdrasil
    Vedrfolnir,
    /// (Latvian) – Ghost, shade, formed after a death of a human
    Veli,
    /// Chuvash dragon
    VeriSelen,
    /// (Hindu) – Corpses possessed by vampiric spirits
    Vetala,
    /// (Catalan) – Dragon with breasts and an eagle's beak
    Víbria,
    /// (German) – Gluttonous dog-cat-fox hybrid
    Vielfras,
    /// (Slavic) – Weather spirit
    Vila,
    /// (Latvian) – Animalistic, werewolf-like monster
    Vilkacis,
    /// (Colombian) – Handsome demon
    Virunas,
    /// (Mayan) – Mystical dragon
    VisionSerpent,
    /// (Norse) – Rooster that sits atop the tree
    Vídopnir,
    /// (Slavic) – Male water spirit
    Vodyanoy,
    /// (Greek) – Undead wolf-human hybrid
    Vrykolakas,
    /// (Norse) – Nature spirit
    Vaettir,
    /// (German) – Forest spirit
    Waldgeist,
    /// (Abenaki) – Water spirits
    WanaGamesAk,
    /// (Japanese) – Crocodilian water monster
    Wani,
    /// (Japanese) – Demon in the form of a burning human-headed ox cart
    Wanyudo,
    /// (Indonesian Muslim) – Egg-laying bird
    WarakNgendog,
    /// (English and Scandinavian O.N. vargr) – Giant, demonic wolf
    Warg,
    /// (Worldwide) – Male witch
    Warlock,
    /// (Abenaki) – Aurora spirits
    WassanMonGaneehlaAk,
    /// (Chinese) – Water spirit
    WaterMonkey,
    /// (Alchemy) – Water elemental
    WaterSprite,
    /// (Australia Aboriginal) – Goanna spirits
    WatiKutjara,
    /// (Abenaki) – Shapeshifting snail spirit
    WaWonDeeAMegw,
    /// (German) – Female spirit
    WeisseFrauen,
    /// (Mapuche) – Demon
    Wekufe,
    /// (Algonquian) – Anthropophagous spirit
    Wendigo,
    /// (Inuit) – Water spirit
    Wentshukumishiteu,
    /// (Worldwide) – Feline-human shapeshifter
    Werecat,
    /// (Africa) – Hyena-human shapeshifter
    Werehyena,
    /// (Worldwide) – Wolf-human shapeshifter
    Werewolf,
    /// (Worldwide) – Ghost of a murdered or mistreated woman
    WhiteLady,
    /// (Australian Aboriginal) – Giant frog-headed goanna with six legs
    Whowie,
    /// (European) – Hairy, bipedal, man-like creature
    WildMan,
    /// (Worldwide) – Spectral fire
    WillOTheWisp,
    /// (Scottish) – Malevolent spirit
    WirryCow,
    /// (Worldwide) – Person who practices magic
    Witch,
    /// (Dutch) – Female, ancestral spirit
    WitteWieven,
    /// (German) – Forest animal comprised from various animal parts,/// (similar to a Chimera)
    Wolpertinger,
    /// (Australia Aboriginal) – Weather spirit
    Wondjina,
    /// (Scottish) – Water spirit or ghostly apparition
    Wraith,
    /// (Scottish) – Wolf-headed humanoid spirit
    Wulver,
    /// (Chinese) – Beheaded ghost
    WuTouGui,
    /// English dragon
    Wyrm,
    /// (Germanic Heraldic) – Flying reptile, usually with two legs and two wings
    Wyvern,
    /// (Asturian) – Female water spirit
    Xana,
    /// (Greek)
    Xanthus,
    /// (Mayan) – Bird
    Xecotcovach,
    /// (Aztec) – Giant
    Xelhua,
    /// (mythology), (Chinese) – Ape or four-winged bird
    Xiao,
    /// (Chinese) – Headless giant
    XingTian,
    /// (Aztec) – Drought spirit
    Xiuhcoatl,
    /// (Albanian) – Elves
    Xhindi,
    /// (South America) – Sea monster
    Yacumama,
    /// (Indigenous people of the Amazon) – Mythical water people, with backwards heads and feet
    Yacuruna,
    /// (Japanese) – Malevolent, nocturnal spirit
    Yadokai,
    /// (Japanese) – Demon who rides through the night on a headless horse
    YagyoSan,
    /// (Buddhist, Hindu, and Jainism) – Male nature spirit
    Yaksha,
    /// (Keralite) – Vampire
    Yakshi,
    /// (Buddhist, Hindu, and Jainism) – Female nature spirit
    Yakshini,
    /// (Japanese) – Disease and misfortune spirit
    YakubyoGami,
    /// (Medieval Bestiaries) – Antelope- or goat-like animal with swiveling horns
    Yale,
    /// (Tamil) – Lion-like beast
    Yazhi,
    /// (English) – Nature spirit
    YalleryBrown,
    /// (Yama,/// (East Asia)) – Wrathful god
    Yama,
    /// (Japanese) – Echo spirit
    YamaBiko,
    /// (Japanese) – Savage, mountain-dwelling humanoid
    YamaBito,
    /// (Japanese) – Monkey-like mountain spirit
    YamaChichi,
    /// (Japanese) – Dog-like mountain spirit
    YamaInu,
    /// (Japanese) – Mountain giant
    YamaOtoko,
    /// (Japanese) – Gigantic, eight-headed serpent
    YamataNoOrochi,
    /// (Japanese) – Malevolent, mountain-dwelling hag
    YamaUba,
    /// (Japanese) – Hairy, one-eyed spirit
    YamaWaro,
    /// (Japanese) – Spirit which causes strange noises
    Yanari,
    /// (Chinese) – Animalistic demon or fallen gods
    Yaoguai,
    /// (Australian Aboriginal) – Diminutive, sucker-fingered vampire
    YaraMaYhaWho,
    /// (Japanese) – Three-legged crow of Amaterasu
    Yatagarasu,
    /// (Japanese) – Serpent spirits
    YatoNoKami,
    /// (English) – Headless dog
    YethHound,
    /// (Himalayan) – Mountain bigfoot
    Yeti,
    /// (Turkic) – Either a dragon or a giant
    Yilbegan,
    /// (Japanese) – Mountain dwelling spirit
    Yobuko,
    /// (Japanese) – Supernatural monster
    Yokai,
    /// (Japanese) – Underworld hag
    YomotsuShikome,
    /// Korean dragon
    Yong,
    /// (Japanese) – Fairy
    Yosei,
    /// (Japanese) – Mysterious bird that sings at night, sometimes indicating that the okuri-inu is near
    Yosuzume,
    /// (Chinese) – Wandering ghost
    YouHunYeGui,
    /// (Australian Aboriginal) – Nocturnal human-ape hybrid, also Yahoo
    Yowie,
    /// (Heraldic) – Boar-camel-ox-serpent hybrid
    Ypotryll,
    /// (Chinese) – Distressed ghost
    YuanGui,
    /// (Japanese) – Childlike snow spirit
    Yukinko,
    /// (Japanese) – Female snow spirit
    YukiOnna,
    /// (Japanese) – Ghost
    Yurei,
    /// (Tatar) – 100-year-old snake that transforms into a beautiful human
    Yuxa,
    /// (Persian) – Dragon
    Zahhak,
    /// (Baltic) – Serpentine fertility spirit
    Zaltys,
    /// (Jewish) – Giant
    Zamzummim,
    /// (Albanian) – Mountain fairy who bless warriors
    ZanaEMalit,
    /// (Romanian) – Nature spirit
    Zână,
    /// (Japanese) – House spirit
    ZashikiWarashi,
    /// (Romanian) – Wolf-headed dragon
    Zburator,
    /// (Slavic mythology) – Disembodied, heroic spirit
    Zduhac,
    /// (Greek) – God of lightning and storms
    Zeus,
    /// (Japanese) – Rain-making dragon
    ZennyoRyuo,
    /// (Slavic) – Glowing bird
    ZharPtitsa,
    /// (Chinese) – Pig-headed dragon
    Zhulong,
    /// (Chinese) – Fire elemental bird
    ZhuQue,
    /// (Lithuanian) – Forest spirit in the form of a glowing skeleton
    Ziburinis,
    /// (Tatar) – Flying chicken-legged reptile
    Zilant,
    /// (West Africa) – Water spirits
    Zin,
    /// (Jewish) – Giant bird
    Ziz,
    /// (Slovenia) – White golden-horned deer
    Zlatorog,
    /// (Romanian folklore) – Giant with a habit of kidnapping young girls
    Zmeu,
    /// Slavic dragon
    Zmiy,
    /// (Vodou/Worldwide) – Re-animated corpse
    Zombie,
    /// (Japanese) – Animated clock
    Zorigami,
    /// (Japanese) – Tutelary spirit
    Zuijin,
    /// (Japanese) – Faceless ghost
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
            Legendary::ABaoAQu =>  v = String::from("(Malay) – Entity that lives in the Tower of Victory in Chitor."),
            Legendary::Aatxe =>  v = String::from("(Basque) – Bull spirit."),
            Legendary::Abaasy =>  v = String::from("(Yakuts) – Iron-toothed demons."),
            Legendary::Abada =>  v = String::from("(African) – Unicorn that inhabits the African Congo."),
            Legendary::Abaia =>  v = String::from("(Tatar) – Forest spirit. (Melanesia) – Huge magical eel."),
            Legendary::Abarimon =>  v = String::from("(Medieval Bestiaries) – Savage humanoid with backward feet."),
            Legendary::Abath =>  v = String::from("(Malay) – One-horned animal."),
            Legendary::AburaSumashi =>  v = String::from("(Japanese) – Creature from a mountain pass in Kumamoto Prefecture."),
            Legendary::Acephali =>  v = String::from("(Greek) – Headless humanoids."),
            Legendary::Acheri =>  v = String::from("(Mitologia Hindu) – Disease-bringing ghost."),
            Legendary::Achlis =>  v = String::from("(Roman) – Curious elk."),
            Legendary::AdarLlwchGwin =>  v = String::from("(Welsh) – Giant birds that understand human languages."),
            Legendary::Adaro =>  v = String::from("(Solomon Islands) – Malevolent merfolk."),
            Legendary::Adhene =>  v = String::from("(Manx) – Nature spirit."),
            Legendary::Adlet =>  v = String::from("(Inuit) – Vampiric dog-human hybrid"),
            Legendary::Adroanzi =>  v = String::from("(Lugbara) – Nature spirit."),
            Legendary::Adze =>  v = String::from("(Ewe people) – African vampiric-forest being."),
            Legendary::Aerico =>  v = String::from("(Greek) – Disease demon."),
            Legendary::AEsir =>  v = String::from("(Norse) – Norse deities."),
            Legendary::Afanc =>  v = String::from("(Welsh) – Lake monster (exact lake varies by story)."),
            Legendary::Agni =>  v = String::from("(Hindu) – God of fire and sacrifices."),
            Legendary::Agathodaemon =>  v = String::from("(Greek) – Spirit of vinefields and grainfields."),
            Legendary::Agloolik =>  v = String::from("(Inuit) – Ice spirit that aids hunters and fishermen."),
            Legendary::Agogwe =>  v = String::from("(East Africa) – Small, ape-like humanoid."),
            Legendary::Ahkiyyini =>  v = String::from("(Inuit) – Animated skeleton that causes shipwrecks."),
            Legendary::Ahuizotl =>  v = String::from("(Aztec) – Anthropophagous dog-monkey hybrid."),
            Legendary::Ahura =>  v = String::from("(Zoroastrianism) – Zoroastrian spirits."),
            Legendary::Aigamuxa =>  v = String::from("(Khoikhoi) – Anthropophagous humanoid with eyes in its instep."),
            Legendary::Aigikampoi =>  v = String::from("(Etruscan) – Fish-tailed goat."),
            Legendary::Airavata =>  v = String::from("(Hindu) – Divine elephant."),
            Legendary::Aitu =>  v = String::from("(Polynesian) – Malevolent spirits or demons."),
            Legendary::Aitvaras =>  v = String::from("(Lithuanian) – Household spirit."),
            Legendary::Ajatar =>  v = String::from("(Finnish) – Dragon/snake female spirit, is said to spread diseases"),
            Legendary::Akateko =>  v = String::from("(Japanese) – Tree-dwelling monster."),
            Legendary::Akhlut =>  v = String::from("(Inuit) – Orca-wolf shapeshifter."),
            Legendary::Akka =>  v = String::from("(Finnish) – Female spirits or minor goddesses."),
            Legendary::Akki =>  v = String::from("(Japanese) – Large, grotesque humanoid."),
            Legendary::Akkorokamui =>  v = String::from("(Ainu) – Sea monster."),
            Legendary::Akuma =>  v = String::from("(Japanese) – Evil spirit or devil"),
            Legendary::Akupara =>  v = String::from("(Hindu) – Giant turtle that supports the world."),
            Legendary::AkurojinNoHi =>  v = String::from("(Japanese) – Ghostly flame which causes disease."),
            Legendary::Al =>  v = String::from("(Armenian and Persian) – Spirit that steals unborn babies and livers from pregnant women."),
            Legendary::Ala =>  v = String::from("(Slavic) – Bad weather demon."),
            Legendary::Alal =>  v = String::from("(Chaldean) – Queen of the full moon."),
            Legendary::Alan =>  v = String::from("(Philippine) – Winged humanoid that steals reproductive waste to make children."),
            Legendary::Alce =>  v = String::from("(Heraldic) – Wingless griffin."),
            Legendary::Aleya =>  v = String::from("(Bengali) – Spirit of a dead fisherman."),
            Legendary::Alicanto =>  v = String::from("(Chilean) – Bird that eats gold and silver."),
            Legendary::Alicorn =>  v = String::from("(Bestiario medieval) – Winged unicorn."),
            Legendary::Alkonost =>  v = String::from("(Slavic) – Angelic bird with human head and breasts."),
            Legendary::Allocamelus =>  v = String::from("(Heraldic) – Ass-camel hybrid."),
            Legendary::Almas =>  v = String::from("(Mongolian) – Savage humanoid."),
            Legendary::AlMiRaj =>  v = String::from("(Islamic) – One-horned rabbit."),
            Legendary::Aloja =>  v = String::from("(Catalan) – Female water spirit."),
            Legendary::AlomBagWinnosis =>  v = String::from("(Abenaki) – Little people and tricksters."),
            Legendary::Alp =>  v = String::from("(German) – Male night-demon."),
            Legendary::Alphyn =>  v = String::from("(Heraldic) – Lion-like creature, sometimes with dragon or goat forelegs."),
            Legendary::AlpLuachra =>  v = String::from("(Irish) – Parasitic fairy."),
            Legendary::AlRakim =>  v = String::from("(Islamic) – Guard dog of the Seven Sleepers."),
            Legendary::Alseid =>  v = String::from("(Greek) – Grove nymph."),
            Legendary::Alu =>  v = String::from("(Assyrian) – Leprous demon."),
            Legendary::Alux =>  v = String::from("(Mayan) – Little people."),
            Legendary::Amaburakosagi =>  v = String::from("(Japanese) – Ritual disciplinary demon from Shikoku."),
            Legendary::Amala =>  v = String::from("(Tsimshian) – Giant who holds up the world."),
            Legendary::Amamehagi =>  v = String::from("(Japanese) – Ritual disciplinary demon from Hokuriku."),
            Legendary::Amanojaku =>  v = String::from("(Japanese) – Small demon."),
            Legendary::Amarok =>  v = String::from("(Inuit) – Giant wolf."),
            Legendary::Amarum =>  v = String::from("(Quechua) – Water boa spirit."),
            Legendary::AmazakeBabaa =>  v = String::from("(Japanese) – Disease-causing hag."),
            Legendary::Amemasu =>  v = String::from("(Ainu) – Lake monster."),
            Legendary::Ammit =>  v = String::from("(Ancient Egyptian) – Female demon who was part lion, hippopotamus and crocodile and devoured the souls of the wicked."),
            Legendary::Amoronagu =>  v = String::from("(Japanese) – Tennyo from the island of Amami Ōshima."),
            Legendary::Amphiptere =>  v = String::from("(Heraldic) – Winged serpent."),
            Legendary::Amphisbaena =>  v = String::from("(Greek) – Serpent with a head at each end."),
            Legendary::Anak =>  v = String::from("(Jewish) – Giant."),
            Legendary::Androsphinx =>  v = String::from("(Ancient Egyptian) – Human-headed sphinx."),
            Legendary::Angel =>  v = String::from("(mainly Christian, Jewish, Islamic traditions) – Divine beings of Heaven who act as mediators between God and humans; the counterparts of Demons."),
            Legendary::Anqa =>  v = String::from("(Arabian) – Legendary Huge Satanic Eagle with Human Face. sometimes can resurrect herself like phoenix did."),
            Legendary::AniHyuntikwalaski =>  v = String::from("(Cherokee) – Lightning spirit."),
            Legendary::Ankou =>  v = String::from("(French) – Skeletal grave watcher with a lantern and scythe."),
            Legendary::Anmo =>  v = String::from("(Japanese) – Ritual disciplinary demon from Iwate Prefecture."),
            Legendary::Antaeus =>  v = String::from("(Greek) – Giant who was extremely strong as long as he remained in contact with the ground."),
            Legendary::Anubis =>  v = String::from("(Ancient Egyptian) – God of the Underworld"),
            Legendary::AnteroVipunen =>  v = String::from("(Finnish) – Subterranean giant."),
            Legendary::Anzu =>  v = String::from("(Sumerian) – Divine storm bird"),
            Legendary::AoAo =>  v = String::from("(Guaraní) – Anthropophagous peccary or sheep."),
            Legendary::Aobozu =>  v = String::from("(Japanese) – Blue monk who kidnaps children."),
            Legendary::Apkallu =>  v = String::from("(Sumerian) – Fish-human hybrid that attends the god Enki."),
            Legendary::Apsaras =>  v = String::from("(Buddhist and Hindu) – Female cloud spirit."),
            Legendary::Aqrabuamelu =>  v = String::from("(Akkadian) – Human-scorpion hybrid."),
            Legendary::ArdatLili =>  v = String::from("(Akkadian) – Disease demon."),
            Legendary::ArgusPanoptes =>  v = String::from("(Greek) – Hundred-eyed giant."),
            Legendary::ArikuraNoBaba =>  v = String::from("(Japanese) – Old woman with magical powers."),
            Legendary::Arimaspi =>  v = String::from("(Greek) – One-eyed humanoid."),
            Legendary::Arion =>  v = String::from("(Greek) – Swift green-maned talking horse."),
            Legendary::ArkanSonney =>  v = String::from("(Manx) – Fairy hedgehog."),
            Legendary::Asag =>  v = String::from("(Sumerian) – Hideous rock demon."),
            Legendary::Asakku =>  v = String::from("(Sumerian) – Demon."),
            Legendary::Asanbosam =>  v = String::from("(West Africa) – Iron-toothed vampire."),
            Legendary::Asena =>  v = String::from("(Turkic) – Blue-maned wolf."),
            Legendary::ASeneeKiWakw =>  v = String::from("(Abenaki) – Stone giant."),
            Legendary::AshiMagari =>  v = String::from("(Japanese) – Invisible tendril that impedes movement."),
            Legendary::Asiman =>  v = String::from("(Dahomey) – Vampiric possession spirit."),
            Legendary::Askefrue =>  v = String::from("(Germanic) – Female tree spirit."),
            Legendary::AskWeeDaEed =>  v = String::from("(Abenaki) – Fire elemental and spectral fire."),
            Legendary::Asobibi =>  v = String::from("(Japanese) – Spectral fire from Kōchi Prefecture."),
            Legendary::Aspidochelone =>  v = String::from("(Medieval Bestiaries) – Island-sized whale or sea turtle."),
            Legendary::Asrai =>  v = String::from("(English) – Water spirit."),
            Legendary::Astomi =>  v = String::from("(Greek) – Humanoid sustained by pleasant smells instead of food."),
            Legendary::Asura =>  v = String::from("(Hindu) – Hindu malevolent divinities."),
            Legendary::Aswang =>  v = String::from("(Philippine) – Carrion-eating humanoid."),
            Legendary::Atomy =>  v = String::from("(English) – Surprisingly small creature."),
            Legendary::AtoOiKozo =>  v = String::from("(Japanese) – Invisible spirit that follows people."),
            Legendary::Atshen =>  v = String::from("(Inuit) – Anthropophagous spirit."),
            Legendary::Auloniad =>  v = String::from("(Greek) – Pasture nymph."),
            Legendary::Avalerion =>  v = String::from("(Medieval Bestiary) – King of the birds."),
            Legendary::AwaHonDo =>  v = String::from("(Abenaki) – Insect spirit."),
            Legendary::Axex =>  v = String::from("(Ancient Egyptian) – Falcon-lion hybrid."),
            Legendary::Ayakashi =>  v = String::from("(Japanese) – Sea serpent that travels over boats in an arc while dripping oil."),
            Legendary::AyakashiNoAyashibi =>  v = String::from("(Japanese) – Spectral fire from Ishikawa Prefecture."),
            Legendary::Aziza =>  v = String::from("(Dahomey) – Little people that help hunters."),
            Legendary::Azukiarai =>  v = String::from("(Japanese) – Spirit that washes azuki beans along riversides."),
            Legendary::Azukitogi =>  v = String::from("(Japanese) – Spirit that washes azuki beans along riversides."),
            Legendary::Azukibabaa =>  v = String::from("(Japanese) – Bean-grinding hag who devours people."),
            Legendary::Ba =>  v = String::from("(Egyptian) – Soul of the deceased, depicted as a bird or a human-headed bird"),
            Legendary::BabaYaga =>  v = String::from("(Slavic) – Forest spirit and hag"),
            Legendary::Baccoo =>  v = String::from("(Guyanese/Surinamese) – Malevolent little people"),
            Legendary::Badalisc =>  v = String::from("(Italian) – Goat-like creature from the southern central Alps"),
            Legendary::Bagiennik =>  v = String::from("(Slavic) – Malevolent water spirit"),
            Legendary::Bahamut =>  v = String::from("(Arabian) – Giant fish"),
            Legendary::BaiZe =>  v = String::from("(Chinese) – Talking beast which handed down knowledge on harmful spirits"),
            Legendary::BaJiaoGui =>  v = String::from("(Chinese) – Banana tree spirit"),
            Legendary::Bak =>  v = String::from("(Indian) - Assamese shape-shifting aqueous creature"),
            Legendary::BakeKujira =>  v = String::from("(Japanese) – Ghostly whale skeleton that drifts along the coastline of Shimane Prefecture"),
            Legendary::Bakeneko =>  v = String::from("(Japanese) – Magical cat"),
            Legendary::Bakezori =>  v = String::from("(Japanese) – Animated straw sandal"),
            Legendary::Bakhtak =>  v = String::from("(Iranian) – Night demon"),
            Legendary::Baku =>  v = String::from("(Japanese) – Dream-devouring, tapir-like creature"),
            Legendary::Bakunawa =>  v = String::from("(Philippine) – Sea serpent that causes eclipses"),
            Legendary::Balaur =>  v = String::from("(Romanian) – Multi-headed dragon"),
            Legendary::Baloz =>  v = String::from("(Albanian) – Sea monster"),
            Legendary::Bannik =>  v = String::from("(Slavic) – Bathhouse spirit"),
            Legendary::Banshee =>  v = String::from("(Irish) – Screaming death spirit"),
            Legendary::BaobhanSith =>  v = String::from("(Celtic Mythology) – Beautiful vampiric seductresses who prey on young travelers"),
            Legendary::Barbegazi =>  v = String::from("(Swiss) – Dwarf with giant, snowshoe-like feet"),
            Legendary::Bardha =>  v = String::from("(Albanian) – Mountain spirit"),
            Legendary::Bardi =>  v = String::from("(Trabzon) – Shapechanging death spirit"),
            Legendary::Barghest =>  v = String::from("Yorkshire black dog"),
            Legendary::BarJuchne =>  v = String::from("(Jewish) – Gigantic bird"),
            Legendary::BarnacleGeese =>  v = String::from("(Medieval folklore) – Geese which hatch from barnacles"),
            Legendary::Barong =>  v = String::from("(Balinese) – Tutelary spirit"),
            Legendary::Basajaun =>  v = String::from("(Basque) – Ancestral, megalith-building race"),
            Legendary::BasCelik =>  v = String::from("(Serbian) – Powerful, evil winged man whose soul is not held by his body and can be subdued only by causing him to suffer dehydration"),
            Legendary::Bashe =>  v = String::from("(Chinese) – Elephant-swallowing serpent"),
            Legendary::BasiliscoChilote =>  v = String::from("(Chilota) – Chicken-serpent hybrid"),
            Legendary::Basilisk =>  v = String::from("(Italian) – Multi-limbed, venomous lizard"),
            Legendary::Bathala =>  v = String::from("(Philippine) – Primordial god of creation"),
            Legendary::Batibat =>  v = String::from("(Philippine) – Female night-demon"),
            Legendary::Batsu =>  v = String::from("(Chinese) – Drought spirit"),
            Legendary::Baubas =>  v = String::from("(Lithuanian) – Malevolent spirit"),
            Legendary::Baykok =>  v = String::from("(Ojibwa) – Flying skeleton"),
            Legendary::BeastOfBrayRoad =>  v = String::from("(American Folklore) – Werewolf"),
            Legendary::BeanNighe =>  v = String::from("(Irish) – Death spirit; a type of Banshee/Bean Sídhe)"),
            Legendary::Behemoth =>  v = String::from("(Jewish) – Massive beast, possibly like a dinosaur"),
            Legendary::Bendigeidfran =>  v = String::from("(Welsh) – Giant king"),
            Legendary::Bennu =>  v = String::from("(Egyptian) – Heron-like, regenerative bird, equivalent to (or inspiration for) the Phoenix"),
            Legendary::Berehynia =>  v = String::from("(Slavic) – Water spirit"),
            Legendary::Bergrisar =>  v = String::from("(Norse) – Mountain giants who live alongside the Hrimthursar (lit. 'Rime-Giants') in Jotunheim"),
            Legendary::Bergsra =>  v = String::from("(Norse) – Mountain spirit"),
            Legendary::BestialBeast =>  v = String::from("(Brazilian) – Centauroid specter"),
            Legendary::BetobetoSan =>  v = String::from("(Japanese) – Invisible spirit which follows people at night, making the sound of footsteps"),
            Legendary::Bhuta =>  v = String::from("(Buddhist and Hindu) – Ghost of someone killed by execution or suicide"),
            Legendary::BiBlouk =>  v = String::from("(Khoikhoi) – Female, cannibalistic, partially invisible monster"),
            Legendary::Bies =>  v = String::from("(Slavic) – Demon"),
            Legendary::Bigfoot =>  v = String::from("(American Folklore) – Forest-dwelling hominid cryptid."),
            Legendary::Binbogami =>  v = String::from("(Japanese) – Spirit of poverty"),
            Legendary::BishopFish =>  v = String::from("(Medieval Bestiaries) – Fish-like humanoid"),
            Legendary::BiwaBokuboku =>  v = String::from("(Japanese) – Animated biwa"),
            Legendary::BlackAnnis =>  v = String::from("(English) – Blue-faced hag"),
            Legendary::BlackDog =>  v = String::from("(British) – Canine death spirit"),
            Legendary::BlackShuck =>  v = String::from("Norfolk, Essex, and Suffolk black dog"),
            Legendary::Blafard =>  v = String::from("Imaginary creature from the early United States of America"),
            Legendary::Blemmyae =>  v = String::from("(Medieval Bestiary) – Headless humanoid with face in torso"),
            Legendary::BloodyBones =>  v = String::from("(Irish) – Water bogeyman"),
            Legendary::Bludnik =>  v = String::from("(Slavic) – Mischievous gnome"),
            Legendary::BlueCrow =>  v = String::from("(Brazilian) – Giant amazonian bird"),
            Legendary::Bluecap =>  v = String::from("(English) – Mine-dwelling fairy"),
            Legendary::Bodach =>  v = String::from("(Scottish) – Malevolent spirit"),
            Legendary::Bogeyman =>  v = String::from("(English) – Malevolent spirit"),
            Legendary::Boggart =>  v = String::from("(English) – Malevolent household spirit"),
            Legendary::Boginki =>  v = String::from("(Slavic) – Nature spirit"),
            Legendary::Bogle =>  v = String::from("(Scottish) – Malevolent spirit"),
            Legendary::BoiTata =>  v = String::from("(Brazilian) – Giant snake"),
            Legendary::Bolla =>  v = String::from("(Albanian) – Dragon"),
            Legendary::Bonnacon =>  v = String::from("(Medieval Bestiaries) – Bull-horse hybrid with flaming dung"),
            Legendary::BooHag =>  v = String::from("(American Folklore) – Vampire-like creature that steals energy from sleeping victims"),
            Legendary::Boobrie =>  v = String::from("(Scottish) – Roaring water bird"),
            Legendary::Bozaloshtsh =>  v = String::from("(Slavic) – Death spirit"),
            Legendary::Brag =>  v = String::from("(English) – Malevolent water horse"),
            Legendary::Brownie =>  v = String::from("(English and Scottish) – Benevolent household spirit"),
            Legendary::Broxa =>  v = String::from("(Jewish) – Nocturnal bird that drains goats of their milk"),
            Legendary::Bucca =>  v = String::from("(Cornish) – Male sea-spirit, a merman, that inhabited mines and coastal communities as a hobgoblin during storms"),
            Legendary::Bokkenrijders =>  v = String::from("(Dutch) – Ghosts/devils riding flying goats; co-opted by bandits to instil fear during raids"),
            Legendary::Bugbear =>  v = String::from("(English) – Bearlike goblin"),
            Legendary::Buggane =>  v = String::from("(Manx) – Ogre-like humanoid"),
            Legendary::BugulNoz =>  v = String::from("(Celtic) – Extremely ugly, but kind, forest spirit"),
            Legendary::Bukavac =>  v = String::from("(Serbia) – Six-legged lake monster"),
            Legendary::Bunyip =>  v = String::from("(Australian Aboriginal) – Horse-walrus hybrid lake monster"),
            Legendary::BunnyMan =>  v = String::from("(American Folklore) West Virginia Urban Legend – Spirit/Maniac that wears a bunny costume and wields an axe"),
            Legendary::BushDaiDai =>  v = String::from("(Guyanese) – Spirit that seduces and kills men"),
            Legendary::Byangoma =>  v = String::from("(Bengali) – Fortune-telling birds"),
            Legendary::Bysen =>  v = String::from("(Scandinavian) – Diminutive forest spirit"),
            Legendary::Cabeiri =>  v = String::from("(Greek) – Smith and wine spirit"),
            Legendary::Cacus =>  v = String::from("(Roman) – Fire-breathing giant"),
            Legendary::Cadejo =>  v = String::from("(Central America) – Cow-sized dog-goat hybrid"),
            Legendary::Cailleach =>  v = String::from("(Scottish) – Divine creator and weather deity hag"),
            Legendary::Caipora =>  v = String::from("(Tupi) – Fox-human hybrid and nature spirit"),
            Legendary::Caladrius =>  v = String::from("(Medieval Bestiary) – White bird that can foretell if a sick person will recover or die"),
            Legendary::Calingi =>  v = String::from("(Medieval Bestiary) – Humanoid with an eight-year lifespan"),
            Legendary::Callitrix =>  v = String::from("(Medieval Bestiary) – Apes who always bear twins, one the mother loves, the other it hates"),
            Legendary::CalydonianBoar =>  v = String::from("(Greek) – Giant, chthonic boar"),
            Legendary::Calygreyhound =>  v = String::from("(Heraldic) – Wildcat-deer/antelope-eagle-ox-lion hybrid :>"),
            Legendary::Camahueto =>  v = String::from("(Chilota) – One-horned calf"),
            Legendary::Cambion =>  v = String::from("(Medieval folklore) – Offspring of a human and an incubus or succubus"),
            Legendary::Campe =>  v = String::from("(Greek) – Dragon-human-scorpion hybrid"),
            Legendary::Camulatz =>  v = String::from("(Mayan) – Bird that ate the heads of the first men"),
            Legendary::Candileja =>  v = String::from("(Colombian) – Spectral, fiery hag"),
            Legendary::Canaima =>  v = String::from("(Guyanese) – Were-jaguar"),
            Legendary::Canotila =>  v = String::from("(Lakota) – Little people and tree spirits"),
            Legendary::Caoineag =>  v = String::from("(Scottish) – Death spirit (a particular type of Banshee/Bean Sídhe)"),
            Legendary::Chapa =>  v = String::from("(Lakota) – Beaver spirit"),
            Legendary::Chareng =>  v = String::from("///(Manipuri)-Semi-hornbill, semi-human creature"),
            Legendary::Capcaun =>  v = String::from("(Romanian) – Large, monstrous humanoid"),
            Legendary::Carbuncle =>  v = String::from("(Latin America) – Small creature with a jewel on its head"),
            Legendary::Catoblepas =>  v = String::from("(Medieval Bestiary) – Scaled buffalo-hog hybrid"),
            Legendary::CatSidhe =>  v = String::from("(Scottish) – Fairy cat"),
            Legendary::Ceasg =>  v = String::from("(Scottish) — Benevolent Scottish mermaids"),
            Legendary::CeffylDwr =>  v = String::from("(Welsh) – Malevolent water horse"),
            Legendary::Centaur =>  v = String::from("(Greek) – Human-horse hybrid"),
            Legendary::Centicore =>  v = String::from("(Indian) – Horse-Antelope-Lion-Bear hybrid"),
            Legendary::Cerastes =>  v = String::from("(Greek) – Extremely flexible, horned snake"),
            Legendary::Cerberus =>  v = String::from("(Greek) – Three-headed dog that guards the entrance to the underworld"),
            Legendary::Cercopes =>  v = String::from("(Greek) – Mischievous forest spirit"),
            Legendary::Cericopithicus =>  v = String::from("(Medieval Bestiary) – Apes who always bear twins, one the mother loves, the other it hates"),
            Legendary::CeryneianHind =>  v = String::from("(Greek) – Hind with golden antlers and bronze or brass hooves"),
            Legendary::Cetan =>  v = String::from("(Lakota) – Hawk spirit"),
            Legendary::Cetus =>  v = String::from("(Greek) The Cetus was variously described as a sea monster or sea serpent. Other versions describe Cetus as a monster with the head of a boar or a greyhound and the body of a whale or dolphin, and a divided, fan-like tail. Cetus was said to be a colossal beast the size of a ship, its skull alone measuring 40 feet (12.2 meters) in length, its spines being a cubit in thickness, and its skeleton taller at the shoulder than an elephant."),
            Legendary::Chakora =>  v = String::from("(Hindu) – Lunar bird"),
            Legendary::Chalkydri =>  v = String::from("(Apocryphal writings) – Angelic birds"),
            Legendary::Chamrosh =>  v = String::from("(Persian) – Dog-bird hybrid"),
            Legendary::Chaneque =>  v = String::from("(Aztec) – Little people and nature spirits"),
            Legendary::Changeling =>  v = String::from("(European) – Humanoid child (fairy, elf, troll, etc.) substituted for a kidnapped human child"),
            Legendary::Charybdis =>  v = String::from("(Greek) – Sea monster in the form of a giant mouth"),
            Legendary::Chenoo =>  v = String::from("(Mi'kmaq/Algonquian) – Giant, human-eating ice monsters; former humans who either committed terrible crime(s) or were possessed by evil spirits, turning their hearts to ice"),
            Legendary::Chepi =>  v = String::from("(Narragansett) – Ancestral spirit that instructs tribe members"),
            Legendary::Cherufe =>  v = String::from("(Mapuche) – Volcano-dwelling monster"),
            Legendary::ChevalMallet =>  v = String::from("(French) – Evil horse who runs away with travelers"),
            Legendary::ChevalGauvin =>  v = String::from("(French) – Evil horse who drowns riders, similar to kelpie"),
            Legendary::Chibaiskweda =>  v = String::from("(Abenaki) – Ghost of an improperly buried person"),
            Legendary::Chichevache =>  v = String::from("Human-faced cow that feeds on good women"),
            Legendary::Chickcharney =>  v = String::from("(Bahamian) – Bird-mammal hybrid"),
            Legendary::Chimaera =>  v = String::from("(Greek) – Lion-goat-snake hybrid"),
            Legendary::Chindi =>  v = String::from("(Navajo) – Vengeful ghost that causes dust devils"),
            Legendary::Chinthe =>  v = String::from("(Burmese) – Temple-guarding feline, similar to Chinese Shi and Japanese Shisa"),
            Legendary::Chitauli =>  v = String::from("(Zulu) – Human-lizard hybrid"),
            Legendary::Chochinobake =>  v = String::from("(Japanese) – Animated paper lantern"),
            Legendary::Chol =>  v = String::from("(Biblical mythology) – Regenerative bird"),
            Legendary::Chollima =>  v = String::from("(Korean) – Supernaturally fast horse"),
            Legendary::Chonchon =>  v = String::from("(Mapuche) – Disembodied, flying head"),
            Legendary::Choorile =>  v = String::from("(Guyanese) – Ghost of a woman that died in childbirth"),
            Legendary::Chromandi =>  v = String::from("(Medieval Bestiary) – Hairy savage with dog teeth"),
            Legendary::Chrysaor =>  v = String::from("(Greek) – The giant son of the gorgon Medusa."),
            Legendary::Chrysomallus =>  v = String::from("(Greek mythology) – Golden winged ram"),
            Legendary::Chukwa =>  v = String::from("(Hindu) – Giant turtle that supports the world"),
            Legendary::Chupacabra =>  v = String::from("(Latin America) – Cryptid beast named for its habit of sucking the blood of livestock"),
            Legendary::Churel =>  v = String::from("(Hindu) – Vampiric, female ghost"),
            Legendary::Ciguapa =>  v = String::from("(Dominican Republic) – Malevolent seductress"),
            Legendary::Cihuateteo =>  v = String::from("(Aztec) – Ghost of women that died in childbirth"),
            Legendary::Cikavac =>  v = String::from("(Serbian) – Bird that serves its owner"),
            Legendary::CinnamonBird =>  v = String::from("(Medieval Bestiaries) – Giant bird that makes its nest out of cinnamon"),
            Legendary::Cipactli =>  v = String::from("(Aztec) – Sea monster, crocodile-fish hybrid"),
            Legendary::CireinCroin =>  v = String::from("(Scottish) – Sea serpent"),
            Legendary::Coblynau =>  v = String::from("(Welsh) – Little people and mine spirits"),
            Legendary::Cockatrice =>  v = String::from("(Medieval Bestiaries) – Chicken-lizard hybrid"),
            Legendary::Cofgod =>  v = String::from("(English) – Cove god"),
            Legendary::ColchisBull =>  v = String::from("(Greek) – Bronze-hoofed bulls"),
            Legendary::ColoColo =>  v = String::from("(Mapuche) – Rat-bird hybrid that can shapeshift into a serpent"),
            Legendary::CorycianNymphs =>  v = String::from("(Greek) – Nymph of the Corycian Cave"),
            Legendary::CretanBull =>  v = String::from("(Greek) – Monstrous bull"),
            Legendary::Crinaeae =>  v = String::from("(Greek) – Fountain nymph"),
            Legendary::Criosphinx =>  v = String::from("(Ancient Egypt) – Ram-headed sphinx"),
            Legendary::Crocotta =>  v = String::from("(Medieval Bestiaries) – Monstrous dog-wolf"),
            Legendary::TheCuBird =>  v = String::from("(Mexican) – El Pájaro Cu; a bird."),
            Legendary::Cuco =>  v = String::from("(Latin America) – Bogeyman"),
            Legendary::Cucuy =>  v = String::from("(Latin America) – Malevolent spirit"),
            Legendary::Cuegle =>  v = String::from("(Cantabrian) – Monstrous, three-armed humanoid"),
            Legendary::Cuelebre =>  v = String::from("(Asturian and Cantabrian) – Dragon"),
            Legendary::Curupira =>  v = String::from("(Tupi) – Nature spirit"),
            Legendary::CuSith =>  v = String::from("(Scottish) – Gigantic fairy dog"),
            Legendary::CwnAnnwn =>  v = String::from("(Welsh) – Underworld hunting dog"),
            Legendary::Cyclops =>  v = String::from("(Greek) – One-eyed giant"),
            Legendary::Cyhyraeth =>  v = String::from("(Welsh) – Death spirit"),
            Legendary::Cynocephalus =>  v = String::from("(Medieval Bestiaries) – Dog-headed humanoid"),
            Legendary::Dactyl =>  v = String::from("(Greek) – Little people and smith and healing spirits"),
            Legendary::Daemon =>  v = String::from("(Greek) – Incorporeal spirit"),
            Legendary::Dahu =>  v = String::from("(France, Switzerland and the north of Italy) – Similar to a deer or ibex; legs on one side of its body are shorter than on the other side"),
            Legendary::Daidarabotchi =>  v = String::from("(Japanese) – Giant responsible for creating many geographical features in Japan"),
            Legendary::Daitengu =>  v = String::from("(Japanese) – Most powerful class of tengu, each of whom lives on a separate mountain"),
            Legendary::Daitya =>  v = String::from("(Hindu) – Giant"),
            Legendary::Danava =>  v = String::from("(Hindu) – Water demon"),
            Legendary::Daphnaie =>  v = String::from("(Greek) – Laurel tree nymph"),
            Legendary::DatsueBa =>  v = String::from("(Japanese) – Old woman who steals clothes from the souls of the dead"),
            Legendary::DeadSeaApes =>  v = String::from("(Islamic) – Human tribe turned into apes for ignoring Moses' message"),
            Legendary::DedMoroz =>  v = String::from("(Russia) – A winter spirit who delivers gifts to children on New Year's Eve"),
            Legendary::DeerWoman =>  v = String::from("(Native American) – Human-deer hybrid"),
            Legendary::Deity =>  v = String::from("(Global) – Preternatural or supernatural possibly immortal being"),
            Legendary::Demigod =>  v = String::from("(Global) – Half human, half god"),
            Legendary::Dhampir =>  v = String::from("(Balkans) – Human/vampire hybrid"),
            Legendary::DiaoSiGui =>  v = String::from("(Chinese) – Hanged ghost"),
            Legendary::Dilong =>  v = String::from("(Chinese) – Earth dragon"),
            Legendary::Dip =>  v = String::from("(Catalan) – Demonic and vampiric dog"),
            Legendary::DiPenates =>  v = String::from("(Roman) – House spirit"),
            Legendary::Dipsa =>  v = String::from("(Medieval Bestiaries) – Extremely venomous snake"),
            Legendary::Dirawong =>  v = String::from("(Australian Aboriginal) – Goanna spirit"),
            Legendary::DiSmaUndarJordi =>  v = String::from("(Gotland) – Little people and nature spirits"),
            Legendary::Diwata =>  v = String::from("(Philippine) – Tree spirit"),
            Legendary::Djall =>  v = String::from("(Albanian) – Devil"),
            Legendary::DobharChu =>  v = String::from("(Irish) – King otter"),
            Legendary::DoGakwHoWad =>  v = String::from("(Abenaki) – Little people"),
            Legendary::Dokkaebi =>  v = String::from("(Korean) – Grotesque, horned humanoids"),
            Legendary::Dokkalfar =>  v = String::from("(Norse) – Male ancestral spirits; the Dark Elves"),
            Legendary::Dola =>  v = String::from("(Slavic) – Tutelary and fate spirit"),
            Legendary::Domovoi =>  v = String::from("(Slavic) – House spirit"),
            Legendary::Doppelganger =>  v = String::from("(German) – Ghostly double"),
            Legendary::Drac =>  v = String::from("(French) – Winged sea serpent"),
            Legendary::Drakon =>  v = String::from("(Greek) – Greek dragons"),
            Legendary::Drakaina =>  v = String::from("(Greek) – Dragons depicted with female characteristics"),
            Legendary::Dragon =>  v = String::from("(normally) winged reptiles"),
            Legendary::DragonTurtle =>  v = String::from("(Chinese) – Giant turtle with dragon-like head"),
            Legendary::Drangue =>  v = String::from("(Albanian) – Semi-human winged warriors"),
            Legendary::Draugr =>  v = String::from("(Norse) – Undead"),
            Legendary::Drekavac =>  v = String::from("(Slavic) – Restless ghost of an unbaptised child"),
            Legendary::DropBear =>  v = String::from("(Australian) – Large carnivorous koala that hunts by dropping on its prey from trees"),
            Legendary::Drow =>  v = String::from("(Scottish) – Cavern spirit"),
            Legendary::Drude =>  v = String::from("(German) – Possessing demon"),
            Legendary::Druk =>  v = String::from("(Bhutanese) – Dragon"),
            Legendary::Dryad =>  v = String::from("(Greek) – Tree nymph"),
            Legendary::Duende =>  v = String::from("(Spanish and Portuguese) – Little people and forest spirits"),
            Legendary::Duergar =>  v = String::from("(English) – Malevolent little people"),
            Legendary::Dullahan =>  v = String::from("(Irish) – Headless death spirit"),
            Legendary::Duwende =>  v = String::from("(Philippine) – Little people, some are house spirits, others nature spirits"),
            Legendary::Dvergr =>  v = String::from("(Norse) – Subterranean little people smiths"),
            Legendary::Dvorovoi =>  v = String::from("(Slavic) – Courtyard spirit"),
            Legendary::Dwarf =>  v = String::from("(Germanic) – Little people nature spirits"),
            Legendary::Dybbuk =>  v = String::from("(sometimes the soul of a wicked deceased) that possesses the living"),
            Legendary::DzeeDzeeBonDa =>  v = String::from("(Abenaki) – Hideous monster"),
            Legendary::Dzunukwa =>  v = String::from("(Kwakwaka'wakw) – Child-eating hag"),
            Legendary::EasterBunny =>  v = String::from("(Christianity) – Anthropomorphic lagomorph."),
            Legendary::EasterBilby =>  v = String::from("(Australian) – Anthropomorphic bilby."),
            Legendary::EachUisge =>  v = String::from("(Scottish) – Malevolent water horse"),
            Legendary::EagleSpirit =>  v = String::from("(Many cultures worldwide) – Leadership or guidance totem"),
            Legendary::EbuGogo =>  v = String::from("(Flores) – Diminutive humanoids, possibly inspired by Homo floresiensis"),
            Legendary::Echidna =>  v = String::from("(Greek)"),
            Legendary::Echeneis =>  v = String::from("(Medieval Bestiaries) – Remora, said to attach to ships to slow them down"),
            Legendary::Edimmu =>  v = String::from("(Sumerian) – Ghosts of those not buried properly"),
            Legendary::Egbere =>  v = String::from("(Yoruba) – Humanoid that carries a magical mat"),
            Legendary::Eikthyrnir =>  v = String::from("(Norse)"),
            Legendary::Einherjar =>  v = String::from("(Norse) – Spirits of brave warriors"),
            Legendary::Ekek =>  v = String::from("(Philippine) – Flesh-eating, winged humanoids"),
            Legendary::ElbowWitch =>  v = String::from("(Ojibwa) – Hags with awls in their elbows"),
            Legendary::Eldjotnar =>  v = String::from("(Norse) – Fire Giants who reside in Muspelheim, with Surtr as their leader"),
            Legendary::Eleionomae =>  v = String::from("(Greek) – Marsh nymph"),
            Legendary::Elemental =>  v = String::from("(Alchemy) – Personification of one of the Classical elements"),
            Legendary::Elepaio =>  v = String::from("(Hawaiian) – Monarch flycatcher spirit that guides canoe-builders to the proper trees"),
            Legendary::Elf =>  v = String::from("(Germanic) – Nature and fertility spirit"),
            Legendary::Eloko =>  v = String::from("(Central Africa) – Little people and malevolent nature spirits"),
            Legendary::Emere =>  v = String::from("(Yoruba) – Child that can move back and forth between the material world and the afterlife at will"),
            Legendary::Emim =>  v = String::from("(Jewish) – Giant"),
            Legendary::Empusa =>  v = String::from("(Greek) – Female demon that waylays travelers and seduces and kills men"),
            Legendary::Encantado =>  v = String::from("(Brazilian) – Dolphin-human shapeshifter"),
            Legendary::EnchantedMoor =>  v = String::from("(Portuguese) – Enchanted princesses"),
            Legendary::Enfield =>  v = String::from("(Heraldic) – Fox-greyhound-lion-wolf-eagle hybrid"),
            Legendary::Engkanto =>  v = String::from("(Philippine) – Neutral nature spirit"),
            Legendary::Enko =>  v = String::from("(Japanese) – Kappa of Shikoku and western Honshū"),
            Legendary::Ent =>  v = String::from("(worldwide/fantasy) -Living tree that is said to live for years"),
            Legendary::Epimeliad =>  v = String::from("(Greek) – Apple tree nymph"),
            Legendary::Erchitu =>  v = String::from("(Sardinia) – Ox-human, wereox"),
            Legendary::ErGui =>  v = String::from("(Chinese) – Hungry ghost"),
            Legendary::Erinyes =>  v = String::from("(Greek) – Winged spirits of vengeance or justice, also known as Furies"),
            Legendary::Erlking =>  v = String::from("(German) – Death spirit"),
            Legendary::ErymanthianBoar =>  v = String::from("(Greek) – Giant boar"),
            Legendary::EthiopianPegasus =>  v = String::from("(Medieval Bestiaries) – Horned, winged horse"),
            Legendary::Etiainen =>  v = String::from("(Finnish mythology) – Spirit being of a living person"),
            Legendary::Ettin =>  v = String::from("(English) – Three-headed giant"),
            Legendary::Eurynomos =>  v = String::from("(Greek) – Blue-black, carrion-eater in the underworld"),
            Legendary::Ewah =>  v = String::from("(Cherokee) – Human-cougar hybrid"),
            Legendary::Eerinis =>  v = String::from("(Lithuanian) – Lake spirit"),
            Legendary::Fachen =>  v = String::from("(Irish and Scottish) – Monster with half a body"),
            Legendary::Fafnir =>  v = String::from("(Germanic mythology) – Dwarf who was cursed and turned into a dragon. He was later slain by Sigurd in the Saga of Nibelung."),
            Legendary::Fairy =>  v = String::from("(many cultures worldwide, esp. Germanic mythology/folklore) – Nature spirits"),
            Legendary::Familiar =>  v = String::from("(English) – Animal servant"),
            Legendary::FarDarrig =>  v = String::from("(Irish) – Little people that constantly play pranks"),
            Legendary::Farfadet =>  v = String::from("(some half-meter tall), wrinkled, and brown-skinned helpful sprites."),
            Legendary::Fates =>  v = String::from("(Greek) – Three time-controlling sisters"),
            Legendary::Faun =>  v = String::from("(Roman) – Human-goat hybrid nature spirit"),
            Legendary::FearGorta =>  v = String::from("(Irish) – Hunger ghost"),
            Legendary::FeatheredSerpent =>  v = String::from("Mesoamerican dragon"),
            Legendary::FeiLian =>  v = String::from("(Chinese) – Chinese wind god"),
            Legendary::Fenghuang =>  v = String::from("(Chinese) – Chinese Phoenix, female in marriage symbol"),
            Legendary::Fenodyree =>  v = String::from("(Manx) – House spirit"),
            Legendary::Fenrir =>  v = String::from("(Norse) – Gigantic, ravenous wolf"),
            Legendary::Fetch =>  v = String::from("(Irish) – Double or doppelgänger"),
            Legendary::Fext =>  v = String::from("(Slavic) – Undead"),
            Legendary::Finfolk =>  v = String::from("(Orkney) – Fish-human hybrid that kidnaps humans for servants"),
            Legendary::FirBolg =>  v = String::from("(Irish) – Ancestral race"),
            Legendary::FireBird =>  v = String::from("(Many cultures worldwide) – Regenerative solar bird"),
            Legendary::Firedrake =>  v = String::from("(Germanic) – Dragon"),
            Legendary::FishMan =>  v = String::from("(Cantabrian) – Amphibious, scaled humanoid"),
            Legendary::FlatwoodsMonster =>  v = String::from("(West Virginia) – Alien, humanoid"),
            Legendary::Fomorian =>  v = String::from("(Irish) – Goat-headed giant"),
            Legendary::ForestBull =>  v = String::from("(Medieval Bestiaries) – Giant horned red cattle"),
            Legendary::Freybug =>  v = String::from("// Norfolk black dog"),
            Legendary::Fuath =>  v = String::from("(Celtic) – Malevolent water spirit"),
            Legendary::Fucanglong =>  v = String::from("(Chinese) – Underworld dragon"),
            Legendary::Funayurei =>  v = String::from("(Japanese) – Ghosts of people who drowned at sea"),
            Legendary::FuruUtsubo =>  v = String::from("(Japanese) – Animated jar"),
            Legendary::FutakuchiOnna =>  v = String::from("(Japanese) – Woman with a second mouth on the back of her head"),
            Legendary::Fylgja =>  v = String::from("(Scandinavian) – Animal familiar"),
            Legendary::Gaasyendietha =>  v = String::from("(Seneca) – Dragon"),
            Legendary::Gagana =>  v = String::from("(Russian) – Iron-beaked bird with copper talons"),
            Legendary::Gaki =>  v = String::from("(Japanese) – Ghosts of especially greedy people"),
            Legendary::Gallu =>  v = String::from("(Mesopotamian) – Underworld demons"),
            Legendary::Galtzagorriak =>  v = String::from("(Basque) – Small demonic servants"),
            Legendary::Gamayun =>  v = String::from("(Russian) – Prophetic human-headed bird"),
            Legendary::Gana =>  v = String::from("(Hindu) – Attendants of Shiva"),
            Legendary::Gancanagh =>  v = String::from("(Irish) – Male fairy that seduces human women"),
            Legendary::Gandabherunda =>  v = String::from("(Hindu) – Double-headed bird"),
            Legendary::Gandharva =>  v = String::from("(Hindu) – Male nature spirits, often depicted as part human, part animal"),
            Legendary::Gargouille =>  v = String::from("(French) – Water dragon"),
            Legendary::Garkain =>  v = String::from("(Australian Aboriginal) – A flying humanoid who envelops his victims"),
            Legendary::Garmr =>  v = String::from("(Norse) – Giant, ravenous hound"),
            Legendary::Garuda =>  v = String::from("(Hindu) – Human-eagle hybrid"),
            Legendary::Gashadokuro =>  v = String::from("(Japanese) – Giant malevolent skeletons"),
            Legendary::Gaueko =>  v = String::from("(Basque) – Wolf capable of walking upright"),
            Legendary::Geb =>  v = String::from("(Egyptian) – God of the Earth, married to Nut"),
            Legendary::Ged =>  v = String::from("(Heraldic) – The fish pike"),
            Legendary::Gegenees =>  v = String::from("(Greek) – Six-armed giant"),
            Legendary::GeniusLoci =>  v = String::from("(Roman) – Spirit that protects a specific place"),
            Legendary::German =>  v = String::from("(Slavic) – Male spirit associated with bringing rain and hail"),
            Legendary::Geryon =>  v = String::from("(Greek) – Three-headed six-armed giant with three torsos and (in some sources) six legs"),
            Legendary::GhillieDhu =>  v = String::from("(Scottish) – Tree guardian"),
            Legendary::Ghost =>  v = String::from("Disembodied spirits of those that have died"),
            Legendary::Ghoul =>  v = String::from("(Arabian) – Cannibalistic shapeshifting desert genie often classified as undead."),
            Legendary::Giant =>  v = String::from("(Worldwide) – Immensely large and strong humanoids"),
            Legendary::GiantAnimal =>  v = String::from("(Worldwide) – Unusually large beasts"),
            Legendary::GichiAnamiEBizhiw =>  v = String::from("(Ojibwa) – Bison-snake-bird-cougar hybrid water spirit"),
            Legendary::Gidim =>  v = String::from("(Sumerian) – Ghost"),
            Legendary::Gigantes =>  v = String::from("(Greek) – Race of giants that fought the Olympian gods, sometimes depicted with snake-legs"),
            Legendary::Gigelorum =>  v = String::from("(Scottish) – Smallest animal"),
            Legendary::Girtablilu =>  v = String::from("(Akkadian) – Human-scorpion hybrid"),
            Legendary::Gjenganger =>  v = String::from("(Scandinavian) – Corporeal ghost"),
            Legendary::Glaistig =>  v = String::from("(Scottish) – Human-goat hybrid"),
            Legendary::Glashtyn =>  v = String::from("(Manx) – Malevolent water horse"),
            Legendary::Gnome =>  v = String::from("(Alchemy) – Diminutive Earth elemental"),
            Legendary::Goblin =>  v = String::from("(Medieval) – Grotesque, mischievous little people"),
            Legendary::Gog =>  v = String::from("(English) – Giant protector of London"),
            Legendary::GoldDiggingAnt =>  v = String::from("(Medieval Bestiaries) – Dog-sized ant that digs for gold in sandy areas"),
            Legendary::Golem =>  v = String::from("(Jewish) – Animated construct"),
            Legendary::Gorgades =>  v = String::from("(Medieval Bestiary) – Hairy humanoid"),
            Legendary::Gorgon =>  v = String::from("(Greek) – Fanged, snake-haired humanoids that turn anyone who sees them into stone"),
            Legendary::Goryo =>  v = String::from("(Japanese) – Vengeful ghosts, usually of martyrs"),
            Legendary::Grassman =>  v = String::from("(Ohio, USA) – Ape-like cryptid"),
            Legendary::Gremlin =>  v = String::from("(Folklore) – Creatures that sabotage airplanes"),
            Legendary::Griffin =>  v = String::from("(Heraldic) – Lion-eagle hybrid"),
            Legendary::Grigori =>  v = String::from("(Christian, Jewish, and Islamic mythology) – Fallen angels, father of Nephilim"),
            Legendary::Grim =>  v = String::from("(English and Scandinavian) – Tutelary spirits of churches"),
            Legendary::GrimReaper =>  v = String::from("(Worldwide) – Death angel often thought to be God's/Satan's assistant"),
            Legendary::Grindylow =>  v = String::from("(English) – Malevolent water spirit"),
            Legendary::Gualichu =>  v = String::from("(Mapuche) – Malevolent spirit"),
            Legendary::GuardianAngel =>  v = String::from("(Christian, Jewish, and Islamic belief) – Subclassification of angels that guard and protect a specific person or living being"),
            Legendary::GudElim =>  v = String::from("(Akkadian) – Human-bull hybrid"),
            Legendary::Guhin =>  v = String::from("(Japanese) – Anthropomorphic bird"),
            Legendary::GuiPo =>  v = String::from("(Chinese) – Ghost that manifests as an old woman"),
            Legendary::GuiShu =>  v = String::from("(Chinese) – Ghostly tree that confuses travelers by moving"),
            Legendary::Gulon =>  v = String::from("(Germanic) – Gluttonous dog-cat-fox hybrid"),
            Legendary::Gumiho =>  v = String::from("(Korean mythology) – Demonic fox with thousands of tails believed to possess an army of spirits and magic in its tails"),
            Legendary::Gurangatch =>  v = String::from("(Australian Aboriginal) - An enormous reptile-fish whose movements carved out the landscape south of the Blue Mountains"),
            Legendary::Gurumapa =>  v = String::from("(Nepalese) – Child-eating demon"),
            Legendary::Gwyllgi =>  v = String::from("(Welsh) – Black dog"),
            Legendary::Gwyllion =>  v = String::from("(Welsh) – Malevolent spirit"),
            Legendary::Gyascutus =>  v = String::from("(American folklore) – Four-legged herbivore"),
            Legendary::Gytrash =>  v = String::from("(Lincolnshire and Yorkshire) – Black dog"),
            Legendary::Gyuki =>  v = String::from("(Japanese) – Bull-headed monster"),
            Legendary::Habrok =>  v = String::from("(Norse) – listed as the 'best' hawk"),
            Legendary::Hadhayosh =>  v = String::from("(Persian) – gigantic land animal"),
            Legendary::Hades =>  v = String::from("(Greek) – Ruler of the Underworld"),
            Legendary::Haetae =>  v = String::from("(Korean) – dog-lion hybrid"),
            Legendary::Hag =>  v = String::from("(Many cultures worldwide) – wise old woman who is usually a malevolent spirit or a disguised goddess"),
            Legendary::Haietlik =>  v = String::from("(Nuu-chah-nulth) – water serpent"),
            Legendary::HaiUri =>  v = String::from("(Khoikhoi) – male cannibalistic partially invisible monster"),
            Legendary::Hakutaku =>  v = String::from("(Japanese) – talking beast which handed down knowledge on harmful spirits"),
            Legendary::Hakuturi =>  v = String::from("(Māori) – nature guardian"),
            Legendary::HalfElf =>  v = String::from("(Norse) – human-elf hybrid"),
            Legendary::Haltija =>  v = String::from("(Finnish) – spirit that protects a specific place"),
            Legendary::Hamadryad =>  v = String::from("(Greek) – oak tree nymph"),
            Legendary::Hamingja =>  v = String::from("(Scandinavian) – personal protection spirit"),
            Legendary::Hamsa =>  v = String::from("(Buddhist, Hindu and Jainism) – mystic bird"),
            Legendary::HanauEpe =>  v = String::from("(Rapa Nui) – long-eared humanoid"),
            Legendary::HantuAir =>  v = String::from("(Malay) – shapeshifting water spirit"),
            Legendary::HantuDemon =>  v = String::from("(Philippine) – demon"),
            Legendary::HantuRaya =>  v = String::from("(Malay) – demonic servant"),
            Legendary::Harionago =>  v = String::from("(Japanese) – humanoid female with barbed, prehensile hair"),
            Legendary::Harpy =>  v = String::from("(Greek) – birdlike human-headed death spirit"),
            Legendary::Haugbui =>  v = String::from("(Norse) – undead being who cannot leave its burial mound"),
            Legendary::Havsrå =>  v = String::from("(Norse) – saltwater spirit"),
            Legendary::Helloi =>  v = String::from("(Manipuri mythology) – celestial maidens, daughters of the Sky God Soraren"),
            Legendary::HeadlessHorseman =>  v = String::from("(European) – humanoid spirit who haunts or kills"),
            Legendary::HeadlessMule =>  v = String::from("(Brazilian) – fire-spewing, headless, spectral mule"),
            Legendary::Hecatonchires =>  v = String::from("(Greek) – primordial giants with 100 hands and fifty heads"),
            Legendary::Heikegani =>  v = String::from("(Japanese) – crabs with human-faced shells, the spirits of warriors killed in the Battle of Dan-no-ura"),
            Legendary::Heinzelmannchen =>  v = String::from("(German) – household spirit"),
            Legendary::Helead =>  v = String::from("(Greek) – fen nymph"),
            Legendary::Hellhound =>  v = String::from("(Many cultures worldwide) – underworld dog"),
            Legendary::Heracles =>  v = String::from("(Greek) – gatekeeper of Olympus"),
            Legendary::Hercinia =>  v = String::from("(Medieval Bestiaries) – glowing bird"),
            Legendary::Herensuge =>  v = String::from("(Basque) – dragon"),
            Legendary::Hesperides =>  v = String::from("(Greek) – nymph daughters of Atlas"),
            Legendary::Hidebehind =>  v = String::from("(United States) – nocturnal forest creature"),
            Legendary::Hiderigami =>  v = String::from("(Japanese) – drought spirit"),
            Legendary::Hieracosphinx =>  v = String::from("(Ancient Egypt) – falcon-headed sphinx"),
            Legendary::Hihi =>  v = String::from("(Japanese) – baboon monster"),
            Legendary::Hiisi =>  v = String::from("(Finnish) – nature guardian"),
            Legendary::Hippalectryon =>  v = String::from("(Greek)"),
            Legendary::Hippocamp =>  v = String::from("(Etruscan, Greek and Phoenician) – horse-fish hybrid"),
            Legendary::Hippogriff =>  v = String::from("(Medieval Bestiaries) – hybrid of a griffin and horse; a lion-eagle-horse hybrid"),
            Legendary::Hippopodes =>  v = String::from("(Medieval Bestiary) – horse-hoofed humanoid"),
            Legendary::Hircocervus =>  v = String::from("(Medieval Bestiary) – deer-goat hybrid"),
            Legendary::Hitodama =>  v = String::from("(Japanese) – ghosts of the newly dead, which take the form of fireballs"),
            Legendary::HitotsumeKozo =>  v = String::from("(Japanese) – one-eyed childlike spirit"),
            Legendary::Hob =>  v = String::from("(English) – house spirit"),
            Legendary::Hobbididance =>  v = String::from("(English) – malevolent spirit"),
            Legendary::Hobgoblin =>  v = String::from("(Medieval) – friendly or amusing goblin"),
            Legendary::Hodag =>  v = String::from("(Native American) – frog-mammoth-lizard hybrid"),
            Legendary::Hokhokw =>  v = String::from("(Kwakiutl) – bird"),
            Legendary::Hoko =>  v = String::from("(Japanese) – dog-like Chinese tree spirit"),
            Legendary::Homa =>  v = String::from("(Persian) – eagle-lion hybrid, similar to a griffin"),
            Legendary::HombreCaiman =>  v = String::from("(Colombian) – human-alligator hybrid"),
            Legendary::HombreGato =>  v = String::from("(Latin America) – human-cat hybrid"),
            Legendary::Homunculus =>  v = String::from("(Alchemy) – small animated construct"),
            Legendary::Hoo =>  v = String::from("(Japanese) – rooster-swallow-fowl-snake-goose-tortoise-stag-fish hybrid"),
            Legendary::Hoopoe =>  v = String::from("near passerine bird common to Africa and Eurasia that features in many mythologies in those continents"),
            Legendary::HoopSnake =>  v = String::from("snake which rolls by taking its tail in its mouth"),
            Legendary::HornedSerpent =>  v = String::from("(Native American) – serpentine rain spirit"),
            Legendary::Hotoke =>  v = String::from("(Japanese) – deceased person"),
            Legendary::Houri =>  v = String::from("(Islamic) – heavenly beings"),
            Legendary::Hraesvelg =>  v = String::from("(Norse) – giant, who in eagle form, creates the wind by beating his wings"),
            Legendary::Hrímþursar =>  v = String::from("(Norse) – frost giants who are the main inhabitants of either Jotunheim or Niflheim"),
            Legendary::Huaychivo =>  v = String::from("(Mayan) – human-deer hybrid"),
            Legendary::HuginnAndMuninn =>  v = String::from("(Norse) – pair of ravens associated with the Norse god Odin whose names mean Thought and Memory."),
            Legendary::Huldufolk =>  v = String::from("(Icelandic/Faroese) – secret mound/rock dwelling elves"),
            Legendary::Hulder =>  v = String::from("(Scandinavian) – forest spirit"),
            Legendary::HuliJing =>  v = String::from("(Chinese) – nine-tailed fox spirit"),
            Legendary::Huma =>  v = String::from("(Persian) – regenerative fire bird"),
            Legendary::Humbaba =>  v = String::from("(Akkadian) – lion-faced giant"),
            Legendary::Hundun =>  v = String::from("(Chinese) – chaos spirit"),
            Legendary::Hupia =>  v = String::from("(Taíno) – nocturnal ghost"),
            Legendary::Hyakume =>  v = String::from("(Japanese) – hundred-eyes creature"),
            Legendary::Hydra =>  v = String::from("(Greek) – multi-headed water serpent/dragon"),
            Legendary::Hydros =>  v = String::from("(Medieval Bestiary) – snake whose poison causes the victim to swell up"),
            Legendary::Hydrus =>  v = String::from("(Medieval Bestiary) – snake from the Nile River that would kill crocodiles from the inside"),
            Legendary::Hyosube =>  v = String::from("(Japanese) – hair-covered kappa"),
            Legendary::Hypnalis =>  v = String::from("(Medieval Bestiary) – snake that kills its victims in their sleep"),
            Legendary::Hudhud =>  v = String::from("(mythology) – Hoopoe"),
            Legendary::Ishigaq =>  v = String::from("(Inuit) – Little people"),
            Legendary::IslandSatyr =>  v = String::from("(Medieval Bestiaries) – Savage human-goat hybrid from a remote island chain"),
            Legendary::Isonade =>  v = String::from("(Japanese) – Shark-like sea monster"),
            Legendary::IttanMomen =>  v = String::from("(Japanese) – Ghostly aerial phenomenon that attacks people"),
            Legendary::IwanaBozu =>  v = String::from("(Japanese) – Char which appeared as a Buddhist monk"),
            Legendary::Jackalope =>  v = String::from("(American) – Rabbit with antlers"),
            Legendary::JackInIrons =>  v = String::from("(English) – Malevolent giant"),
            Legendary::JackOLantern =>  v = String::from("(Medieval folklore) – Vegetal lantern"),
            Legendary::Jaculus =>  v = String::from("(Medieval Bestiaries) – Winged serpent or small dragon"),
            Legendary::Jasconius =>  v = String::from("(Medieval folklore) – Island-sized fish"),
            Legendary::JasyJaterei =>  v = String::from("(Guaraní) – Nature guardian and bogeyman"),
            Legendary::Jatayu =>  v = String::from("(Hindu mythology) – Vulture demigod"),
            Legendary::Jaud =>  v = String::from("(Slavic) – Vampirised premature baby"),
            Legendary::Jenglot =>  v = String::from("(Java) – Vampiric little people"),
            Legendary::Jengu =>  v = String::from("(Sawa) – Water spirit"),
            Legendary::Jentil =>  v = String::from("(Basque) – Megalith-building giant"),
            Legendary::Jenu =>  v = String::from("(Mi'kmaq) – Anthropophagous giant"),
            Legendary::Jerff =>  v = String::from("(Swedish) – Gluttonous dog-cat-fox hybrid"),
            Legendary::JerseyDevil =>  v = String::from("(American) – Demonic dragon or flying demon who was given birth to by an American living in New Jersey"),
            Legendary::Jian =>  v = String::from("(Chinese) – One-eyed, one-winged bird who requires a mate for survival"),
            Legendary::Jiangshi =>  v = String::from("(Chinese) – Life-draining, reanimated corpse"),
            Legendary::Jiaolong =>  v = String::from("(Chinese) – Dragon"),
            Legendary::Jibakurei =>  v = String::from("(Japanese) – Spirit that protects a specific place"),
            Legendary::Jievaras =>  v = String::from("(Lithuanian) – House spirit"),
            Legendary::Jikininki =>  v = String::from("(Japanese) – Corpse-eating ghost"),
            Legendary::Jinn =>  v = String::from("(Arabian, Islamic) – Spiritual creatures; genii"),
            Legendary::JipijkaM =>  v = String::from("(Mi'kmaq) – Underwater horned snake; lives in lakes and eats humans"),
            Legendary::Jiufeng =>  v = String::from("(Chinese) – Nine-headed bird worshiped by ancient natives in Hubei Province."),
            Legendary::JiuTouNiao =>  v = String::from("(Chinese) – Nine-headed, demonic bird"),
            Legendary::Jogah =>  v = String::from("(Iroquois) – Little people nature spirit"),
            Legendary::Jormungandr =>  v = String::from("(Norse) – Sea serpent"),
            Legendary::Jorogumo =>  v = String::from("(Japanese) – Spider woman"),
            Legendary::Jotai =>  v = String::from("(Japanese) – Animated folding screen cloth"),
            Legendary::Jotunn =>  v = String::from("(Norse) – Gigantic nature spirits"),
            Legendary::Jujak =>  v = String::from("(Korean) – Bird"),
            Legendary::Jumbee =>  v = String::from("(Guyanese) – Malevolent spirit"),
            Legendary::Kabouter =>  v = String::from("(Dutch) – Little people that live underground, in mushrooms, or as house spirits"),
            Legendary::Kachina =>  v = String::from("(Hopi and Puebloan) – Nature spirit"),
            Legendary::Kahaku =>  v = String::from("(Japanese) – Little people and water spirits"),
            Legendary::Kajsa =>  v = String::from("(Scandinavian) – Wind spirit"),
            Legendary::Kalakeyas =>  v = String::from("(Hindu) – Descendants of Kala"),
            Legendary::Kallikantzaroi =>  v = String::from("(Greek) – Grotesque, malevolent spirit"),
            Legendary::Kamaitachi =>  v = String::from("(Japanese) – Wind spirit"),
            Legendary::Kamatayan =>  v = String::from("(Philippine) – Philippine counterpart of Death"),
            Legendary::Kami =>  v = String::from("(Japanese) – Nature spirit"),
            Legendary::Kamikiri =>  v = String::from("(Japanese) – Hair-cutting spirit"),
            Legendary::KanbariNyudo =>  v = String::from("(Japanese) – Bathroom spirit"),
            Legendary::KanglaSha =>  v = String::from("(Manipuri mythology) – Great Dragon in the Kangla Palace"),
            Legendary::Kanbo =>  v = String::from("(Japanese) – Drought spirit"),
            Legendary::Kanedama =>  v = String::from("(Japanese) – Money spirit"),
            Legendary::Kappa =>  v = String::from("(Japanese) – Little people and water spirit"),
            Legendary::Kapre =>  v = String::from("(Philippine) – Malevolent tree spirit"),
            Legendary::Karakoncolos =>  v = String::from("(Bulgarian and Turkish), also in Bosnia and Herzegovina and Serbia known as Karanđoloz – Troublesome spirit"),
            Legendary::Karakura =>  v = String::from("(Turkish) – Male night-demon"),
            Legendary::KarasuTengu =>  v = String::from("(Japanese) – Tengu with a bird's bill"),
            Legendary::Karkadann =>  v = String::from("(Persian) – One-horned giant animal"),
            Legendary::Karkinos =>  v = String::from("(Greek) – Giant crab"),
            Legendary::Karura =>  v = String::from("(Japanese) – Eagle-human hybrid"),
            Legendary::Karzelek =>  v = String::from("(Polish) – Little people and mine spirits"),
            Legendary::KasaObake =>  v = String::from("(Japanese) – Animated parasol"),
            Legendary::Kasha =>  v = String::from("(Japanese) – Cat-like demon which descends from the sky and carries away corpses"),
            Legendary::Kashanbo =>  v = String::from("(Japanese) – Kappa who climb into the mountains for the winter"),
            Legendary::KatawaGuruma =>  v = String::from("(Japanese) – Woman riding on a flaming wheel"),
            Legendary::KatsuraOtoko =>  v = String::from("(Japanese) – Handsome man from the moon"),
            Legendary::Katallan =>  v = String::from("(Albanian) – Man-eating giant"),
            Legendary::Kaukas =>  v = String::from("(Lithuanian) – Nature spirit"),
            Legendary::KawaUso =>  v = String::from("(Japanese) – Supernatural river otter"),
            Legendary::KawaZaru =>  v = String::from("(Japanese) – Smelly, cowardly water spirit"),
            Legendary::KeLets =>  v = String::from("(Chukchi mythology) – Ogre or evil spirit"),
            Legendary::Keelut =>  v = String::from("(Inuit) – Hairless dog"),
            Legendary::KeeWakw =>  v = String::from("(Abenaki) – Half-human half-animal cannibalistic giant"),
            Legendary::Kekkai =>  v = String::from("(Japanese) – Amorphous afterbirth spirit"),
            Legendary::Kelpie =>  v = String::from("(Irish and Scottish) – Malevolent water horse"),
            Legendary::Ker =>  v = String::from("(Greek) – Female death spirit"),
            Legendary::KesaranPasaran =>  v = String::from("(Japanese) – Mysterious, white, fluffy creature"),
            Legendary::Keukegen =>  v = String::from("(Japanese) – Disease spirit"),
            Legendary::Keythong =>  v = String::from("(Heraldic) – Wingless griffin"),
            Legendary::Khyah =>  v = String::from("(Nepalese) – Fat, hairy ape-like creature"),
            Legendary::Kigatilik =>  v = String::from("(Inuit) – Night-demon"),
            Legendary::Kholomodumo =>  v = String::from("(Sotho) – Gluttonous monster that was one of the first beasts of creation"),
            Legendary::Kijimunaa =>  v = String::from("(Japanese) – Tree sprite from Okinawa"),
            Legendary::Kijo =>  v = String::from("(Japanese) – She-devil"),
            Legendary::Kikimora =>  v = String::from("(Slavic) – Female house spirit"),
            Legendary::Killmoulis =>  v = String::from("(English and Scottish) – Ugly, mischievous mill spirit"),
            Legendary::Kinnara =>  v = String::from("(Hindu) – Human-bird hybrid"),
            Legendary::KinU =>  v = String::from("(Japanese) – Bird"),
            Legendary::Kirin =>  v = String::from("(Japanese) – Japanese Unicorn"),
            Legendary::Kishi =>  v = String::from("(Angola) – Malevolent, two-faced seducer"),
            Legendary::Kitsune =>  v = String::from("(Japanese) – Fox spirit"),
            Legendary::KitsuneTsuki =>  v = String::from("(Japanese) – Person possessed by a fox spirit"),
            Legendary::Kiyohime =>  v = String::from("(Japanese) – Woman who transformed into a serpentine demon out of the rage of unrequited love"),
            Legendary::Klabautermann =>  v = String::from("(German) – Ship spirit"),
            Legendary::Knocker =>  v = String::from("(Cornish and Welsh) – Little people and mine spirits"),
            Legendary::Knucker =>  v = String::from("(English) – Water dragon"),
            Legendary::Kobalos =>  v = String::from("(Greek) – Goblin like thieves and tricksters"),
            Legendary::Kobold =>  v = String::from("(German) – Little people and mine or house spirits"),
            Legendary::Kodama =>  v = String::from("(Japanese) – Tree spirit"),
            Legendary::Kofewalt =>  v = String::from("(Germanic) – House spirit"),
            Legendary::KoGok =>  v = String::from("(Abenaki) – Hideous monster"),
            Legendary::Kokakucho =>  v = String::from("(Japanese) – Ubume bird"),
            Legendary::Komainu =>  v = String::from("(Japanese) – Protective animal"),
            Legendary::KonakiJiji =>  v = String::from("(Japanese) – Infant that cries until it is picked up, then increases its weight and crushes its victim"),
            Legendary::KonohaTengu =>  v = String::from("(Japanese) – Bird-like creature"),
            Legendary::KoroPokGuru =>  v = String::from("(Ainu) – Little people"),
            Legendary::Korrigan =>  v = String::from("(Breton) – Little people and nature spirits"),
            Legendary::Kraken =>  v = String::from("(Scandinavian) – Sea monster"),
            Legendary::Krasnoludek =>  v = String::from("(Slavic) – Little people nature spirits"),
            Legendary::Krasue =>  v = String::from("(Southeast Asian) – Vampiric, floating head"),
            Legendary::Krampus =>  v = String::from("(Germany) – Christmas Devil who punishes badly-behaved children"),
            Legendary::KuarahyJara =>  v = String::from("(Guaraní) – Forest spirit"),
            Legendary::Kubikajiri =>  v = String::from("(Japanese) – Female corpse-chewing graveyard spirit"),
            Legendary::KuchisakeOnna =>  v = String::from("(Japanese) – Vengeful ghost of a woman mutilated by her husband"),
            Legendary::KudaGitsune =>  v = String::from("(Japanese) – Miniature fox spirit"),
            Legendary::Kudan =>  v = String::from("(Japanese) – Human-faced calf which predicts a calamity before dying"),
            Legendary::Kui =>  v = String::from("(Chinese) – One-legged monster"),
            Legendary::Kukudhi =>  v = String::from("(Albanian) – Female demon who spreads sickness"),
            Legendary::Kukwes =>  v = String::from("(Mi'kmaq) – Large, hairy, greedy, human-eating bipedal monsters whose scream can kill"),
            Legendary::Kulshedra =>  v = String::from("(Albanian) – Drought-causing dragon"),
            Legendary::Kumakatok =>  v = String::from("(Philippine) – Death spirits"),
            Legendary::Kumiho =>  v = String::from("(Korean) – Fox spirit"),
            Legendary::Kun =>  v = String::from("(Chinese) – Giant fish"),
            Legendary::Kupua =>  v = String::from("(Hawaiian) – Shapeshifting tricksters"),
            Legendary::Kurabokko =>  v = String::from("(Japanese) – Guardian spirit of a warehouse"),
            Legendary::KurageNoHinotama =>  v = String::from("(Japanese) – Jellyfish which floats through the air as a fireball"),
            Legendary::Kurma =>  v = String::from("(Hindu mythology) – Second avatar of Vishnu in the form of a Turtle"),
            Legendary::Kurupi =>  v = String::from("(Guaraní) – Wild man and fertility spirit"),
            Legendary::Kushtaka =>  v = String::from("(Tlingit) – Shapeshifting 'land otter man'"),
            Legendary::KyeRyong =>  v = String::from("(Korean) – Chicken-lizard hybrid"),
            Legendary::Kyourinrin =>  v = String::from("(Japanese) – Animated scroll or paper"),
            Legendary::KyubiNoKitsune =>  v = String::from("(Japanese) – Nine-tailed fox"),
            Legendary::Kyuketsuki =>  v = String::from("(Japanese) – Vampire"),
            Legendary::LaBarTu =>  v = String::from("(Assyrian) – Disease demon"),
            Legendary::LabbMu =>  v = String::from("(Akkadian) – Sea snake"),
            Legendary::Ladyidday =>  v = String::from("(Slavic) – Sunstroke spirit"),
            Legendary::Ladon =>  v = String::from("(Greek) – Dragon guarding the golden apples of the Hesperides"),
            Legendary::Laelaps =>  v = String::from("(Greek) – Enchanted dog that always caught his prey"),
            Legendary::Laestrygonians =>  v = String::from("(Greek) – Anthropophagic giants"),
            Legendary::Lakanica =>  v = String::from("(Slavic) – Field spirit"),
            Legendary::LakeMonster =>  v = String::from("(Worldwide) – Gigantic animals reported to inhabit various lakes around the world"),
            Legendary::Lakhey =>  v = String::from("(Nepalese) – Demon with fangs"),
            Legendary::LaLlorona =>  v = String::from("(Latin America) – Death spirit associated with drowning"),
            Legendary::Lamassu =>  v = String::from("(Akkadian and Sumerian) – Protective spirit with the form of a winged bull or human-headed lion"),
            Legendary::LambtonWorm =>  v = String::from("(English) – Giant worm"),
            Legendary::Lamia =>  v = String::from("(Greek) – Child-devouring monster"),
            Legendary::Lamiak =>  v = String::from("(Basque) – Water spirit with duck-like feet"),
            Legendary::LaMojana =>  v = String::from("(Colombian) – Shapeshifting, female water spirit"),
            Legendary::Lampades =>  v = String::from("(Greek) – Underworld nymph"),
            Legendary::Landvaettir =>  v = String::from("(Norse) – Nature spirits"),
            Legendary::Langmeidong =>  v = String::from("(Manipuri mythology) – Semi human, semi hornbill creature"),
            Legendary::Lares =>  v = String::from("(Roman) – House spirit"),
            Legendary::LaSayona =>  v = String::from("(Venezuela) – Female ghost that punishes unfaithful husbands"),
            Legendary::LaTunda =>  v = String::from("(Colombian) – Nature spirit that seduces and kills men"),
            Legendary::LavaBear =>  v = String::from("Miniature bear thought to inhabit the lava beds of south central Oregon"),
            Legendary::LaukuDvasios =>  v = String::from("(Lithuanian) – Field spirit"),
            Legendary::Lauma =>  v = String::from("(Baltic) – Sky spirit"),
            Legendary::Lavellan =>  v = String::from("(Scottish) – Gigantic water rat"),
            Legendary::LeananSidhe =>  v = String::from("(Celtic) – Fairy lover"),
            Legendary::Leanashe =>  v = String::from("(Irish) – Possessing spirit or vampire"),
            Legendary::Leimakids =>  v = String::from("(Greek) – Meadow nymph"),
            Legendary::Leokampoi =>  v = String::from("(Etruscan) – Fish-tailed lion"),
            Legendary::Leontophone =>  v = String::from("(Medieval Bestiary) – Tiny animal poisonous to lions"),
            Legendary::Leprechaun =>  v = String::from("(Irish) – Cobbler spirit"),
            Legendary::Leszi =>  v = String::from("(Slavic) – Tree spirit"),
            Legendary::Leuce =>  v = String::from("(Greek) – White poplar tree nymph"),
            Legendary::Leucrota =>  v = String::from("(Medieval Bestiary) – Crocotta-lion hybrid"),
            Legendary::Leviathan =>  v = String::from("(Jewish) – Sea monster seen in Job 41"),
            Legendary::Leyak =>  v = String::from("(Balinese) – Anthropophagous flying head with entrails"),
            Legendary::LibyanAegipanes =>  v = String::from("(Medieval Bestiaries) – Human-horse hybrid"),
            Legendary::LibyanSatyr =>  v = String::from("(Medieval Bestiaries) – Human-goat hybrid"),
            Legendary::Liderc =>  v = String::from("(Hungary) – Magical chicken that transforms into a humanoid"),
            Legendary::LightningBird =>  v = String::from("(Southern Africa) – Magical bird found at sites of lightning strikes"),
            Legendary::Likho =>  v = String::from("(Slavic) – One-eyed hag or goblin"),
            Legendary::Lilin =>  v = String::from("(Jewish) – Night-demoness"),
            Legendary::Lilitu =>  v = String::from("(Assyrian) – Winged demon"),
            Legendary::Limnades =>  v = String::from("(Greek) – Lake nymph"),
            Legendary::Lindworm =>  v = String::from("(Germanic) – Dragon"),
            Legendary::Ljosalfar =>  v = String::from("(Norse) – Sunlight spirits; the Light Elves"),
            Legendary::Ljubi =>  v = String::from("(Albanian)- Demoness"),
            Legendary::LlamhigynYDwr =>  v = String::from("(Welsh) – Frog-bat-lizard hybrid"),
            Legendary::LochNessMonster =>  v = String::from("(Scottish) – Serpentine sea monster"),
            Legendary::Loki =>  v = String::from("(Norse mythology) – God of night"),
            Legendary::LoLol =>  v = String::from("(Abenaki) – Hideous monster"),
            Legendary::Long =>  v = String::from("Chinese dragon"),
            Legendary::Longana =>  v = String::from("(Italian) – Female human-goat hybrid and water spirit"),
            Legendary::LongMa =>  v = String::from("(Chinese) – Dragon-horse hybrid"),
            Legendary::Loogaroo =>  v = String::from("(French America) – Shapeshifting, female vampire"),
            Legendary::LouCarcolh =>  v = String::from("(French) – Snake-mollusk hybrid"),
            Legendary::LoupGarou =>  v = String::from("(French) – Werewolf"),
            Legendary::LovelandFrog =>  v = String::from("(Ohio) – Cryptid, Humanoid Frog"),
            Legendary::LubberFiend =>  v = String::from("(English) – House spirit"),
            Legendary::Luduan =>  v = String::from("(Chinese) – Truth-detecting animal"),
            Legendary::Lugat =>  v = String::from("(Albanian) – Vampire"),
            Legendary::Luison =>  v = String::from("(Guaraní) – Werewolf | Cadaver-eating dog"),
            Legendary::Lusca =>  v = String::from("Sea Monster"),
            Legendary::Lutin =>  v = String::from("(French) – Amusing goblin"),
            Legendary::Lyngbakr =>  v = String::from("(Icelandic) Whale-like sea monster"),
            Legendary::Lynx =>  v = String::from("(Medieval Bestiaries) – Feline guide spirit"),
            Legendary::MaaAlused =>  v = String::from("(Estonian mythology) – Subterranean spirit"),
            Legendary::Machlyes =>  v = String::from("(Medieval bestiaries) – Hermaphroditic humanoid"),
            Legendary::Macrocephali =>  v = String::from("(Medieval bestiaries) – Giant-headed humanoid"),
            Legendary::MadamKoiKoi =>  v = String::from("(West African Mythology ) – Female ghost"),
            Legendary::Madremonte =>  v = String::from("(Colombian folklore) – Nature guardian"),
            Legendary::Maero =>  v = String::from("(Māori) – Savage, arboreal humanoids"),
            Legendary::Magog =>  v = String::from("(English folklore) – Giant protector of London"),
            Legendary::MahaPudma =>  v = String::from("(Hindu mythology) – Giant elephant that holds up the world"),
            Legendary::Mairu =>  v = String::from("(Basque mythology) – Megalith-building giant"),
            Legendary::MajasGari =>  v = String::from("(Latvian mythology) – Benevolent house spirit"),
            Legendary::Majitu =>  v = String::from("// in Swahili mythology, shape-shifting spirits that can pass as humans"),
            Legendary::Makara =>  v = String::from("(Indian mythology) – Aquatic beings"),
            Legendary::MakuraGaeshi =>  v = String::from("(Japanese mythology) – Pillow-moving spirit"),
            Legendary::MalltYNos =>  v = String::from("(Welsh mythology) – Spirit of the hunt"),
            Legendary::MamiWata =>  v = String::from("(Africa and the African diaspora) – Supernaturally beautiful water spirits"),
            Legendary::Manananggal =>  v = String::from("(Philippine mythology) – Vampires that sever their torsos from their legs to fly around"),
            Legendary::Mandi =>  v = String::from("(Medieval bestiaries) – Humanoid with a forty-year lifespan"),
            Legendary::Mandrake =>  v = String::from("(Medieval folklore) – Diminutive, animated construct"),
            Legendary::Manes =>  v = String::from("(Roman mythology) – Ancestral spirits"),
            Legendary::Mannegishi =>  v = String::from("(Cree) – Little people with six fingers and no noses"),
            Legendary::Manticore =>  v = String::from("(Persian mythology) – Lion-human-scorpion hybrid"),
            Legendary::Mapinguari =>  v = String::from("(Brazilian mythology) – Giant sloth"),
            Legendary::Mara =>  v = String::from("(Scandinavian folklore) – Female night-demon"),
            Legendary::Marabbecca =>  v = String::from("(Italian folklore) – Malevolent water spirit"),
            Legendary::Mareikura =>  v = String::from("(Tuamotu) – Attendant of Kiho-tumu, the supreme god"),
            Legendary::MaresOfDiomedes =>  v = String::from("(Greek mythology) – Man-eating horses"),
            Legendary::Marid =>  v = String::from("(Arabian mythology) – Jinn associated fortune tellers"),
            Legendary::Marmennill =>  v = String::from("(Norse mythology) – Mermen with prophetic abilities"),
            Legendary::MaroDeives =>  v = String::from("(Lithuanian mythology) – Disease spirits"),
            Legendary::MaskiMonGweZoOs =>  v = String::from("(Abenaki mythology) – Shapeshifting toad spirit"),
            Legendary::Matagot =>  v = String::from("(French mythology) – Spirit that takes animal form; usually that of a black cat"),
            Legendary::Matsya =>  v = String::from("(Hindu mythology) – First Avatar of Vishnu in the form of a half-fish and half-man"),
            Legendary::Mayura =>  v = String::from("(Hindu mythology) – Peacock spirit"),
            Legendary::Mazzikin =>  v = String::from("(Jewish mythology) – Invisible, malevolent spirit"),
            Legendary::MboiTuI =>  v = String::from("(Guaraní mythology) – Snake-parrot hybrid"),
            Legendary::Mbwiri =>  v = String::from("(Central Africa) – Possessing demon"),
            Legendary::Medusa =>  v = String::from("(Gorgon) with numerous snake heads"),
            Legendary::MelekTaus =>  v = String::from("// biblical bird"),
            Legendary::Meliae =>  v = String::from("(Greek mythology) – Ash tree nymph"),
            Legendary::Melusine =>  v = String::from("(Medieval folklore) – Female water spirit, with the form of a winged mermaid or serpent"),
            Legendary::Menehune =>  v = String::from("(Hawaiian mythology) – Little people and craftsmen"),
            Legendary::Menninkainen =>  v = String::from("(Finnish mythology) – Little people and nature spirits"),
            Legendary::Merlion =>  v = String::from("(Singapore) – Combination of a lion and a fish, the symbol of Singapore"),
            Legendary::Mermaid =>  v = String::from("(multiple cultures) – Human-fish hybrid"),
            Legendary::Merman =>  v = String::from("(multiple cultures) – Human-fish hybrid"),
            Legendary::Merlin =>  v = String::from("(English mythology) – Elderly wizard"),
            Legendary::Merrow =>  v = String::from("(Irish mythology and Scottish) – Human-fish hybrid"),
            Legendary::MeteeKolenOl =>  v = String::from("(Abenaki mythology) – Ice-hearted wizards"),
            Legendary::Mimi =>  v = String::from("(Australian Aboriginal mythology) – Extremely elongated humanoid that has to live in rock crevasses to avoid blowing away"),
            Legendary::MinkaBird =>  v = String::from("(Australian Aboriginal mythology) – Death spirit"),
            Legendary::Minokawa =>  v = String::from("(Philippine) – Giant swallow"),
            Legendary::Minotaur =>  v = String::from("(Greek mythology) – Human-bull hybrid"),
            Legendary::Mishibizhiw =>  v = String::from("(Ojibwa) – Feline water spirit"),
            Legendary::MisiGinebig =>  v = String::from("(Ojibwa) – Serpentine rain spirit"),
            Legendary::MisiKinepikw =>  v = String::from("(Cree) – Serpentine rain spirit"),
            Legendary::Mizuchi =>  v = String::from("(Japanese mythology) – Water dragon"),
            Legendary::Mogwai =>  v = String::from("(Chinese mythology) – Vengeful ghost or demon"),
            Legendary::Mohan =>  v = String::from("(Latin American folklore) – Nature spirit"),
            Legendary::MokeleMbembe =>  v = String::from("(Congo) – Water-dwelling creature"),
            Legendary::Mokoi =>  v = String::from("(Australian Aboriginal mythology) – Malevolent spirit that kills sorcerers"),
            Legendary::Mokorea =>  v = String::from("(underground world)"),
            Legendary::Monai =>  v = String::from("(Guaraní mythology) – Giant snake with antennae"),
            Legendary::Monocerus =>  v = String::from("(Medieval bestiaries) – One-horned stag-horse-elephant-boar hybrid, sometimes treated as distinct from the unicorn"),
            Legendary::MonoGrande =>  v = String::from("(South America) – Giant monkey"),
            Legendary::Monopod =>  v = String::from("(Medieval bestiaries) – Dwarf with one giant foot"),
            Legendary::MooinjerVeggey =>  v = String::from("(Manx folklore) – Nature spirit"),
            Legendary::Mora =>  v = String::from("(Slavic mythology) – Disembodied spirit"),
            Legendary::Morgens =>  v = String::from("(Breton and Welsh mythology) – Water spirits"),
            Legendary::MorinjiNoOkama =>  v = String::from("(Japanese mythology) – Animated tea kettle"),
            Legendary::Mormolykeia =>  v = String::from("(Greek) – Underworld spirit"),
            Legendary::Moroi =>  v = String::from("(Romanian) – Vampiric ghost"),
            Legendary::MossPeople =>  v = String::from("(Continental Germanic mythology) – Little people and tree spirits"),
            Legendary::Mothman =>  v = String::from("(American folklore) – Large grey winged humanoid with glowing red eyes"),
            Legendary::Mugwump =>  v = String::from("(Canadian folklore) – Fish-like lake monster"),
            Legendary::Mujina =>  v = String::from("(Japanese mythology) – Shapeshifting badger spirit"),
            Legendary::Muldjewangk =>  v = String::from("(Australian Aboriginal mythology) – Water monster"),
            Legendary::Multo =>  v = String::from("(Philippine mythology) – Spirit of a deceased person seeking justice or has unfinished business"),
            Legendary::Mummy =>  v = String::from("(Egyptian) – Undead creature who revives"),
            Legendary::MumaPadurii =>  v = String::from("(Romanian folklore) – Forest-dwelling hag"),
            Legendary::MungoonGali =>  v = String::from("(Australian Aboriginal) – Giant goanna"),
            Legendary::Muscaliet =>  v = String::from("(Medieval bestiaries) – Hare-squirrel-boar hybrid that has an intense body heat"),
            Legendary::Muse =>  v = String::from("(Greek mythology) – Spirits that inspire artists"),
            Legendary::Mushusshu =>  v = String::from("(Mesopotamian mythology)"),
            Legendary::Musimon =>  v = String::from("(Heraldic) – Sheep-goat hybrid"),
            Legendary::Myling =>  v = String::from("(Scandinavian folklore) – Ghosts of unbaptized children"),
            Legendary::Myrmecoleon =>  v = String::from("(Medieval bestiaries) – Ant-lion hybrid"),
            Legendary::Nachzehrer =>  v = String::from("(German) – Anthropophagous undead"),
            Legendary::Naga =>  v = String::from("(Buddhist and Hindu) – Nature and water spirits, serpentine or human-serpent hybrids"),
            Legendary::NagaFireballs =>  v = String::from("(Thai) – Spectral fire"),
            Legendary::Nagual =>  v = String::from("(Mesoamerica) – Human-animal shapeshifter"),
            Legendary::Naiad =>  v = String::from("(Greek) – Freshwater nymph"),
            Legendary::Nakki =>  v = String::from("(Finnish) – Water spirit"),
            Legendary::Namahage =>  v = String::from("(Japanese) – Ritual disciplinary demon from the Oga Peninsula"),
            Legendary::Namazu =>  v = String::from("(Japanese) – Giant catfish whose thrashing causing earthquakes"),
            Legendary::NandoBaba =>  v = String::from("(Japanese) – Old woman who hides under the floor in abandoned storerooms"),
            Legendary::NangTakian =>  v = String::from("(Thai) – Tree spirit"),
            Legendary::NanomKeeaPoDa =>  v = String::from("(Abenaki) – Earthquake spirit"),
            Legendary::Napaeae =>  v = String::from("(Greek) – Grotto nymph"),
            Legendary::Narasimha =>  v = String::from("(Hindu mythology) – Avatar of Vishnu in the form of half-man/half-lion"),
            Legendary::Narecnitsi =>  v = String::from("(Slavic) – Fate spirit"),
            Legendary::Nariphon =>  v = String::from("(Thai) – Pod people"),
            Legendary::Nargun =>  v = String::from("(Gunai) – Water monster"),
            Legendary::Nasnas =>  v = String::from("(Arabian) – Half-human, half-demon creature with half a body"),
            Legendary::Nav =>  v = String::from("(Slavic) – Ghost"),
            Legendary::Nawao =>  v = String::from("(Hawaiian) – Savage humanoid"),
            Legendary::NDamKenoWet =>  v = String::from("(Abenaki) – Fish-human hybrid"),
            Legendary::Neptune =>  v = String::from("(Roman mythology) – God of freshwater and sea"),
            Legendary::Neck =>  v = String::from("(Germanic mythology) – Female water spirit"),
            Legendary::Negret =>  v = String::from("(Catalan) – Little people that turn into coins"),
            Legendary::Nekomata =>  v = String::from("(Japanese) – Split-tailed magical cat"),
            Legendary::Nekomusume =>  v = String::from("(Japanese) – Cat in the form of a girl"),
            Legendary::NemeanLion =>  v = String::from("(Greek) – Lion with impenetrable skin"),
            Legendary::Nephilim =>  v = String::from("(Abrahamic mythology) – Gigantic sons of Grigori and human women"),
            Legendary::Nereid =>  v = String::from("(Greek) – Nymph daughters of Nereus"),
            Legendary::Ngen =>  v = String::from("(Mapuche) – Nature spirit"),
            Legendary::Nguruvilu =>  v = String::from("(Mapuche) – Fox-like water snake"),
            Legendary::Nian =>  v = String::from("(Chinese) – Predatory animal"),
            Legendary::Nightmarchers =>  v = String::from("(Hawaiian) – Warrior ghosts"),
            Legendary::Nikusui =>  v = String::from("(Japanese) – Monster which appears as a young woman and sucks all of the flesh off of its victim's body"),
            Legendary::Nimerigar =>  v = String::from("(Shoshone) – Aggressive little people"),
            Legendary::Ningyo =>  v = String::from("(Japanese) – Monkey-fish hybrid"),
            Legendary::NinkiNanka =>  v = String::from("(Western Africa) – Large reptile, possibly a dragon"),
            Legendary::Nisse =>  v = String::from("(Scandinavian) – House spirit"),
            Legendary::Niohoggr =>  v = String::from("(Norse) – Dragon"),
            Legendary::Nivatakavachas =>  v = String::from("(Hindu) – Ocean demon"),
            Legendary::Nix =>  v = String::from("(Germanic) – Female water spirit"),
            Legendary::Nobusuma =>  v = String::from("(Japanese) – Supernatural wall, also a monstrous flying squirrel"),
            Legendary::Nocnitsa =>  v = String::from("(Slavic) – Nightmare spirit"),
            Legendary::NopperaBo =>  v = String::from("(Japanese) – Faceless ghost"),
            Legendary::Nozuchi =>  v = String::from("(Japanese) – Small sea serpent"),
            Legendary::Nuckelavee =>  v = String::from("(Scottish) – Malevolent human-horse-fish hybrid"),
            Legendary::Nue =>  v = String::from("(Japanese) – Monkey-raccoon dog-tiger-snake hybrid"),
            Legendary::NuGui =>  v = String::from("(Chinese) – Vengeful female ghost"),
            Legendary::Nukekubi =>  v = String::from("(Japanese) – Disembodied, flying head that attacks people"),
            Legendary::NukuMaiTore =>  v = String::from("(Māori) – Forest spirit"),
            Legendary::Nuli =>  v = String::from("(Medieval Bestiary) – Humanoid with backwards, eight-toed feet"),
            Legendary::Numen =>  v = String::from("(Roman) – Tutelary spirit"),
            Legendary::Nuno =>  v = String::from("(Philippine) – Malevolent little people"),
            Legendary::Nuppeppo =>  v = String::from("(Japanese) – Animated chunk of dead flesh"),
            Legendary::Nurarihyon =>  v = String::from("(Japanese) – Head-sized ball-like creature that floats in the sea and teases sailors"),
            Legendary::NureOnna =>  v = String::from("(Japanese) – Female monster who appears on the beach"),
            Legendary::Nurikabe =>  v = String::from("(Japanese) – Spirit that manifests as an impassable, invisible wall"),
            Legendary::NyamiNyami =>  v = String::from("(Zimbabwean) mythology) – Snake-spirit of the Zambezi River"),
            Legendary::Nykstukas =>  v = String::from("(Lithuanian) – Cavern spirit"),
            Legendary::Nymph =>  v = String::from("(Greek) – Nature spirit"),
            Legendary::Obake =>  v = String::from("(Japanese) – Shapeshifting spirits"),
            Legendary::Obariyon =>  v = String::from("(Japanese) – Spook which rides piggyback on a human victim and becomes unbearably heavy"),
            Legendary::Obayifo =>  v = String::from("(Ashanti) – Vampiric possession spirit"),
            Legendary::Obia =>  v = String::from("(West Africa) – Gigantic animal that serves witches"),
            Legendary::Oceanid =>  v = String::from("(Greek) – Nymph daughters of Oceanus"),
            Legendary::Odei =>  v = String::from("(Basque) – Storm spirit"),
            Legendary::Odin =>  v = String::from("(Norse mythology) – King of Asgard"),
            Legendary::Odmience =>  v = String::from("(Slavic) – Changeling"),
            Legendary::Og =>  v = String::from("(Jewish) – Giant king of the Amorites"),
            Legendary::Ogopogo =>  v = String::from("(Canadian) Canadian Lake Monster"),
            Legendary::Ogun =>  v = String::from("(South Western Nigeria)"),
            Legendary::Ogre =>  v = String::from("(Medieval folklore) – Large, grotesque humanoid"),
            Legendary::Oiwa =>  v = String::from("(Japanese) – Ghost of a woman with a distorted face who was murdered by her husband"),
            Legendary::Ojancanu =>  v = String::from("(Cantabrian) – Giant cyclops who embodies evil."),
            Legendary::Okiku =>  v = String::from("(Japanese) – Spirit of a plate-counting servant girl, associated with the 'Okiku-Mushi' worm"),
            Legendary::Okubi =>  v = String::from("(Japanese) – Death spirit"),
            Legendary::OkuriInu =>  v = String::from("(Japanese) – Dog or wolf that follows travelers at night, similar to the Black dog of English folklore"),
            Legendary::OleHigue =>  v = String::from("(Guyanese) – Vampiric hag who takes the form of a fireball at night"),
            Legendary::Omukade =>  v = String::from("(Japanese) – Giant, human-eating centipede that lives in the mountains"),
            Legendary::Oni =>  v = String::from("(Japanese) – Large, grotesque humanoid demon, usually having red skin and horns"),
            Legendary::Onibi =>  v = String::from("(Japanese) – Spectral fire"),
            Legendary::Onmoraki =>  v = String::from("(Japanese) – Bird-demon created from the spirits of freshly dead corpses"),
            Legendary::Onocentaur =>  v = String::from("(Medieval Bestiaries) – Human-donkey hybrid"),
            Legendary::Onoskelis =>  v = String::from("(Greek) – Shapeshifting demon"),
            Legendary::Onryo =>  v = String::from("(Japanese) – Vengeful ghost that manifests in a physical rather than a spectral form"),
            Legendary::Onza =>  v = String::from("(Aztec and Latin American folklore) – Wild cat, possibly a subspecies of cougar"),
            Legendary::OozlumBird =>  v = String::from("(Unknown origin) – Bird that flies backwards"),
            Legendary::Ophiotaurus =>  v = String::from("(Greek) – Bull-serpent hybrid"),
            Legendary::Opinicus =>  v = String::from("(Heraldic) – Lion-eagle hybrid, similar to a griffin, but with leonine forelimbs"),
            Legendary::OrangBunian =>  v = String::from("(Malay) – Forest spirit"),
            Legendary::OrangMinyak =>  v = String::from("(Malay) – Spectral rapist"),
            Legendary::Ordog =>  v = String::from("(Hungarian) – Shapeshifting demon"),
            Legendary::Oread =>  v = String::from("(Greek) – Mountain nymph"),
            Legendary::Ork =>  v = String::from("(Tyrolean) – Little people and house spirits"),
            Legendary::Orobas =>  v = String::from("(European) – Horse-headed, honest oracle classed as a demon"),
            Legendary::OrphanBird =>  v = String::from("(Medieval Bestiaries) – Peacock-eagle-swan-crane hybrid"),
            Legendary::Orthrus =>  v = String::from("(Greek) – Two-headed dog"),
            Legendary::Osiris =>  v = String::from("(Hellenized) – God of the dead and the judge of the underworld"),
            Legendary::Oshun =>  v = String::from("(Nigeria) – God of love and fertility"),
            Legendary::Otso =>  v = String::from("(Finnish) – Bear spirit"),
            Legendary::Ouroboros =>  v = String::from("(Worldwide) – Mystic serpent/dragon that eats its own tail"),
            Legendary::Ovinnik =>  v = String::from("(Slavic) – Malevolent threshing house spirit"),
            Legendary::Owlman =>  v = String::from("(Cornish) – Owl-like humanoid"),
            Legendary::PaasselkaDevils =>  v = String::from("(Finnish) – Spectral fire"),
            Legendary::Pamola =>  v = String::from("(Abenaki) – Weather spirit"),
            Legendary::Panes =>  v = String::from("(Greek) – Human-goat hybrids descended from the god Pan"),
            Legendary::Pandi =>  v = String::from("(Medieval Bestiary) – White-haired humanoid with giant ears and eight fingers and toes"),
            Legendary::Panis =>  v = String::from("(Hindu) – Demons with herds of stolen cows"),
            Legendary::Panlong =>  v = String::from("(Chinese) – Water dragon"),
            Legendary::Panotti =>  v = String::from("(Medieval Bestiaries) – Humanoid with gigantic ears"),
            Legendary::Panther =>  v = String::from("(Medieval Bestiaries) – Feline with sweet breath"),
            Legendary::Parandrus =>  v = String::from("(Medieval Bestiaries) – Shapeshifting animal whose natural form was a large ruminant"),
            Legendary::Pard =>  v = String::from("(Medieval Bestiaries) – Fast, spotted feline believed to mate with lions to produce leopards"),
            Legendary::Pardalokampoi =>  v = String::from("(Etruscan) – Fish-tailed leopard"),
            Legendary::Patagon =>  v = String::from("(Medieval folklore) – Giant race reputed to live in the area of Patagonia"),
            Legendary::Patasola =>  v = String::from("(Latin America) – Anthropophagous, one-legged humanoid"),
            Legendary::Patupairehe =>  v = String::from("(Māori) – White-skinned nature spirits"),
            Legendary::Pech =>  v = String::from("(Scottish) – Strong little people"),
            Legendary::Pegaeae =>  v = String::from("(Greek) – Spring nymph"),
            Legendary::Pegasus =>  v = String::from("(Greek) – Winged horse"),
            Legendary::Pegacorn =>  v = String::from("Pegasus-unicorn hybrid"),
            Legendary::Pelesit =>  v = String::from("(Malay) – Servant spirit"),
            Legendary::Peluda =>  v = String::from("(French) – Dragon"),
            Legendary::Penanggalan =>  v = String::from("(Malay) – Vampires that sever their heads from their bodies to fly around, usually with their intestines or other internal organs trailing behind"),
            Legendary::Peng =>  v = String::from("(Chinese) – Giant bird"),
            Legendary::Penghou =>  v = String::from("(Chinese) – Tree spirit"),
            Legendary::Peri =>  v = String::from("(Persian) – Winged humanoid"),
            Legendary::Peryton =>  v = String::from("(Allegedly Medieval folklore) – Deer-bird hybrid"),
            Legendary::Pesanta =>  v = String::from("(Catalan) – Nightmare demon in the form of a cat or dog"),
            Legendary::Peuchen =>  v = String::from("(Chilota and Mapuche) – Vampiric, flying, shapeshifting serpent"),
            Legendary::PhiTaiHong =>  v = String::from("(Thai) – Ghost of a person who has died suddenly of a violent or cruel death"),
            Legendary::Phoenix =>  v = String::from("(Phoenician) – Regenerative bird reborn from its own ashes"),
            Legendary::Piasa =>  v = String::from("(Native American mythology) – Winged, antlered feline-like dragon"),
            Legendary::Piatek =>  v = String::from("(Armenian) – Large land animal"),
            Legendary::PictishBeast =>  v = String::from("(Pictish stones) – Stylistic animal, possibly a dragon"),
            Legendary::Pillan =>  v = String::from("(Mapuche) – Nature spirit"),
            Legendary::Plagg =>  v = String::from("([Japanese spirit])"),
            Legendary::PimSkwaWagenOwad =>  v = String::from("(Abenaki) – Water spirit"),
            Legendary::Piru =>  v = String::from("(Finnish) – Minor demon"),
            Legendary::Pishacha =>  v = String::from("(Hindu) – Carrion-eating demon"),
            Legendary::Pishtaco =>  v = String::from("(Peru) – Monster man that steals its victim's body fat for cannibalistic purposes"),
            Legendary::PitaSkog =>  v = String::from("(Abenaki) – Serpentine rain spirit"),
            Legendary::Pixie =>  v = String::from("(Cornish) – Little people and nature spirits"),
            Legendary::Pixiu =>  v = String::from("(Chinese) – Winged lion"),
            Legendary::PiYao =>  v = String::from("(Chinese) – Horned, dragon-lion hybrid"),
            Legendary::Plakavac =>  v = String::from("(Slavic) – Vampire created when a mother strangles her child"),
            Legendary::PokWejeeMen =>  v = String::from("(Abenaki) – Tree spirit"),
            Legendary::Polevik =>  v = String::from("(Polish) – Little people and field spirits"),
            Legendary::PolloMaligno =>  v = String::from("(Colombian) – Man-eating chicken spirit"),
            Legendary::Polong =>  v = String::from("(Malay) – Invisible servant spirit"),
            Legendary::Poltergeist =>  v = String::from("(German) – Ghost that moves objects"),
            Legendary::Pombero =>  v = String::from("(Guaraní) – Wild man and nature spirit"),
            Legendary::Ponaturi =>  v = String::from("(Māori) – Grotesque, malevolent humanoid"),
            Legendary::Pontianak =>  v = String::from("(Malay) – Undead, vampiric women who died in childbirth"),
            Legendary::PopeLickMonster =>  v = String::from("(American Folklore) Kentucky Urban Legend – Cryptid, a murderous creature that is part man, sheep, and goat"),
            Legendary::Poukai =>  v = String::from("(Māori) – Giant bird"),
            Legendary::Preta =>  v = String::from("(Buddhist, Hindu, and Jain) – Ghosts of especially greedy people"),
            Legendary::Pricolici =>  v = String::from("(Romanian – Roman) – Undead wolf"),
            Legendary::Psoglav =>  v = String::from("(Serbia) – Dog-headed monster"),
            Legendary::Psotnik =>  v = String::from("(Slavic) – Mischievous spirit"),
            Legendary::Psychai =>  v = String::from("(Greek) – Butterfly-winged nymphs, daughters of Psyche"),
            Legendary::Psychopomp =>  v = String::from("(Greek) – Creatures, spirits, angels, or deities in many religions who escort newly deceased souls from Earth to the afterlife"),
            Legendary::Puca =>  v = String::from("(Welsh) – Shapeshifting animal spirit"),
            Legendary::Puki =>  v = String::from("(Icelandic) – Malevolent little person"),
            Legendary::Puck =>  v = String::from("(English) – House spirit"),
            Legendary::Putz =>  v = String::from("(German) – House spirit"),
            Legendary::Pugot =>  v = String::from("(Philippine) – Headless humanoid"),
            Legendary::Puk =>  v = String::from("(Frisian) – House spirit"),
            Legendary::Pukis =>  v = String::from("(Latvian) – Dragon"),
            Legendary::Puckwudgie =>  v = String::from("(Native American mythology) – Troll-like gray-skinned being"),
            Legendary::Pygmy =>  v = String::from("(Greek) – Little people"),
            Legendary::Pyrausta =>  v = String::from("(Greek) – Insect-dragon hybrid"),
            Legendary::Python =>  v = String::from("(Greek) – Serpentine dragon"),
            Legendary::Qalupalik =>  v = String::from("(Inuit mythology) – Aquatic human abductor"),
            Legendary::Qilin =>  v = String::from("(Chinese) – Dragon-ox-deer hybrid"),
            Legendary::Qiqirn =>  v = String::from("(Inuit) – Large, bald dog spirit"),
            Legendary::Qliphoth =>  v = String::from("(Jewish) – Evil spirits"),
            Legendary::QuestingBeast =>  v = String::from("(Arthurian legend) – Serpent-leopard-lion-hart hybrid"),
            Legendary::Quetzalcoatl =>  v = String::from("(Aztec) – Important Aztec god whose name means 'feathered serpent'; he is not to be confused with the quetzal, a type of bird"),
            Legendary::Quinotaur =>  v = String::from("(Frankish) – Five-horned bull"),
            Legendary::Ra =>  v = String::from("(Norse) – Spirit that protects a specific place"),
            Legendary::Rabisu =>  v = String::from("(Akkadian) – Vampiric spirit that ambushes people"),
            Legendary::Radande =>  v = String::from("(Swedish) – Tree spirit"),
            Legendary::Ragana =>  v = String::from("(Lithuanian) – Malevolent witch"),
            Legendary::Raiju =>  v = String::from("(Japanese) – Lightning spirit"),
            Legendary::RainBird =>  v = String::from("(Native American) – Rain spirit"),
            Legendary::RainbowCrow =>  v = String::from("(Lenape) – Crow spirit"),
            Legendary::RainbowFish =>  v = String::from("(Hindu) – Whale-sized, multi-colored fish"),
            Legendary::RainbowSerpent =>  v = String::from("(Australian Aboriginal) – Snake"),
            Legendary::Rakshasa =>  v = String::from("(Buddhist and Hindu) – Shapeshifting demon"),
            Legendary::Ramidreju =>  v = String::from("(Cantabrian) – Extremely long, weasel-like animal"),
            Legendary::Rarog =>  v = String::from("(Slavic) – Whirlwind spirit"),
            Legendary::RavenMocker =>  v = String::from("(Cherokee) – Life-draining spirit"),
            Legendary::RavenSpirit =>  v = String::from("(Native American, Norse, and Siberian) – Trickster spirit"),
            Legendary::Ratatoskr =>  v = String::from("(Norse) – Squirrel spirit"),
            Legendary::RaystownRay =>  v = String::from("(American Folklore) – Possible plesiosaur or serpent"),
            Legendary::Redcap =>  v = String::from("(English) – Evil, ugly humanoid"),
            Legendary::ReEm =>  v = String::from("(Jewish) – Gigantic land animal"),
            Legendary::Reichsadler =>  v = String::from("(Heraldic) – Eagle, sometimes depicted with two heads"),
            Legendary::Rephaite =>  v = String::from("(Jewish) – Giant"),
            Legendary::ReptilianHumanoid =>  v = String::from("(Global) – Human-lizard hybrid"),
            Legendary::Revenant =>  v = String::from("(Medieval folklore) – Reanimated dead"),
            Legendary::Roc =>  v = String::from("(Arabian and Persian) – Gigantic bird"),
            Legendary::Rokurokubi =>  v = String::from("(Japanese) – Long-necked, humanoid trickster"),
            Legendary::Rompo =>  v = String::from("(Africa and India) – Skeletal creature with elements of a rabbit, badger, and bear"),
            Legendary::Rong =>  v = String::from("(Vietnamese) dragon"),
            Legendary::Rougarou =>  v = String::from("(French America) – Human-wolf shapeshifter"),
            Legendary::Rusalka =>  v = String::from("(Slavic) – Female water spirit"),
            Legendary::Ryu =>  v = String::from("Japanese dragon"),
            Legendary::Saci =>  v = String::from("(Brazilian) – One-legged nature spirit"),
            Legendary::Sagari =>  v = String::from("(Japanese) – Horse head that dangles from trees on Kyūshū"),
            Legendary::Sakabashira =>  v = String::from("(Japanese) – Haunted pillar, installed upside-down"),
            Legendary::Salamander =>  v = String::from("(Alchemy) – Fire elemental"),
            Legendary::Samebito =>  v = String::from("(Japanese) – Shark-man servant of the dragon king of the sea"),
            Legendary::Samodiva =>  v = String::from("(Slavic) – Nature spirit"),
            Legendary::Sampati =>  v = String::from("(Hindu) – The demigod Jatayu's brother"),
            Legendary::Sandman =>  v = String::from("(Northern Europe) – Nursery spirit that induces sleep in children"),
            Legendary::Sango =>  v = String::from("(South Western Nigeria) – Yoruba king of arts, music, dance and entertainment"),
            Legendary::Santelmo =>  v = String::from("(Philippine) – Spirits in the form of fireballs that roam around the forest"),
            Legendary::SantaClaus =>  v = String::from("(North Pole-European folklore) – Elderly man who delivers gifts to well-behaved children on the night of Christmas Eve"),
            Legendary::Sanziana =>  v = String::from("(Romanian) – Nature spirit"),
            Legendary::Sarimanok =>  v = String::from("(Philippine) – Bird of good fortune"),
            Legendary::Sarngika =>  v = String::from("(Hindu) – Bird spirit"),
            Legendary::Sarugami =>  v = String::from("(Japanese) – Wicked monkey spirit who was defeated by a dog"),
            Legendary::Satori =>  v = String::from("(Japanese) – Mind-reading humanoid"),
            Legendary::Satan =>  v = String::from("(Heaven--Abrahamic mythology) – Ruler of Hell"),
            Legendary::Satyr =>  v = String::from("(Greek) – Human-goat hybrid and fertility spirit"),
            Legendary::Satyrus =>  v = String::from("(Medieval Bestiary) – Apes who always bear twins, one the mother loves, the other it hates"),
            Legendary::SazaeOni =>  v = String::from("(Japanese) – Shapeshifting turban snail spirit"),
            Legendary::Sceadugenga =>  v = String::from("(English) – Shapeshifting undead"),
            Legendary::Scitalis =>  v = String::from("(Medieval Bestiaries) – Snake which mesmerizes its prey"),
            Legendary::ScorpionMan =>  v = String::from("(Sumerian) – Human-scorpion hybrid"),
            Legendary::Scylla =>  v = String::from("(Greek) – Human-snake hybrid with a snake's tail, twelve legs, and six long-necked snake heads"),
            Legendary::SeaBee =>  v = String::from("(Heraldic) – Fish-tailed bee"),
            Legendary::SeaLion =>  v = String::from("(Heraldic) a legendary creature that has the head and upper body of a lion, but with webbed forelimbs and a fish tail."),
            Legendary::SeaMonk =>  v = String::from("(Medieval folklore) – Fish-like humanoid"),
            Legendary::SeaMonster =>  v = String::from("(Worldwide) – Giant, marine animals"),
            Legendary::SeaSerpent =>  v = String::from("(Worldwide) – Serpentine sea monster"),
            Legendary::SeaWyvern =>  v = String::from("(Heraldic) – Fish-tailed wyvern"),
            Legendary::Seko =>  v = String::from("(Japanese) – Water spirit which can be heard making merry at night"),
            Legendary::Selkie =>  v = String::from("(Faroese, Icelandic, Irish, and Scottish) – Human-seal shapeshifter"),
            Legendary::SenpokuKanpoku =>  v = String::from("(Japanese) – Human-faced frog which guides newly deceased souls to the graveyard"),
            Legendary::Seps =>  v = String::from("(Medieval Bestiaries) – Snake with corrosive venom"),
            Legendary::Serpent =>  v = String::from("(Worldwide) – Snake spirit"),
            Legendary::Serpopard =>  v = String::from("(Ancient Egypt) – Serpent-leopard hybrid"),
            Legendary::Shachihoko =>  v = String::from("(Japanese) – Tiger-carp hybrid"),
            Legendary::Shade =>  v = String::from("(Worldwide) – Spiritual imprint"),
            Legendary::ShadowPeople =>  v = String::from("(American) – Malevolent ghost"),
            Legendary::Shahbaz =>  v = String::from("(Persian) – Giant eagle or hawk"),
            Legendary::Shaitan =>  v = String::from("(Islam) – Islamic version of the Devil (Satan) from the Bible"),
            Legendary::ShangYang =>  v = String::from("(Chinese) – Rain bird"),
            Legendary::Shedim =>  v = String::from("(Jewish) – Chicken-legged demon"),
            Legendary::Shedu =>  v = String::from("(Akkadian and Sumerian) – Protective spirit who takes the form of a winged bull or human-headed lion"),
            Legendary::Shellycoat =>  v = String::from("(English, Scottish and German, as schellenrocc) – Water spirit"),
            Legendary::Shen =>  v = String::from("(Chinese) – Shapeshifing sea monster"),
            Legendary::Shenlong =>  v = String::from("(Chinese) – Weather dragon"),
            Legendary::Shibaten =>  v = String::from("(Japanese) – Water spirit from Shikoku"),
            Legendary::Shikigami =>  v = String::from("(Japanese) – Servant spirit"),
            Legendary::ShikiOji =>  v = String::from("(Japanese) – Child-sized servant spirit"),
            Legendary::Shikome =>  v = String::from("(Japanese) – Underworld hag"),
            Legendary::Shinigami =>  v = String::from("(Japanese) – 'Death god'"),
            Legendary::ShiroBozu =>  v = String::from("(Japanese) – White, faceless spirit"),
            Legendary::Shirouneri =>  v = String::from("(Japanese) – Animated mosquito netting or dust cloth"),
            Legendary::Shiryo =>  v = String::from("(Japanese) – Spirit of a dead person"),
            Legendary::Shisa =>  v = String::from("(Japanese) – Lion-dog hybrid"),
            Legendary::Shishi =>  v = String::from("(Chinese) – Protective animal"),
            Legendary::Shojo =>  v = String::from("(Japanese) – Red-haired sea-sprites who love alcohol"),
            Legendary::Shokera =>  v = String::from("(Japanese) – Creature that peers in through skylights"),
            Legendary::Shtriga =>  v = String::from("(Albanian) – Vampire witch that feeds on children"),
            Legendary::ShuiGui =>  v = String::from("(Chinese) – Drowned ghost"),
            Legendary::ShugMonkey =>  v = String::from("(English) – Dog/monkey"),
            Legendary::Shunoban =>  v = String::from("(Japanese) – Red-faced ghoul"),
            Legendary::ShutenDoji =>  v = String::from("(Japanese) – Ruler of the Oni"),
            Legendary::Sídhe =>  v = String::from("(Irish and Scottish) – Ancestral or nature spirit"),
            Legendary::Sigbin =>  v = String::from("(Philippine) – Goat-like vampire"),
            Legendary::Sileni =>  v = String::from("(Greek) – Bald, fat, thick-lipped, and flat-nosed followers of Dionysus"),
            Legendary::Simargl =>  v = String::from("(Slavic) – Winged dog"),
            Legendary::Simurgh =>  v = String::from("(Persian) – Dog-lion-peacock hybrid"),
            Legendary::Singa =>  v = String::from("(Batak) – Feline animal"),
            Legendary::SintHolo =>  v = String::from("(Choctaw) – Serpentine rain spirit"),
            Legendary::Siren =>  v = String::from("(Greek) – Human-bird hybrid"),
            Legendary::Sirin =>  v = String::from("(Slavic) – Demonic human-headed bird"),
            Legendary::Sirrush =>  v = String::from("(Akkadian) – Dragon with aquiline hind legs and feline forelegs"),
            Legendary::Sisiutl =>  v = String::from("(American Indian) – Two-headed sea serpent"),
            Legendary::SiTeCah =>  v = String::from("(Paiute) – Red-haired giants"),
            Legendary::Sjora =>  v = String::from("(Norse) – Freshwater spirit"),
            Legendary::Sjovaettir =>  v = String::from("(Norse) – Sea spirit"),
            Legendary::SkinWalker =>  v = String::from("(American Indian) – Animal-human shapeshifter"),
            Legendary::Skogsra =>  v = String::from("(Scandinavian) – Forest spirit"),
            Legendary::Skoll =>  v = String::from("(Norse) – Wolf that chases the Sun"),
            Legendary::Skookum =>  v = String::from("(Chinook Jargon) – Hairy giant"),
            Legendary::Skeleton =>  v = String::from("(Medieval folklore) – Living skeletons"),
            Legendary::Skrzak =>  v = String::from("(Slavic) – Flying imp"),
            Legendary::SkyWomen =>  v = String::from("(Polish) – Weather spirit"),
            Legendary::Sleipnir =>  v = String::from("(Norse) – Eight-legged horse"),
            Legendary::Sluagh =>  v = String::from("(Irish and Scottish) – Restless ghost"),
            Legendary::SodehikiKozo =>  v = String::from("(Japanese) – Invisible spirit which pulls on sleeves"),
            Legendary::Sogenbi =>  v = String::from("(Japanese) – Fiery ghost of an oil-stealing monk"),
            Legendary::Soragami =>  v = String::from("(Japanese) – Ritual disciplinary demon"),
            Legendary::SorakiGaeshi =>  v = String::from("(Japanese) – Sound of trees being cut down, when later none seem to have been cut"),
            Legendary::Sorobanbozu =>  v = String::from("(Japanese) – Ghost with an abacus"),
            Legendary::Sotangitsune =>  v = String::from("(Japanese) – Fox spirit from Kyoto"),
            Legendary::Soucouyant =>  v = String::from("(Trinidad and Tobago) – Vampiric hag who takes the form of a fireball at night"),
            Legendary::Spearfinger =>  v = String::from("(Cherokee) – Sharp-fingered hag"),
            Legendary::Spectre =>  v = String::from("(Worldwide) – Terrifying ghost"),
            Legendary::Sphinx =>  v = String::from("(Greek) – Winged woman-headed lion"),
            Legendary::Spiridus =>  v = String::from("(Romanian) – Little people"),
            Legendary::Spirit =>  v = String::from("Ghosts"),
            Legendary::Spriggan =>  v = String::from("(Cornish) – Guardians of graveyards and ruins"),
            Legendary::Sprite =>  v = String::from("(Medieval folklore) – little people, ghosts or elves"),
            Legendary::Squonk =>  v = String::from("(American) – Ugly and lonely creature capable of evading capture by dissolving itself into a pool of tears"),
            Legendary::Stihi =>  v = String::from("(Albanian) – Demonic dragon who guards a treasure"),
            Legendary::Strigoi =>  v = String::from("(Romanian) – Vampire"),
            Legendary::Strix =>  v = String::from("(Roman) – Vampiric bird"),
            Legendary::Struthopodes =>  v = String::from("(Medieval Bestiaries) – Humanoid whose males have enormous feet, and females have tiny feet"),
            Legendary::Strzyga =>  v = String::from("(Slavic) – Vampiric undead"),
            Legendary::Stuhac =>  v = String::from("(Slavic) – Malevolent mountain spirit"),
            Legendary::StymphalianBird =>  v = String::from("(Greek) – Metallic bird"),
            Legendary::Suangi =>  v = String::from("(New Guinea) – Cannibalistic sorcerer"),
            Legendary::Succubus =>  v = String::from("(Medieval folklore) – Female night-demon"),
            Legendary::Sudice =>  v = String::from("(Slavic) – Fortune spirit"),
            Legendary::SunakakeBaba =>  v = String::from("(Japanese) – Sand-throwing hag"),
            Legendary::Sunekosuri =>  v = String::from("(Japanese) – Small dog- or cat-like creature that rubs against a person's legs at night"),
            Legendary::Surma =>  v = String::from("(Finnish) – Hellhound"),
            Legendary::Suzaku =>  v = String::from("(Japanese) – Japanese version of the Chinese Vermillion Bird"),
            Legendary::Svaoilfari =>  v = String::from("(Norse) – Unnatural strong horse, father of Sleipnir"),
            Legendary::Svartalfar =>  v = String::from("(Norse) – Cavern spirits; the Black Elves"),
            Legendary::Swallower =>  v = String::from("(Ancient Egyptian) – Crocodile-leopard-hippopotamus hybrid"),
            Legendary::SwanMaiden =>  v = String::from("(Worldwide) – Swan-human shapeshifter"),
            Legendary::Sylph =>  v = String::from("(Alchemy) – Air elemental"),
            Legendary::Sylvan =>  v = String::from("(Medieval folklore) – Forest spirit"),
            Legendary::Syrbotae =>  v = String::from("(Medieval Bestiaries) – African giant"),
            Legendary::Syrictae =>  v = String::from("(Medieval Bestiaries) – Reptilian humanoid"),
            Legendary::Tachash =>  v = String::from("(Jewish) – Large land animal"),
            Legendary::Tailypo =>  v = String::from("(Appalachia) – Powerful animal, that takes revenge on those who steal its tail"),
            Legendary::Taimatsumaru =>  v = String::from("(Japanese) – Tengu surrounded in demonic fire"),
            Legendary::Takam =>  v = String::from("(Persian) – Nature spirit"),
            Legendary::TakaOnna =>  v = String::from("(Japanese) – Female spirit which can stretch itself to peer into the second story of a building"),
            Legendary::Talos =>  v = String::from("(Greek) – Giant made of bronze"),
            Legendary::Tangie =>  v = String::from("(Scottish) – Shapeshifting water spirit"),
            Legendary::Taniwha =>  v = String::from("(Māori) – Water spirit"),
            Legendary::Tantankororin =>  v = String::from("(Japanese) – Unharvested persimmon which becomes a monster"),
            Legendary::Tanuki =>  v = String::from("(Japanese) – Shapeshifting raccoon dog"),
            Legendary::TaotaoMona =>  v = String::from("(Mariana Islands) – Ancestral spirits"),
            Legendary::Taotie =>  v = String::from("(Chinese) – Greed spirit"),
            Legendary::Tapairu =>  v = String::from("(Mangaia) – Nature spirit"),
            Legendary::Tarasque =>  v = String::from("(French) – Dragon with leonine, turtle, bear, and human attributes"),
            Legendary::Tartalo =>  v = String::from("(Basque) – One-eyed giant"),
            Legendary::Tartaruchi =>  v = String::from("(Christian) – Demonic punisher"),
            Legendary::TatamiTataki =>  v = String::from("(Japanese) – Poltergeist that hits the tatami mats at night"),
            Legendary::Tatzelwurm =>  v = String::from("(Alpine Folklore) lizard-like creature, often described as having the face of a cat, with a serpent-like body which may be slender or stubby, with four short legs or two forelegs"),
            Legendary::Tatsu =>  v = String::from("Japanese dragon"),
            Legendary::Taurokampoi =>  v = String::from("(Etruscan) – Fish-tailed bull"),
            Legendary::Tavara =>  v = String::from("(Trabzon) – Night-demon[citation needed]"),
            Legendary::TejuJagua =>  v = String::from("(Guaraní) – Lizard with seven dog heads"),
            Legendary::Tecumbalam =>  v = String::from("(Mayan) – Bird"),
            Legendary::Tengu =>  v = String::from("(Japanese) – Anthropomorphic bird"),
            Legendary::Tennin =>  v = String::from("(Japanese) – Angelic humanoid"),
            Legendary::TeNoMe =>  v = String::from("(Japanese) – Ghost of a blind man, with his eyes on his hands"),
            Legendary::Tepegoz =>  v = String::from("(Azerbaijani) – Azerbaijani mythical creature similar to the cyclops Polyphemus"),
            Legendary::TerribleMonster =>  v = String::from("(Jewish) – Lion-eagle-scorpion hybrid made from the blood of murder victims"),
            Legendary::TeumessianFox =>  v = String::from("(Greek) – Gigantic fox"),
            Legendary::Theriocephalus =>  v = String::from("(Medieval folklore) – Animal-headed humanoid"),
            Legendary::ThreeLeggedBird =>  v = String::from("(Asia and Africa) – Solar bird"),
            Legendary::Thunderbird =>  v = String::from("(Native American) – Avian lightning bird spirit"),
            Legendary::Thor =>  v = String::from("(Norse mythology) – God of thunder and storm"),
            Legendary::Tiangou =>  v = String::from("(Chinese) – Meteoric dog"),
            Legendary::Tianlong =>  v = String::from("(Chinese) – Celestial dragon"),
            Legendary::Tibicena =>  v = String::from("(Canarian) – Evil Dog"),
            Legendary::TiddyMun =>  v = String::from("(English) – Bog spirit"),
            Legendary::Tigmamanukan =>  v = String::from("(Philippine) – Asian fairy bluebird"),
            Legendary::Tigris =>  v = String::from("(Jewish) – Giant lion"),
            Legendary::Tikbalang =>  v = String::from("(Philippine) – Anthropomorphic horse"),
            Legendary::Tikoloshe =>  v = String::from("(Zulu) – Little people and water spirit"),
            Legendary::Timingila =>  v = String::from("(Hindu) – Sea monster"),
            Legendary::Tipua =>  v = String::from("(Māori) – Spirit that protects a specific place"),
            Legendary::Titan =>  v = String::from("(Greek) – Primeval god"),
            Legendary::Tiyanak =>  v = String::from("(Philippine) – Demons that are souls of dead unbaptized babies"),
            Legendary::Tizheruk =>  v = String::from("(Inuit) – Sea serpent"),
            Legendary::Tlahuelpuchi =>  v = String::from("(Tlaxcalan) – Shapeshifting vampire"),
            Legendary::TofuKozo =>  v = String::from("(Japanese) – Spirit child carrying a block of tofu"),
            Legendary::ToireNoHanakosan =>  v = String::from("(Japanese) – Ghost who lurks in grade school restroom stalls"),
            Legendary::Tomte =>  v = String::from("(Scandinavian) – House spirit"),
            Legendary::Topielec =>  v = String::from("(Slavic) – Water spirit"),
            Legendary::Totetsu =>  v = String::from("(Japanese) – Greed spirit"),
            Legendary::Toyol =>  v = String::from("(Malay) – Servant spirit"),
            Legendary::Trasgo =>  v = String::from("(Spanish and Portuguese) – Grotesque, mischievous little people"),
            Legendary::Trauco =>  v = String::from("(Chilota) – Fertility spirit"),
            Legendary::Trenti =>  v = String::from("(Cantabrian) – Diminutive demon"),
            Legendary::Trickster =>  v = String::from("Character in a story which exhibits a great degree of intellect or secret knowledge, and uses it to play tricks or otherwise disobey normal rules and conventional behaviour"),
            Legendary::Tripurasura =>  v = String::from("(Hindu) – Demonic inhabitants of Tripura"),
            Legendary::Tritons =>  v = String::from("(Greek) – Male human-fish hybrid"),
            Legendary::Troll =>  v = String::from("(Norse) – Nature spirit"),
            Legendary::Trow =>  v = String::from("(Orkney and Shetland) – Little people and nature spirits"),
            Legendary::TsiNoo =>  v = String::from("(Abenaki) – Vampiric demon"),
            Legendary::Tsuchigumo =>  v = String::from("(Japanese) – Shapeshifting, giant spider"),
            Legendary::Tsuchinoko =>  v = String::from("(Japanese) – Plump snake-like creature"),
            Legendary::Tsukumogami =>  v = String::from("(Japanese) – Inanimate object that becomes animated after existing for 100 years"),
            Legendary::TsulKalu =>  v = String::from("(Cherokee) – Giant nature spirit"),
            Legendary::TsuraraOnna =>  v = String::from("(Japanese) – Icicle woman"),
            Legendary::TsurubeOtoshi =>  v = String::from("(Japanese) – Monster which drops or lowers a bucket from the top of a tree to catch people"),
            Legendary::TugarinZmeyevich =>  v = String::from("(Slavic) – Evil shapeshifter"),
            Legendary::TylwythTeg =>  v = String::from("(Welsh) – Nature spirit"),
            Legendary::Tupilaq =>  v = String::from("(Inuit) – Animated construct"),
            Legendary::Turehu =>  v = String::from("(Māori) – Pale spirit"),
            Legendary::Turst =>  v = String::from("(Swiss) – legendary figure who turns people into dogs"),
            Legendary::Turul =>  v = String::from("(Hungarian) – Giant falcon that helped shape the origins of the Magyars"),
            Legendary::Tyger =>  v = String::from("(Heraldry) – Like a real tiger, but lacks stripes. It has the tufted tail of a lion and a thick mane along the neck like a horse"),
            Legendary::Typhon =>  v = String::from("(Greek) – Winged, snake-legged giant"),
            Legendary::Tzitzimitl =>  v = String::from("(Aztec) – Skeletal star spirit"),
            Legendary::Ubume =>  v = String::from("(Japanese) – Ghosts of women who died in childbirth"),
            Legendary::UchekLangmeidong =>  v = String::from("///(Manipuri mythology) – Semi human, semi hornbill creature"),
            Legendary::UmaNoAshi =>  v = String::from("(Japanese) – Horse's leg which dangles from a tree and kicks passersby"),
            Legendary::Umibozu =>  v = String::from("(Japanese) – Ghost of drowned priest"),
            Legendary::UmiNyobo =>  v = String::from("(Japanese) – Female sea monster who steals fish"),
            Legendary::Undead =>  v = String::from("(Worldwide) – Dead that behave as if alive"),
            Legendary::UnderwaterPanther =>  v = String::from("(Native American) – Feline water spirit"),
            Legendary::Undine =>  v = String::from("(Alchemy) – Water elemental"),
            Legendary::Unhcegila =>  v = String::from("(Lakota) – Dragon"),
            Legendary::Unicorn =>  v = String::from("(Medieval Bestiaries) – Horse-like creature with the legs of an antelope, the tail of a lion and a single magical healing horn."),
            Legendary::Unktehi =>  v = String::from("(Lakota) – Serpentine rain spirit"),
            Legendary::Unktehila =>  v = String::from("(Lakota) – Reptilian water monster"),
            Legendary::Upinis =>  v = String::from("(Lithuanian) – River spirit"),
            Legendary::Urayuli =>  v = String::from("(Native American) – Hairy giant"),
            Legendary::Urias =>  v = String::from("(Romanian) – Giant"),
            Legendary::Urmahlullu =>  v = String::from("(Mesopotamian) – Lion-human hybrid guardian spirit"),
            Legendary::UshiOni =>  v = String::from("(Japanese) – Bull-headed monster"),
            Legendary::Utukku =>  v = String::from("(Akkadian) – ″Underworld messenger spirit″"),
            Legendary::Uwan =>  v = String::from("(Japanese) – Spirit that shouts to surprise people"),
            Legendary::Vadatajs =>  v = String::from("(Latvian) – Spirit that misleads people"),
            Legendary::Vahana =>  v = String::from("(Hindu) – Divine mounts"),
            Legendary::Vaibhavi =>  v = String::from("(Indian) – Deadly snake"),
            Legendary::Valkyrie =>  v = String::from("(Norse) – Female spirit that leads souls of dead warriors to Valhalla"),
            Legendary::Valva =>  v = String::from("(Romanian) – Female nature spirit"),
            Legendary::Valravn =>  v = String::from("(Danish) – Supernatural raven"),
            Legendary::Vampire =>  v = String::from("(Slavic) – Reanimated corpse that feeds on blood"),
            Legendary::Vanara =>  v = String::from("(Hindu) – Human-ape hybrid"),
            Legendary::Vantoase =>  v = String::from("(Romanian) – Female weather spirit"),
            Legendary::Varaha =>  v = String::from("(Hindu mythology) – Third Avatar of Vishnu in the form of a boar"),
            Legendary::Varcolac =>  v = String::from("(Romanian) – Vampire or werewolf"),
            Legendary::Vardoger =>  v = String::from("(Scandinavian) – Ghostly double"),
            Legendary::Vedrfolnir =>  v = String::from("(Norse) – Hawk sitting between the eyes of an eagle in the crown of the World Tree Yggdrasil"),
            Legendary::Veli =>  v = String::from("(Latvian) – Ghost, shade, formed after a death of a human"),
            Legendary::VeriSelen =>  v = String::from("Chuvash dragon"),
            Legendary::Vetala =>  v = String::from("(Hindu) – Corpses possessed by vampiric spirits"),
            Legendary::Víbria =>  v = String::from("(Catalan) – Dragon with breasts and an eagle's beak"),
            Legendary::Vielfras =>  v = String::from("(German) – Gluttonous dog-cat-fox hybrid"),
            Legendary::Vila =>  v = String::from("(Slavic) – Weather spirit"),
            Legendary::Vilkacis =>  v = String::from("(Latvian) – Animalistic, werewolf-like monster"),
            Legendary::Virunas =>  v = String::from("(Colombian) – Handsome demon"),
            Legendary::VisionSerpent =>  v = String::from("(Mayan) – Mystical dragon"),
            Legendary::Vídopnir =>  v = String::from("(Norse) – Rooster that sits atop the tree"),
            Legendary::Vodyanoy =>  v = String::from("(Slavic) – Male water spirit"),
            Legendary::Vrykolakas =>  v = String::from("(Greek) – Undead wolf-human hybrid"),
            Legendary::Vaettir =>  v = String::from("(Norse) – Nature spirit"),
            Legendary::Waldgeist =>  v = String::from("(German) – Forest spirit"),
            Legendary::WanaGamesAk =>  v = String::from("(Abenaki) – Water spirits"),
            Legendary::Wani =>  v = String::from("(Japanese) – Crocodilian water monster"),
            Legendary::Wanyudo =>  v = String::from("(Japanese) – Demon in the form of a burning human-headed ox cart"),
            Legendary::WarakNgendog =>  v = String::from("(Indonesian Muslim) – Egg-laying bird"),
            Legendary::Warg =>  v = String::from("(English and Scandinavian O.N. vargr) – Giant, demonic wolf"),
            Legendary::Warlock =>  v = String::from("(Worldwide) – Male witch"),
            Legendary::WassanMonGaneehlaAk =>  v = String::from("(Abenaki) – Aurora spirits"),
            Legendary::WaterMonkey =>  v = String::from("(Chinese) – Water spirit"),
            Legendary::WaterSprite =>  v = String::from("(Alchemy) – Water elemental"),
            Legendary::WatiKutjara =>  v = String::from("(Australia Aboriginal) – Goanna spirits"),
            Legendary::WaWonDeeAMegw =>  v = String::from("(Abenaki) – Shapeshifting snail spirit"),
            Legendary::WeisseFrauen =>  v = String::from("(German) – Female spirit"),
            Legendary::Wekufe =>  v = String::from("(Mapuche) – Demon"),
            Legendary::Wendigo =>  v = String::from("(Algonquian) – Anthropophagous spirit"),
            Legendary::Wentshukumishiteu =>  v = String::from("(Inuit) – Water spirit"),
            Legendary::Werecat =>  v = String::from("(Worldwide) – Feline-human shapeshifter"),
            Legendary::Werehyena =>  v = String::from("(Africa) – Hyena-human shapeshifter"),
            Legendary::Werewolf =>  v = String::from("(Worldwide) – Wolf-human shapeshifter"),
            Legendary::WhiteLady =>  v = String::from("(Worldwide) – Ghost of a murdered or mistreated woman"),
            Legendary::Whowie =>  v = String::from("(Australian Aboriginal) – Giant frog-headed goanna with six legs"),
            Legendary::WildMan =>  v = String::from("(European) – Hairy, bipedal, man-like creature"),
            Legendary::WillOTheWisp =>  v = String::from("(Worldwide) – Spectral fire"),
            Legendary::WirryCow =>  v = String::from("(Scottish) – Malevolent spirit"),
            Legendary::Witch =>  v = String::from("(Worldwide) – Person who practices magic"),
            Legendary::WitteWieven =>  v = String::from("(Dutch) – Female, ancestral spirit"),
            Legendary::Wolpertinger =>  v = String::from("(similar to a Chimera)"),
            Legendary::Wondjina =>  v = String::from("(Australia Aboriginal) – Weather spirit"),
            Legendary::Wraith =>  v = String::from("(Scottish) – Water spirit or ghostly apparition"),
            Legendary::Wulver =>  v = String::from("(Scottish) – Wolf-headed humanoid spirit"),
            Legendary::WuTouGui =>  v = String::from("(Chinese) – Beheaded ghost"),
            Legendary::Wyrm =>  v = String::from("English dragon"),
            Legendary::Wyvern =>  v = String::from("(Germanic Heraldic) – Flying reptile, usually with two legs and two wings"),
            Legendary::Xana =>  v = String::from("(Asturian) – Female water spirit"),
            Legendary::Xanthus =>  v = String::from("(Greek)"),
            Legendary::Xecotcovach =>  v = String::from("(Mayan) – Bird"),
            Legendary::Xelhua =>  v = String::from("(Aztec) – Giant"),
            Legendary::Xiao =>  v = String::from("(mythology), (Chinese) – Ape or four-winged bird"),
            Legendary::XingTian =>  v = String::from("(Chinese) – Headless giant"),
            Legendary::Xiuhcoatl =>  v = String::from("(Aztec) – Drought spirit"),
            Legendary::Xhindi =>  v = String::from("(Albanian) – Elves"),
            Legendary::Yacumama =>  v = String::from("(South America) – Sea monster"),
            Legendary::Yacuruna =>  v = String::from("(Indigenous people of the Amazon) – Mythical water people, with backwards heads and feet"),
            Legendary::Yadokai =>  v = String::from("(Japanese) – Malevolent, nocturnal spirit"),
            Legendary::YagyoSan =>  v = String::from("(Japanese) – Demon who rides through the night on a headless horse"),
            Legendary::Yaksha =>  v = String::from("(Buddhist, Hindu, and Jainism) – Male nature spirit"),
            Legendary::Yakshi =>  v = String::from("(Keralite) – Vampire"),
            Legendary::Yakshini =>  v = String::from("(Buddhist, Hindu, and Jainism) – Female nature spirit"),
            Legendary::YakubyoGami =>  v = String::from("(Japanese) – Disease and misfortune spirit"),
            Legendary::Yale =>  v = String::from("(Medieval Bestiaries) – Antelope- or goat-like animal with swiveling horns"),
            Legendary::Yazhi =>  v = String::from("(Tamil) – Lion-like beast"),
            Legendary::YalleryBrown =>  v = String::from("(English) – Nature spirit"),
            Legendary::Yama =>  v = String::from("(East Asia)) – Wrathful god"),
            Legendary::YamaBiko =>  v = String::from("(Japanese) – Echo spirit"),
            Legendary::YamaBito =>  v = String::from("(Japanese) – Savage, mountain-dwelling humanoid"),
            Legendary::YamaChichi =>  v = String::from("(Japanese) – Monkey-like mountain spirit"),
            Legendary::YamaInu =>  v = String::from("(Japanese) – Dog-like mountain spirit"),
            Legendary::YamaOtoko =>  v = String::from("(Japanese) – Mountain giant"),
            Legendary::YamataNoOrochi =>  v = String::from("(Japanese) – Gigantic, eight-headed serpent"),
            Legendary::YamaUba =>  v = String::from("(Japanese) – Malevolent, mountain-dwelling hag"),
            Legendary::YamaWaro =>  v = String::from("(Japanese) – Hairy, one-eyed spirit"),
            Legendary::Yanari =>  v = String::from("(Japanese) – Spirit which causes strange noises"),
            Legendary::Yaoguai =>  v = String::from("(Chinese) – Animalistic demon or fallen gods"),
            Legendary::YaraMaYhaWho =>  v = String::from("(Australian Aboriginal) – Diminutive, sucker-fingered vampire"),
            Legendary::Yatagarasu =>  v = String::from("(Japanese) – Three-legged crow of Amaterasu"),
            Legendary::YatoNoKami =>  v = String::from("(Japanese) – Serpent spirits"),
            Legendary::YethHound =>  v = String::from("(English) – Headless dog"),
            Legendary::Yeti =>  v = String::from("(Himalayan) – Mountain bigfoot"),
            Legendary::Yilbegan =>  v = String::from("(Turkic) – Either a dragon or a giant"),
            Legendary::Yobuko =>  v = String::from("(Japanese) – Mountain dwelling spirit"),
            Legendary::Yokai =>  v = String::from("(Japanese) – Supernatural monster"),
            Legendary::YomotsuShikome =>  v = String::from("(Japanese) – Underworld hag"),
            Legendary::Yong =>  v = String::from("Korean dragon"),
            Legendary::Yosei =>  v = String::from("(Japanese) – Fairy"),
            Legendary::Yosuzume =>  v = String::from("(Japanese) – Mysterious bird that sings at night, sometimes indicating that the okuri-inu is near"),
            Legendary::YouHunYeGui =>  v = String::from("(Chinese) – Wandering ghost"),
            Legendary::Yowie =>  v = String::from("(Australian Aboriginal) – Nocturnal human-ape hybrid, also Yahoo"),
            Legendary::Ypotryll =>  v = String::from("(Heraldic) – Boar-camel-ox-serpent hybrid"),
            Legendary::YuanGui =>  v = String::from("(Chinese) – Distressed ghost"),
            Legendary::Yukinko =>  v = String::from("(Japanese) – Childlike snow spirit"),
            Legendary::YukiOnna =>  v = String::from("(Japanese) – Female snow spirit"),
            Legendary::Yurei =>  v = String::from("(Japanese) – Ghost"),
            Legendary::Yuxa =>  v = String::from("(Tatar) – 100-year-old snake that transforms into a beautiful human"),
            Legendary::Zahhak =>  v = String::from("(Persian) – Dragon"),
            Legendary::Zaltys =>  v = String::from("(Baltic) – Serpentine fertility spirit"),
            Legendary::Zamzummim =>  v = String::from("(Jewish) – Giant"),
            Legendary::ZanaEMalit =>  v = String::from("(Albanian) – Mountain fairy who bless warriors"),
            Legendary::Zână =>  v = String::from("(Romanian) – Nature spirit"),
            Legendary::ZashikiWarashi =>  v = String::from("(Japanese) – House spirit"),
            Legendary::Zburator =>  v = String::from("(Romanian) – Wolf-headed dragon"),
            Legendary::Zduhac =>  v = String::from("(Slavic mythology) – Disembodied, heroic spirit"),
            Legendary::Zeus =>  v = String::from("(Greek) – God of lightning and storms"),
            Legendary::ZennyoRyuo =>  v = String::from("(Japanese) – Rain-making dragon"),
            Legendary::ZharPtitsa =>  v = String::from("(Slavic) – Glowing bird"),
            Legendary::Zhulong =>  v = String::from("(Chinese) – Pig-headed dragon"),
            Legendary::ZhuQue =>  v = String::from("(Chinese) – Fire elemental bird"),
            Legendary::Ziburinis =>  v = String::from("(Lithuanian) – Forest spirit in the form of a glowing skeleton"),
            Legendary::Zilant =>  v = String::from("(Tatar) – Flying chicken-legged reptile"),
            Legendary::Zin =>  v = String::from("(West Africa) – Water spirits"),
            Legendary::Ziz =>  v = String::from("(Jewish) – Giant bird"),
            Legendary::Zlatorog =>  v = String::from("(Slovenia) – White golden-horned deer"),
            Legendary::Zmeu =>  v = String::from("(Romanian folklore) – Giant with a habit of kidnapping young girls"),
            Legendary::Zmiy =>  v = String::from("Slavic dragon"),
            Legendary::Zombie =>  v = String::from("(Vodou/Worldwide) – Re-animated corpse"),
            Legendary::Zorigami =>  v = String::from("(Japanese) – Animated clock"),
            Legendary::Zuijin =>  v = String::from("(Japanese) – Tutelary spirit"),
            Legendary::ZunberaBo =>  v = String::from("(Japanese) – Faceless ghost"),
        }
        // We **finally** return the string
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