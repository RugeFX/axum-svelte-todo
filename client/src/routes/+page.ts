// TODO: give the correct types
export const load = async ({ fetch }: { fetch: typeof globalThis.fetch }) => {
	const response = await fetch(
		"http://localhost:3000"
	)

	const user = await response.json()
	return user
}
