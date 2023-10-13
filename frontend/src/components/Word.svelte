<script lang="ts">
    export let word: string;

    function showWordMenu(this) {
        let options = ["Select", "Add Morphemes", "Anagrams"];
    }
</script>

<span class="bg-red" on:hover=showWordMenu>
    <Slot />
</span>
