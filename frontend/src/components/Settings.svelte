<script>
    import Category from 'Category.svelte';
    import Pattern from 'Pattern.svelte';

    let hidden = false;

    function addCategory() {
        
    }
</script>

{#if !hidden}
<div id="settingsDisplay">
    <h2>Settings</h2>
    <div id="settingsContents">
        
    <h3>Categories</h3>
    {#each data.categories as cat}
        <Category {cat.elements} {cat.symbol} />
    {/each}
    <button type='submit' on:click={addCategory}>Add new category</button>

    <h3>Patterns</h3>
    {#each data.patterns as pat}
        <Pattern />
    {/each}
</div>
{/if}

