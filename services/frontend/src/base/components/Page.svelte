<script>
    export let title;
    export let cmp;
    export let cmpProps;
</script>

<svelte:head>
	<title>{title || 'FTrees'}</title>
</svelte:head>

<div class="page">
    <svelte:component this={cmp} this.props={cmpProps || {}}/>
</div>
