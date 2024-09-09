<script lang="ts">
	import type { ActionData, PageData } from './$types';

	export let data: PageData;
	export let form: ActionData;

	$: console.log(data.todos);
</script>

<main class="container mx-auto space-y-10 py-6">
	<h1 class="text-3xl font-bold">Welcome to Todos</h1>
	<form method="POST" class="space-y-2">
		<fieldset class="w-full">
			<label class="flex flex-col gap-1 w-full">
				Title
				<input
					class="p-2 resize-y bg-zinc-900 border border-zinc-700 rounded-md w-full"
					name="title"
					type="text"
				/>
			</label>
		</fieldset>

		<fieldset class="w-full">
			<label class="flex flex-col gap-1 w-full">
				Body
				<textarea
					class="p-2 resize-y bg-zinc-900 border border-zinc-700 rounded-md w-full"
					name="body"
				></textarea>
			</label>
		</fieldset>

		<div class="flex w-full justify-end items-center mt-2">
			<button
				class="px-4 py-2 bg-orange-500 text-white font-semibold rounded-md text-lg hover:bg-orange-600"
				type="submit">Create Todo</button
			>
		</div>
		{#if form?.status === 'success'}
			<p>Added your todo bro</p>
		{/if}
	</form>

	<h2 class="text-2xl font-bold">Todo List</h2>
	<ul
		class="w-full grid rounded-md overflow-hidden border border-zinc-700 divide-y divide-zinc-600"
	>
		{#each data.todos as { id, title }}
			<li class="text-lg font-semibold bg-zinc-900 p-4">
				<a class="hover:underline" href={`/details/${id}`}
					>{title || 'bruh you bypassed our validation step kudos to u'}</a
				>
			</li>
		{/each}
	</ul>
</main>
