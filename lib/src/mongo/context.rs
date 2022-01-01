use mongodb::sync::ClientSession;


pub struct MongoContext {
    pub session: ClientSession
}

impl MongoContext {
    pub fn new(session: ClientSession) -> MongoContext {
        MongoContext {
            session: session,
        }
    }

    pub fn commit(&mut self) -> Result<(), ()> {
        self.close_transaction(false)
    }
    
    pub fn abort(&mut self) -> Result<(), ()> {
        self.close_transaction(true)
    }

    fn close_transaction(&mut self, abort: bool) -> Result<(), ()> {
        if abort {
            if let Err(_) = self.session.abort_transaction() {
                return Err(())
            }
        } 
        else if let Err(_) = self.session.commit_transaction() {
            return Err(())
        }
        return Ok(())
    }
}