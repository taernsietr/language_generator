<script lang="typescript">
    function toggleInfo() { console.log("debug") }
    function toggleSettings() { console.log("debug") }
</script>

<div class="bg-bg1 flex flex-1 flex-col m-4 p-4 place-content-center shadow-xl text-fg">
    <button class="bg-bg2 flex m-4 p-4 place-content-center text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={toggleInfo}>Toggle Info</button>
    <button class="bg-bg2 flex m-4 p-4 place-content-center text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={toggleSettings}>Toggle Settings</button>
</div>
