<script>
    export let elements;
    export let symbol;
</script>

<div>
    <input type="text" maxlength="1" value={symbol} />
    <span>=</span>
    <input type="text" value={elements} />
    <button type="submit">Remove</button>
</div>

