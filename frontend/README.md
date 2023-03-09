This is the readme for the frontend of this project.  

This part of the project is a frontend written in JavaScript using the
[Svelte](https://github.com/sveltejs/svelte) framework.  

It is being developed around the Rust API, so capabilities of the application as
a whole will tend to be added to the API before they can be accessed via this
frontend.  

Overall, I think the design should be pretty self-explanatory, and there is an
information button on the page itself meant to guide users if they aren't sure
how to use it. Nevertheless, I will add clearer instructions on this readme at a
later date.  

## Running the project as-is 
If you want to test the project, the frontend server can be run with one of the
following commands (assuming you have [Node](https://nodejs.org) installed):  

```bash
npm run dev 
npm run host # this exposes the server to the local network, use carefully!
```
