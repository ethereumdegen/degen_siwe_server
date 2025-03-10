# Degen SIWE Server 


A sign-in-with-ethereum backend microservice 



## DevOps 

1. Deploy this to DigitalOcean App Platform directly.  Make sure port 8080 is being used to serve the web traffic and make sure the ENV variables are set (see the template).   



### How it works 

- Two tables are needed in a supabase database (db_conn_url is the database connection url ENV variable).   Create these tables in the database using the db migrations scripts provided in this repo.  

- Use the two endpoints in session_controller.rs to generate and validate web3 challenges from a wallet like metamask.  Auth session tokens will be created and stored in the database upon successful login (signing) .  You can use the created_at timestamp in the user session to handle expiry yourself. 