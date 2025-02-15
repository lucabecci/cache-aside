pub struct Event {
    pub id: String,
    pub name: String,
    pub version: u64,
}


 impl Event {
    pub fn new(id: String, name: String, version: u64) -> Self {}

     pub fn validate_version(&self, version: u64) -> bool {
         self.version == version
     }
 }