<script>
	/** @type {boolean} */
	export let dropShadow = true;

	/** @type {boolean} */
	export let rounded = true;
</script>

<article class="bg-white p-4 {rounded ? 'rounded-md' : ''} {dropShadow ? 'drop-shadow-md' : ''}">
	<header>
		<slot name="header" />
	</header>

	<div>
		<slot name="main" />
	</div>

	<footer>
		<slot name="footer" />
	</footer>
</article>
