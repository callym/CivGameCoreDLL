use core::ptr::NonNull;

use crate::cv::init_core::CvInitCore;

use super::init_core::InitCore;

/// cbindgen:ignore
extern "thiscall" {
  pub type CvGlobals;

  #[link_name = "?getInstance@CvGlobals@@SAAAV1@XZ"]
  fn CvGlobals_GetInstance() -> NonNull<CvGlobals>;

  #[link_name = "?getInitCore@CvGlobals@@QAEAAVCvInitCore@@XZ"]
  fn CvGlobals_getInitCore(cvGlobals: NonNull<CvGlobals>) -> NonNull<CvInitCore>;

  #[link_name = "?getLoadedInitCore@CvGlobals@@QAEAAVCvInitCore@@XZ"]
  fn CvGlobals_getLoadedInitCore(cvGlobals: NonNull<CvGlobals>) -> NonNull<CvInitCore>;

  #[link_name = "?getIniInitCore@CvGlobals@@QAEAAVCvInitCore@@XZ"]
  fn CvGlobals_getIniInitCore(cvGlobals: NonNull<CvGlobals>) -> NonNull<CvInitCore>;

  #[link_name = "?getMaxCivPlayers@CvGlobals@@QBEHXZ"]
  fn CvGlobals_getMaxCivPlayers(cvGlobals: NonNull<CvGlobals>) -> libc::c_int;
}

pub struct Globals {
  cpp: NonNull<CvGlobals>,
}

impl Globals {
  pub fn new() -> Self {
    Self {
      cpp: unsafe { CvGlobals_GetInstance() },
    }
  }

  pub fn init_core(&self) -> InitCore {
    unsafe { InitCore::new(CvGlobals_getInitCore(self.cpp)) }
  }

  pub fn loaded_init_core(&self) -> InitCore {
    unsafe { InitCore::new(CvGlobals_getLoadedInitCore(self.cpp)) }
  }

  pub fn ini_init_core(&self) -> InitCore {
    unsafe { InitCore::new(CvGlobals_getIniInitCore(self.cpp)) }
  }

  pub fn max_players(&self) -> i32 {
    unsafe { CvGlobals_getMaxCivPlayers(self.cpp) }
  }
}
