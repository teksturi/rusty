TYPE {nature_bit}       BYTE   : UINT8;  END_TYPE
TYPE {nature_signed}    SINT   : INT8;   END_TYPE
TYPE {nature_unsigned}  USINT  : UINT8;  END_TYPE

TYPE {nature_bit}       WORD   : UINT16; END_TYPE
TYPE {nature_signed}    INT    : INT16;  END_TYPE
TYPE {nature_unsigned}  UINT   : UINT16; END_TYPE

TYPE {nature_bit}       DWORD   : UINT32; END_TYPE
TYPE {nature_signed}    DINT    : INT32;  END_TYPE
TYPE {nature_unsigned}  UDINT   : UINT32; END_TYPE

TYPE {nature_bit}       LWORD   : UINT64; END_TYPE
TYPE {nature_signed}    LINT    : INT64;  END_TYPE
TYPE {nature_unsigned}  ULINT   : UINT64; END_TYPE

TYPE {nature_date}     DATE   : INT64; END_TYPE
TYPE {nature_date}      LDATE  : DATE;  END_TYPE
TYPE {nature_date}      D      : DATE;  END_TYPE
TYPE {nature_date}      LD     : DATE; END_TYPE

TYPE {nature_duration}  TIME   : INT64; END_TYPE
TYPE {nature_duration}  LTIME  : TIME;  END_TYPE
TYPE {nature_duration}  T      : TIME;  END_TYPE
TYPE {nature_duration}  LT     : TIME; END_TYPE

TYPE {nature_date}      DATE_AND_TIME   : INT64;         END_TYPE
TYPE {nature_date}      LDATE_AND_TIME  : DATE_AND_TIME; END_TYPE
TYPE {nature_date}      DT              : DATE_AND_TIME; END_TYPE
TYPE {nature_date}      LDT             : DATE_AND_TIME; END_TYPE

TYPE {nature_date}      TIME_OF_DAY     : INT64;         END_TYPE
TYPE {nature_date}      LTIME_OF_DAY    : TIME_OF_DAY;   END_TYPE
TYPE {nature_date}      TOD             : TIME_OF_DAY;   END_TYPE
TYPE {nature_date}      LTOD            : TIME_OF_DAY;   END_TYPE

TYPE {nature_real}      REAL   : REAL32;   END_TYPE
TYPE {nature_real}      LREAL  : REAL64;   END_TYPE