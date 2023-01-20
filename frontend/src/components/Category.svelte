<script lang="ts">
    export let elements: String;
    export let symbol: String;
</script>

<div>
    <input class="bg-bg2 text-center max-w-xs ml-2 p-2 text-fg" type="text" maxlength="1" value={symbol} />
    <input class="bg-bg2 text-center ml-2 p-2 text-fg" type="text" value={String(elements).replace(/,/g, " ")} />
    <button class="bg-bg2 basis-1/4 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit">X</button>
</div>

