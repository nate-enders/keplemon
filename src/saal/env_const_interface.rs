// This wrapper file was generated automatically by the GenDllWrappers program.
#![allow(non_snake_case)]
#![allow(dead_code)]
use std::os::raw::c_char;

extern "C" {
    //  Notes: This function has been deprecated since v9.0.
    //  Initializes the EnvInit DLL for use in the program.
    //  If this function returns an error, it is recommended that you stop the program immediately.
    //
    //  An error will occur if you forget to load and initialize all the prerequisite DLLs, as listed in the DLL Prerequisites section of the accompanying documentation, before using this DLL.
    //
    //  When the function is called, the GEO model is set to WGS-72 and the FK model is set to FK5.  If the user plans to use the SGP4 propagator, do NOT change this default setting. Otherwise, SGP4 won't work
    pub fn EnvInit(apAddr: i64) -> i32;
    //  Returns information about the EnvConst DLL.
    //  The returned string provides information about the version number, build date, and the platform of the EnvConst DLL.
    pub fn EnvGetInfo(infoStr: *const c_char);
    //  Reads Earth constants (GEO) model and fundamental catalogue (FK) model settings from a file.
    //  The users can use NAME=VALUE pair to setup the GEO and FK models in the input file.
    //
    //  For GEO model, the valid names are GEOCONST, BCONST and the valid values are WGS-72, WGS72,  72, WGS-84, WGS84, 84, EGM-96, EGM96, 96, EGM-08, EGM08, 08, JGM-2, JGM2, 2, SEM68R, 68, GEM5, 5, GEM9, and 9.
    //
    //  For FK model, the valid name is FKCONST and the valid values are: FK4, 4, FK5, 5.
    //
    //  All the string literals are case-insensitive.
    pub fn EnvLoadFile(envFile: *const c_char) -> i32;
    //  Saves the current Earth constants (GEO) model and fundamental catalogue (FK) model settings to a file.
    //  Returns zero indicating the GEO and FK settings have been successfully saved to the file. Other values indicate an error.
    pub fn EnvSaveFile(envConstFile: *const c_char, saveMode: i32, saveForm: i32) -> i32;
    //  Returns the current fundamental catalogue (FK) setting.
    //  The FK model is shared among all the Standardized Astrodynamic Algorithms DLLs in the program.
    pub fn EnvGetFkIdx() -> i32;
    //  Changes the fundamental catalogue (FK) setting to the specified value.
    //  If the users enter an invalid value for the fkIdx, the program will continue to use the current setting.
    //
    //  The FK model is globally shared among the Standardized Astrodynamic Algorithms DLLs. If its setting is changed, the new setting takes effect immediately.
    //  The FK model must be set to FK5 to use the SGP4 propagator.
    pub fn EnvSetFkIdx(xf_FkMod: i32);
    //  Returns the current Earth constants (GEO) setting.
    //  <br>
    //  The GEO model is shared among all the Standardized Astrodynamic Algorithms DLLs in the program.
    //  <br>
    //  The following table lists possible values of the return value GEO setting:
    //  <table>
    //  <caption>table</caption>
    //  <tr>
    //  <td><b>Value</b></td>
    //  <td><b>Value interpretation</b></td>
    //  </tr>
    //  <tr><td>84</td><td>WGS-84</td></tr>
    //  <tr><td>96</td><td>EGM-96</td></tr>
    //  <tr><td>08</td><td>EGM-08</td></tr>
    //  <tr><td>72</td><td>WGS-72 (default)</td></tr>
    //  <tr><td>2</td><td>JGM2</td></tr>
    //  <tr><td>68</td><td>STEM68R, SEM68R</td></tr>
    //  <tr><td>5</td><td>GEM5</td></tr>
    //  <tr><td>9</td><td>GEM9</td></tr>
    //  </table>
    pub fn EnvGetGeoIdx() -> i32;
    //  Changes the Earth constants (GEO) setting to the specified value.
    //  <br>
    //  If you specify an invalid value for xf_GeoMod, the program will continue to use the current setting.
    //  <br>
    //  The GEO model is globally shared among the Standardized Astrodynamic Algorithms DLLs. If its setting is changed, the new setting takes effect immediately
    //  <br>
    //  The following table lists possible values of the parameter value GEO setting:
    //  <table>
    //  <caption>table</caption>
    //  <tr>
    //  <td><b>Value</b></td>
    //  <td><b>Value interpretation</b></td>
    //  </tr>
    //  <tr><td>84</td><td>WGS-84</td></tr>
    //  <tr><td>96</td><td>EGM-96</td></tr>
    //  <tr><td>08</td><td>EGM-08</td></tr>
    //  <tr><td>72</td><td>WGS-72 (default)</td></tr>
    //  <tr><td>2</td><td>JGM2</td></tr>
    //  <tr><td>68</td><td>STEM68R, SEM68R</td></tr>
    //  <tr><td>5</td><td>GEM5</td></tr>
    //  <tr><td>9</td><td>GEM9</td></tr>
    //  </table>
    //  <br>
    //  The GEO model must be set to WGS-72 to use the SGP4 propagator.
    pub fn EnvSetGeoIdx(xf_GeoMod: i32);
    //  Returns the name of the current Earth constants (GEO) model.
    //  <br>
    //  The geoStr parameter may contain one of the following values:
    //  <table>
    //  <caption>table</caption>
    //  <tr><td>WGS-84</td></tr>
    //  <tr><td>EGM-96</td></tr>
    //  <tr><td>EGM-08</td></tr>
    //  <tr><td>WGS-72</td></tr>
    //  <tr><td>JGM2</td></tr>
    //  <tr><td>SEM68R</td></tr>
    //  <tr><td>GEM5</td></tr>
    //  <tr><td>GEM9</td></tr>
    //  </table>
    pub fn EnvGetGeoStr(geoStr: *const c_char);
    //  Changes the Earth constants (GEO) setting to the model specified by a string literal.
    //  <br>
    //  If you specify an invalid value for geoStr, the program will continue to use the current setting.
    //  <br>
    //  The GEO model is globally shared among the Standardized Astrodynamic Algorithms DLLs. If its setting is changed, the new setting takes effect immediately.
    //  <br>
    //  The following table lists possible values of the parameter value GEO setting:
    //  <table>
    //  <caption>table</caption>
    //  <tr>
    //  <td><b>geoStr (any string in the row)</b></td>
    //  <td><b>Interpretation</b></td>
    //  </tr>
    //  <tr><td>'WGS-84', 'WGS84', '84'</td><td>WGS-84</td></tr>
    //  <tr><td>'EGM-96', 'EGM96', '96'</td><td>EGM-96</td></tr>
    //  <tr><td>'EGM-08', 'EGM08', '8'</td><td>EGM-08</td></tr>
    //  <tr><td>'WGS-72', 'WGS72', '72'</td><td>WGS-72 (default)</td></tr>
    //  <tr><td>'JGM-2, 'JGM2', '2'</td><td>JGM-2</td></tr>
    //  <tr><td>'SEM68R', '68'</td><td>STEM68R, SEM68R</td></tr>
    //  <tr><td>'GEM5', '5'</td><td>GEM5</td></tr>
    //  <tr><td>'GEM9', '9'</td><td>GEM9</td></tr>
    //  </table>
    //  <br>
    //  The GEO model must be set to WGS-72 to use the SGP4 propagator.
    pub fn EnvSetGeoStr(geoStr: *const c_char);
    //  Retrieves the value of one of the constants from the current Earth constants (GEO) model.
    pub fn EnvGetGeoConst(xf_GeoCon: i32) -> f64;
    //  Retrieves the value of one of the constants from the current fundamental catalogue (FK) model.
    pub fn EnvGetFkConst(xf_FkCon: i32) -> f64;
    //  Returns a handle that can be used to access the fundamental catalogue (FK) data structure.
    //  <br>
    //  This function is needed when calling the ThetaGrnwch function from TimeFunc.dll.
    //  <br>
    //  The handle returned by this function is sometimes called a pointer for historical reasons. The name EnvGetFkPtr comes from the fact that the handle used to be called a pointer.
    pub fn EnvGetFkPtr() -> i64;
    //  Specifies the shape of the earth that will be used by the Astro Standards software, either spherical earth or oblate earth
    pub fn EnvSetEarthShape(earthShape: i32);
    //  Returns the value representing the shape of the earth being used by the Astro Standards software, either spherical earth or oblate earth
    pub fn EnvGetEarthShape() -> i32;
}

// Indexes of Earth Constant fields
// Earth flattening (reciprocal; unitless)
pub static XF_GEOCON_FF: i32 = 1;
// J2 (unitless)
pub static XF_GEOCON_J2: i32 = 2;
// J3 (unitless)
pub static XF_GEOCON_J3: i32 = 3;
// J4 (unitless)
pub static XF_GEOCON_J4: i32 = 4;
// Ke (er**1.5/min)
pub static XF_GEOCON_KE: i32 = 5;
// Earth radius (km/er)
pub static XF_GEOCON_KMPER: i32 = 6;
// Earth rotation rate w.r.t. fixed equinox (rad/min)
pub static XF_GEOCON_RPTIM: i32 = 7;

// J2/2 (unitless)
pub static XF_GEOCON_CK2: i32 = 8;
// -3/8 J4 (unitless)
pub static XF_GEOCON_CK4: i32 = 9;
// Converts km/sec to er/kem
pub static XF_GEOCON_KS2EK: i32 = 10;
// Earth rotation rate w.r.t. fixed equinox (rad/kemin)
pub static XF_GEOCON_THDOT: i32 = 11;
// J5 (unitless)
pub static XF_GEOCON_J5: i32 = 12;
// Gravitational parameter km^3/(solar s)^2
pub static XF_GEOCON_MU: i32 = 13;

// Indexes of FK Constant fields
// Earth rotation rate w.r.t. moving equinox (rad/day)
pub static XF_FKCON_C1: i32 = 1;
// Earth rotation acceleration(rad/day**2)
pub static XF_FKCON_C1DOT: i32 = 2;
// Greenwich angle (1970; rad)
pub static XF_FKCON_THGR70: i32 = 3;

// Indexes represent geopotential models GEO
// Earth constants - JGM2
pub const XF_GEOMOD_JGM2: i32 = 2;
// Earth constants - GEM5
pub const XF_GEOMOD_GEM5: i32 = 5;
// Earth constants - EGM-08
pub const XF_GEOMOD_EGM08: i32 = 8;
// Earth constants - GEM9
pub const XF_GEOMOD_GEM9: i32 = 9;
// Earth constants - STEM68
pub const XF_GEOMOD_STEM68: i32 = 68;
// Earth constants - WGS-72
pub const XF_GEOMOD_WGS72: isize = 72;
// Earth constants - WGS-84
pub const XF_GEOMOD_WGS84: isize = 84;
// Earth constants - EGM-96
pub const XF_GEOMOD_EGM96: isize = 96;
// Invalid earth model
pub static XF_GEOMOD_UNKNOWN: i32 = 100;

//*******************************************************************************

// Indexes represent fundamental catalogue FK
// Fundamental Catalog - FK5
pub static XF_FKMOD_4: i32 = 4;
// Fundamental Catalog - FK4
pub static XF_FKMOD_5: i32 = 5;

// ========================= End of auto generated code ==========================

pub fn get_earth_radius() -> f64 {
    unsafe { EnvGetGeoConst(XF_GEOCON_KMPER) }
}

pub fn get_ke() -> f64 {
    unsafe { EnvGetGeoConst(XF_GEOCON_KE) }
}

pub fn set_geo_model(model: i32) {
    unsafe { EnvSetGeoIdx(model) }
}
