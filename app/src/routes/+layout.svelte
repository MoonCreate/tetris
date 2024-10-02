<script>
    import "../app.css"
    import initWasm from "core_heart";
    const { children } = $props();
</script>

{#await initWasm()}
    <div>Please wait patiently....</div>
{:then _} 
    {@render children()}
{/await}