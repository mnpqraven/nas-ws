<script>
const tParseMdx = {
  path: 'utils/parse_mdx',
  methods: ['POST'],
  description: 'Parse a mdx file decoded in base64',
  input: '{\n  test: string\n}',
  output: '{\n  test: string\n}'
}
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
