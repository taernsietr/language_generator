<script lang="ts">
    export let elements: string;
    export let symbol: string;
    export let id: string;

    function destroyCategory() {
        let thisObject = document.getElementById(id);
        thisObject ? thisObject.remove() : console.log("[Error]: Category element not found"); 
    }
</script>

<div class="grid grid-flow-row grid-cols-10 m-4 p-4" id={id}>
    <input class="bg-bg2 col-span-2 text-center ml-2 p-2 text-fg" type="text" maxlength="1" value={symbol} />
    <input class="bg-bg2 col-span-7 text-center ml-2 p-2 text-fg" type="text" value={String(elements).replace(/,/g, " ")} />
    <button class="bg-bg2 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={destroyCategory}>X</button>
</div>

