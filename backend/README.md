This is the readme for the backend of this project.  

This part of the project is basically an API written in Rust using the
[Actix](https://github.com/actix/actix-web) framework, which interacts with the
[Angelspeech](https://github.com/taernsietr/angelspeech) library, also written
by me.

Currently, the API only supports using `TextGenerator`, which is, as the name
implies, a simple structure that allows for shallow word/text generation, based
on a category / syllable pattern system.  

## API endpoints

If running locally, a `.env` file specifying a `SERVER_ADDR` should be used.  

```GET /generators```  Returns the currently loaded TextGenerators.  

```GET /generators/settings```  Returns the settings for the specified
generator. Requires a `generator` to be passed in as a query param.  

```POST /generators/save```  Updates the settings for an existing (loaded)
generator or creates a new one.  Requires a valid JSON body with the desired
settings, e.g.:  ```json { "name": "default-settings", "categories":
{ "F":["n","r"], "C":["p","t","k","s","m","n","h","w"], "V":["i","a","u"] },
"patterns": [ ["CV", "Any", "Default"], ["CVF", "Any", "Default"] ] } ```

```GET /generators/text```  returns randomly generated text based on the
specified generator. Requires a `generator`, `min` and `max` syllable lengths
and a `text_length` to be passed as query params.  

## Running the project as-is If you want to test the project, the backend
server can be run with `cargo run` (assuming you have
[Rust](https://www.rust-lang.org) installed).
