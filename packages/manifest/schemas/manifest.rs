use serde :: { Serialize , Deserialize } ; use serde_json :: Value ; use std :: collections :: BTreeMap ; # [doc = "Generated from schemas/0.1.0.json"] # [derive (Clone , PartialEq , Debug , Deserialize , Serialize)] pub struct Unknown { # [serde (skip_serializing_if = "Option::is_none")] pub build : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub deploy : Option < String > , pub format : String , # [serde (skip_serializing_if = "Option::is_none")] pub import_redirects : Option < Vec < Unknown1 >> , pub language : String , # [serde (skip_serializing_if = "Option::is_none")] pub meta : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub module : Option < String > , pub name : String , pub schema : String } # [doc = "Generated from schemas/0.1.0.json/properties/import_redirects/items"] # [derive (Clone , PartialEq , Debug , Deserialize , Serialize)] pub struct Unknown1 { pub schema : String , pub uri : String }