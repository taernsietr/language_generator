<script lang="ts">  
    import { categories, queuedCategories } from '../store';

    import Category from './Category.svelte';
    import Button from './Button.svelte';

    function addCategory() {
        let temp = $categories;
        temp.push(["", ""]);
        categories.set(temp);
        queuedCategories.set(temp);
    }
    
</script>

<div class="flex flex-col m-2 p-2 place-content-center">
    <h3 class="text-center text-blue">Categories</h3>
    <Button fn={addCategory} label={"Add new category"} />
    {#each $categories as cat}
        <Category 
            symbol = { cat[0] }
            elements = { cat[1] } 
            id = { $categories.indexOf(cat) }
        />
    {/each}
</div>

