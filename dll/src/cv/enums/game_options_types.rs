#[derive(Debug, num_enum::TryFromPrimitive, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GameOptionTypes {
  NoGameOption = -1,

  AdvancedStart,
  NoCityRazing,
  NoCityFlipping,
  FlippingAfterConquest,
  NoBarbarians,
  RagingBarbarians,
  AggressiveAi,
  LeadAnyCiv,
  RandomPersonalities,
  PickReligion,
  NoTechTrading,
  NoTechBrokering,
  PermanentAlliances,
  AlwaysWar,
  AlwaysPeace,
  OneCityChallenge,
  NoChangingWarPeace,
  NewRandomSeed,
  LockMods,
  CompleteKills,
  NoVassalStates,
  NoGoodyHuts,
  NoEvents,
  NoEspionage,
  NumTypes,
}