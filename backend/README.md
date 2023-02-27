This is the readme for the backend of this project.  

This part of the project is basically an API written in Rust using the
[Actix](https://github.com/actix/actix-web) framework.  

Currently, the API only supports using SimpleGenerator, which is, as the name
implies, a simple structure that allows for shallow word/text generation, based
on a category / syllable pattern system.  

## API endpoints

If running locally, a ```.env``` file specifying a ```SERVER_ADDR``` should be
used.  

```GET /sg/generators```  
Returns the currently loaded SimpleGenerators.  

```GET /sg/settings```  
Returns the settings for the specified generator. Requires a ```generator``` to 
be passed in as a query param.  

```POST /sg/save```  
Updates the settings for an existing (loaded) generator or creates a new one.
Requires a valid JSON body with the desired settings, e.g.:  
```json
{
    "name": "default-settings",
    "categories": {
        "F":["n","r"],
        "C":["p","t","k","s","m","n","h","w"],
        "V":["i","a","u"]
    },
    "patterns": ["CV","CVF"]
}
```

```GET /sg/randtext```  
returns randomly generated text based on the specified generator. Requires a
```generator```, ```min``` and ```max``` syllable lengths and a ```text_length``` 
to be passed as query params.  
