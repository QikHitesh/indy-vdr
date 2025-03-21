pub const NODE: &str = "0";
pub const NYM: &str = "1";
pub const GET_TXN: &str = "3";
pub const TXN_AUTHR_AGRMT: &str = "4"; // TODO Use nonabbreviated names as in updated design
pub const TXN_AUTHR_AGRMT_AML: &str = "5";
pub const GET_TXN_AUTHR_AGRMT: &str = "6";
pub const GET_TXN_AUTHR_AGRMT_AML: &str = "7";
pub const DISABLE_ALL_TXN_AUTHR_AGRMTS: &str = "8";
pub const LEDGERS_FREEZE: &str = "9";
pub const GET_FROZEN_LEDGERS: &str = "10";
pub const ATTRIB: &str = "100";
pub const HANDLE: &str = "99994";
pub const HANDLE_GET: &str = "10501";
pub const SCHEMA: &str = "101";
pub const CRED_DEF: &str = "102";
pub const GET_ATTR: &str = "104";
pub const GET_NYM: &str = "105";
pub const GET_SCHEMA: &str = "107";
pub const GET_CRED_DEF: &str = "108";
pub const POOL_UPGRADE: &str = "109";
pub const POOL_RESTART: &str = "118";
pub const POOL_CONFIG: &str = "111";
pub const REVOC_REG_DEF: &str = "113";
pub const REVOC_REG_ENTRY: &str = "114";
pub const GET_REVOC_REG_DEF: &str = "115";
pub const GET_REVOC_REG: &str = "116";
pub const GET_REVOC_REG_DELTA: &str = "117";
pub const GET_VALIDATOR_INFO: &str = "119";
pub const AUTH_RULE: &str = "120";
pub const GET_AUTH_RULE: &str = "121";
pub const AUTH_RULES: &str = "122";
// RichSchema objects
pub const RICH_SCHEMA_CTX: &str = "200";
pub const RICH_SCHEMA: &str = "201";
pub const RICH_SCHEMA_ENCODING: &str = "202";
pub const RICH_SCHEMA_MAPPING: &str = "203";
pub const RICH_SCHEMA_CRED_DEF: &str = "204";
pub const RICH_SCHEMA_PRES_DEF: &str = "205";

pub const GET_RICH_SCHEMA_BY_ID: &str = "300";
pub const GET_RICH_SCHEMA_BY_METADATA: &str = "301";

pub const REQUESTS: [&str; 35] = [
    NODE,
    NYM,
    GET_TXN,
    ATTRIB,
    HANDLE,
    HANDLE_GET,
    SCHEMA,
    CRED_DEF,
    GET_ATTR,
    GET_NYM,
    GET_SCHEMA,
    GET_CRED_DEF,
    POOL_UPGRADE,
    POOL_RESTART,
    POOL_CONFIG,
    REVOC_REG_DEF,
    REVOC_REG_ENTRY,
    GET_REVOC_REG_DEF,
    GET_REVOC_REG,
    GET_REVOC_REG_DELTA,
    GET_VALIDATOR_INFO,
    AUTH_RULE,
    TXN_AUTHR_AGRMT,
    TXN_AUTHR_AGRMT_AML,
    GET_TXN_AUTHR_AGRMT,
    GET_TXN_AUTHR_AGRMT_AML,
    DISABLE_ALL_TXN_AUTHR_AGRMTS,
    LEDGERS_FREEZE,
    GET_FROZEN_LEDGERS,
    RICH_SCHEMA_CTX,
    RICH_SCHEMA,
    RICH_SCHEMA_ENCODING,
    RICH_SCHEMA_MAPPING,
    RICH_SCHEMA_CRED_DEF,
    RICH_SCHEMA_PRES_DEF,
];

// likely matches REQUESTS_FOR_STATE_PROOFS
pub const READ_REQUESTS: [&str; 14] = [
    GET_NYM,
    GET_TXN_AUTHR_AGRMT,
    GET_TXN_AUTHR_AGRMT_AML,
    GET_SCHEMA,
    GET_CRED_DEF,
    HANDLE_GET,
    GET_ATTR,
    GET_REVOC_REG,
    GET_REVOC_REG_DEF,
    GET_REVOC_REG_DELTA,
    GET_AUTH_RULE,
    GET_TXN,
    GET_RICH_SCHEMA_BY_ID,
    GET_RICH_SCHEMA_BY_METADATA,
];

pub const TRUSTEE: &str = "0";
pub const STEWARD: &str = "2";
pub const ENDORSER: &str = "101";
pub const NETWORK_MONITOR: &str = "201";
pub const ROLE_REMOVE: &str = "";
pub const RS_SCHEMA_TYPE_VALUE: &str = "sch";
pub const RS_ENCODING_TYPE_VALUE: &str = "enc";
pub const RS_CONTEXT_TYPE_VALUE: &str = "ctx";
pub const RS_MAPPING_TYPE_VALUE: &str = "map";
pub const RS_CRED_DEF_TYPE_VALUE: &str = "cdf";
pub const RS_PRES_DEF_TYPE_VALUE: &str = "pdf";

pub const ROLES: [&str; 4] = [TRUSTEE, STEWARD, ENDORSER, NETWORK_MONITOR];

pub fn txn_name_to_code(txn: &str) -> Option<&str> {
    if REQUESTS.contains(&txn) {
        return Some(txn);
    }

    match txn {
        "NODE" => Some(NODE),
        "NYM" => Some(NYM),
        "GET_TXN" => Some(GET_TXN),
        "ATTRIB" => Some(ATTRIB),
        "SCHEMA" => Some(SCHEMA),
        "CRED_DEF" | "CLAIM_DEF" => Some(CRED_DEF),
        "GET_ATTR" => Some(GET_ATTR),
        "GET_NYM" => Some(GET_NYM),
        "GET_SCHEMA" => Some(GET_SCHEMA),
        "GET_CRED_DEF" => Some(GET_CRED_DEF),
        "POOL_UPGRADE" => Some(POOL_UPGRADE),
        "POOL_RESTART" => Some(POOL_RESTART),
        "POOL_CONFIG" => Some(POOL_CONFIG),
        "REVOC_REG_DEF" => Some(REVOC_REG_DEF),
        "REVOC_REG_ENTRY" => Some(REVOC_REG_ENTRY),
        "GET_REVOC_REG_DEF" => Some(GET_REVOC_REG_DEF),
        "GET_REVOC_REG" => Some(GET_REVOC_REG),
        "GET_REVOC_REG_DELTA" => Some(GET_REVOC_REG_DELTA),
        "GET_VALIDATOR_INFO" => Some(GET_VALIDATOR_INFO),
        "AUTH_RULE" => Some(AUTH_RULE),
        "TXN_AUTHR_AGRMT" => Some(TXN_AUTHR_AGRMT),
        "TXN_AUTHR_AGRMT_AML" => Some(TXN_AUTHR_AGRMT_AML),
        "GET_TXN_AUTHR_AGRMT" => Some(GET_TXN_AUTHR_AGRMT),
        "GET_TXN_AUTHR_AGRMT_AML" => Some(GET_TXN_AUTHR_AGRMT_AML),
        "DISABLE_ALL_TXN_AUTHR_AGRMTS" => Some(DISABLE_ALL_TXN_AUTHR_AGRMTS),
        "LEDGERS_FREEZE" => Some(LEDGERS_FREEZE),
        "GET_FROZEN_LEDGERS" => Some(GET_FROZEN_LEDGERS),
        val => Some(val),
    }
}
