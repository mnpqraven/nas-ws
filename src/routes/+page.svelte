<script>
	import { tParseMdx } from '../type_registry';
</script>

<h1>API</h1>
<div>
	<p>path: <code>{tParseMdx.path}</code></p>
	<p>methods: <code>{tParseMdx.methods.join()}</code></p>
	<p>description: <code>{tParseMdx.path}</code></p>
	<p>input:</p>
	<pre><code>{tParseMdx.input}</code></pre>
	<p>output:</p>
	<pre><code>{tParseMdx.output}</code></pre>
</div>

<style>
	div {
		width: fit-content;
		border: 1px solid pink;
	}
</style>
