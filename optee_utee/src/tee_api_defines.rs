use libc::*;
use super::tee_api_types::*;

pub const TEE_INT_CORE_API_SPEC_VERSION: uint32_t = 0x0000000A;

pub const TEE_HANDLE_NULL: uint32_t = 0;

pub const TEE_TIMEOUT_INFINITE: uint32_t = 0xFFFFFFFF;

pub fn TEE_PARAM_TYPES(t0: uint32_t, t1: uint32_t, t2: uint32_t, t3: uint32_t) -> uint32_t {
    let tmp:uint32_t = t1<<4 | t2 <<8 | t3 << 12;
    return t0 | tmp;
}

// API Error Codes
pub const TEE_SUCCESS: uint32_t                       = 0x00000000;
pub const TEE_ERROR_CORRUPT_OBJECT: uint32_t          = 0xF0100001;
pub const TEE_ERROR_CORRUPT_OBJECT_2: uint32_t        = 0xF0100002;
pub const TEE_ERROR_STORAGE_NOT_AVAILABLE: uint32_t   = 0xF0100003;
pub const TEE_ERROR_STORAGE_NOT_AVAILABLE_2: uint32_t = 0xF0100004;
pub const TEE_ERROR_GENERIC: uint32_t                 = 0xFFFF0000;
pub const TEE_ERROR_ACCESS_DENIED: uint32_t           = 0xFFFF0001;
pub const TEE_ERROR_CANCEL: uint32_t                  = 0xFFFF0002;
pub const TEE_ERROR_ACCESS_CONFLICT: uint32_t         = 0xFFFF0003;
pub const TEE_ERROR_EXCESS_DATA: uint32_t             = 0xFFFF0004;
pub const TEE_ERROR_BAD_FORMAT: uint32_t              = 0xFFFF0005;
pub const TEE_ERROR_BAD_PARAMETERS: uint32_t          = 0xFFFF0006;
pub const TEE_ERROR_BAD_STATE: uint32_t               = 0xFFFF0007;
pub const TEE_ERROR_ITEM_NOT_FOUND: uint32_t          = 0xFFFF0008;
pub const TEE_ERROR_NOT_IMPLEMENTED: uint32_t         = 0xFFFF0009;
pub const TEE_ERROR_NOT_SUPPORTED: uint32_t           = 0xFFFF000A;
pub const TEE_ERROR_NO_DATA: uint32_t                 = 0xFFFF000B;
pub const TEE_ERROR_OUT_OF_MEMORY: uint32_t           = 0xFFFF000C;
pub const TEE_ERROR_BUSY: uint32_t                    = 0xFFFF000D;
pub const TEE_ERROR_COMMUNICATION: uint32_t           = 0xFFFF000E;
pub const TEE_ERROR_SECURITY: uint32_t                = 0xFFFF000F;
pub const TEE_ERROR_SHORT_BUFFER: uint32_t            = 0xFFFF0010;
pub const TEE_ERROR_EXTERNAL_CANCEL: uint32_t         = 0xFFFF0011;
pub const TEE_ERROR_OVERFLOW: uint32_t                = 0xFFFF300F;
pub const TEE_ERROR_TARGET_DEAD: uint32_t             = 0xFFFF3024;
pub const TEE_ERROR_STORAGE_NO_SPACE: uint32_t        = 0xFFFF3041;
pub const TEE_ERROR_MAC_INVALID: uint32_t             = 0xFFFF3071;
pub const TEE_ERROR_SIGNATURE_INVALID: uint32_t       = 0xFFFF3072;
pub const TEE_ERROR_TIME_NOT_SET: uint32_t            = 0xFFFF5000;
pub const TEE_ERROR_TIME_NEEDS_RESET: uint32_t        = 0xFFFF5001;

// Parameter Type Constants
pub const TEE_PARAM_TYPE_NONE: uint32_t          = 0;
pub const TEE_PARAM_TYPE_VALUE_INPUT: uint32_t   = 1;
pub const TEE_PARAM_TYPE_VALUE_OUTPUT: uint32_t  = 2;
pub const TEE_PARAM_TYPE_VALUE_INOUT: uint32_t   = 3;
pub const TEE_PARAM_TYPE_MEMREF_INPUT: uint32_t  = 5;
pub const TEE_PARAM_TYPE_MEMREF_OUTPUT: uint32_t = 6;
pub const TEE_PARAM_TYPE_MEMREF_INOUT: uint32_t  = 7;

// Login Type Constants
pub const TEE_LOGIN_PUBLIC: uint32_t            = 0x00000000;
pub const TEE_LOGIN_USER: uint32_t              = 0x00000001;
pub const TEE_LOGIN_GROUP: uint32_t             = 0x00000002;
pub const TEE_LOGIN_APPLICATION: uint32_t       = 0x00000004;
pub const TEE_LOGIN_APPLICATION_USER: uint32_t  = 0x00000005;
pub const TEE_LOGIN_APPLICATION_GROUP: uint32_t = 0x00000006;
pub const TEE_LOGIN_TRUSTED_APP: uint32_t       = 0xF0000000;

// Origin Code Constants
pub const TEE_ORIGIN_API: uint32_t         = 0x00000001;
pub const TEE_ORIGIN_COMMS: uint32_t       = 0x00000002;
pub const TEE_ORIGIN_TEE: uint32_t         = 0x00000003;
pub const TEE_ORIGIN_TRUSTED_APP: uint32_t = 0x00000004;

// Property Sets pseudo handles
pub const TEE_PROPSET_TEE_IMPLEMENTATION: TEE_PropSetHandle = 0xFFFFFFFD as *mut _;
pub const TEE_PROPSET_CURRENT_CLIENT: TEE_PropSetHandle = 0xFFFFFFFE as *mut _;
pub const TEE_PROPSET_CURRENT_TA: TEE_PropSetHandle = 0xFFFFFFFF as *mut _;

// Memory Access Rights Constants
pub const TEE_MEMORY_ACCESS_READ: uint32_t      = 0x00000001;
pub const TEE_MEMORY_ACCESS_WRITE: uint32_t     = 0x00000002;
pub const TEE_MEMORY_ACCESS_ANY_OWNER: uint32_t = 0x00000004;

// Memory Management Constant
pub const TEE_MALLOC_FILL_ZERO: uint32_t = 0x00000000;

// Other constants
pub const TEE_STORAGE_PRIVATE: uint32_t = 0x00000001;

pub const TEE_DATA_FLAG_ACCESS_READ: uint32_t          = 0x00000001;
pub const TEE_DATA_FLAG_ACCESS_WRITE: uint32_t         = 0x00000002;
pub const TEE_DATA_FLAG_ACCESS_WRITE_META: uint32_t    = 0x00000004;
pub const TEE_DATA_FLAG_SHARE_READ: uint32_t           = 0x00000010;
pub const TEE_DATA_FLAG_SHARE_WRITE: uint32_t          = 0x00000020;
pub const TEE_DATA_FLAG_OVERWRITE: uint32_t            = 0x00000400;
pub const TEE_DATA_MAX_POSITION: uint32_t              = 0xFFFFFFFF;
pub const TEE_OBJECT_ID_MAX_LEN: uint32_t              = 64;
pub const TEE_USAGE_EXTRACTABLE: uint32_t              = 0x00000001;
pub const TEE_USAGE_ENCRYPT: uint32_t                  = 0x00000002;
pub const TEE_USAGE_DECRYPT: uint32_t                  = 0x00000004;
pub const TEE_USAGE_MAC: uint32_t                      = 0x00000008;
pub const TEE_USAGE_SIGN: uint32_t                     = 0x00000010;
pub const TEE_USAGE_VERIFY: uint32_t                   = 0x00000020;
pub const TEE_USAGE_DERIVE: uint32_t                   = 0x00000040;
pub const TEE_HANDLE_FLAG_PERSISTENT: uint32_t         = 0x00010000;
pub const TEE_HANDLE_FLAG_INITIALIZED: uint32_t        = 0x00020000;
pub const TEE_HANDLE_FLAG_KEY_SET: uint32_t            = 0x00040000;
pub const TEE_HANDLE_FLAG_EXPECT_TWO_KEYS: uint32_t    = 0x00080000;
pub const TEE_OPERATION_CIPHER: uint32_t               = 1;
pub const TEE_OPERATION_MAC: uint32_t                  = 3;
pub const TEE_OPERATION_AE: uint32_t                   = 4;
pub const TEE_OPERATION_DIGEST: uint32_t               = 5;
pub const TEE_OPERATION_ASYMMETRIC_CIPHER: uint32_t    = 6;
pub const TEE_OPERATION_ASYMMETRIC_SIGNATURE: uint32_t = 7;
pub const TEE_OPERATION_KEY_DERIVATION: uint32_t       = 8;
pub const TEE_OPERATION_STATE_INITIAL: uint32_t        = 0x00000000;
pub const TEE_OPERATION_STATE_ACTIVE: uint32_t         = 0x00000001;

// Algorithm Identifiers
pub const TEE_ALG_AES_ECB_NOPAD: uint32_t                = 0x10000010;
pub const TEE_ALG_AES_CBC_NOPAD: uint32_t                = 0x10000110;
pub const TEE_ALG_AES_CTR: uint32_t                      = 0x10000210;
pub const TEE_ALG_AES_CTS: uint32_t                      = 0x10000310;
pub const TEE_ALG_AES_XTS: uint32_t                      = 0x10000410;
pub const TEE_ALG_AES_CBC_MAC_NOPAD: uint32_t            = 0x30000110;
pub const TEE_ALG_AES_CBC_MAC_PKCS5: uint32_t            = 0x30000510;
pub const TEE_ALG_AES_CMAC: uint32_t                     = 0x30000610;
pub const TEE_ALG_AES_CCM: uint32_t                      = 0x40000710;
pub const TEE_ALG_AES_GCM: uint32_t                      = 0x40000810;
pub const TEE_ALG_DES_ECB_NOPAD: uint32_t                = 0x10000011;
pub const TEE_ALG_DES_CBC_NOPAD: uint32_t                = 0x10000111;
pub const TEE_ALG_DES_CBC_MAC_NOPAD: uint32_t            = 0x30000111;
pub const TEE_ALG_DES_CBC_MAC_PKCS5: uint32_t            = 0x30000511;
pub const TEE_ALG_DES3_ECB_NOPAD: uint32_t               = 0x10000013;
pub const TEE_ALG_DES3_CBC_NOPAD: uint32_t               = 0x10000113;
pub const TEE_ALG_DES3_CBC_MAC_NOPAD: uint32_t           = 0x30000113;
pub const TEE_ALG_DES3_CBC_MAC_PKCS5: uint32_t           = 0x30000513;
pub const TEE_ALG_RSASSA_PKCS1_V1_5_MD5: uint32_t        = 0x70001830;
pub const TEE_ALG_RSASSA_PKCS1_V1_5_SHA1: uint32_t       = 0x70002830;
pub const TEE_ALG_RSASSA_PKCS1_V1_5_SHA224: uint32_t     = 0x70003830;
pub const TEE_ALG_RSASSA_PKCS1_V1_5_SHA256: uint32_t     = 0x70004830;
pub const TEE_ALG_RSASSA_PKCS1_V1_5_SHA384: uint32_t     = 0x70005830;
pub const TEE_ALG_RSASSA_PKCS1_V1_5_SHA512: uint32_t     = 0x70006830;
pub const TEE_ALG_RSASSA_PKCS1_V1_5_MD5SHA1: uint32_t    = 0x7000F830;
pub const TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA1: uint32_t   = 0x70212930;
pub const TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA224: uint32_t = 0x70313930;
pub const TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA256: uint32_t = 0x70414930;
pub const TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA384: uint32_t = 0x70515930;
pub const TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA512: uint32_t = 0x70616930;
pub const TEE_ALG_RSAES_PKCS1_V1_5: uint32_t             = 0x60000130;
pub const TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA1: uint32_t   = 0x60210230;
pub const TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA224: uint32_t = 0x60310230;
pub const TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA256: uint32_t = 0x60410230;
pub const TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA384: uint32_t = 0x60510230;
pub const TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA512: uint32_t = 0x60610230;
pub const TEE_ALG_RSA_NOPAD: uint32_t                    = 0x60000030;
pub const TEE_ALG_DSA_SHA1: uint32_t                     = 0x70002131;
pub const TEE_ALG_DSA_SHA224: uint32_t                   = 0x70003131;
pub const TEE_ALG_DSA_SHA256: uint32_t                   = 0x70004131;
pub const TEE_ALG_DH_DERIVE_SHARED_SECRET: uint32_t      = 0x80000032;
pub const TEE_ALG_MD5: uint32_t                          = 0x50000001;
pub const TEE_ALG_SHA1: uint32_t                         = 0x50000002;
pub const TEE_ALG_SHA224: uint32_t                       = 0x50000003;
pub const TEE_ALG_SHA256: uint32_t                       = 0x50000004;
pub const TEE_ALG_SHA384: uint32_t                       = 0x50000005;
pub const TEE_ALG_SHA512: uint32_t                       = 0x50000006;
pub const TEE_ALG_MD5SHA1: uint32_t                      = 0x5000000F;
pub const TEE_ALG_HMAC_MD5: uint32_t                     = 0x30000001;
pub const TEE_ALG_HMAC_SHA1: uint32_t                    = 0x30000002;
pub const TEE_ALG_HMAC_SHA224: uint32_t                  = 0x30000003;
pub const TEE_ALG_HMAC_SHA256: uint32_t                  = 0x30000004;
pub const TEE_ALG_HMAC_SHA384: uint32_t                  = 0x30000005;
pub const TEE_ALG_HMAC_SHA512: uint32_t                  = 0x30000006;
pub const TEE_ALG_ECDSA_P192: uint32_t                   = 0x70001041;
pub const TEE_ALG_ECDSA_P224: uint32_t                   = 0x70002041;
pub const TEE_ALG_ECDSA_P256: uint32_t                   = 0x70003041;
pub const TEE_ALG_ECDSA_P384: uint32_t                   = 0x70004041;
pub const TEE_ALG_ECDSA_P521: uint32_t                   = 0x70005041;
pub const TEE_ALG_ECDH_P192: uint32_t                    = 0x80001042;
pub const TEE_ALG_ECDH_P224: uint32_t                    = 0x80002042;
pub const TEE_ALG_ECDH_P256: uint32_t                    = 0x80003042;
pub const TEE_ALG_ECDH_P384: uint32_t                    = 0x80004042;
pub const TEE_ALG_ECDH_P521: uint32_t                    = 0x80005042;

// Object Types
pub const TEE_TYPE_AES: uint32_t              = 0xA0000010;
pub const TEE_TYPE_DES: uint32_t              = 0xA0000011;
pub const TEE_TYPE_DES3: uint32_t             = 0xA0000013;
pub const TEE_TYPE_HMAC_MD5: uint32_t         = 0xA0000001;
pub const TEE_TYPE_HMAC_SHA1: uint32_t        = 0xA0000002;
pub const TEE_TYPE_HMAC_SHA224: uint32_t      = 0xA0000003;
pub const TEE_TYPE_HMAC_SHA256: uint32_t      = 0xA0000004;
pub const TEE_TYPE_HMAC_SHA384: uint32_t      = 0xA0000005;
pub const TEE_TYPE_HMAC_SHA512: uint32_t      = 0xA0000006;
pub const TEE_TYPE_RSA_PUBLIC_KEY: uint32_t   = 0xA0000030;
pub const TEE_TYPE_RSA_KEYPAIR: uint32_t      = 0xA1000030;
pub const TEE_TYPE_DSA_PUBLIC_KEY: uint32_t   = 0xA0000031;
pub const TEE_TYPE_DSA_KEYPAIR: uint32_t      = 0xA1000031;
pub const TEE_TYPE_DH_KEYPAIR: uint32_t       = 0xA1000032;
pub const TEE_TYPE_ECDSA_PUBLIC_KEY: uint32_t = 0xA0000041;
pub const TEE_TYPE_ECDSA_KEYPAIR: uint32_t    = 0xA1000041;
pub const TEE_TYPE_ECDH_PUBLIC_KEY: uint32_t  = 0xA0000042;
pub const TEE_TYPE_ECDH_KEYPAIR: uint32_t     = 0xA1000042;
pub const TEE_TYPE_GENERIC_SECRET: uint32_t   = 0xA0000000;
pub const TEE_TYPE_CORRUPTED_OBJECT: uint32_t = 0xA00000BE;
pub const TEE_TYPE_DATA: uint32_t             = 0xA00000BF;

// List of Object or Operation Attributes
pub const TEE_ATTR_SECRET_VALUE: uint32_t         = 0xC0000000;
pub const TEE_ATTR_RSA_MODULUS: uint32_t          = 0xD0000130;
pub const TEE_ATTR_RSA_PUBLIC_EXPONENT: uint32_t  = 0xD0000230;
pub const TEE_ATTR_RSA_PRIVATE_EXPONENT: uint32_t = 0xC0000330;
pub const TEE_ATTR_RSA_PRIME1: uint32_t           = 0xC0000430;
pub const TEE_ATTR_RSA_PRIME2: uint32_t           = 0xC0000530;
pub const TEE_ATTR_RSA_EXPONENT1: uint32_t        = 0xC0000630;
pub const TEE_ATTR_RSA_EXPONENT2: uint32_t        = 0xC0000730;
pub const TEE_ATTR_RSA_COEFFICIENT: uint32_t      = 0xC0000830;
pub const TEE_ATTR_DSA_PRIME: uint32_t            = 0xD0001031;
pub const TEE_ATTR_DSA_SUBPRIME: uint32_t         = 0xD0001131;
pub const TEE_ATTR_DSA_BASE: uint32_t             = 0xD0001231;
pub const TEE_ATTR_DSA_PUBLIC_VALUE: uint32_t     = 0xD0000131;
pub const TEE_ATTR_DSA_PRIVATE_VALUE: uint32_t    = 0xC0000231;
pub const TEE_ATTR_DH_PRIME: uint32_t             = 0xD0001032;
pub const TEE_ATTR_DH_SUBPRIME: uint32_t          = 0xD0001132;
pub const TEE_ATTR_DH_BASE: uint32_t              = 0xD0001232;
pub const TEE_ATTR_DH_X_BITS: uint32_t            = 0xF0001332;
pub const TEE_ATTR_DH_PUBLIC_VALUE: uint32_t      = 0xD0000132;
pub const TEE_ATTR_DH_PRIVATE_VALUE: uint32_t     = 0xC0000232;
pub const TEE_ATTR_RSA_OAEP_LABEL: uint32_t       = 0xD0000930;
pub const TEE_ATTR_RSA_PSS_SALT_LENGTH: uint32_t  = 0xF0000A30;
pub const TEE_ATTR_ECC_PUBLIC_VALUE_X: uint32_t   = 0xD0000141;
pub const TEE_ATTR_ECC_PUBLIC_VALUE_Y: uint32_t   = 0xD0000241;
pub const TEE_ATTR_ECC_PRIVATE_VALUE: uint32_t    = 0xC0000341;
pub const TEE_ATTR_ECC_CURVE: uint32_t            = 0xF0000441;
pub const TEE_ATTR_BIT_PROTECTED: uint32_t        = (1 << 28);
pub const TEE_ATTR_BIT_VALUE: uint32_t            =	(1 << 29);

// List of Supported ECC Curves
pub const TEE_ECC_CURVE_NIST_P192: uint32_t = 0x00000001;
pub const TEE_ECC_CURVE_NIST_P224: uint32_t = 0x00000002;
pub const TEE_ECC_CURVE_NIST_P256: uint32_t = 0x00000003;
pub const TEE_ECC_CURVE_NIST_P384: uint32_t = 0x00000004;
pub const TEE_ECC_CURVE_NIST_P521: uint32_t = 0x00000005;

// Panicked Functions Identification
// TA Interface
pub const TEE_PANIC_ID_TA_CLOSESESSIONENTRYPOINT: uint32_t              = 0x00000101;
pub const TEE_PANIC_ID_TA_CREATEENTRYPOINT: uint32_t                    = 0x00000102;
pub const TEE_PANIC_ID_TA_DESTROYENTRYPOINT: uint32_t                   = 0x00000103;
pub const TEE_PANIC_ID_TA_INVOKECOMMANDENTRYPOINT: uint32_t             = 0x00000104;
pub const TEE_PANIC_ID_TA_OPENSESSIONENTRYPOINT: uint32_t               = 0x00000105;
/* Property Access */
pub const TEE_PANIC_ID_TEE_ALLOCATEPROPERTYENUMERATOR: uint32_t         = 0x00000201;
pub const TEE_PANIC_ID_TEE_FREEPROPERTYENUMERATOR: uint32_t             = 0x00000202;
pub const TEE_PANIC_ID_TEE_GETNEXTPROPERTY: uint32_t                    = 0x00000203;
pub const TEE_PANIC_ID_TEE_GETPROPERTYASBINARYBLOCK: uint32_t           = 0x00000204;
pub const TEE_PANIC_ID_TEE_GETPROPERTYASBOOL: uint32_t                  = 0x00000205;
pub const TEE_PANIC_ID_TEE_GETPROPERTYASIDENTITY: uint32_t              = 0x00000206;
pub const TEE_PANIC_ID_TEE_GETPROPERTYASSTRING: uint32_t                = 0x00000207;
pub const TEE_PANIC_ID_TEE_GETPROPERTYASU32: uint32_t                   = 0x00000208;
pub const TEE_PANIC_ID_TEE_GETPROPERTYASUUID: uint32_t                  = 0x00000209;
pub const TEE_PANIC_ID_TEE_GETPROPERTYNAME: uint32_t                    = 0x0000020A;
pub const TEE_PANIC_ID_TEE_RESETPROPERTYENUMERATOR: uint32_t            = 0x0000020B;
pub const TEE_PANIC_ID_TEE_STARTPROPERTYENUMERATOR: uint32_t            = 0x0000020C;
// Panic Function
pub const TEE_PANIC_ID_TEE_PANIC: uint32_t                              = 0x00000301;
// Internal Client API
pub const TEE_PANIC_ID_TEE_CLOSETASESSION: uint32_t                     = 0x00000401;
pub const TEE_PANIC_ID_TEE_INVOKETACOMMAND: uint32_t                    = 0x00000402;
pub const TEE_PANIC_ID_TEE_OPENTASESSION: uint32_t                      = 0x00000403;
// Cancellation
pub const TEE_PANIC_ID_TEE_GETCANCELLATIONFLAG: uint32_t                = 0x00000501;
pub const TEE_PANIC_ID_TEE_MASKCANCELLATION: uint32_t                   = 0x00000502;
pub const TEE_PANIC_ID_TEE_UNMASKCANCELLATION: uint32_t                 = 0x00000503;
// Memory Management
pub const TEE_PANIC_ID_TEE_CHECKMEMORYACCESSRIGHTS: uint32_t            = 0x00000601;
pub const TEE_PANIC_ID_TEE_FREE: uint32_t                               = 0x00000602;
pub const TEE_PANIC_ID_TEE_GETINSTANCEDATA: uint32_t                    = 0x00000603;
pub const TEE_PANIC_ID_TEE_MALLOC: uint32_t                             = 0x00000604;
pub const TEE_PANIC_ID_TEE_MEMCOMPARE: uint32_t                         = 0x00000605;
pub const TEE_PANIC_ID_TEE_MEMFILL: uint32_t                            = 0x00000606;
pub const TEE_PANIC_ID_TEE_MEMMOVE: uint32_t                            = 0x00000607;
pub const TEE_PANIC_ID_TEE_REALLOC: uint32_t                            = 0x00000608;
pub const TEE_PANIC_ID_TEE_SETINSTANCEDATA: uint32_t                    = 0x00000609;
// Generic Object
pub const TEE_PANIC_ID_TEE_CLOSEOBJECT: uint32_t                        = 0x00000701;
pub const TEE_PANIC_ID_TEE_GETOBJECTBUFFERATTRIBUTE: uint32_t           = 0x00000702;
// deprecated
pub const TEE_PANIC_ID_TEE_GETOBJECTINFO: uint32_t                      = 0x00000703;
pub const TEE_PANIC_ID_TEE_GETOBJECTVALUEATTRIBUTE: uint32_t            = 0x00000704;
// deprecated
pub const TEE_PANIC_ID_TEE_RESTRICTOBJECTUSAGE: uint32_t                = 0x00000705;
pub const TEE_PANIC_ID_TEE_GETOBJECTINFO1: uint32_t                     = 0x00000706;
pub const TEE_PANIC_ID_TEE_RESTRICTOBJECTUSAGE1: uint32_t               = 0x00000707;
// Transient Object
pub const TEE_PANIC_ID_TEE_ALLOCATETRANSIENTOBJECT: uint32_t            = 0x00000801;
// deprecated
pub const TEE_PANIC_ID_TEE_COPYOBJECTATTRIBUTES: uint32_t               = 0x00000802;
pub const TEE_PANIC_ID_TEE_FREETRANSIENTOBJECT: uint32_t                = 0x00000803;
pub const TEE_PANIC_ID_TEE_GENERATEKEY: uint32_t                        = 0x00000804;
pub const TEE_PANIC_ID_TEE_INITREFATTRIBUTE: uint32_t                   = 0x00000805;
pub const TEE_PANIC_ID_TEE_INITVALUEATTRIBUTE: uint32_t                 = 0x00000806;
pub const TEE_PANIC_ID_TEE_POPULATETRANSIENTOBJECT: uint32_t            = 0x00000807;
pub const TEE_PANIC_ID_TEE_RESETTRANSIENTOBJECT: uint32_t               = 0x00000808;
pub const TEE_PANIC_ID_TEE_COPYOBJECTATTRIBUTES1: uint32_t              = 0x00000809;
// Persistent Object
// deprecated
pub const TEE_PANIC_ID_TEE_CLOSEANDDELETEPERSISTENTOBJECT: uint32_t     = 0x00000901;
pub const TEE_PANIC_ID_TEE_CREATEPERSISTENTOBJECT: uint32_t             = 0x00000902;
pub const TEE_PANIC_ID_TEE_OPENPERSISTENTOBJECT: uint32_t               = 0x00000903;
pub const TEE_PANIC_ID_TEE_RENAMEPERSISTENTOBJECT: uint32_t             = 0x00000904;
pub const TEE_PANIC_ID_TEE_CLOSEANDDELETEPERSISTENTOBJECT1: uint32_t    = 0x00000905;
// Persistent Object Enumeration
pub const TEE_PANIC_ID_TEE_ALLOCATEPERSISTENTOBJECTENUMERATOR: uint32_t = 0x00000A01;
pub const TEE_PANIC_ID_TEE_FREEPERSISTENTOBJECTENUMERATOR: uint32_t     = 0x00000A02;
pub const TEE_PANIC_ID_TEE_GETNEXTPERSISTENTOBJECT: uint32_t            = 0x00000A03;
pub const TEE_PANIC_ID_TEE_RESETPERSISTENTOBJECTENUMERATOR: uint32_t    = 0x00000A04;
pub const TEE_PANIC_ID_TEE_STARTPERSISTENTOBJECTENUMERATOR: uint32_t    = 0x00000A05;
// Data Stream Access
pub const TEE_PANIC_ID_TEE_READOBJECTDATA: uint32_t                     = 0x00000B01;
pub const TEE_PANIC_ID_TEE_SEEKOBJECTDATA: uint32_t                     = 0x00000B02;
pub const TEE_PANIC_ID_TEE_TRUNCATEOBJECTDATA: uint32_t                 = 0x00000B03;
pub const TEE_PANIC_ID_TEE_WRITEOBJECTDATA: uint32_t                    = 0x00000B04;
// Generic Operation
pub const TEE_PANIC_ID_TEE_ALLOCATEOPERATION: uint32_t                  = 0x00000C01;
pub const TEE_PANIC_ID_TEE_COPYOPERATION: uint32_t                      = 0x00000C02;
pub const TEE_PANIC_ID_TEE_FREEOPERATION: uint32_t                      = 0x00000C03;
pub const TEE_PANIC_ID_TEE_GETOPERATIONINFO: uint32_t                   = 0x00000C04;
pub const TEE_PANIC_ID_TEE_RESETOPERATION: uint32_t                     = 0x00000C05;
pub const TEE_PANIC_ID_TEE_SETOPERATIONKEY: uint32_t                    = 0x00000C06;
pub const TEE_PANIC_ID_TEE_SETOPERATIONKEY2: uint32_t                   = 0x00000C07;
pub const TEE_PANIC_ID_TEE_GETOPERATIONINFOMULTIPLE: uint32_t           = 0x00000C08;
// Message Digest
pub const TEE_PANIC_ID_TEE_DIGESTDOFINAL: uint32_t                      = 0x00000D01;
pub const TEE_PANIC_ID_TEE_DIGESTUPDATE: uint32_t                       = 0x00000D02;
// Symmetric Cipher
pub const TEE_PANIC_ID_TEE_CIPHERDOFINAL: uint32_t                      = 0x00000E01;
pub const TEE_PANIC_ID_TEE_CIPHERINIT: uint32_t                         = 0x00000E02;
pub const TEE_PANIC_ID_TEE_CIPHERUPDATE: uint32_t                       = 0x00000E03;
// MAC
pub const TEE_PANIC_ID_TEE_MACCOMPAREFINAL: uint32_t                    = 0x00000F01;
pub const TEE_PANIC_ID_TEE_MACCOMPUTEFINAL: uint32_t                    = 0x00000F02;
pub const TEE_PANIC_ID_TEE_MACINIT: uint32_t                            = 0x00000F03;
pub const TEE_PANIC_ID_TEE_MACUPDATE: uint32_t                          = 0x00000F04;
// Authenticated Encryption
pub const TEE_PANIC_ID_TEE_AEDECRYPTFINAL: uint32_t                     = 0x00001001;
pub const TEE_PANIC_ID_TEE_AEENCRYPTFINAL: uint32_t                     = 0x00001002;
pub const TEE_PANIC_ID_TEE_AEINIT: uint32_t                             = 0x00001003;
pub const TEE_PANIC_ID_TEE_AEUPDATE: uint32_t                           = 0x00001004;
pub const TEE_PANIC_ID_TEE_AEUPDATEAAD: uint32_t                        = 0x00001005;
// Asymmetric
pub const TEE_PANIC_ID_TEE_ASYMMETRICDECRYPT: uint32_t                  = 0x00001101;
pub const TEE_PANIC_ID_TEE_ASYMMETRICENCRYPT: uint32_t                  = 0x00001102;
pub const TEE_PANIC_ID_TEE_ASYMMETRICSIGNDIGEST: uint32_t               = 0x00001103;
pub const TEE_PANIC_ID_TEE_ASYMMETRICVERIFYDIGEST: uint32_t             = 0x00001104;
// Key Derivation
pub const TEE_PANIC_ID_TEE_DERIVEKEY: uint32_t                          = 0x00001201;
// Random Data Generation
pub const TEE_PANIC_ID_TEE_GENERATERANDOM: uint32_t                     = 0x00001301;
// Time
pub const TEE_PANIC_ID_TEE_GETREETIME: uint32_t                         = 0x00001401;
pub const TEE_PANIC_ID_TEE_GETSYSTEMTIME: uint32_t                      = 0x00001402;
pub const TEE_PANIC_ID_TEE_GETTAPERSISTENTTIME: uint32_t                = 0x00001403;
pub const TEE_PANIC_ID_TEE_SETTAPERSISTENTTIME: uint32_t                = 0x00001404;
pub const TEE_PANIC_ID_TEE_WAIT: uint32_t                               = 0x00001405;
// Memory Allocation and Size of Objects
pub const TEE_PANIC_ID_TEE_BIGINTFMMCONTEXTSIZEINU32: uint32_t          = 0x00001501;
pub const TEE_PANIC_ID_TEE_BIGINTFMMSIZEINU32: uint32_t                 = 0x00001502;
// Initialization
pub const TEE_PANIC_ID_TEE_BIGINTINIT: uint32_t                         = 0x00001601;
pub const TEE_PANIC_ID_TEE_BIGINTINITFMM: uint32_t                      = 0x00001602;
pub const TEE_PANIC_ID_TEE_BIGINTINITFMMCONTEXT: uint32_t               = 0x00001603;
// Converter
pub const TEE_PANIC_ID_TEE_BIGINTCONVERTFROMOCTETSTRING: uint32_t       = 0x00001701;
pub const TEE_PANIC_ID_TEE_BIGINTCONVERTFROMS32: uint32_t               = 0x00001702;
pub const TEE_PANIC_ID_TEE_BIGINTCONVERTTOOCTETSTRING: uint32_t         = 0x00001703;
pub const TEE_PANIC_ID_TEE_BIGINTCONVERTTOS32: uint32_t                 = 0x00001704;
// Logical Operation
pub const TEE_PANIC_ID_TEE_BIGINTCMP: uint32_t                          = 0x00001801;
pub const TEE_PANIC_ID_TEE_BIGINTCMPS32: uint32_t                       = 0x00001802;
pub const TEE_PANIC_ID_TEE_BIGINTGETBIT: uint32_t                       = 0x00001803;
pub const TEE_PANIC_ID_TEE_BIGINTGETBITCOUNT: uint32_t                  = 0x00001804;
pub const TEE_PANIC_ID_TEE_BIGINTSHIFTRIGHT: uint32_t                   = 0x00001805;
// Basic Arithmetic
pub const TEE_PANIC_ID_TEE_BIGINTADD: uint32_t                          = 0x00001901;
pub const TEE_PANIC_ID_TEE_BIGINTDIV: uint32_t                          = 0x00001902;
pub const TEE_PANIC_ID_TEE_BIGINTMUL: uint32_t                          = 0x00001903;
pub const TEE_PANIC_ID_TEE_BIGINTNEG: uint32_t                          = 0x00001904;
pub const TEE_PANIC_ID_TEE_BIGINTSQUARE: uint32_t                       = 0x00001905;
pub const TEE_PANIC_ID_TEE_BIGINTSUB: uint32_t                          = 0x00001906;
// Modular Arithmetic
pub const TEE_PANIC_ID_TEE_BIGINTADDMOD: uint32_t                       = 0x00001A01;
pub const TEE_PANIC_ID_TEE_BIGINTINVMOD: uint32_t                       = 0x00001A02;
pub const TEE_PANIC_ID_TEE_BIGINTMOD: uint32_t                          = 0x00001A03;
pub const TEE_PANIC_ID_TEE_BIGINTMULMOD: uint32_t                       = 0x00001A04;
pub const TEE_PANIC_ID_TEE_BIGINTSQUAREMOD: uint32_t                    = 0x00001A05;
pub const TEE_PANIC_ID_TEE_BIGINTSUBMOD: uint32_t                       = 0x00001A06;
// Other Arithmetic
pub const TEE_PANIC_ID_TEE_BIGINTCOMPUTEEXTENDEDGCD: uint32_t           = 0x00001B01;
pub const TEE_PANIC_ID_TEE_BIGINTISPROBABLEPRIME: uint32_t              = 0x00001B02;
pub const TEE_PANIC_ID_TEE_BIGINTRELATIVEPRIME: uint32_t                = 0x00001B03;
// Fast Modular Multiplication
pub const TEE_PANIC_ID_TEE_BIGINTCOMPUTEFMM: uint32_t                   = 0x00001C01;
pub const TEE_PANIC_ID_TEE_BIGINTCONVERTFROMFMM: uint32_t               = 0x00001C02;
pub const TEE_PANIC_ID_TEE_BIGINTCONVERTTOFMM: uint32_t                 = 0x00001C03;

pub const TEE_NUM_PARAMS: uint32_t = 4;
