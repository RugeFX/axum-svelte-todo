<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;

	$: console.log(data.todos);
</script>

<h1>Welcome to Todos</h1>
<pre>{JSON.stringify(data, null, 2)}</pre>
