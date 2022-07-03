const BASE_ADDRESS: &str = "https://toshibamobileservice.azurewebsites.net";
const LOGIN_PATH: &str = "/api/Consumer/Login";
pub const LOGIN_URL: &str = const_format::concatcp!(BASE_ADDRESS, LOGIN_PATH);

const MAPPING_PATH: &str = "/api/AC/GetConsumerACMapping";
pub const MAPPING_URL: &str = const_format::concatcp!(BASE_ADDRESS, MAPPING_PATH);

const STATUS_PATH: &str = "/api/AC/GetCurrentACState";
pub const STATUS_URL: &str = const_format::concatcp!(BASE_ADDRESS, STATUS_PATH);

const SETTINGS_PATH: &str = "/api/AC/GetConsumerProgramSettings";
pub const SETTINGS_URL: &str = const_format::concatcp!(BASE_ADDRESS, SETTINGS_PATH);

const DEVICE_URL: &str = "/api/AC/GetRegisteredACByUniqueId";
