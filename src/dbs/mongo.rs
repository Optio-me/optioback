use mongodb::{Client, options::ClientOptions, Database};

pub struct Connect {
    client: Client,
    pub dbref: Database,
}

#[allow(dead_code)]
impl Connect {
    pub async fn client(&self) -> Client {
        self.client.clone()
    }

    pub async fn init() -> Connect {

        //Define options for connection
        let mut client_options =
            ClientOptions::parse("...") // Place auth here for database
                .await
                .expect("Ooops, error parsing options...");
        
        client_options.app_name = Some("rsnapp".to_string());
       
        //Initialise connection
        let client = Client::with_options(client_options)
            .expect("Unable to initialise database!");

        let dbref = client.database("rsnapp"); //Set database name
        Connect { client: client, dbref: dbref } //Finalise!
    }
}
