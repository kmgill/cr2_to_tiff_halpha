
pub const _14_BIT_MAX : f32 = 16383.0;
pub const _16_BIT_MAX : f32 = std::u16::MAX as f32;

// http://lclevy.free.fr/cr2/
// 14bit value
pub const SENSOR_DARK_LEVEL : f32 = 1023.0;

// Canon EOS 50D with IR block filter removed. 
// Tamron 150-600mm lens @ 250mm zoom, masked to f/10
// Exposure 1/400s, ISO 160
pub const DEFAULT_CENTER_OF_MASS_THRESHOLD : f32 = 20000.0;

// Strings
pub mod status {
    pub const EMPTY : &str = "";
    pub const OK : &str = "ok";
    pub const INVALID_PIXEL_COORDINATES : &str = "Invalid pixel coordinates";
    pub const PARENT_NOT_EXISTS_OR_UNWRITABLE : &str = "Parent does not exist or cannot be written";
    pub const FILE_NOT_FOUND: &str = "File not found";
    pub const ARRAY_SIZE_MISMATCH : &str = "Array size mismatch";
    pub const NOT_IMPLEMENTED : &str = "Not yet implemented";
    pub const DIMENSIONS_DO_NOT_MATCH_VECTOR_LENGTH : &str = "Image dimensions do not match supplied vector length";    
}

pub mod param {
    // Parameters
    pub const PARAM_VERBOSE : &str = "v";
    pub const PARAM_OUTPUT : &str = "output";
    pub const PARAM_OUTPUT_SHORT : &str = "o";
    pub const PARAM_DARK : &str = "dark";
    pub const PARAM_DARK_SHORT : &str = "d";
    pub const PARAM_FLAT : &str = "flat";
    pub const PARAM_FLAT_SHORT : &str = "f";
    pub const PARAM_INPUTS : &str = "inputs";
    pub const PARAM_INPUTS_SHORT : &str = "i";
}

