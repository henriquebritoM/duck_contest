use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Duck {
    name: [char; 32],
    race: [char; 32],
    habilities: Vec<String>
}

impl Duck {

    pub fn try_desserialize(str: String) -> Result<Duck, ()> {

        let tmp: Result<Duck, serde_json::Error> = serde_json::from_str(&str);
        
        return match tmp {
            Ok(r) => Ok(r),
            Err(_) => Err(()),
        };
    }

    pub fn try_serialize(duck: Duck) -> Result<String, ()> {

        let tmp:Result<String, serde_json::Error> = serde_json::to_string(&duck);
        
        return match tmp {
            Ok(r) => Ok(r),
            Err(_) => Err(()),
        };
    }
}