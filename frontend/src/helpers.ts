export async function loadJSON(url: any) {
    let data = await fetch(`http://127.0.0.1:8080/sg/${url}`);
    return await data.json();
}
